/// A version number for a specific component of an OpenGL implementation
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub is_embedded: bool,
    pub revision: Option<u32>,
    pub vendor_info: String,
}

impl Version {
    /// Create a new OpenGL version number
    #[allow(dead_code)]
    pub(crate) fn new(major: u32, minor: u32, revision: Option<u32>, vendor_info: String) -> Self {
        Version {
            major: major,
            minor: minor,
            is_embedded: false,
            revision: revision,
            vendor_info,
        }
    }

    /// Create a new OpenGL ES version number
    #[allow(dead_code)]
    pub(crate) fn new_embedded(major: u32, minor: u32, vendor_info: String) -> Self {
        Version {
            major,
            minor,
            is_embedded: true,
            revision: None,
            vendor_info,
        }
    }

    /// According to the OpenGL specification, the version information is
    /// expected to follow the following syntax:
    ///
    /// ~~~bnf
    /// <major>       ::= <number>
    /// <minor>       ::= <number>
    /// <revision>    ::= <number>
    /// <vendor-info> ::= <string>
    /// <release>     ::= <major> "." <minor> ["." <release>]
    /// <version>     ::= <release> [" " <vendor-info>]
    /// ~~~
    ///
    /// Note that this function is intentionally lenient in regards to parsing,
    /// and will try to recover at least the first two version numbers without
    /// resulting in an `Err`.
    /// # Notes
    /// `WebGL 2` version returned as `OpenGL ES 3.0`
    pub(crate) fn parse(mut src: &str) -> Result<Version, &str> {
        let webgl_sig = "WebGL ";
        // According to the WebGL specification
        // VERSION	WebGL<space>1.0<space><vendor-specific information>
        // SHADING_LANGUAGE_VERSION	WebGL<space>GLSL<space>ES<space>1.0<space><vendor-specific information>
        let is_webgl = src.starts_with(webgl_sig);
        let is_es = if is_webgl {
            let pos = src.rfind(webgl_sig).unwrap_or(0);
            src = &src[pos + webgl_sig.len()..];
            true
        } else {
            let es_sig = " ES ";
            match src.rfind(es_sig) {
                Some(pos) => {
                    src = &src[pos + es_sig.len()..];
                    true
                }
                None => false,
            }
        };

        let glsl_es_sig = "GLSL ES ";
        let is_glsl = match src.find(glsl_es_sig) {
            Some(pos) => {
                src = &src[pos + glsl_es_sig.len()..];
                true
            }
            None => false,
        };

        let (version, vendor_info) = match src.find(' ') {
            Some(i) => (&src[..i], src[i + 1..].to_string()),
            None => (src, String::new()),
        };

        // TODO: make this even more lenient so that we can also accept
        // `<major> "." <minor> [<???>]`
        let mut it = version.split('.');
        let major = it.next().and_then(|s| s.parse().ok());
        let minor = it.next().and_then(|s| {
            let trimmed = if s.starts_with('0') {
                "0"
            } else {
                s.trim_end_matches('0')
            };
            trimmed.parse().ok()
        });
        let revision = if is_webgl {
            None
        } else {
            it.next().and_then(|s| s.parse().ok())
        };

        match (major, minor, revision) {
            (Some(major), Some(minor), revision) => Ok(Version {
                // Return WebGL 2.0 version as OpenGL ES 3.0
                major: if is_webgl && !is_glsl {
                    major + 1
                } else {
                    major
                },
                minor,
                is_embedded: is_es,
                revision,
                vendor_info,
            }),
            (_, _, _) => Err(src),
        }
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (
            self.major,
            self.minor,
            self.revision,
            self.vendor_info.as_str(),
        ) {
            (major, minor, Some(revision), "") => write!(f, "{}.{}.{}", major, minor, revision),
            (major, minor, None, "") => write!(f, "{}.{}", major, minor),
            (major, minor, Some(revision), vendor_info) => {
                write!(f, "{}.{}.{}, {}", major, minor, revision, vendor_info)
            }
            (major, minor, None, vendor_info) => write!(f, "{}.{}, {}", major, minor, vendor_info),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn test_version_parse() {
        assert_eq!(Version::parse("1"), Err("1"));
        assert_eq!(Version::parse("1."), Err("1."));
        assert_eq!(Version::parse("1 h3l1o. W0rld"), Err("1 h3l1o. W0rld"));
        assert_eq!(Version::parse("1. h3l1o. W0rld"), Err("1. h3l1o. W0rld"));
        assert_eq!(
            Version::parse("1.2.3"),
            Ok(Version::new(1, 2, Some(3), String::new()))
        );
        assert_eq!(
            Version::parse("1.2"),
            Ok(Version::new(1, 2, None, String::new()))
        );
        assert_eq!(
            Version::parse("1.2 h3l1o. W0rld"),
            Ok(Version::new(1, 2, None, "h3l1o. W0rld".to_string()))
        );
        assert_eq!(
            Version::parse("1.2.h3l1o. W0rld"),
            Ok(Version::new(1, 2, None, "W0rld".to_string()))
        );
        assert_eq!(
            Version::parse("1.2. h3l1o. W0rld"),
            Ok(Version::new(1, 2, None, "h3l1o. W0rld".to_string()))
        );
        assert_eq!(
            Version::parse("1.2.3.h3l1o. W0rld"),
            Ok(Version::new(1, 2, Some(3), "W0rld".to_string()))
        );
        assert_eq!(
            Version::parse("1.2.3 h3l1o. W0rld"),
            Ok(Version::new(1, 2, Some(3), "h3l1o. W0rld".to_string()))
        );
        assert_eq!(
            Version::parse("OpenGL ES 3.1"),
            Ok(Version::new_embedded(3, 1, String::new()))
        );
        assert_eq!(
            Version::parse("OpenGL ES 2.0 Google Nexus"),
            Ok(Version::new_embedded(2, 0, "Google Nexus".to_string()))
        );
        assert_eq!(
            Version::parse("GLSL ES 1.1"),
            Ok(Version::new_embedded(1, 1, String::new()))
        );
        assert_eq!(
            Version::parse("OpenGL ES GLSL ES 3.20"),
            Ok(Version::new_embedded(3, 2, String::new()))
        );
        assert_eq!(
            // WebGL 2.0 should parse as OpenGL ES 3.0
            Version::parse("WebGL 2.0 (OpenGL ES 3.0 Chromium)"),
            Ok(Version::new_embedded(
                3,
                0,
                "(OpenGL ES 3.0 Chromium)".to_string()
            ))
        );
        assert_eq!(
            Version::parse("WebGL GLSL ES 3.00 (OpenGL ES GLSL ES 3.0 Chromium)"),
            Ok(Version::new_embedded(
                3,
                0,
                "(OpenGL ES GLSL ES 3.0 Chromium)".to_string()
            ))
        );
    }
}
