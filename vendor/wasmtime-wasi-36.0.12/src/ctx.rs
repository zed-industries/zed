use crate::cli::{StdinStream, StdoutStream, WasiCliCtx};
use crate::clocks::{HostMonotonicClock, HostWallClock, WasiClocksCtx};
use crate::p2::filesystem::Dir;
use crate::random::WasiRandomCtx;
use crate::sockets::{SocketAddrCheck, SocketAddrUse, WasiSocketsCtx};
use crate::{DirPerms, FilePerms, OpenMode};
use anyhow::Result;
use cap_rand::RngCore;
use cap_std::ambient_authority;
use std::future::Future;
use std::mem;
use std::net::SocketAddr;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::{stderr, stdin, stdout};

/// Builder-style structure used to create a [`WasiCtx`].
///
/// This type is used to create a [`WasiCtx`] that is considered per-[`Store`]
/// state. The [`build`][WasiCtxBuilder::build] method is used to finish the
/// building process and produce a finalized [`WasiCtx`].
///
/// # Examples
///
/// ```
/// use wasmtime_wasi::WasiCtx;
///
/// let mut wasi = WasiCtx::builder();
/// wasi.arg("./foo.wasm");
/// wasi.arg("--help");
/// wasi.env("FOO", "bar");
///
/// let wasi: WasiCtx = wasi.build();
/// ```
///
/// [`Store`]: wasmtime::Store
#[derive(Default)]
pub struct WasiCtxBuilder {
    cli: WasiCliCtx,
    clocks: WasiClocksCtx,
    random: WasiRandomCtx,
    sockets: WasiSocketsCtx,
    allow_blocking_current_thread: bool,
    preopens: Vec<(Dir, String)>,
    built: bool,
}

impl WasiCtxBuilder {
    /// Creates a builder for a new context with default parameters set.
    ///
    /// The current defaults are:
    ///
    /// * stdin is closed
    /// * stdout and stderr eat all input and it doesn't go anywhere
    /// * no env vars
    /// * no arguments
    /// * no preopens
    /// * clocks use the host implementation of wall/monotonic clocks
    /// * RNGs are all initialized with random state and suitable generator
    ///   quality to satisfy the requirements of WASI APIs.
    /// * TCP/UDP are allowed but all addresses are denied by default.
    /// * `wasi:sockets/ip-name-lookup` is denied by default.
    ///
    /// These defaults can all be updated via the various builder configuration
    /// methods below.
    pub fn new() -> Self {
        Self::default()
    }

    /// Provides a custom implementation of stdin to use.
    ///
    /// By default stdin is closed but an example of using the host's native
    /// stdin looks like:
    ///
    /// ```
    /// use wasmtime_wasi::WasiCtx;
    /// use wasmtime_wasi::cli::stdin;
    ///
    /// let mut wasi = WasiCtx::builder();
    /// wasi.stdin(stdin());
    /// ```
    ///
    /// Note that inheriting the process's stdin can also be done through
    /// [`inherit_stdin`](WasiCtxBuilder::inherit_stdin).
    pub fn stdin(&mut self, stdin: impl StdinStream + 'static) -> &mut Self {
        self.cli.stdin = Box::new(stdin);
        self
    }

    /// Same as [`stdin`](WasiCtxBuilder::stdin), but for stdout.
    pub fn stdout(&mut self, stdout: impl StdoutStream + 'static) -> &mut Self {
        self.cli.stdout = Box::new(stdout);
        self
    }

    /// Same as [`stdin`](WasiCtxBuilder::stdin), but for stderr.
    pub fn stderr(&mut self, stderr: impl StdoutStream + 'static) -> &mut Self {
        self.cli.stderr = Box::new(stderr);
        self
    }

    /// Configures this context's stdin stream to read the host process's
    /// stdin.
    ///
    /// Note that concurrent reads of stdin can produce surprising results so
    /// when using this it's typically best to have a single wasm instance in
    /// the process using this.
    pub fn inherit_stdin(&mut self) -> &mut Self {
        self.stdin(stdin())
    }

