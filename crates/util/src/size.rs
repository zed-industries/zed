pub fn format_file_size(size: u64, use_decimal: bool) -> String {
    if use_decimal {
        if size < 1000 {
            format!("{size}B")
        } else if size < 1000 * 1000 {
            format!("{:.1}KB", size as f64 / 1000.0)
        } else {
            format!("{:.1}MB", size as f64 / (1000.0 * 1000.0))
        }
    } else if size < 1024 {
        format!("{size}B")
    } else if size < 1024 * 1024 {
        format!("{:.1}KiB", size as f64 / 1024.0)
    } else {
        format!("{:.1}MiB", size as f64 / (1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size_decimal() {
        assert_eq!(format_file_size(0, true), "0B");
        assert_eq!(format_file_size(999, true), "999B");
        assert_eq!(format_file_size(1000, true), "1.0KB");
        assert_eq!(format_file_size(1500, true), "1.5KB");
        assert_eq!(format_file_size(999999, true), "1000.0KB");
        assert_eq!(format_file_size(1000000, true), "1.0MB");
        assert_eq!(format_file_size(1500000, true), "1.5MB");
        assert_eq!(format_file_size(10000000, true), "10.0MB");
    }

    #[test]
    fn test_format_file_size_binary() {
        assert_eq!(format_file_size(0, false), "0B");
        assert_eq!(format_file_size(1023, false), "1023B");
        assert_eq!(format_file_size(1024, false), "1.0KiB");
        assert_eq!(format_file_size(1536, false), "1.5KiB");
        assert_eq!(format_file_size(1048575, false), "1024.0KiB");
        assert_eq!(format_file_size(1048576, false), "1.0MiB");
        assert_eq!(format_file_size(1572864, false), "1.5MiB");
        assert_eq!(format_file_size(10485760, false), "10.0MiB");
    }
}
