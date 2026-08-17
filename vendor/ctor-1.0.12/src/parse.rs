//! Parser for the `#[ctor]` macro.

#[macro_export]
#[doc(hidden)]
macro_rules! __ctor_parse {
    ( $($input:tt)* ) => {
        $crate::__perform!(
            ($($input)*),
            $crate::__chain[
                $crate::__parse_item[$crate::__ctor_features],
                $crate::__extract_unsafe,
                $crate::__ctor_parse_impl,
            ]
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __ctor_parse_internal {
    ( $features:path, $($input:tt)* ) => {
        $crate::__perform!(
            ($($input)*),
            $crate::__chain[
                $crate::__parse_item[$features],
                $crate::__extract_unsafe,
                $crate::__ctor_parse_impl,
            ]
        );
    };
}

/// Parse a processed `ctor` item. This is intentionally verbose to avoid
/// excessive nesting of macro calls in user code.
#[macro_export]
#[doc(hidden)]
macro_rules! __ctor_parse_impl {
    // Step 1: Feature check

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt: $anonymous_spec:ident,
            body_link_section = $body_link_section:tt: $body_link_section_spec:ident,
            crate_path = $crate_path:tt: $crate_path_spec:ident,
            export_name_prefix = $export_name_prefix:tt: $export_name_prefix_spec:ident,
            link_section = $link_section:tt: $link_section_spec:ident,
            naked = $naked:tt: $naked_spec:ident,
            priority = $priority:tt: $priority_spec:ident,
            priority_enabled = $priority_enabled:tt: $priority_enabled_spec:ident,
            proc_macro = $proc_macro:tt: $proc_macro_spec:ident,
            std = $std:tt: $std_spec:ident,
            r#unsafe = $no_fail_on_missing_unsafe:tt: $no_fail_on_missing_unsafe_spec:ident,
            used_linker = $used_linker:tt: $used_linker_spec:ident,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    )) => {
        // Process and validate the priority
        $crate::__map_priority!(
            @entry next=$crate::__ctor_parse_impl[[next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    linker_options = (
                        body_link_section = $body_link_section,
                        export_name_prefix = $export_name_prefix,
                        link_section = $link_section,
                        used_linker = $used_linker,
                    ),
                    no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe,
                ),
                self = $self,
                meta = $meta,
                unsafe = $unsafe,
                item = $item
            )]], input=(
                export_name_prefix = ($export_name_prefix: $export_name_prefix_spec),
                link_section = ($link_section: $link_section_spec),
                naked = ($naked: $naked_spec),
                priority = ($priority: $priority_spec),
                priority_enabled = ($priority_enabled: $priority_enabled_spec),
            )
        );
    };

    ( [next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    )], $priority:tt ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                linker_options = $linker_options,
                no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe,
                priority = $priority,
            ),
            self = $self,
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 2: Check function shape
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = ($($unsafe:tt)?),
        item = ($vis:vis $(unsafe)? $( extern $abi:literal )? fn $name:ident () $( -> () )? {
            $($body:tt)*
        })
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                linker_options = $linker_options,
                link_name = $name,
                no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe,
                priority = $priority,
            ),
            self = $self,
            meta = $meta,
            unsafe = ($($unsafe)?),
            item = ($vis $($unsafe)? $( extern $abi )? fn $name () {
                $($body)*
            })
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? { $literal:literal };)
    ) ) => {
        compile_error!("Trivial const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
    };

    // Allow dynamic #[ctor]s - a const expression determines the actual functions.
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : &[fn()] = const $body:block;)
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                linker_options = $linker_options,
                link_name = $ident,
                no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe,
                priority = $priority,
            ),
            self = $self,
            meta = $meta,
            unsafe = $unsafe,
            item = ($vis static $ident : &[fn()] = const $body;)
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? const $body:block;)
    ) ) => {
        compile_error!("Static const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? $literal:literal;)
    ) ) => {
        compile_error!("Trivial const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : &'static dyn $($rest:tt)*)
    ) ) => {
        compile_error!("&'static dyn types are not supported. Use a Box<dyn ...> instead.");
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : &'static (dyn $($dyn:tt)*) $($rest:tt)*)
    ) ) => {
        compile_error!("&'static dyn types are not supported. Use a Box<dyn ...> instead.");
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = ($($unsafe:tt)?),
        item = ($vis:vis static $ident:ident : & $lt:lifetime $ty:ty = $($body:tt)*)
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                linker_options = $linker_options,
                link_name = $ident,
                no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe,
                priority = $priority,
            ),
            self = $self,
            meta = $meta,
            unsafe = ($($unsafe)?),
            item = ($vis static $ident : & $lt $ty = $($body)*)
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = ($($unsafe:tt)?),
        item = ($vis:vis static $ident:ident : $ty:ty = $($body:tt)*)
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                linker_options = $linker_options,
                link_name = $ident,
                no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe,
                priority = $priority,
            ),
            self = $self,
            meta = $meta,
            unsafe = ($($unsafe)?),
            item = ($vis static $ident : $ty = $($body)*)
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($item:item)
    ) ) => {
        compile_error!("Invalid ctor item. \
            Expected a function with no args, \
            return value, or type parameters or a static variable.\n\
            Valid forms are:\n\
             - [pub] [unsafe] [extern $abi] fn $name() { ... }\n\
             - static $name : [&'static] $ty = [unsafe] { ... };");
    };

    // Step 3: Compute no_fail_on_missing_unsafe

    // Compile error iff no_fail_on_missing_unsafe is not present AND unsafe is not present
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            link_name = $link_name:tt,
            no_fail_on_missing_unsafe = (),
            priority = $priority:tt,
        ),
        self = ( $macro_name:ident ),
        meta = $meta:tt,
        unsafe = (),
        item = $item:tt
    ) ) => {
        compile_error!(concat!("Missing unsafe keyword in #[ctor] annotation. \
        Use #[ctor(unsafe)]. This error can be suppressed by passing \
        `--cfg linktime_no_fail_on_missing_unsafe` in `RUSTFLAGS` or placing this in your \
        `config.toml` file.\n\n\
        \n\
        #[ctor]\n\
        ^^^^^^^------- replace this with #[ctor(unsafe)]"));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            link_name = $link_name:tt,
            no_fail_on_missing_unsafe = (),
            priority = $priority:tt,
        ),
        self = ($macro_name:ident ($($self:tt)*) ),
        meta = $meta:tt,
        unsafe = (),
        item = $item:tt
    ) ) => {
        compile_error!(concat!("Missing unsafe keyword in #[ctor] annotation. \
        Use #[ctor(unsafe, ", stringify!($($self)*), ")]. This error can be suppressed by passing \
        `--cfg linktime_no_fail_on_missing_unsafe` in `RUSTFLAGS` or placing this in your \
        `config.toml` file.\n\n\
        \n\
        #[ctor(", stringify!($($self)*), ")]\n\
        ^------- replace this with #[ctor(unsafe, ", stringify!($($self)*), ")]\n"));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            linker_options = $linker_options:tt,
            link_name = $link_name:tt,
            no_fail_on_missing_unsafe = $no_fail_on_missing_unsafe:tt,
            priority = $priority:tt,
        ),
        self = $self:tt,
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                linker_options = $linker_options,
                link_name = $link_name,
                priority = $priority,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 4: Wrap in anonymous const
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = (),
            linker_options = $linker_options:tt,
            link_name = $link_name:tt,
            priority = $priority:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                linker_options = $linker_options,
                link_name = $link_name,
                priority = $priority,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = anonymous,
            linker_options = $linker_options:tt,
            link_name = $link_name:tt,
            priority = $priority:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        const _: () = {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    linker_options = $linker_options,
                    link_name = $link_name,
                    priority = $priority,
                ),
                meta = $meta,
                unsafe = $unsafe,
                item = $item
            ));
        };
    };

    // Step 5: Compute used_linker
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            linker_options = (
                body_link_section = $body_link_section:tt,
                export_name_prefix = $export_name_prefix:tt,
                link_section = $link_section:tt,
                used_linker = (),
            ),
            link_name = $link_name:tt,
            priority = $priority:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                linker_options = (
                    body_link_section = $body_link_section,
                    export_name_prefix = $export_name_prefix,
                    link_section = $link_section,
                    used_linker_meta = (#[used]),
                ),
                link_name = $link_name,
                priority = $priority,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            linker_options = (
                body_link_section = $body_link_section:tt,
                export_name_prefix = $export_name_prefix:tt,
                link_section = $link_section:tt,
                used_linker = used_linker,
            ),
            link_name = $link_name:tt,
            priority = $priority:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                linker_options = (
                    body_link_section = $body_link_section,
                    export_name_prefix = $export_name_prefix,
                    link_section = $link_section,
                    used_linker_meta = (#[used(linker)]),
                ),
                link_name = $link_name,
                priority = $priority,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 6: Compute export name suffix

    // No prefix, no computation
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            linker_options = (
                body_link_section = $body_link_section:tt,
                export_name_prefix = (),
                link_section = $link_section:tt,
                used_linker_meta = $used_linker_meta:tt,
            ),
            link_name = $link_name:tt,
            priority = $priority:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                linker_options = (
                    body_link_section = $body_link_section,
                    export_name = (),
                    link_section = $link_section,
                    used_linker_meta = $used_linker_meta,
                ),
                priority = $priority,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            linker_options = (
                body_link_section = $body_link_section:tt,
                export_name_prefix = $export_name_prefix:tt,
                link_section = $link_section:tt,
                used_linker_meta = $used_linker_meta:tt,
            ),
            link_name = $link_name:tt,
            priority = $priority:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                linker_options = (
                    body_link_section = $body_link_section,
                    export_name = (($export_name_prefix), ("_", env!("CARGO_PKG_NAME"), "_",
                        ::core::module_path!(), "_",
                        stringify!($link_name),
                        "_L", line!(), "C", column!())),
                    link_section = $link_section,
                    used_linker_meta = $used_linker_meta,
                ),
                priority = $priority,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 7: Compute priority

    // naked with no export name
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            linker_options = (
                body_link_section = $body_link_section:tt,
                export_name = (),
                link_section = $link_section:tt,
                used_linker_meta = $used_linker_meta:tt,
            ),
            priority = naked,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                body_link_section = $body_link_section,
                export_name = (),
                link_section = ($link_section),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // naked with export name
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            linker_options = (
                body_link_section = $body_link_section:tt,
                export_name = (($($prefix:tt)*), ($($suffix:tt)*)),
                link_section = $link_section:tt,
                used_linker_meta = $used_linker_meta:tt,
            ),
            priority = naked,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        // AIX uses 80000000 as the priority
        #[cfg(target_os = "aix")]
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                body_link_section = $body_link_section,
                export_name = (concat!($($prefix)*, "80000000", $($suffix)*)),
                link_section = ($link_section),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));

        #[cfg(not(target_os = "aix"))]
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                body_link_section = $body_link_section,
                export_name = (concat!($($prefix)*, $($suffix)*)),
                link_section = ($link_section),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            linker_options = (
                body_link_section = $body_link_section:tt,
                export_name = $export_name:tt,
                link_section = $link_section:tt,
                used_linker_meta = $used_linker_meta:tt,
            ),
            priority = $priority:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        #[cfg(target_vendor = "apple")]
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                body_link_section = $body_link_section,
                export_name = $export_name,
                priority = $priority,
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));

        // Get a priority literal
        #[cfg(not(target_vendor = "apple"))]
        $crate::__priority_to_literal!($crate::__ctor_parse_impl,[
            @priority next=$next[$next_args],
            features = (
                body_link_section = $body_link_section,
                export_name = $export_name,
                link_section = $link_section,
                used_linker_meta = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ] = $priority);
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            linker_options = (
                body_link_section = $body_link_section:tt,
                export_name = $export_name:tt,
                link_section = $link_section:tt,
                used_linker_meta = $used_linker_meta:tt,
            ),
            priority = $priority:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        compile_error!(concat!("Invalid priority: ", stringify!($priority)));
    };

    ( [@priority next=$next:path[$next_args:tt],
        features = (
            body_link_section = $body_link_section:tt,
            export_name = (),
            link_section = $link_section:tt,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ], ($($priority:tt)*)) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                body_link_section = $body_link_section,
                export_name = (),
                link_section = (concat!($link_section, ".", $($priority)*)),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( [@priority next=$next:path[$next_args:tt],
        features = (
            body_link_section = $body_link_section:tt,
            export_name = (($($prefix:tt)*), ($($suffix:tt)*)),
            link_section = $link_section:tt,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ], ($($priority:tt)*)) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                body_link_section = $body_link_section,
                export_name = (concat!($($prefix)*, $($priority)*, $($suffix)*)),
                link_section = (concat!($link_section, ".", $($priority)*)),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 8: Compute body link section meta
    ( @entry next=$next:path[$next_args:tt], input=(
        link_args = (
            body_link_section = $($body_link_section:literal)? $( () )?,
            $($link_args:tt)*
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                body_link_meta = ( $([link_section = $body_link_section])? ),
                $($link_args)*
            ),
            body_link_meta = ( $([link_section = $body_link_section])? ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 9: Delegate on item type
    ( @entry next=$next:path[$next_args:tt], input=(
        link_args = $link_args:tt,
        body_link_meta = ($($body_link_meta:tt)?),
        meta = ($($meta:tt)*),
        unsafe = ($($unsafe:tt)*),
        item = ($vis:vis $(unsafe)? $( extern $abi:literal )? fn $name:ident () $( -> () )? {
            $($body:tt)*
        })
    ) ) => {
        $($meta)*
        #[allow(dead_code)]
        $vis $($unsafe)* $( extern $abi )? fn $name () {
            // The outer function may be attached to a struct, so we generate an
            // inner function that is freestanding and call it from both places.
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            $(#[allow(unsafe_code)] #$body_link_meta)?
            $($unsafe)* $( extern $abi )? fn __ctor_private_inner() {
                $($body)*
            }

            $crate::__ctor_parse_impl!(@ctor $link_args body={ $($unsafe)* { __ctor_private_inner() } });
            $($unsafe)* { __ctor_private_inner() }
        }
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        link_args = $link_args:tt,
        body_link_meta = ($($body_link_meta:tt)?),
        meta = ($($meta:tt)*),
        unsafe = ($($unsafe:tt)*),
        item = ($vis:vis static $ident:ident : &[fn()] = const $body:block;)
    ) ) => {
        $($meta)*
        $vis static $ident: &[fn()] = /*const*/ {
            const __EXTERN_C_FNS: [extern "C" fn(); $ident.len()] = {
                use ::core::mem::MaybeUninit;
                let mut array: MaybeUninit<[extern "C" fn(); $ident.len()]> = MaybeUninit::uninit();
                let mut array_ptr: *mut extern "C" fn() = array.as_mut_ptr() as _;

                extern "C" fn bind_array<const N: usize>() {
                    $ident[N]()
                }

                unsafe {
                    let array_ptr = array.as_mut_ptr() as *mut extern "C" fn();
                    const LEN: usize = $ident.len();
                    if LEN > 0 { array_ptr.add(0).write(bind_array::<0>); }
                    if LEN > 1 { array_ptr.add(1).write(bind_array::<1>); }
                    if LEN > 2 { array_ptr.add(2).write(bind_array::<2>); }
                    if LEN > 3 { array_ptr.add(3).write(bind_array::<3>); }
                    if LEN > 4 { array_ptr.add(4).write(bind_array::<4>); }
                    if LEN > 5 { array_ptr.add(5).write(bind_array::<5>); }
                    if LEN > 6 { array_ptr.add(6).write(bind_array::<6>); }
                    if LEN > 7 { array_ptr.add(7).write(bind_array::<7>); }
                    if LEN > 8 { array_ptr.add(8).write(bind_array::<8>); }
                    if LEN > 9 { array_ptr.add(9).write(bind_array::<9>); }
                    if LEN > 10 { array_ptr.add(10).write(bind_array::<10>); }
                    if LEN > 11 { array_ptr.add(11).write(bind_array::<11>); }
                    if LEN > 12 { array_ptr.add(12).write(bind_array::<12>); }
                    if LEN > 13 { array_ptr.add(13).write(bind_array::<13>); }
                    if LEN > 14 { array_ptr.add(14).write(bind_array::<14>); }
                    if LEN > 15 { array_ptr.add(15).write(bind_array::<15>); }
                    if LEN > 16 {
                        panic!("Unexpected array length, expected <= 16");
                    }
                }
                unsafe { array.assume_init() }
            };
            $crate::__ctor_parse_impl!(@ctor $link_args fns=__EXTERN_C_FNS);

            $body
        };
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        link_args = $link_args:tt,
        body_link_meta = ($($body_link_meta:tt)?),
        meta = ($($meta:tt)*),
        unsafe = ($($unsafe:tt)*),
        item = ($vis:vis static $ident:ident : & $lt:lifetime $ty:ty = &$($body:tt)*)
    ) ) => {
        $($meta)*
        $vis static $ident: & $lt $crate::statics::Static<$ty> = {
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            $(#[allow(unsafe_code)] #$body_link_meta)?
            fn init() -> $ty {
                return $($body)*
            }

            static __STATIC_CTOR: $crate::statics::Static<$ty> = {
                unsafe { $crate::statics::Static::<$ty>::new(init) }
            };
            &__STATIC_CTOR
        };
        $crate::__ctor_parse_impl!(@ctor $link_args body={ _ = &*$ident } );
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        link_args = $link_args:tt,
        body_link_meta = ($($body_link_meta:tt)?),
        meta = ($($meta:tt)*),
        unsafe = ($($unsafe:tt)*),
        item = ($vis:vis static $ident:ident : & $lt:lifetime $ty:ty = $($body:tt)*)
    ) ) => {
        $($meta)*
        $vis static $ident: $crate::statics::Static<&'static $ty> = {
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            $(#[allow(unsafe_code)] #$body_link_meta)?
            fn init() -> &'static $ty {
                return $($body)*
            }
            unsafe { $crate::statics::Static::<&'static $ty>::new(init) }
        };
        $crate::__ctor_parse_impl!(@ctor $link_args body={ _ = &*$ident } );
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        link_args = $link_args:tt,
        body_link_meta = ($($body_link_meta:tt)?),
        meta = ($($meta:tt)*),
        unsafe = ($($unsafe:tt)*),
        item = ($vis:vis static $ident:ident : $ty:ty = $($body:tt)*)
    ) ) => {
        $($meta)*
        $vis static $ident: $crate::statics::Static<$ty> = {
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            $(#[allow(unsafe_code)] #$body_link_meta)?
            fn init() -> $ty {
                return $($body)*
            }
            unsafe { $crate::statics::Static::<$ty>::new(init) }
        };
        $crate::__ctor_parse_impl!(@ctor $link_args body={ _ = &*$ident } );
    };

    // ctor definitions

    // linux-style, one ctor
    ( @ctor (
        body_link_meta = ($($body_link_meta:tt)?),
        export_name=(),
        link_section=($($link_section:tt)*),
        used=(#$used_linker_meta:tt),
     ) body=$body:tt ) => {
        const _: () = {
            #[allow(unsafe_code)]
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            #[link_section = $($link_section)*]
            #$used_linker_meta
            static __CTOR_PRIVATE_REF: unsafe extern "C" fn() = {
                #[allow(unused_unsafe)]
                $(#[allow(unsafe_code)] #$body_link_meta)?
                extern "C" fn __ctor_private() {
                    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
                    {
                        static DISARMED: ::core::sync::atomic::AtomicBool = ::core::sync::atomic::AtomicBool::new(false);
                        if DISARMED.swap(true, ::core::sync::atomic::Ordering::Relaxed) {
                            return;
                        }
                    }
                    $body
                }
                __ctor_private
            };
        };
    };

    // linux-style, multiple ctor
    ( @ctor (
        body_link_meta = ($($body_link_meta:tt)?),
        export_name=(),
        link_section=($($link_section:tt)*),
        used=(#$used_linker_meta:tt),
     ) fns=$ident:ident ) => {
        #[allow(unsafe_code)]
        #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
        #[link_section = $($link_section)*]
        #$used_linker_meta
        static __CTOR_PRIVATE_REF: unsafe extern "C" fn() = {
            #[allow(unused_unsafe)]
            $(#[allow(unsafe_code)] #$body_link_meta)?
            extern "C" fn __ctor_private() {
                #[cfg(all(target_family = "wasm", target_os = "unknown"))]
                {
                    static DISARMED: ::core::sync::atomic::AtomicBool = ::core::sync::atomic::AtomicBool::new(false);
                    if DISARMED.swap(true, ::core::sync::atomic::Ordering::Relaxed) {
                        return;
                    }
                }
                for f in $ident {
                    f();
                }
            }
            __ctor_private
        };
    };

    // "collect"-style, one ctor
    ( @ctor (
        body_link_meta = ($($body_link_meta:tt)?),
        export_name=(),
        priority=$priority:tt,
        used=(#$used_linker_meta:tt),
     ) body=$body:tt ) => {
        const _: () = {
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            #[allow(unsafe_code, unused_unsafe)]
            $(#$body_link_meta)?
            extern "C" fn __ctor_private() {
                #[cfg(all(target_family = "wasm", target_os = "unknown"))]
                {
                    static DISARMED: ::core::sync::atomic::AtomicBool = ::core::sync::atomic::AtomicBool::new(false);
                    if DISARMED.swap(true, ::core::sync::atomic::Ordering::Relaxed) {
                        return;
                    }
                }
                $body
            }

            $crate::__register_ctor!(priority = $priority, fn = __ctor_private);
        };
    };

    // "collect"-style, multiple ctors
    ( @ctor (
        body_link_meta = ($($body_link_meta:tt)?),
        export_name=(),
        priority=$priority:tt,
        used=(#$used_linker_meta:tt),
     ) fns=$ident:ident ) => {
        const _: () = {
            $crate::__register_ctor!(priority = $priority, fn = (array $ident));
        };
    };

    // AIX-style, one ctor
    ( @ctor (
        body_link_meta = ($($body_link_meta:tt)?),
        export_name=($($link_name:tt)*),
        link_section=$link_section:tt,
        used=(#$used_linker_meta:tt),
     ) body=$body:tt ) => {
        const _: () = {
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            #[allow(unused_unsafe, unsafe_code)]
            #[no_mangle]
            #[export_name = $($link_name)*]
            $(#$body_link_meta)?
            extern "C" fn __ctor_private() {
                #[cfg(all(target_family = "wasm", target_os = "unknown"))]
                {
                    static DISARMED: ::core::sync::atomic::AtomicBool = ::core::sync::atomic::AtomicBool::new(false);
                    if DISARMED.swap(true, ::core::sync::atomic::Ordering::Relaxed) {
                        return;
                    }
                }
                $body
            }
        };
    };

    // AIX-style, multiple ctor
    ( @ctor (
        body_link_meta = ($($body_link_meta:tt)?),
        export_name=($($link_name:tt)*),
        link_section=$link_section:tt,
        used=(#$used_linker_meta:tt),
     ) fns=$ident:ident ) => {
        const _: () = {
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            #[allow(unused_unsafe, unsafe_code)]
            #[no_mangle]
            #[export_name = $($link_name)*]
            $(#$body_link_meta)?
            extern "C" fn __ctor_private() {
                #[cfg(all(target_family = "wasm", target_os = "unknown"))]
                {
                    static DISARMED: ::core::sync::atomic::AtomicBool = ::core::sync::atomic::AtomicBool::new(false);
                    if DISARMED.swap(true, ::core::sync::atomic::Ordering::Relaxed) {
                        return;
                    }
                }
                for f in $ident {
                    f();
                }
            }
        };
    };

    (@ctor $features:tt body=$body:tt) => {
        compile_error!(concat!("Invalid ctor features: ", stringify!($features)));
    };
}

/// Map the priority input to a priority value. This is somewhat complex because
/// the default for priority changes based on whether priority is enabled and
/// whether link options are specified.
#[macro_export]
#[doc(hidden)]
macro_rules! __map_priority {
    // Priority specified, priority not enabled
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = $enp:tt,
        link_section = $ls:tt,
        naked = $naked:tt,
        priority = ($priority:tt: value),
        priority_enabled = ((), $pe_spec:ident),
    ) ) => {
        compile_error!(concat!("The crate \"priority\" feature was not enabled: `priority = ", stringify!($priority), "` is not supported."));
    };

    // Priority unspecified, link options not, priority not enabled => naked
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = ($enp:tt: default),
        link_section = ($ls:tt: default),
        naked = ($naked:tt: default),
        priority = ($priority:tt: default),
        priority_enabled = ((): $pe_spec:ident),
    ) ) => {
        $next!($next_args, naked);
    };

    // Priority unspecified, link options not, priority enabled => default (500)
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = ($enp:tt: default),
        link_section = ($ls:tt: default),
        naked = ($naked:tt: default),
        priority = ($priority:tt: default),
        priority_enabled = (priority_enabled: $pe_spec:ident),
    ) ) => {
        $next!($next_args, 500);
    };

    // Priority specified (or default) = early, link options not, priority enabled
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = ($enp:tt: default),
        link_section = ($ls:tt: default),
        naked = ($naked:tt: default),
        priority = (early: $p_spec:ident),
        priority_enabled = $pe:tt,
    ) ) => {
        $next!($next_args, 101);
    };

    // Priority specified (or default) = default, link options not, priority enabled
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = ($enp:tt: default),
        link_section = ($ls:tt: default),
        naked = ($naked:tt: default),
        priority = (default: $p_spec:ident),
        priority_enabled = $pe:tt,
    ) ) => {
        $next!($next_args, 500);
    };

    // Priority specified = late, link options not, priority enabled
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = ($enp:tt: default),
        link_section = ($ls:tt: default),
        naked = ($naked:tt: default),
        priority = (late: $p_spec:ident),
        priority_enabled = $pe:tt,
    ) ) => {
        #[cfg(all(target_os = "aix", not(target_vendor = "apple")))]
        $next!($next_args, 89999999);

        #[cfg(all(not(target_os = "aix"), not(target_vendor = "apple")))]
        $next!($next_args, 65535);

        #[cfg(target_vendor = "apple")]
        $next!($next_args, ($crate::collect::LATE));
    };

    // Priority specified, link options not, priority enabled
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = ($enp:tt: default),
        link_section = ($ls:tt: default),
        naked = ($naked:tt: default),
        priority = ($priority:tt: $p_spec:ident),
        priority_enabled = $pe:tt,
    ) ) => {
        $next!($next_args, $priority);
    };

    // Priority specified, link options specified
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = $enp:tt,
        link_section = $ls:tt,
        naked = $naked:tt,
        priority = ($priority:tt: value),
        priority_enabled = $pe:tt,
    ) ) => {
        compile_error!(concat!("Priority must not be specified if naked, export_name_prefix, or link_section are specified."));
    };

    // Priority unspecified, link options specified -> naked
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = $enp:tt,
        link_section = $ls:tt,
        naked = $naked:tt,
        priority = ($priority:tt: default),
        priority_enabled = $pe:tt,
    ) ) => {
        $next!($next_args, naked);
    };

    // Naked specified
    ( @entry next=$next:path[$next_args:tt], input=(
        export_name_prefix = $enp:tt,
        link_section = $ls:tt,
        naked = (naked: value),
        priority = $priority:tt,
        priority_enabled = $pe:tt,
    ) ) => {
        $next!($next_args, naked);
    };

    ( @entry next=$next:path[$next_args:tt], input=$input:tt ) => {
        compile_error!(concat!("Unexpected priority input: ", stringify!($input)));
    };
}
