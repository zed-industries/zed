pub static CSS: &[u8] = include_bytes!("fonts.css");
// An array of (file_name, file_contents) pairs
pub static LICENSES: [(&str, &[u8]); 2] = [
    (
        "fonts/OPEN-SANS-LICENSE.txt",
        include_bytes!("OPEN-SANS-LICENSE.txt"),
    ),
    (
        "fonts/SOURCE-CODE-PRO-LICENSE.txt",
        include_bytes!("SOURCE-CODE-PRO-LICENSE.txt"),
    ),
];
// An array of (file_name, file_contents) pairs
pub static OPEN_SANS: [(&str, &[u8]); 10] = [
    (
        "fonts/open-sans-v17-all-charsets-300.woff2",
        include_bytes!("open-sans-v17-all-charsets-300.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-300italic.woff2",
        include_bytes!("open-sans-v17-all-charsets-300italic.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-regular.woff2",
        include_bytes!("open-sans-v17-all-charsets-regular.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-italic.woff2",
        include_bytes!("open-sans-v17-all-charsets-italic.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-600.woff2",
        include_bytes!("open-sans-v17-all-charsets-600.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-600italic.woff2",
        include_bytes!("open-sans-v17-all-charsets-600italic.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-700.woff2",
        include_bytes!("open-sans-v17-all-charsets-700.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-700italic.woff2",
        include_bytes!("open-sans-v17-all-charsets-700italic.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-800.woff2",
        include_bytes!("open-sans-v17-all-charsets-800.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-800italic.woff2",
        include_bytes!("open-sans-v17-all-charsets-800italic.woff2"),
    ),
];

// A (file_name, file_contents) pair
pub static SOURCE_CODE_PRO: (&str, &[u8]) = (
    "fonts/source-code-pro-v11-all-charsets-500.woff2",
    include_bytes!("source-code-pro-v11-all-charsets-500.woff2"),
);
