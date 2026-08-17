#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unexpected pointer width for target platform");

use std::{
    ffi::OsString,
    io::{Error, ErrorKind},
};

use web_sys::window;

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Platform, Result,
};

// Get the user agent
fn user_agent() -> Option<String> {
    window()?.navigator().user_agent().ok()
}

// Get the document domain
fn document_domain() -> Option<String> {
    window()?.document()?.location()?.hostname().ok()
}

impl Target for Os {
    fn langs(self) -> Result<String> {
        if let Some(window) = window() {
            Ok(window
                .navigator()
                .languages()
                .to_vec()
                .into_iter()
                .filter_map(|l| l.as_string())
                .collect::<Vec<String>>()
                .join(";"))
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "Failed to retrieve languages: Window object is missing",
            ))
        }
    }

    fn realname(self) -> Result<OsString> {
        Ok("Anonymous".to_string().into())
    }

    fn username(self) -> Result<OsString> {
        Ok("anonymous".to_string().into())
    }

    fn devicename(self) -> Result<OsString> {
        let orig_string = user_agent().unwrap_or_default();
        let start = if let Some(s) = orig_string.rfind(' ') {
            s
        } else {
            return Ok("Unknown Browser".to_string().into());
        };
        let string = orig_string
            .get(start + 1..)
            .unwrap_or("Unknown Browser")
            .replace('/', " ");
        let string = if let Some(s) = string.rfind("Safari") {
            if let Some(s) = orig_string.rfind("Chrome") {
                if let Some(e) =
                    orig_string.get(s..).unwrap_or_default().find(' ')
                {
                    orig_string
                        .get(s..)
                        .unwrap_or("Chrome")
                        .get(..e)
                        .unwrap_or("Chrome")
                        .replace('/', " ")
                } else {
                    "Chrome".to_string()
                }
            } else if orig_string.contains("Linux") {
                "GNOME Web".to_string()
            } else {
                string.get(s..).unwrap_or("Safari").replace('/', " ")
            }
        } else if string.contains("Edg ") {
            string.replace("Edg ", "Edge ")
        } else if string.contains("OPR ") {
            string.replace("OPR ", "Opera ")
        } else {
            string
        };

        Ok(string.into())
    }

    fn hostname(self) -> Result<String> {
        document_domain()
            .filter(|x| !x.is_empty())
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    "Domain missing, failed to retrieve document domain from window"
                )
            })
    }

    fn distro(self) -> Result<String> {
        let string = user_agent()
            .ok_or_else(|| Error::from(ErrorKind::PermissionDenied))?;
        let err = || Error::new(ErrorKind::InvalidData, "Parsing failed");
        let begin = string.find('(').ok_or_else(err)?;
        let end = string.find(')').ok_or_else(err)?;
        let string = &string[begin + 1..end];

        Ok(if string.contains("Win32") || string.contains("Win64") {
            let begin = if let Some(b) = string.find("NT") {
                b
            } else {
                return Ok("Windows".to_string());
            };
            let end = if let Some(e) = string.find('.') {
                e
            } else {
                return Ok("Windows".to_string());
            };
            let string = &string[begin + 3..end];

            format!("Windows {}", string)
        } else if string.contains("Linux") {
            let string = if string.contains("X11") || string.contains("Wayland")
            {
                let begin = if let Some(b) = string.find(';') {
                    b
                } else {
                    return Ok("Unknown Linux".to_string());
                };
                &string[begin + 2..]
            } else {
                string
            };

            if string.starts_with("Linux") {
                "Unknown Linux".to_string()
            } else {
                let end = if let Some(e) = string.find(';') {
                    e
                } else {
                    return Ok("Unknown Linux".to_string());
                };
                string[..end].to_string()
            }
        } else if let Some(begin) = string.find("Mac OS X") {
            if let Some(end) = string[begin..].find(';') {
                string[begin..begin + end].to_string()
            } else {
                string[begin..].to_string().replace('_', ".")
            }
        } else {
            string.to_string()
        })
    }

    #[inline(always)]
    fn desktop_env(self) -> DesktopEnv {
        DesktopEnv::WebBrowser
    }

    fn platform(self) -> Platform {
        let string = user_agent().unwrap_or_default();
        let begin = if let Some(b) = string.find('(') {
            b
        } else {
            return Platform::Unknown("Unknown".to_string());
        };
        let end = if let Some(e) = string.find(')') {
            e
        } else {
            return Platform::Unknown("Unknown".to_string());
        };
        let string = &string[begin + 1..end];

        if string.contains("Win32") || string.contains("Win64") {
            Platform::Windows
        } else if string.contains("Linux") {
            Platform::Linux
        } else if string.contains("Mac OS X") {
            Platform::MacOS
        } else {
            Platform::Unknown(string.to_string())
        }
    }

    #[inline(always)]
    fn arch(self) -> Result<Arch> {
        Ok(if cfg!(target_pointer_width = "64") {
            Arch::Wasm64
        } else {
            Arch::Wasm32
        })
    }
}
