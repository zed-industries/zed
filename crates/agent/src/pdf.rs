use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Maximum PDF file size in bytes (25MB).
/// Anthropic has a 32MB max request size, but we leave headroom for the rest of the request.
const MAX_PDF_SIZE_BYTES: usize = 25 * 1024 * 1024;

/// A validated PDF document.
///
/// This type guarantees that:
/// - The file path has a `.pdf` extension
/// - The content has a valid PDF magic header (`%PDF-`)
/// - The content is within size limits for LLM providers
///
/// A `Pdf` can only be constructed via `Pdf::load()`, which enforces all validation rules.
/// Once you have a `Pdf`, you know it's valid by construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pdf {
    path: PathBuf,
    content: Arc<[u8]>,
}

/// Errors that can occur when loading a PDF.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfError {
    /// The file path doesn't have a .pdf extension.
    NotPdfExtension,
    /// The file doesn't have a valid PDF header.
    InvalidHeader,
    /// The file exceeds the maximum allowed size.
    TooLarge { size_bytes: usize },
    /// The file is empty.
    Empty,
}

impl std::fmt::Display for PdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfError::NotPdfExtension => {
                write!(f, "File does not have a .pdf extension")
            }
            PdfError::InvalidHeader => {
                write!(f, "File is not a valid PDF (missing %PDF- header)")
            }
            PdfError::TooLarge { size_bytes } => {
                let size_mb = *size_bytes as f64 / (1024.0 * 1024.0);
                let max_mb = MAX_PDF_SIZE_BYTES as f64 / (1024.0 * 1024.0);
                write!(
                    f,
                    "PDF file is too large ({:.1}MB). Maximum supported size is {:.0}MB",
                    size_mb, max_mb
                )
            }
            PdfError::Empty => {
                write!(f, "PDF file is empty")
            }
        }
    }
}

impl std::error::Error for PdfError {}

impl Pdf {
    /// Check if a path has a PDF extension.
    ///
    /// Use this for quick filtering before attempting to load a file as PDF.
    /// This only checks the extension, not the content.
    pub fn is_pdf_path(path: &Path) -> bool {
        has_pdf_extension(path)
    }

    /// Load and validate a PDF from a path and its content.
    ///
    /// This validates:
    /// - The path has a `.pdf` extension
    /// - The content starts with the PDF magic header (`%PDF-`)
    /// - The content is within size limits
    ///
    /// # Example
    /// ```ignore
    /// let content = std::fs::read("document.pdf")?;
    /// let pdf = Pdf::load("document.pdf", content)?;
    /// println!("Loaded PDF: {} ({} bytes)", pdf.path().display(), pdf.size());
    /// ```
    pub fn load(path: impl Into<PathBuf>, content: Vec<u8>) -> Result<Self, PdfError> {
        let path = path.into();

        if !has_pdf_extension(&path) {
            return Err(PdfError::NotPdfExtension);
        }

        if content.is_empty() {
            return Err(PdfError::Empty);
        }

        if !content.starts_with(b"%PDF-") {
            return Err(PdfError::InvalidHeader);
        }

        if content.len() > MAX_PDF_SIZE_BYTES {
            return Err(PdfError::TooLarge {
                size_bytes: content.len(),
            });
        }

        Ok(Self {
            path,
            content: content.into(),
        })
    }

    /// Returns the path of the PDF file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the raw PDF content.
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Returns the size of the PDF in bytes.
    pub fn size(&self) -> usize {
        self.content.len()
    }

    /// Encodes the PDF content as base64.
    ///
    /// This is the format expected by LLM APIs for document content.
    pub fn to_base64(&self) -> String {
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD.encode(&self.content)
    }
}

