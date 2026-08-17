mod seek;

use seek::SeekFuture;

use crate::io::SeekFrom;

pub use futures_io::AsyncSeek as Seek;

#[doc = r#"
    Extension methods for [`Seek`].

    [`Seek`]: ../trait.Seek.html
"#]
pub trait SeekExt: Seek {
    #[doc = r#"
        Seeks to a new position in a byte stream.

        Returns the new position in the byte stream.

        A seek beyond the end of stream is allowed, but behavior is defined by the
        implementation.

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::fs::File;
        use async_std::io::SeekFrom;
        use async_std::prelude::*;

        let mut file = File::open("a.txt").await?;

        let file_len = file.seek(SeekFrom::End(0)).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn seek(
        &mut self,
        pos: SeekFrom,
    ) -> SeekFuture<'_, Self>
    where
        Self: Unpin,
    {
        SeekFuture { seeker: self, pos }
    }
}

impl<T: Seek + ?Sized> SeekExt for T {}
