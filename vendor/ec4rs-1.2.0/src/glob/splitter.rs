// Problem.
// OsStr cannot be cast to &[u8] on Windows.
// On Unixes and WASM it's fine.

#[cfg(target_family = "unix")]
mod cnv {
    use std::ffi::OsStr;
    #[allow(clippy::unnecessary_wraps)]
    pub fn to_bytes(s: &OsStr) -> Option<&[u8]> {
        use std::os::unix::ffi::OsStrExt;
        Some(s.as_bytes())
    }
}

#[cfg(target_os = "wasi")]
mod cnv {
    use std::ffi::OsStr;
    #[allow(clippy::unnecessary_wraps)]
    pub fn to_bytes(s: &OsStr) -> Option<&[u8]> {
        use std::os::wasi::ffi::OsStrExt;
        Some(s.as_bytes())
    }
}

#[cfg(all(not(target_family = "unix"), not(target_os = "wasi")))]
mod cnv {
    use std::ffi::OsStr;
    pub fn to_bytes(s: &OsStr) -> Option<&[u8]> {
        s.to_str().map(|s| s.as_ref())
    }
}

#[derive(Clone)]
pub struct Splitter<'a> {
    iter: std::path::Components<'a>,
    part: &'a [u8],
    matched_sep: bool,
}

impl<'a> Splitter<'a> {
    pub fn new(path: &'a std::path::Path) -> Option<Self> {
        Splitter {
            iter: path.components(),
            part: "".as_bytes(),
            matched_sep: false,
        }
        .next()
    }

    pub fn match_end(mut self) -> Option<Self> {
        use std::path::Component as C;
        if self.part.is_empty() {
            let next = self.iter.next_back();
            if matches!(next, None | Some(C::CurDir | C::RootDir | C::Prefix(_))) {
                return Some(self);
            }
        }
        None
    }

    pub fn next(mut self) -> Option<Self> {
        use std::path::Component;
        self.part = match self.iter.next_back()? {
            Component::Normal(p) => cnv::to_bytes(p)?,
            Component::ParentDir => "..".as_bytes(),
            _ => "".as_bytes(),
        };
        Some(self)
    }

    pub fn match_any(mut self, path_sep: bool) -> Option<Self> {
        if !self.part.is_empty() {
            self.part = self.part.split_last().unwrap().1;
            Some(self)
        } else if path_sep {
            self.match_sep()?.next()
        } else {
            None
        }
    }

    pub fn next_char(mut self) -> Option<(Self, char)> {
        if let Some((idx, c)) = self.find_next_char() {
            self.part = self.part.split_at(idx).0;
            Some((self, c))
        } else {
            Some((self.next()?, '/'))
        }
    }

    fn find_next_char(&self) -> Option<(usize, char)> {
        let mut idx = self.part.len().checked_sub(1)?;
        let mut byte = self.part[idx];
        while byte.leading_ones() == 1 {
            idx = idx.checked_sub(1)?;
            byte = self.part[idx];
        }
        // TODO: Do the UTF-8 character decode here ourselves.
        let c = std::str::from_utf8(&self.part[idx..])
            .ok()?
            .chars()
            .next_back()?;
        Some((idx, c))
    }

    pub fn match_sep(mut self) -> Option<Self> {
        let is_empty = self.part.is_empty();
        is_empty.then(|| {
            self.matched_sep = true;
            self
        })
    }

    pub fn match_suffix(mut self, suffix: &str) -> Option<Self> {
        if self.part.is_empty() && self.matched_sep {
            self.matched_sep = false;
            self = self.next()?;
        }
        if let Some(rest) = self.part.strip_suffix(suffix.as_bytes()) {
            self.part = rest;
            Some(self)
        } else {
            None
        }
    }

    pub fn match_number(mut self, lower: isize, upper: isize) -> Option<Self> {
        let mut q = std::collections::VecDeque::<char>::new();
        let mut allow_zero: bool = true;
        let mut last_ok = self.clone();
        while let Some((next_ok, c)) = self.next_char() {
            if c.is_numeric() && (c != '0' || allow_zero) {
                last_ok = next_ok.clone();
                allow_zero = c == '0';
                q.push_front(c);
            } else if c == '-' {
                last_ok = next_ok.clone();
                q.push_front('-');
                break;
            } else {
                break;
            }
            self = next_ok;
        }
        let i = q.iter().collect::<String>().parse::<isize>().ok()?;
        if i < lower || i > upper {
            return None;
        }
        Some(last_ok)
    }
}
