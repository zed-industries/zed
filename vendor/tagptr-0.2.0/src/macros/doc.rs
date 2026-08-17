/// All macros for generating documentation.

/// A macro for generating arbitrary documented code items
macro_rules! doc_comment {
    ($docs:expr, $($item:tt)*) => {
        #[doc = $docs]
        $($item)*
    };
}

/********** macros for generating constants docs **************************************************/

/// A macro for generating the docs for the `TAG_BITS` constant.
macro_rules! doc_tag_bits {
    () => {
        "The number of available tag bits for this type."
    };
}

/// A macro for generating the docs for the `TAG_MASK` constant.
macro_rules! doc_tag_mask {
    () => {
        "The bitmask for the lower bits available for storing the tag value."
    };
}

/// A macro for generating the docs for the `PTR_MASK` constants.
macro_rules! doc_ptr_mask {
    () => {
        "The bitmask for the (higher) bits for storing the pointer itself."
    };
}

/********** macros for generating function docs ***************************************************/

macro_rules! doc_null {
    () => {
        "Creates a new `null` pointer."
    };
}

macro_rules! doc_new {
    () => {
        "Creates a new unmarked pointer."
    };
}

macro_rules! doc_from_usize {
    () => {
        "Creates a new pointer from the numeric (integer) representation of a \
        potentially marked pointer."
    };
}

macro_rules! doc_into_raw {
    () => {
        "Returns the internal representation of the pointer *as is*, i.e. any \
        potential tag value is **not** stripped."
    };
}

macro_rules! doc_into_usize {
    () => {
        "Returns the numeric (integer) representation of the pointer with its \
        tag value."
    };
}

macro_rules! doc_cast {
    () => {
        "Casts to a pointer of another type."
    };
}

macro_rules! doc_compose {
    () => {
        "Composes a new marked pointer from a raw `ptr` and a `tag` value.\n\n\
        The supplied `ptr` is assumed to be well-aligned (i.e. has no tag bits \
        set) and calling this function may lead to unexpected results when \
        this is not the case."
    };
}

macro_rules! doc_clear_tag {
    ("non-null" $example_type_path:path) => {
        concat!(
            doc_clear_tag!(),
            "# Examples\n\n\
            ```\nuse core::ptr::NonNull;\n\n\
            type TagNonNull = ",
            stringify!($example_type_path),
            ";\n\n\
            let reference = &mut 1;\n\
            let ptr = TagNonNull::compose(NonNull::from(reference), 0b11);\n\
            assert_eq!(ptr.clear_tag(), TagNonNull::from(reference));\n```"
        )
    };
    ($example_type_path:path) => {
        concat!(
            doc_clear_tag!(),
            "# Examples\n\n\
            ```\nuse core::ptr;\n\n\
            type TagPtr = ",
            stringify!($example_type_path),
            ";\n\n\
            let reference = &mut 1;\n\
            let ptr = TagPtr::compose(reference, 0b11);\n\
            assert_eq!(ptr.clear_tag(), TagPtr::new(reference));\n```"
        )
    };
    () => {
        "Clears the marked pointer's tag value.\n\n"
    };
}

macro_rules! doc_split_tag {
    ("non-null" $example_type_path:path) => {
        concat!(
            doc_split_tag!(),
            "# Examples\n\n\
            ```\nuse core::ptr;\n\n\
            type TagNonNull = ",
            stringify!($example_type_path),
            ";\n\n\
            let reference = &mut 1;\n\
            let ptr = TagNonNull::compose(NonNull::from(reference), 0b11);\n\
            assert_eq!(ptr.split_tag(), (TagNonNull::from(reference), 0b11));\n```"
        )
    };
    ($example_type_path:path) => {
        concat!(
            doc_split_tag!(),
            "# Examples\n\n\
            ```\nuse core::ptr;\n\n\
            type TagPtr = ",
            stringify!($example_type_path),
            ";\n\n\
            let reference = &mut 1;\n\
            let ptr = TagPtr::compose(reference, 0b11);\n\
            assert_eq!(ptr.split_tag(), (TagPtr::new(reference), 0b11));\n```"
        )
    };
    () => {
        "Splits the tag value from the marked pointer, returning both the cleared pointer and the \
        separated tag value.\n\n"
    };
}

macro_rules! doc_set_tag {
    ("non-null" $example_type_path:path) => {
        concat!(
            doc_set_tag!(),
            "\n\n# Examples\n\n\
            ```\nuse core::ptr;\n\n\
            type TagNonNull = ",
            stringify!($example_type_path),
            ";\n\n\
            let reference = &mut 1;\n\
            let ptr = TagNonNull::compose(NonNull::from(reference), 0b11);\n\
            assert_eq!(ptr.set_tag(0b10).decompose(), (NonNull::from(reference), 0b10));\n```"
        )
    };
    ($example_type_path:path) => {
        concat!(
            doc_set_tag!(),
            "\n\n# Examples\n\n\
            ```\nuse core::ptr;\n\n\
            type TagPtr = ",
            stringify!($example_type_path),
            ";\n\n\
            let reference = &mut 1;\n\
            let ptr = TagPtr::compose(reference, 0b11);\n\
            assert_eq!(ptr.set_tag(0b10).decompose(), (reference as *mut _, 0b10));\n```"
        )
    };
    () => {
        "Sets the marked pointer's tag value to `tag` and overwrites any previous value."
    };
}

