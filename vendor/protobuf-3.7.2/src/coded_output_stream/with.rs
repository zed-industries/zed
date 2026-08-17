use std::io::Write;

use crate::CodedOutputStream;

pub(crate) trait WithCodedOutputStream {
    fn with_coded_output_stream<T, F>(self, cb: F) -> crate::Result<T>
    where
        F: FnOnce(&mut CodedOutputStream) -> crate::Result<T>;
}

impl<'a> WithCodedOutputStream for &'a mut (dyn Write + 'a) {
    fn with_coded_output_stream<T, F>(self, cb: F) -> crate::Result<T>
    where
        F: FnOnce(&mut CodedOutputStream) -> crate::Result<T>,
    {
        let mut os = CodedOutputStream::new(self);
        let r = cb(&mut os)?;
        os.flush()?;
        Ok(r)
    }
}

impl<'a> WithCodedOutputStream for &'a mut Vec<u8> {
    fn with_coded_output_stream<T, F>(mut self, cb: F) -> crate::Result<T>
    where
        F: FnOnce(&mut CodedOutputStream) -> crate::Result<T>,
    {
        let mut os = CodedOutputStream::vec(&mut self);
        let r = cb(&mut os)?;
        os.flush()?;
        Ok(r)
    }
}