    /// Configures this context's stdout stream to write to the host process's
    /// stdout.
    ///
    /// Note that unlike [`inherit_stdin`](WasiCtxBuilder::inherit_stdin)
    /// multiple instances printing to stdout works well.
    pub fn inherit_stdout(&mut self) -> &mut Self {
        self.stdout(stdout())
    }

    /// Configures this context's stderr stream to write to the host process's
    /// stderr.
    ///
    /// Note that unlike [`inherit_stdin`](WasiCtxBuilder::inherit_stdin)
    /// multiple instances printing to stderr works well.
    pub fn inherit_stderr(&mut self) -> &mut Self {
        self.stderr(stderr())
    }

    /// Configures all of stdin, stdout, and stderr to be inherited from the
    /// host process.
    ///
    /// See [`inherit_stdin`](WasiCtxBuilder::inherit_stdin) for some rationale
    /// on why this should only be done in situations of
    /// one-instance-per-process.
    pub fn inherit_stdio(&mut self) -> &mut Self {
        self.inherit_stdin().inherit_stdout().inherit_stderr()
    }

    /// Configures whether or not blocking operations made through this
    /// `WasiCtx` are allowed to block the current thread.
    ///
    /// WASI is currently implemented on top of the Rust
    /// [Tokio](https://tokio.rs/) library. While most WASI APIs are
    /// non-blocking some are instead blocking from the perspective of
    /// WebAssembly. For example opening a file is a blocking operation with
    /// respect to WebAssembly but it's implemented as an asynchronous operation
    /// on the host. This is currently done with Tokio's
    /// [`spawn_blocking`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html).
    ///
    /// When WebAssembly is used in a synchronous context, for example when
    /// [`Config::async_support`] is disabled, then this asynchronous operation
    /// is quickly turned back into a synchronous operation with a `block_on` in
    /// Rust. This switching back-and-forth between a blocking a non-blocking
    /// context can have overhead, and this option exists to help alleviate this
    /// overhead.
    ///
    /// This option indicates that for WASI functions that are blocking from the
    /// perspective of WebAssembly it's ok to block the native thread as well.
    /// This means that this back-and-forth between async and sync won't happen
    /// and instead blocking operations are performed on-thread (such as opening
    /// a file). This can improve the performance of WASI operations when async
    /// support is disabled.
    ///
    /// [`Config::async_support`]: https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html#method.async_support
    pub fn allow_blocking_current_thread(&mut self, enable: bool) -> &mut Self {
        self.allow_blocking_current_thread = enable;
        self
    }

