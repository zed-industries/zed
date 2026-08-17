//! Channel orders for packed RGBA types.

use crate::{cast::ComponentOrder, rgb};

/// RGBA color packed in ABGR order.
///
/// See [Packed](crate::cast::Packed) for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Abgr;

impl<S, T> ComponentOrder<rgb::Rgba<S, T>, [T; 4]> for Abgr {
    #[inline]
    fn pack(color: rgb::Rgba<S, T>) -> [T; 4] {
        let [red, green, blue, alpha]: [T; 4] = color.into();
        [alpha, blue, green, red]
    }

    #[inline]
    fn unpack(packed: [T; 4]) -> rgb::Rgba<S, T> {
        let [alpha, blue, green, red] = packed;
        rgb::Rgba::new(red, green, blue, alpha)
    }
}

/// RGBA color packed in ARGB order.
///
/// See [Packed](crate::cast::Packed) for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Argb;

impl<S, T> ComponentOrder<rgb::Rgba<S, T>, [T; 4]> for Argb {
    #[inline]
    fn pack(color: rgb::Rgba<S, T>) -> [T; 4] {
        let [red, green, blue, alpha]: [T; 4] = color.into();
        [alpha, red, green, blue]
    }

    #[inline]
    fn unpack(packed: [T; 4]) -> rgb::Rgba<S, T> {
        let [alpha, red, green, blue] = packed;
        rgb::Rgba::new(red, green, blue, alpha)
    }
}

/// RGBA color packed in BGRA order.
///
/// See [Packed](crate::cast::Packed) for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bgra;

impl<S, T> ComponentOrder<rgb::Rgba<S, T>, [T; 4]> for Bgra {
    #[inline]
    fn pack(color: rgb::Rgba<S, T>) -> [T; 4] {
        let [red, green, blue, alpha]: [T; 4] = color.into();
        [blue, green, red, alpha]
    }

    #[inline]
    fn unpack(packed: [T; 4]) -> rgb::Rgba<S, T> {
        let [blue, green, red, alpha] = packed;
        rgb::Rgba::new(red, green, blue, alpha)
    }
}

/// RGBA color packed in RGBA order.
///
/// See [Packed](crate::cast::Packed) for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rgba;

impl<S, T> ComponentOrder<rgb::Rgba<S, T>, [T; 4]> for Rgba {
    #[inline]
    fn pack(color: rgb::Rgba<S, T>) -> [T; 4] {
        let [red, green, blue, alpha]: [T; 4] = color.into();
        [red, green, blue, alpha]
    }

    #[inline]
    fn unpack(packed: [T; 4]) -> rgb::Rgba<S, T> {
        let [red, green, blue, alpha] = packed;
        rgb::Rgba::new(red, green, blue, alpha)
    }
}

#[cfg(feature = "approx")]
#[cfg(test)]
mod test {
    use super::{Abgr, Argb, Bgra, Rgba};
    use crate::{cast::Packed, Srgb, Srgba};