fn has_pdf_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pdf_content(size: usize) -> Vec<u8> {
        let mut content = b"%PDF-1.4 ".to_vec();
        if size > content.len() {
            content.resize(size, b'x');
        }
        content
    }

    #[test]
    fn test_is_pdf_path() {
        assert!(Pdf::is_pdf_path(Path::new("document.pdf")));
        assert!(Pdf::is_pdf_path(Path::new("document.PDF")));
        assert!(Pdf::is_pdf_path(Path::new("document.Pdf")));
        assert!(Pdf::is_pdf_path(Path::new("/path/to/document.pdf")));

        assert!(!Pdf::is_pdf_path(Path::new("document.txt")));
        assert!(!Pdf::is_pdf_path(Path::new("document.doc")));
        assert!(!Pdf::is_pdf_path(Path::new("document")));
        assert!(!Pdf::is_pdf_path(Path::new("pdf_document.txt")));
    }

    #[test]
    fn test_load_valid_pdf() {
        let content = b"%PDF-1.4 test content".to_vec();
        let pdf = Pdf::load("test.pdf", content.clone()).unwrap();

        assert_eq!(pdf.path(), Path::new("test.pdf"));
        assert_eq!(pdf.content(), &content[..]);
        assert_eq!(pdf.size(), content.len());
    }

    #[test]
    fn test_load_various_pdf_versions() {
        assert!(Pdf::load("a.pdf", b"%PDF-1.0".to_vec()).is_ok());
        assert!(Pdf::load("b.pdf", b"%PDF-1.4".to_vec()).is_ok());
        assert!(Pdf::load("c.pdf", b"%PDF-1.7".to_vec()).is_ok());
        assert!(Pdf::load("d.pdf", b"%PDF-2.0".to_vec()).is_ok());
    }

    #[test]
    fn test_load_wrong_extension() {
        let content = b"%PDF-1.4 valid content".to_vec();
        let result = Pdf::load("document.txt", content);
        assert_eq!(result.unwrap_err(), PdfError::NotPdfExtension);
    }

    #[test]
    fn test_load_empty_content() {
        let result = Pdf::load("test.pdf", vec![]);
        assert_eq!(result.unwrap_err(), PdfError::Empty);
    }

    #[test]
    fn test_load_invalid_header() {
        let result = Pdf::load("test.pdf", b"not a pdf".to_vec());
        assert_eq!(result.unwrap_err(), PdfError::InvalidHeader);

        let result = Pdf::load("test.pdf", b"PDF-1.4".to_vec());
        assert_eq!(result.unwrap_err(), PdfError::InvalidHeader);

        let result = Pdf::load("test.pdf", b"%PDF".to_vec());
        assert_eq!(result.unwrap_err(), PdfError::InvalidHeader);
    }

    #[test]
    fn test_load_size_at_limit() {
        let content = make_pdf_content(MAX_PDF_SIZE_BYTES);
        assert!(Pdf::load("test.pdf", content).is_ok());
    }

    #[test]
    fn test_load_size_over_limit() {
        let content = make_pdf_content(MAX_PDF_SIZE_BYTES + 1);
        let result = Pdf::load("test.pdf", content);
        assert!(matches!(result.unwrap_err(), PdfError::TooLarge { .. }));
    }

    #[test]
    fn test_to_base64() {
        let content = b"%PDF-1.4 test".to_vec();
        let pdf = Pdf::load("test.pdf", content).unwrap();
        let base64 = pdf.to_base64();

        use base64::Engine as _;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&base64)
            .unwrap();
        assert_eq!(decoded, b"%PDF-1.4 test");
    }

    #[test]
    fn test_error_display() {
        assert_eq!(
            PdfError::NotPdfExtension.to_string(),
            "File does not have a .pdf extension"
        );

        assert!(
            PdfError::InvalidHeader
                .to_string()
                .contains("not a valid PDF")
        );

        assert_eq!(PdfError::Empty.to_string(), "PDF file is empty");

        let error = PdfError::TooLarge {
            size_bytes: 30 * 1024 * 1024,
        };
        let msg = error.to_string();
        assert!(msg.contains("too large"));
        assert!(msg.contains("30.0MB"));
    }
}
