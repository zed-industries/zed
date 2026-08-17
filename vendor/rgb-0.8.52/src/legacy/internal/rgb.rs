use super::pixel::{ComponentSlice, ComponentMap, ColorComponentMap};
#[cfg(feature = "as-bytes")]
use super::pixel::{ComponentBytes};
use crate::alt::GRB;
use crate::alt::{BGR, BGRA};
use crate::{RGB, RGBA};
use core::fmt;

impl<T> BGR<T> {
    /// Convenience function for creating a new pixel
    /// Warning: The order of arguments is R,G,B
    #[deprecated(note = "This function has a misleading order of arguments. Use BGR{} literal instead")]
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { b, g, r }
    }
}

macro_rules! impl_rgb {
    ($RGB:ident) => {
        impl<T: Clone> $RGB<T> {
            /// Iterate over color components (R, G, and B)
            #[inline(always)]
            pub fn iter(&self) -> core::iter::Cloned<core::slice::Iter<'_, T>> {
                self.as_slice().iter().cloned()
            }
        }

        impl<T: Copy, B> ComponentMap<$RGB<B>, T, B> for $RGB<T> {
            #[inline(always)]
            fn map<F>(&self, mut f: F) -> $RGB<B>
                where F: FnMut(T) -> B {
                $RGB {
                    r:f(self.r),
                    g:f(self.g),
                    b:f(self.b),
                }
            }
        }

        impl<T: Copy, B> ColorComponentMap<$RGB<B>, T, B> for $RGB<T> {
            #[inline(always)]
            fn map_colors<F>(&self, mut f: F) -> $RGB<B>
                where F: FnMut(T) -> B {
                $RGB {
                    r:f(self.r),
                    g:f(self.g),
                    b:f(self.b),
                }
            }
        }

        impl<T> ComponentSlice<T> for $RGB<T> {
            #[inline(always)]
            fn as_slice(&self) -> &[T] {
                unsafe {
                    core::slice::from_raw_parts(self as *const Self as *const T, 3)
                }
            }

            #[inline(always)]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self as *mut Self as *mut T, 3)
                }
            }
        }

        impl<T> ComponentSlice<T> for [$RGB<T>] {
            #[inline]
            fn as_slice(&self) -> &[T] {
                unsafe {
                    core::slice::from_raw_parts(self.as_ptr() as *const _, self.len() * 3)
                }
            }

            #[inline]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() * 3)
                }
            }
        }

        #[cfg(feature = "as-bytes")]
        impl<T: crate::Pod> ComponentBytes<T> for [$RGB<T>] {}
    };
}

macro_rules! impl_rgb_to_alpha {
    ($RGB:ident, $RGBA:ident) => {
        impl<T: Clone> $RGB<T> {
            /// Convenience function for converting to RGBA
            #[doc(hidden)]
            #[deprecated(note = "use .with_alpha(a) instead; this will become a getter in the future")]
            pub fn alpha(&self, a: T) -> $RGBA<T> {
                self.with_alpha(a)
            }

            /// Convenience function for converting to RGBA
            #[inline(always)]
            #[doc(alias = "alpha")]
            pub fn with_alpha(&self, a: T) -> $RGBA<T> {
                $RGBA {
                    r: self.r.clone(),
                    g: self.g.clone(),
                    b: self.b.clone(),
                    a,
                }
            }

            /// Convenience function for converting to RGBA with alpha channel of a different type than type of the pixels
            #[inline(never)]
            #[deprecated(note = "use .with_alpha(a) instead")]
            pub fn new_alpha<A>(&self, a: A) -> $RGBA<T, A> {
                $RGBA {
                    r: self.r.clone(),
                    g: self.g.clone(),
                    b: self.b.clone(),
                    a,
                }
            }
        }
    };
}

impl<T> core::iter::FromIterator<T> for RGB<T> {
    /// Takes exactly 3 elements from the iterator and creates a new instance.
    /// Panics if there are fewer elements in the iterator.
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
        let mut iter = into_iter.into_iter();
        Self {
            r: iter.next().unwrap(),
            g: iter.next().unwrap(),
            b: iter.next().unwrap(),
        }
    }
}

impl_rgb! {RGB}
impl_rgb_to_alpha! {RGB, RGBA}
impl_rgb! {BGR}
impl_rgb_to_alpha! {BGR, BGRA}
impl_rgb! {GRB}

impl<T: fmt::Display> fmt::Display for RGB<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for RGB<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB {{ #{:02X}{:02X}{:02X} }}", self.r, self.g, self.b)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for RGB<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB {{ #{:02x}{:02x}{:02x} }}", self.r, self.g, self.b)
    }
}

impl<T: fmt::Display> fmt::Display for BGR<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bgr({},{},{})", self.b, self.g, self.r)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for BGR<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BGR {{ #{:02X}{:02X}{:02X} }}", self.b, self.g, self.r)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for BGR<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BGR {{ #{:02x}{:02x}{:02x} }}", self.b, self.g, self.r)
    }
}

#[cfg(test)]
mod rgb_test {
    use super::*;

    #[test]
    fn grb_test() {
        let grb = GRB {g:1,r:2,b:3}.map(|c| c * 2) + 1;
        let rgb: crate::RGB8 = grb.into();
        assert_eq!(rgb, RGB::new(5,3,7));
    }

    #[test]
    fn sanity_check() {
        let neg = RGB::new(1,2,3i32).map(|x| -x);
        assert_eq!(neg.r, -1);
        assert_eq!(neg.g, -2);
        assert_eq!(neg.b, -3);

        let mut px = RGB::new(3,4,5);
        px.as_mut_slice()[1] = 111;
        assert_eq!(111, px.g);

        assert_eq!(RGBA::new(250,251,252,253), RGB::new(250,251,252).with_alpha(253));

        assert_eq!(RGB{r:1u8,g:2,b:3}, RGB::new(1u8,2,3));
        assert!(RGB{r:1u8,g:1,b:2} < RGB::new(2,1,1));

        let mut h = std::collections::HashSet::new();
        h.insert(px);
        assert!(h.contains(&RGB::new(3,111,5)));
        assert!(!h.contains(&RGB::new(111,5,3)));


        #[cfg(feature = "as-bytes")]
        {
            let v = vec![RGB::new(1u8,2,3), RGB::new(4,5,6)];
            assert_eq!(&[1,2,3,4,5,6], v.as_bytes());
        }

        assert_eq!(RGB::new(0u8,0,0), Default::default());
    }

    #[test]
    #[allow(deprecated)]
    fn test_fmt() {
        let red_rgb = RGB::new(255, 0, 0);
        let red_bgr = BGR::new(255, 0, 0);
        assert_eq!("RGB { #FF0000 }", &format!("{:X}", red_rgb));
        assert_eq!("BGR { #0000FF }", &format!("{:X}", red_bgr));

        assert_eq!("RGB { #ff0000 }", &format!("{:x}", red_rgb));
        assert_eq!("BGR { #0000ff }", &format!("{:x}", red_bgr));

        assert_eq!("rgb(255,0,0)", &format!("{}", red_rgb));
        assert_eq!("bgr(0,0,255)", &format!("{}", red_bgr));
    }
}