    #[test]
    fn rgba() {
        let a1: Packed<Rgba, u32> = Srgb::new(0.5, 0.0, 0.0).into_format().into();
        let a2: Packed<Rgba, u32> = Srgb::new(0.0, 1.0, 0.0).into_format().into();
        let a3: Packed<Rgba, u32> = Srgb::new(0.0, 0.0, 0.5).into_format().into();
        let x1: u32 = 0x8000_00FF;
        let x2: u32 = 0x00FF_00FF;
        let x3: u32 = 0x0000_80FF;
        assert_eq!(a1.color, x1);
        assert_eq!(a2.color, x2);
        assert_eq!(a3.color, x3);

        let unpacked: Srgb<u8> = Packed::<Rgba, u32>::from(0x80FF_80FF).into();
        assert_relative_eq!(
            Srgb::new(0.5, 1.0, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );

        let b1: Packed<Rgba, u32> = Srgba::new(0.5, 0.0, 0.0, 0.0).into_format().into();
        let b2: Packed<Rgba, u32> = Srgba::new(0.0, 1.0, 0.0, 0.0).into_format().into();
        let b3: Packed<Rgba, u32> = Srgba::new(0.0, 0.0, 0.5, 0.0).into_format().into();
        let b4: Packed<Rgba, u32> = Srgba::new(0.0, 0.0, 0.0, 1.0).into_format().into();
        let y1: u32 = 0x8000_0000;
        let y2: u32 = 0x00FF_0000;
        let y3: u32 = 0x0000_8000;
        let y4: u32 = 0x0000_00FF;
        assert_eq!(b1.color, y1);
        assert_eq!(b2.color, y2);
        assert_eq!(b3.color, y3);
        assert_eq!(b4.color, y4);

        let unpacked: Srgba<u8> = Packed::<Rgba, u32>::from(0x80FF_80FF).into();
        assert_relative_eq!(
            Srgba::new(0.5, 1.0, 0.5, 1.0),
            unpacked.into_format(),
            epsilon = 0.01
        );
    }

    #[test]
    fn argb() {
        let a1: Packed<Argb, u32> = Srgb::new(0.5, 0.0, 0.0).into_format().into();
        let a2: Packed<Argb, u32> = Srgb::new(0.0, 1.0, 0.0).into_format().into();
        let a3: Packed<Argb, u32> = Srgb::new(0.0, 0.0, 0.5).into_format().into();
        let x1: u32 = 0xFF80_0000;
        let x2: u32 = 0xFF00_FF00;
        let x3: u32 = 0xFF00_0080;
        assert_eq!(a1.color, x1);
        assert_eq!(a2.color, x2);
        assert_eq!(a3.color, x3);

        let unpacked: Srgb<u8> = Packed::<Argb, u32>::from(0x80FF_80FF).into();
        assert_relative_eq!(
            Srgb::new(1.0, 0.5, 1.0),
            unpacked.into_format(),
            epsilon = 0.01
        );

        let b1: Packed<Argb, u32> = Srgba::new(0.5, 0.0, 0.0, 0.0).into_format().into();
        let b2: Packed<Argb, u32> = Srgba::new(0.0, 1.0, 0.0, 0.0).into_format().into();
        let b3: Packed<Argb, u32> = Srgba::new(0.0, 0.0, 0.5, 0.0).into_format().into();
        let b4: Packed<Argb, u32> = Srgba::new(0.0, 0.0, 0.0, 1.0).into_format().into();
        let y1: u32 = 0x0080_0000;
        let y2: u32 = 0x0000_FF00;
        let y3: u32 = 0x0000_0080;
        let y4: u32 = 0xFF00_0000;
        assert_eq!(b1.color, y1);
        assert_eq!(b2.color, y2);
        assert_eq!(b3.color, y3);
        assert_eq!(b4.color, y4);

        let unpacked: Srgba<u8> = Packed::<Argb, u32>::from(0x80FF_80FF).into();
        assert_relative_eq!(
            Srgba::new(1.0, 0.5, 1.0, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );
    }

    #[test]
    fn bgra() {
        let a1: Packed<Bgra, u32> = Srgb::new(0.5, 0.0, 0.0).into_format().into();
        let a2: Packed<Bgra, u32> = Srgb::new(0.0, 1.0, 0.0).into_format().into();
        let a3: Packed<Bgra, u32> = Srgb::new(0.0, 0.0, 0.5).into_format().into();
        let x1: u32 = 0x0000_80FF;
        let x2: u32 = 0x00FF_00FF;
        let x3: u32 = 0x8000_00FF;
        assert_eq!(a1.color, x1);
        assert_eq!(a2.color, x2);
        assert_eq!(a3.color, x3);

        let unpacked: Srgb<u8> = Packed::<Bgra, u32>::from(0x80FF_FF80).into();
        assert_relative_eq!(
            Srgb::new(1.0, 1.0, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );

        let b1: Packed<Bgra, u32> = Srgba::new(0.5, 0.0, 0.0, 0.0).into_format().into();
        let b2: Packed<Bgra, u32> = Srgba::new(0.0, 1.0, 0.0, 0.0).into_format().into();
        let b3: Packed<Bgra, u32> = Srgba::new(0.0, 0.0, 0.5, 0.0).into_format().into();
        let b4: Packed<Bgra, u32> = Srgba::new(0.0, 0.0, 0.0, 1.0).into_format().into();
        let y1: u32 = 0x0000_8000;
        let y2: u32 = 0x00FF_0000;
        let y3: u32 = 0x8000_0000;
        let y4: u32 = 0x0000_00FF;
        assert_eq!(b1.color, y1);
        assert_eq!(b2.color, y2);
        assert_eq!(b3.color, y3);
        assert_eq!(b4.color, y4);

        let unpacked: Srgba<u8> = Packed::<Bgra, u32>::from(0x80FF_FF80).into();
        assert_relative_eq!(
            Srgba::new(1.0, 1.0, 0.5, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );
    }

    #[test]
    fn abgr() {
        let a1: Packed<Abgr, u32> = Srgb::new(0.5, 0.0, 0.0).into_format().into();
        let a2: Packed<Abgr, u32> = Srgb::new(0.0, 1.0, 0.0).into_format().into();
        let a3: Packed<Abgr, u32> = Srgb::new(0.0, 0.0, 0.5).into_format().into();
        let x1: u32 = 0xFF00_0080;
        let x2: u32 = 0xFF00_FF00;
        let x3: u32 = 0xFF80_0000;
        assert_eq!(a1.color, x1);
        assert_eq!(a2.color, x2);
        assert_eq!(a3.color, x3);

        let unpacked: Srgb<u8> = Packed::<Abgr, u32>::from(0x80FF_FF80).into();
        assert_relative_eq!(
            Srgb::new(0.5, 1.0, 1.0),
            unpacked.into_format(),
            epsilon = 0.01
        );

        let b1: Packed<Abgr, u32> = Srgba::new(0.5, 0.0, 0.0, 0.0).into_format().into();
        let b2: Packed<Abgr, u32> = Srgba::new(0.0, 1.0, 0.0, 0.0).into_format().into();
        let b3: Packed<Abgr, u32> = Srgba::new(0.0, 0.0, 0.5, 0.0).into_format().into();
        let b4: Packed<Abgr, u32> = Srgba::new(0.0, 0.0, 0.0, 1.0).into_format().into();
        let y1: u32 = 0x0000_0080;
        let y2: u32 = 0x0000_FF00;
        let y3: u32 = 0x0080_0000;
        let y4: u32 = 0xFF00_0000;
        assert_eq!(b1.color, y1);
        assert_eq!(b2.color, y2);
        assert_eq!(b3.color, y3);
        assert_eq!(b4.color, y4);

        let unpacked: Srgba<u8> = Packed::<Abgr, u32>::from(0x80FF_FF80).into();
        assert_relative_eq!(
            Srgba::new(0.5, 1.0, 1.0, 0.5),
            unpacked.into_format(),
            epsilon = 0.01
        );
    }

    #[test]
    fn u32_to_color() {
        assert_eq!(0xFFFF_FF80, u32::from(Srgb::new(255u8, 255, 128)));
        assert_eq!(0x7FFF_FF80, u32::from(Srgba::new(127u8, 255u8, 255, 128)));
    }
}
