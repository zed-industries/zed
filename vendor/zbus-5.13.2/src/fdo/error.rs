use crate::DBusError;

/// Errors from <https://gitlab.freedesktop.org/dbus/dbus/-/blob/master/dbus/dbus-protocol.h>
#[derive(Clone, Debug, DBusError, PartialEq)]
#[zbus(prefix = "org.freedesktop.DBus.Error", impl_display = true)]
#[allow(clippy::upper_case_acronyms)]
pub enum Error {
    /// Unknown or fall-through zbus error.
    #[zbus(error)]
    ZBus(zbus::Error),

    /// A generic error; "something went wrong" - see the error message for more.
    Failed(String),

    /// There was not enough memory to complete an operation.
    NoMemory(String),

    /// The bus doesn't know how to launch a service to supply the bus name you wanted.
    ServiceUnknown(String),

    /// The bus name you referenced doesn't exist (i.e. no application owns it).
    NameHasNoOwner(String),

    /// No reply to a message expecting one, usually means a timeout occurred.
    NoReply(String),

    /// Something went wrong reading or writing to a socket, for example.
    IOError(String),

    /// A D-Bus bus address was malformed.
    BadAddress(String),

    /// Requested operation isn't supported (like ENOSYS on UNIX).
    NotSupported(String),

    /// Some limited resource is exhausted.
    LimitsExceeded(String),

    /// Security restrictions don't allow doing what you're trying to do.
    AccessDenied(String),

    /// Authentication didn't work.
    AuthFailed(String),

    /// Unable to connect to server (probably caused by ECONNREFUSED on a socket).
    NoServer(String),

    /// Certain timeout errors, possibly ETIMEDOUT on a socket.
    /// Note that `TimedOut` is used for message reply timeouts.
    Timeout(String),

    /// No network access (probably ENETUNREACH on a socket).
    NoNetwork(String),

    /// Can't bind a socket since its address is in use (i.e. EADDRINUSE).
    AddressInUse(String),

    /// The connection is disconnected and you're trying to use it.
    Disconnected(String),

    /// Invalid arguments passed to a method call.
    InvalidArgs(String),

    /// Missing file.
    FileNotFound(String),

    /// Existing file and the operation you're using does not silently overwrite.
    FileExists(String),

    /// Method name you invoked isn't known by the object you invoked it on.
    UnknownMethod(String),

    /// Object you invoked a method on isn't known.
    UnknownObject(String),

    /// Interface you invoked a method on isn't known by the object.
    UnknownInterface(String),

    /// Property you tried to access isn't known by the object.
    UnknownProperty(String),

    /// Property you tried to set is read-only.
    PropertyReadOnly(String),

    /// Certain timeout errors, e.g. while starting a service.
    TimedOut(String),

    /// Tried to remove or modify a match rule that didn't exist.
    MatchRuleNotFound(String),

    /// The match rule isn't syntactically valid.
    MatchRuleInvalid(String),

    /// While starting a new process, the exec() call failed.
    #[zbus(name = "Spawn.ExecFailed")]
    SpawnExecFailed(String),

    /// While starting a new process, the fork() call failed.
    #[zbus(name = "Spawn.ForkFailed")]
    SpawnForkFailed(String),

    /// While starting a new process, the child exited with a status code.
    #[zbus(name = "Spawn.ChildExited")]
    SpawnChildExited(String),

    /// While starting a new process, the child exited on a signal.
    #[zbus(name = "Spawn.ChildSignaled")]
    SpawnChildSignaled(String),

    /// While starting a new process, something went wrong.
    #[zbus(name = "Spawn.Failed")]
    SpawnFailed(String),

    /// We failed to set up the environment correctly.
    #[zbus(name = "Spawn.FailedToSetup")]
    SpawnFailedToSetup(String),

    /// We failed to set up the config parser correctly.
    #[zbus(name = "Spawn.ConfigInvalid")]
    SpawnConfigInvalid(String),

    /// Bus name was not valid.
    #[zbus(name = "Spawn.ServiceNotValid")]
    SpawnServiceNotValid(String),

    /// Service file not found in system-services directory.
    #[zbus(name = "Spawn.ServiceNotFound")]
    SpawnServiceNotFound(String),

    /// Permissions are incorrect on the setuid helper.
    #[zbus(name = "Spawn.PermissionsInvalid")]
    SpawnPermissionsInvalid(String),

    /// Service file invalid (Name, User or Exec missing).
    #[zbus(name = "Spawn.FileInvalid")]
    SpawnFileInvalid(String),

    /// There was not enough memory to complete the operation.
    #[zbus(name = "Spawn.NoMemory")]
    SpawnNoMemory(String),

    /// Tried to get a UNIX process ID and it wasn't available.
    UnixProcessIdUnknown(String),

    /// A type signature is not valid.
    InvalidSignature(String),

    /// A file contains invalid syntax or is otherwise broken.
    InvalidFileContent(String),

    /// Asked for SELinux security context and it wasn't available.
    SELinuxSecurityContextUnknown(String),

    /// Asked for ADT audit data and it wasn't available.
    AdtAuditDataUnknown(String),

    /// There's already an object with the requested object path.
    ObjectPathInUse(String),

    /// The message metadata does not match the payload. e.g. expected number of file descriptors
    /// were not sent over the socket this message was received on.
    InconsistentMessage(String),

    /// The message is not allowed without performing interactive authorization, but could have
    /// succeeded if an interactive authorization step was allowed.
    InteractiveAuthorizationRequired(String),

    /// The connection is not from a container, or the specified container instance does not exist.
    NotContainer(String),
}

/// Alias for a `Result` with the error type [`zbus::fdo::Error`].
///
/// [`zbus::fdo::Error`]: enum.Error.html
pub type Result<T> = std::result::Result<T, Error>;
