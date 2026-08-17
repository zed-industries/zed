use super::pixel::{ComponentSlice, ComponentMap, ColorComponentMap};
#[cfg(feature = "as-bytes")]
use super::pixel::{ComponentBytes};
use crate::alt::{BGRA, ARGB, ABGR, BGR};
use crate::{RGB, RGBA};
use core::fmt;

impl<T, A> RGBA<T, A> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self { r, g, b, a }
    }
}

impl<T> BGRA<T> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// Warning: The order of arguments is R,G,B,A
    #[deprecated(note = "This function has a misleading order of arguments. Use BGRA{} literal instead")]
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { b, g, r, a }
    }
}

/// ```rust,compile_fail
/// let r = rgb::BGRA::<u8,u16>::zeroed();
/// ```
impl<T, A> BGRA<T, A> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// Warning: The order of arguments is R,G,B,A
    #[deprecated(note = "This function has a misleading order of arguments. Use BGRA{} literal instead")]
    pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self { b, g, r, a }
    }
}

impl<T> ARGB<T> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    #[deprecated(note = "This function has a misleading order of arguments. Use ARGB{} literal instead")]
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { a, r, g, b }
    }
}

impl<T, A> ARGB<T, A> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    #[deprecated(note = "This function has a misleading order of arguments. Use ARGB{} literal instead")]
    pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self { a, r, g, b }
    }
}

impl<T> ABGR<T> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    #[deprecated(note = "This function has a misleading order of arguments. Use ABGR{} literal instead")]
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { a, b, g, r }
    }
}

impl<T, A> ABGR<T, A> {
    #[inline(always)]
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B,A
    #[deprecated(note = "This function has a misleading order of arguments. Use ABGR{} literal instead")]
    pub const fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self { a, b, g, r }
    }
}

macro_rules! impl_rgba {
    ($RGBA:ident) => {
        impl<T: Clone> $RGBA<T> {
            /// Iterate over all components (length=4)
            #[inline(always)]
            pub fn iter(&self) -> core::iter::Cloned<core::slice::Iter<'_, T>> {
                self.as_slice().iter().cloned()
            }
        }

        impl<T: Clone, A> $RGBA<T, A> {
            /// Copy RGB components out of the RGBA struct
            ///
            /// Note: you can use `.into()` to convert between other types
            #[inline(always)]
            pub fn bgr(&self) -> BGR<T> {
                BGR {
                    r: self.r.clone(),
                    g: self.g.clone(),
                    b: self.b.clone(),
                }
            }
        }

        impl<T: Copy, A: Clone> $RGBA<T, A> {
            /// Create new RGBA with the same alpha value, but different RGB values
            #[deprecated(note = "Renamed to map_colors()")]
            pub fn map_rgb<F, U, B>(&self, mut f: F) -> $RGBA<U, B>
                where F: FnMut(T) -> U, U: Clone, B: From<A> + Clone
            {
                $RGBA {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                    a: self.a.clone().into(),
                }
            }

            #[doc(hidden)]
            #[deprecated(note = "use .with_alpha(a) instead")]
            /// Create a new RGBA with the new alpha value, but same RGB values
            pub fn alpha(&self, a: A) -> Self {
                self.with_alpha(a)
            }

            #[inline(always)]
            /// Create a new RGBA with the new alpha value, but same RGB values
            pub fn with_alpha(&self, a: A) -> Self {
                Self { r: self.r, g: self.g, b: self.b, a }
            }

            /// Create a new RGBA with a new alpha value created by the callback.
            /// Allows changing of the type used for the alpha channel.
            #[inline]
            pub fn map_alpha<F, B>(&self, f: F) -> $RGBA<T, B>
                where F: FnOnce(A) -> B {
                $RGBA {
                    r: self.r,
                    g: self.g,
                    b: self.b,
                    a: f(self.a.clone()),
                }
            }
        }

        impl<T: Copy, B> ComponentMap<$RGBA<B>, T, B> for $RGBA<T> {
            #[inline(always)]
            fn map<F>(&self, mut f: F) -> $RGBA<B>
            where F: FnMut(T) -> B {
                $RGBA {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                    a: f(self.a),
                }
            }
        }

        impl<T: Copy, A: Copy, B> ColorComponentMap<$RGBA<B, A>, T, B> for $RGBA<T, A> {
            #[inline(always)]
            fn map_colors<F>(&self, mut f: F) -> $RGBA<B, A>
            where F: FnMut(T) -> B {
                $RGBA {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                    a: self.a,
                }
            }
        }

        impl<T> ComponentSlice<T> for $RGBA<T> {
            #[inline(always)]
            fn as_slice(&self) -> &[T] {
                unsafe {
                    core::slice::from_raw_parts(self as *const Self as *const T, 4)
                }
            }

            #[inline(always)]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self as *mut Self as *mut T, 4)
                }
            }
        }

        impl<T> ComponentSlice<T> for [$RGBA<T>] {
            #[inline]
            fn as_slice(&self) -> &[T] {
                unsafe {
                    core::slice::from_raw_parts(self.as_ptr() as *const _, self.len() * 4)
                }
            }

            #[inline]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() * 4)
                }
            }
        }

        #[cfg(feature = "as-bytes")]
        impl<T: crate::Pod> ComponentBytes<T> for [$RGBA<T>] {}
    };
}