    /// Appends multiple environment variables at once for this builder.
    ///
    /// All environment variables are appended to the list of environment
    /// variables that this builder will configure.
    ///
    /// At this time environment variables are not deduplicated and if the same
    /// key is set twice then the guest will see two entries for the same key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasmtime_wasi::WasiCtxBuilder;
    ///
    /// let mut wasi = WasiCtxBuilder::new();
    /// wasi.envs(&[
    ///     ("FOO", "bar"),
    ///     ("HOME", "/somewhere"),
    /// ]);
    /// ```
    pub fn envs(&mut self, env: &[(impl AsRef<str>, impl AsRef<str>)]) -> &mut Self {
        self.cli.environment.extend(
            env.iter()
                .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().to_owned())),
        );
        self
    }

    /// Appends a single environment variable for this builder.
    ///
    /// At this time environment variables are not deduplicated and if the same
    /// key is set twice then the guest will see two entries for the same key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasmtime_wasi::WasiCtxBuilder;
    ///
    /// let mut wasi = WasiCtxBuilder::new();
    /// wasi.env("FOO", "bar");
    /// ```
    pub fn env(&mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> &mut Self {
        self.cli
            .environment
            .push((k.as_ref().to_owned(), v.as_ref().to_owned()));
        self
    }

    /// Configures all environment variables to be inherited from the calling
    /// process into this configuration.
    ///
    /// This will use [`envs`](WasiCtxBuilder::envs) to append all host-defined
    /// environment variables.
    pub fn inherit_env(&mut self) -> &mut Self {
        self.envs(&std::env::vars().collect::<Vec<(String, String)>>())
    }

    /// Appends a list of arguments to the argument array to pass to wasm.
    pub fn args(&mut self, args: &[impl AsRef<str>]) -> &mut Self {
        self.cli
            .arguments
            .extend(args.iter().map(|a| a.as_ref().to_owned()));
        self
    }

    /// Appends a single argument to get passed to wasm.
    pub fn arg(&mut self, arg: impl AsRef<str>) -> &mut Self {
        self.cli.arguments.push(arg.as_ref().to_owned());
        self
    }

    /// Appends all host process arguments to the list of arguments to get
    /// passed to wasm.
    pub fn inherit_args(&mut self) -> &mut Self {
        self.args(&std::env::args().collect::<Vec<String>>())
    }

    /// Configures a "preopened directory" to be available to WebAssembly.
    ///
    /// By default WebAssembly does not have access to the filesystem because
    /// there are no preopened directories. All filesystem operations, such as
    /// opening a file, are done through a preexisting handle. This means that
    /// to provide WebAssembly access to a directory it must be configured
    /// through this API.
    ///
    /// WASI will also prevent access outside of files provided here. For
    /// example `..` can't be used to traverse up from the `host_path` provided here
    /// to the containing directory.
    ///
    /// * `host_path` - a path to a directory on the host to open and make
    ///   accessible to WebAssembly. Note that the name of this directory in the
    ///   guest is configured with `guest_path` below.
    /// * `guest_path` - the name of the preopened directory from WebAssembly's
    ///   perspective. Note that this does not need to match the host's name for
    ///   the directory.
    /// * `dir_perms` - this is the permissions that wasm will have to operate on
    ///   `guest_path`. This can be used, for example, to provide readonly access to a
    ///   directory.
    /// * `file_perms` - similar to `dir_perms` but corresponds to the maximum set
    ///   of permissions that can be used for any file in this directory.
    ///
    /// # Errors
    ///
    /// This method will return an error if `host_path` cannot be opened.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasmtime_wasi::WasiCtxBuilder;
    /// use wasmtime_wasi::{DirPerms, FilePerms};
    ///
    /// # fn main() {}
    /// # fn foo() -> wasmtime::Result<()> {
    /// let mut wasi = WasiCtxBuilder::new();
    ///
    /// // Make `./host-directory` available in the guest as `.`
    /// wasi.preopened_dir("./host-directory", ".", DirPerms::all(), FilePerms::all());
    ///
    /// // Make `./readonly` available in the guest as `./ro`
    /// wasi.preopened_dir("./readonly", "./ro", DirPerms::READ, FilePerms::READ);
    /// # Ok(())
    /// # }
    /// ```
    pub fn preopened_dir(
        &mut self,
        host_path: impl AsRef<Path>,
        guest_path: impl AsRef<str>,
        dir_perms: DirPerms,
        file_perms: FilePerms,
    ) -> Result<&mut Self> {
        let dir = cap_std::fs::Dir::open_ambient_dir(host_path.as_ref(), ambient_authority())?;
        let mut open_mode = OpenMode::empty();
        if dir_perms.contains(DirPerms::READ) {
            open_mode |= OpenMode::READ;
        }
        if dir_perms.contains(DirPerms::MUTATE) {
            open_mode |= OpenMode::WRITE;
        }
        self.preopens.push((
            Dir::new(
                dir,
                dir_perms,
                file_perms,
                open_mode,
                self.allow_blocking_current_thread,
            ),
            guest_path.as_ref().to_owned(),
        ));
        Ok(self)
    }

    /// Set the generator for the `wasi:random/random` number generator to the
    /// custom generator specified.
    ///
    /// Note that contexts have a default RNG configured which is a suitable
    /// generator for WASI and is configured with a random seed per-context.
    ///
    /// Guest code may rely on this random number generator to produce fresh
    /// unpredictable random data in order to maintain its security invariants,
    /// and ideally should use the insecure random API otherwise, so using any
    /// prerecorded or otherwise predictable data may compromise security.
    pub fn secure_random(&mut self, random: impl RngCore + Send + 'static) -> &mut Self {
        self.random.random = Box::new(random);
        self
    }

    /// Configures the generator for `wasi:random/insecure`.
    ///
    /// The `insecure_random` generator provided will be used for all randomness
    /// requested by the `wasi:random/insecure` interface.
    pub fn insecure_random(&mut self, insecure_random: impl RngCore + Send + 'static) -> &mut Self {
        self.random.insecure_random = Box::new(insecure_random);
        self
    }

    /// Configures the seed to be returned from `wasi:random/insecure-seed` to
    /// the specified custom value.
    ///
    /// By default this number is randomly generated when a builder is created.
    pub fn insecure_random_seed(&mut self, insecure_random_seed: u128) -> &mut Self {
        self.random.insecure_random_seed = insecure_random_seed;
        self
    }

    /// Configures the maximum len accepted by
    /// `wasi:random/random.get-random-bytes` and
    /// `wasi:random/insecure.get-insecure-random-bytes`. Calls with a len
    /// larger than this limit will trap.
    ///
    /// This limit protects the host implementation from memory exhaustion from
    /// untrusted guest input. A limit of `u64::MAX` is equivalent to no limit,
    /// but note that this enables a guest to also force the host to attempt an
    /// allocation of that size.
    pub fn max_random_size(&mut self, max_size: u64) -> &mut Self {
        self.random.max_size = max_size;
        self
    }

    /// Configures `wasi:clocks/wall-clock` to use the `clock` specified.
    ///
    /// By default the host's wall clock is used.
    pub fn wall_clock(&mut self, clock: impl HostWallClock + 'static) -> &mut Self {
        self.clocks.wall_clock = Box::new(clock);
        self
    }

    /// Configures `wasi:clocks/monotonic-clock` to use the `clock` specified.
    ///
    /// By default the host's monotonic clock is used.
    pub fn monotonic_clock(&mut self, clock: impl HostMonotonicClock + 'static) -> &mut Self {
        self.clocks.monotonic_clock = Box::new(clock);
        self
    }

    /// Allow all network addresses accessible to the host.
    ///
    /// This method will inherit all network addresses meaning that any address
    /// can be bound by the guest or connected to by the guest using any
    /// protocol.
    ///
    /// See also [`WasiCtxBuilder::socket_addr_check`].
    pub fn inherit_network(&mut self) -> &mut Self {
        self.socket_addr_check(|_, _| Box::pin(async { true }))
    }

    /// A check that will be called for each socket address that is used.
    ///
    /// Returning `true` will permit socket connections to the `SocketAddr`,
    /// while returning `false` will reject the connection.
    pub fn socket_addr_check<F>(&mut self, check: F) -> &mut Self
    where
        F: Fn(SocketAddr, SocketAddrUse) -> Pin<Box<dyn Future<Output = bool> + Send + Sync>>
            + Send
            + Sync
            + 'static,
    {
        self.sockets.socket_addr_check = SocketAddrCheck(Arc::new(check));
        self
    }

    /// Allow usage of `wasi:sockets/ip-name-lookup`
    ///
    /// By default this is disabled.
    pub fn allow_ip_name_lookup(&mut self, enable: bool) -> &mut Self {
        self.sockets.allowed_network_uses.ip_name_lookup = enable;
        self
    }

    /// Allow usage of UDP.
    ///
    /// This is enabled by default, but can be disabled if UDP should be blanket
    /// disabled.
    pub fn allow_udp(&mut self, enable: bool) -> &mut Self {
        self.sockets.allowed_network_uses.udp = enable;
        self
    }

    /// Allow usage of TCP
    ///
    /// This is enabled by default, but can be disabled if TCP should be blanket
    /// disabled.
    pub fn allow_tcp(&mut self, enable: bool) -> &mut Self {
        self.sockets.allowed_network_uses.tcp = enable;
        self
    }

    /// Uses the configured context so far to construct the final [`WasiCtx`].
    ///
    /// Note that each `WasiCtxBuilder` can only be used to "build" once, and
    /// calling this method twice will panic.
    ///
    /// # Panics
    ///
    /// Panics if this method is called twice. Each [`WasiCtxBuilder`] can be
    /// used to create only a single [`WasiCtx`]. Repeated usage of this method
    /// is not allowed and should use a second builder instead.
    pub fn build(&mut self) -> WasiCtx {
        assert!(!self.built);

        let Self {
            cli,
            clocks,
            random,
            sockets,
            allow_blocking_current_thread,
            preopens,
            built: _,
        } = mem::replace(self, Self::new());
        self.built = true;

        WasiCtx {
            cli,
            clocks,
            sockets,
            random,
            allow_blocking_current_thread,
            preopens,
        }
    }
    /// Builds a WASIp1 context instead of a [`WasiCtx`].
    ///
    /// This method is the same as [`build`](WasiCtxBuilder::build) but it
    /// creates a [`WasiP1Ctx`] instead. This is intended for use with the
    /// [`preview1`] module of this crate
    ///
    /// [`WasiP1Ctx`]: crate::preview1::WasiP1Ctx
    /// [`preview1`]: crate::preview1
    ///
    /// # Panics
    ///
    /// Panics if this method is called twice. Each [`WasiCtxBuilder`] can be
    /// used to create only a single [`WasiCtx`] or [`WasiP1Ctx`]. Repeated
    /// usage of this method is not allowed and should use a second builder
    /// instead.
    #[cfg(feature = "preview1")]
    pub fn build_p1(&mut self) -> crate::preview1::WasiP1Ctx {
        let wasi = self.build();
        crate::preview1::WasiP1Ctx::new(wasi)
    }
}