macro_rules! doc_update_tag {
    ("non-null" $example_type_path:path) => {
        concat!(
            doc_update_tag!(),
            "\n\n# Examples\n\n\
            ```\nuse core::ptr;\n\n\
            type TagNonNull = ",
            stringify!($example_type_path),
            ";\n\n\
            let reference = &mut 1;\n\
            let ptr = TagNonNull::compose(reference, 0b11);\n\
            assert_eq!(ptr.update_tag(|tag| tag - 2).decompose(), (NonNull::from(reference), 0b01));\n```"
        )
    };
    ($example_type_path:path) => {
        concat!(
            doc_update_tag!(),
            "\n\n# Examples\n\n\
            ```\nuse core::ptr;\n\n\
            type TagPtr = ",
            stringify!($example_type_path),
            ";\n\n\
            let reference = &mut 1;
            let ptr = TagPtr::compose(reference, 0b11);\n\
            let ptr = ptr.update_tag(|tag| tag - 1);\n\
            assert_eq!(ptr.decompose(), (reference as *mut _, 0b10));\n```"
        )
    };
    () => {
        "Updates the marked pointer's tag value to the result of `func`, which is called with the \
        current tag value."
    };
}

macro_rules! doc_add_tag {
    () => {
        "Adds `value` to the current tag *without* regard for the previous \
        value.\n\n\
        This method does not perform any checks so it may silently overflow \
        the tag bits, result in a pointer to a different value, a null pointer \
        or an unaligned pointer."
    };
}

macro_rules! doc_sub_tag {
    () => {
        "Subtracts `value` from the current tag *without* regard for the \
        previous value.\n\n\
        This method does not perform any checks so it may silently overflow \
        the tag bits, result in a pointer to a different value, a null \
        pointer or an unaligned pointer."
    };
}

macro_rules! doc_decompose {
    () => {
        "Decomposes the marked pointer, returning the raw pointer and the \
        separated tag value."
    };
}

macro_rules! doc_decompose_ptr {
    () => {
        "Decomposes the marked pointer, returning only the separated raw \
        pointer."
    };
}

macro_rules! doc_decompose_non_null {
    () => {
        "Decomposes the marked pointer, returning only the separated raw \
        [`NonNull`] pointer."
    };
}

macro_rules! doc_decompose_tag {
    () => {
        "Decomposes the marked pointer, returning only the separated tag value."
    };
}

macro_rules! doc_as_ref_or_mut {
    ("safety") => {
        "When calling this method, you have to ensure that *either* the \
        pointer is `null` *or* all of the following is true:\n\n\
        - it is properly aligned\n\
        - it must point to an initialized instance of T; in particular, \
        the pointer must be \"de-referencable\" in the sense defined \
        [here].\n\n\
        This applies even if the result of this method is unused! (The \
        part about being initialized is not yet fully decided, but until \
        it is the only safe approach is to ensure that they are indeed \
        initialized.)\n\n\
        Additionally, the lifetime `'a` returned is arbitrarily chosen and \
        does not necessarily reflect the actual lifetime of the data. \
        *You* must enforce Rust's aliasing rules. \
        In particular, for the duration of this lifetime, the memory this \
        pointer points to must not get accessed (read or written) through \
        any other pointer.\n\n\
        [here]: [std::ptr]"
    };
    ($ret_str:expr) => {
        concat!(
            "Decomposes the marked pointer, returning ",
            $ret_str,
            " reference and discarding the tag value."
        )
    };
}

macro_rules! doc_as_ref {
    (@inner, $ret_str:expr) => {
        concat!(
            doc_as_ref_or_mut!($ret_str),
            "\n\n# Safety\n\
            While this method and its mutable counterpart are useful for \
            null-safety, it is important to note that this is still an unsafe \
            operation because the returned value could be pointing to invalid \
            memory.\n\n",
            doc_as_ref_or_mut!("safety")
        )
    };
    ("nullable") => {
        doc_as_ref!(@inner, "an optional")
    };
    ("non-nullable") => {
        doc_as_ref!(@inner, "a")
    };
}

macro_rules! doc_as_mut {
    (@inner, $self_ident:ident, $ret_str:expr) => {
        concat!(
            doc_as_ref_or_mut!($ret_str),
            "\n\n# Safety\n\
            As with [`as_ref`][",
            stringify!($self_ident),
            "::as_ref], this is unsafe because it cannot verify the validity \
            of the returned pointer, nor can it ensure that the lifetime `'a` \
            returned is indeed a valid lifetime for the contained data.\n\n",
            doc_as_ref_or_mut!("safety")
        )
    };
    ("nullable", $self_ident:ident) => {
        doc_as_mut!(@inner, $self_ident, "an optional *mutable*")
    };
    ("non-nullable", $self_ident:ident) => {
        doc_as_mut!(@inner, $self_ident, "a *mutable*")
    };
}

macro_rules! doc_atomic_new {
    () => {
        "Creates a new atomic marked pointer."
    };
}

macro_rules! doc_atomic_into_inner {
    () => {
        "Consumes the atomic marked pointer and returns its contained value.\n\n\
         This is safe because passing `self` by value guarantees no other \
         threads are concurrently accessing the atomic pointer."
    };
}