macro_rules! impl_alpha_conv {
    ($RGB:ident, $RGBA:ident) => {
        /// Assumes 255 is opaque
        impl<T: Copy> From<$RGB<T>> for $RGBA<T, u8> {
            #[inline(always)]
            fn from(other: $RGB<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFF,
                }
            }
        }

        /// Assumes 65535 is opaque
        impl<T: Copy> From<$RGB<T>> for $RGBA<T, u16> {
            #[inline(always)]
            fn from(other: $RGB<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFFFF,
                }
            }
        }
    };
}

impl<T, A> RGBA<T, A> {
    /// Provide a mutable view of only RGB components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[inline(always)]
    pub fn rgb_mut(&mut self) -> &mut RGB<T> {
        unsafe { &mut *(self as *mut Self).cast::<RGB<T>>() }
    }
}

impl<T, A> BGRA<T, A> {
    /// Provide a mutable view of only RGB components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[deprecated(note = "This function will change. Use bgr_mut()")]
    pub fn rgb_mut(&mut self) -> &mut BGR<T> {
        unsafe { &mut *(self as *mut Self).cast::<BGR<T>>() }
    }

    /// Provide a mutable view of only RGB components (leaving out alpha).
    /// Useful to change color without changing opacity.
    #[inline(always)]
    pub fn bgr_mut(&mut self) -> &mut BGR<T> {
        unsafe { &mut *(self as *mut Self).cast::<BGR<T>>() }
    }
}

impl<T> core::iter::FromIterator<T> for RGBA<T> {
    #[inline(always)]
    /// Takes exactly 4 elements from the iterator and creates a new instance.
    /// Panics if there are fewer elements in the iterator.
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
        let mut iter = into_iter.into_iter();
        Self {
            r: iter.next().unwrap(),
            g: iter.next().unwrap(),
            b: iter.next().unwrap(),
            a: iter.next().unwrap(),
        }
    }
}

impl<T: Clone, A> RGBA<T, A> {
    /// Copy RGB components out of the RGBA struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[inline(always)]
    pub fn rgb(&self) -> RGB<T> {
        RGB {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
        }
    }
}

impl<T: Clone, A> ARGB<T, A> {
    /// Copy RGB components out of the ARGB struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[inline(always)]
    pub fn rgb(&self) -> RGB<T> {
        RGB {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
        }
    }
}