/// Per-[`Store`] state which holds state necessary to implement WASI from this
/// crate.
///
/// This structure is created through [`WasiCtxBuilder`] and is stored within
/// the `T` of [`Store<T>`][`Store`]. Access to the structure is provided
/// through the [`WasiView`](crate::WasiView) trait as an implementation on `T`.
///
/// Note that this structure itself does not have any accessors, it's here for
/// internal use within the `wasmtime-wasi` crate's implementation of
/// bindgen-generated traits.
///
/// [`Store`]: wasmtime::Store
///
/// # Example
///
/// ```
/// use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView, WasiCtxBuilder};
///
/// struct MyState {
///     ctx: WasiCtx,
///     table: ResourceTable,
/// }
///
/// impl WasiView for MyState {
///     fn ctx(&mut self) -> WasiCtxView<'_> {
///         WasiCtxView { ctx: &mut self.ctx, table: &mut self.table }
///     }
/// }
///
/// impl MyState {
///     fn new() -> MyState {
///         let mut wasi = WasiCtxBuilder::new();
///         wasi.arg("./foo.wasm");
///         wasi.arg("--help");
///         wasi.env("FOO", "bar");
///
///         MyState {
///             ctx: wasi.build(),
///             table: ResourceTable::new(),
///         }
///     }
/// }
/// ```
#[derive(Default)]
pub struct WasiCtx {
    pub(crate) random: WasiRandomCtx,
    pub(crate) clocks: WasiClocksCtx,
    pub(crate) cli: WasiCliCtx,
    pub(crate) sockets: WasiSocketsCtx,
    pub(crate) allow_blocking_current_thread: bool,
    pub(crate) preopens: Vec<(Dir, String)>,
}

impl WasiCtx {
    /// Convenience function for calling [`WasiCtxBuilder::new`].
    pub fn builder() -> WasiCtxBuilder {
        WasiCtxBuilder::new()
    }

    /// Returns access to the underlying [`WasiRandomCtx`].
    pub fn random(&mut self) -> &mut WasiRandomCtx {
        &mut self.random
    }

    /// Returns access to the underlying [`WasiClocksCtx`].
    pub fn clocks(&mut self) -> &mut WasiClocksCtx {
        &mut self.clocks
    }

    /// Returns access to the underlying [`WasiCliCtx`].
    pub fn cli(&mut self) -> &mut WasiCliCtx {
        &mut self.cli
    }

    /// Returns access to the underlying [`WasiSocketsCtx`].
    pub fn sockets(&mut self) -> &mut WasiSocketsCtx {
        &mut self.sockets
    }
}
