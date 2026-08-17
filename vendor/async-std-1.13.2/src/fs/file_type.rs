cfg_not_docs! {
    pub use std::fs::FileType;
}

cfg_docs! {
    /// The type of a file or directory.
    ///
    /// A file type is returned by [`Metadata::file_type`].
    ///
    /// Note that file types are mutually exclusive, i.e. at most one of methods [`is_dir`],
    /// [`is_file`], and [`is_symlink`] can return `true`.
    ///
    /// This type is a re-export of [`std::fs::FileType`].
    ///
    /// [`Metadata::file_type`]: struct.Metadata.html#method.file_type
    /// [`is_dir`]: #method.is_dir
    /// [`is_file`]: #method.is_file
    /// [`is_symlink`]: #method.is_symlink
    /// [`std::fs::FileType`]: https://doc.rust-lang.org/std/fs/struct.FileType.html
    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    pub struct FileType {
        _private: (),
    }

    impl FileType {
        /// Returns `true` if this file type represents a regular directory.
        ///
        /// If this file type represents a symbolic link, this method returns `false`.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let file_type = fs::metadata(".").await?.file_type();
        /// println!("{:?}", file_type.is_dir());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn is_dir(&self) -> bool {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns `true` if this file type represents a regular file.
        ///
        /// If this file type represents a symbolic link, this method returns `false`.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let file_type = fs::metadata("a.txt").await?.file_type();
        /// println!("{:?}", file_type.is_file());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn is_file(&self) -> bool {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns `true` if this file type represents a symbolic link.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let file_type = fs::metadata("a.txt").await?.file_type();
        /// println!("{:?}", file_type.is_symlink());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn is_symlink(&self) -> bool {
            unreachable!("this impl only appears in the rendered docs")
        }
    }
}