impl<T: Clone, A> BGRA<T, A> {
    /// Copy RGB components out of the RGBA struct
    ///
    /// Note: you can use `.into()` to convert between other types
    #[deprecated(note = "This function will change. Use bgr()")]
    pub fn rgb(&self) -> BGR<T> {
        BGR {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
        }
    }
}

impl_rgba! {RGBA}
impl_rgba! {BGRA}
impl_rgba! {ARGB}
impl_rgba! {ABGR}

impl_alpha_conv! {BGR, BGRA}
impl_alpha_conv! {RGB, BGRA}
impl_alpha_conv! {BGR, RGBA}
impl_alpha_conv! {RGB, RGBA}
impl_alpha_conv! {BGR, ABGR}
impl_alpha_conv! {RGB, ABGR}
impl_alpha_conv! {BGR, ARGB}
impl_alpha_conv! {RGB, ARGB}

impl<T: fmt::Display, A: fmt::Display> fmt::Display for RGBA<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl<T: fmt::Display, A: fmt::Display> fmt::Display for BGRA<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bgra({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

#[test]
fn rgba_test() {
    let neg = RGBA::new(1,2,3i32,1000).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.rgb().r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.rgb().g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.rgb().b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(neg.map_alpha(|x| x+1).a, -999);
    assert_eq!(neg, neg.as_slice().iter().copied().collect());
    assert!(neg < RGBA::new(0,0,0,0));

    #[allow(deprecated)]
    let neg = RGBA::new(1u8,2,3,4).map_rgb(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4i16, neg.a);
    let neg = RGBA::new(1u8,2,3,4).map_colors(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4u8, neg.a);

    let mut px = RGBA{r:1,g:2,b:3,a:4};
    px.as_mut_slice()[3] = 100;
    assert_eq!(1, px.rgb_mut().r);
    assert_eq!(2, px.rgb_mut().g);
    px.rgb_mut().b = 4;
    assert_eq!(4, px.rgb_mut().b);
    assert_eq!(100, px.a);

    #[cfg(feature = "as-bytes")]
    {
        let v = vec![RGBA::new(1u8,2,3,4), RGBA::new(5,6,7,8)];
        assert_eq!(&[1,2,3,4,5,6,7,8], v.as_bytes());
    }
}

#[test]
#[cfg(feature = "as-bytes")]
fn abgr_test() {
    let abgr = ABGR {r:1,g:2,b:3,a:4};
    assert_eq!(4, abgr.as_slice()[0]);
    use crate::AsPixels;
    assert_eq!(abgr, [abgr].as_bytes().as_pixels()[0]);
}

#[test]
#[allow(deprecated)]
fn bgra_test() {
    let neg = BGRA::new(1, 2, 3i32, 1000).map(|x| -x);
    let _ = neg.as_slice();

    #[cfg(feature = "as-bytes")]
    {
        let _ = [neg].as_bytes();
    }
    assert_eq!(neg.r, -1);
    assert_eq!(neg.bgr().r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.bgr().g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.bgr().b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(&[-3,-2,-1,-1000], neg.as_slice());
    assert!(neg < BGRA::new(0, 0, 0, 0));

    let neg = BGRA::new(1u8, 2u8, 3u8, 4u8).map_rgb(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4i16, neg.a);
    #[allow(deprecated)]
    let neg = BGRA::new(1u8, 2u8, 3u8, 4u8).map_c(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4u8, neg.a);

    let mut px = BGRA{r:1,g:2,b:3,a:-9}.alpha(4);
    px.as_mut_slice()[3] = 100;
    assert_eq!(1, px.bgr_mut().r);
    assert_eq!(2, px.bgr_mut().g);
    px.bgr_mut().b = 4;
    assert_eq!(4, px.bgr_mut().b);
    assert_eq!(100, px.a);


    #[cfg(feature = "as-bytes")]
    {
        let v = vec![BGRA::new(3u8, 2, 1, 4), BGRA::new(7, 6, 5, 8)];
        assert_eq!(&[1,2,3,4,5,6,7,8], v.as_bytes());
    }
}
