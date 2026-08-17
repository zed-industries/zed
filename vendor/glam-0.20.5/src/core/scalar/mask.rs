use crate::core::{
    storage::{XY, XYZ, XYZW},
    traits::{scalar::*, vector::*},
};

impl MaskConst for u32 {
    const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
}

impl MaskConst for u64 {
    const MASK: [u64; 2] = [0, 0xff_ff_ff_ff_ff_ff_ff_ff];
}

// u32 (currently unused)

/*
impl MaskVectorConst for XY<u32> {
    const FALSE: Self = Self { x: 0, y: 0 };
}

impl MaskVectorConst for XYZ<u32> {
    const FALSE: Self = Self { x: 0, y: 0, z: 0 };
}

impl MaskVectorConst for XYZW<u32> {
    const FALSE: Self = Self {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
    };
}

impl MaskVector for XY<u32> {
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x & other.x,
            y: self.y & other.y,
        }
    }

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x | other.x,
            y: self.y | other.y,
        }
    }

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl MaskVector for XYZ<u32> {
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x & other.x,
            y: self.y & other.y,
            z: self.z & other.z,
        }
    }

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x | other.x,
            y: self.y | other.y,
            z: self.z | other.z,
        }
    }

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }
}

impl MaskVector for XYZW<u32> {
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x & other.x,
            y: self.y & other.y,
            z: self.z & other.z,
            w: self.w & other.w,
        }
    }

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x | other.x,
            y: self.y | other.y,
            z: self.z | other.z,
            w: self.w | other.w,
        }
    }

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
            w: !self.w,
        }
    }
}

impl MaskVector2 for XY<u32> {
    #[inline(always)]
    fn new(x: bool, y: bool) -> Self {
        Self {
            x: MaskConst::MASK[x as usize],
            y: MaskConst::MASK[y as usize],
        }
    }

    #[inline]
    fn bitmask(self) -> u32 {
        (self.x as u32 & 0x1) | (self.y as u32 & 0x1) << 1
    }

    #[inline]
    fn any(self) -> bool {
        ((self.x | self.y) & 0x1) != 0
    }

    #[inline]
    fn all(self) -> bool {
        ((self.x & self.y) & 0x1) != 0
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 2] {
        [self.x != 0, self.y != 0]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 2] {
        [self.x, self.y]
    }
}

impl MaskVector3 for XYZ<u32> {
    #[inline(always)]
    fn new(x: bool, y: bool, z: bool) -> Self {
        // A SSE2 mask can be any bit pattern but for the `Vec3Mask` implementation of select
        // we expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type
        // can only be created via this function or by `Vec3` methods.
        Self {
            x: MaskConst::MASK[x as usize],
            y: MaskConst::MASK[y as usize],
            z: MaskConst::MASK[z as usize],
        }
    }

    #[inline]
    fn bitmask(self) -> u32 {
        (self.x & 0x1) | (self.y & 0x1) << 1 | (self.z & 0x1) << 2
    }

    #[inline]
    fn any(self) -> bool {
        ((self.x | self.y | self.z) & 0x1) != 0
    }

    #[inline]
    fn all(self) -> bool {
        ((self.x & self.y & self.z) & 0x1) != 0
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 3] {
        [self.x != 0, self.y != 0, self.z != 0]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 3] {
        [self.x, self.y, self.z]
    }
}

impl MaskVector4 for XYZW<u32> {
    #[inline(always)]
    fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        // A SSE2 mask can be any bit pattern but for the `Vec4Mask` implementation of select
        // we expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type
        // can only be created via this function or by `Vec4` methods.
        Self {
            x: MaskConst::MASK[x as usize],
            y: MaskConst::MASK[y as usize],
            z: MaskConst::MASK[z as usize],
            w: MaskConst::MASK[w as usize],
        }
    }

    #[inline]
    fn bitmask(self) -> u32 {
        (self.x & 0x1) | (self.y & 0x1) << 1 | (self.z & 0x1) << 2 | (self.w & 0x1) << 3
    }

    #[inline]
    fn any(self) -> bool {
        ((self.x | self.y | self.z | self.w) & 0x1) != 0
    }

    #[inline]
    fn all(self) -> bool {
        ((self.x & self.y & self.z & self.w) & 0x1) != 0
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 4] {
        [self.x != 0, self.y != 0, self.z != 0, self.w != 0]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
*/

// bool

impl MaskVectorConst for XY<bool> {
    const FALSE: Self = Self { x: false, y: false };
}

impl MaskVectorConst for XYZ<bool> {
    const FALSE: Self = Self {
        x: false,
        y: false,
        z: false,
    };
}

impl MaskVectorConst for XYZW<bool> {
    const FALSE: Self = Self {
        x: false,
        y: false,
        z: false,
        w: false,
    };
}

impl MaskVector for XY<bool> {
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x && other.x,
            y: self.y && other.y,
        }
    }

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x || other.x,
            y: self.y || other.y,
        }
    }

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl MaskVector for XYZ<bool> {
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x && other.x,
            y: self.y && other.y,
            z: self.z && other.z,
        }
    }

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x || other.x,
            y: self.y || other.y,
            z: self.z || other.z,
        }
    }

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }
}

impl MaskVector for XYZW<bool> {
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x && other.x,
            y: self.y && other.y,
            z: self.z && other.z,
            w: self.w && other.w,
        }
    }

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x || other.x,
            y: self.y || other.y,
            z: self.z || other.z,
            w: self.w || other.w,
        }
    }

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
            w: !self.w,
        }
    }
}

impl MaskVector2 for XY<bool> {
    #[inline(always)]
    fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }

    #[inline]
    fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1
    }

    #[inline]
    fn any(self) -> bool {
        self.x || self.y
    }

    #[inline]
    fn all(self) -> bool {
        self.x && self.y
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 2] {
        [self.x, self.y]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 2] {
        [
            MaskConst::MASK[self.x as usize],
            MaskConst::MASK[self.y as usize],
        ]
    }
}

impl MaskVector3 for XYZ<bool> {
    #[inline(always)]
    fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }

    #[inline]
    fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2
    }

    #[inline]
    fn any(self) -> bool {
        self.x || self.y || self.z
    }

    #[inline]
    fn all(self) -> bool {
        self.x && self.y && self.z
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 3] {
        [
            MaskConst::MASK[self.x as usize],
            MaskConst::MASK[self.y as usize],
            MaskConst::MASK[self.z as usize],
        ]
    }
}

impl MaskVector4 for XYZW<bool> {
    #[inline(always)]
    fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2 | (self.w as u32) << 3
    }

    #[inline]
    fn any(self) -> bool {
        self.x || self.y || self.z || self.w
    }

    #[inline]
    fn all(self) -> bool {
        self.x && self.y && self.z && self.w
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 4] {
        [
            MaskConst::MASK[self.x as usize],
            MaskConst::MASK[self.y as usize],
            MaskConst::MASK[self.z as usize],
            MaskConst::MASK[self.w as usize],
        ]
    }
}
