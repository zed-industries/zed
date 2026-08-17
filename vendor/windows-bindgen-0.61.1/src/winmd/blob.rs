use super::*;

pub struct Blob {
    file: &'static File,
    slice: &'static [u8],
}

impl std::fmt::Debug for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.slice)
    }
}

impl std::ops::Deref for Blob {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.slice
    }
}

impl Blob {
    pub fn new(file: &'static File, slice: &'static [u8]) -> Self {
        Self { file, slice }
    }

    pub fn reader(&self) -> &'static Reader {
        self.file.reader()
    }

    fn peek(&self) -> (usize, usize) {
        if self[0] & 0x80 == 0 {
            (self[0] as usize, 1)
        } else if self[0] & 0xC0 == 0x80 {
            ((((self[0] & 0x3F) as usize) << 8) | self[1] as usize, 2)
        } else {
            (
                (((self[0] & 0x1F) as usize) << 24)
                    | ((self[1] as usize) << 16)
                    | ((self[2] as usize) << 8)
                    | self[3] as usize,
                4,
            )
        }
    }

    pub fn decode<D: Decode>(&mut self) -> D {
        D::decode(self.file, self.read_usize())
    }

    pub fn try_read(&mut self, expected: usize) -> bool {
        let (value, offset) = self.peek();
        if value == expected {
            self.offset(offset);
            true
        } else {
            false
        }
    }

    pub fn read_modifiers(&mut self) -> Vec<TypeDefOrRef> {
        let mut mods = vec![];
        loop {
            let (value, offset) = self.peek();
            if value != ELEMENT_TYPE_CMOD_OPT as usize && value != ELEMENT_TYPE_CMOD_REQD as usize {
                break;
            } else {
                self.offset(offset);
                mods.push(TypeDefOrRef::decode(self.file, self.read_usize()))
            }
        }
        mods
    }

    pub fn read_usize(&mut self) -> usize {
        let (value, offset) = self.peek();
        self.offset(offset);
        value
    }

    pub fn read_str(&mut self) -> &'static str {
        let len = self.read_usize();
        let value = unsafe { std::str::from_utf8_unchecked(&self.slice[..len]) };
        self.offset(len);
        value
    }

    pub fn read_utf16(self) -> String {
        let slice = self.slice;
        if slice.as_ptr().align_offset(std::mem::align_of::<u16>()) > 0 {
            let slice = slice
                .chunks_exact(2)
                .take(slice.len() / 2)
                .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
                .collect::<Vec<u16>>();
            String::from_utf16_lossy(&slice)
        } else {
            let slice = unsafe {
                std::slice::from_raw_parts(slice.as_ptr() as *const u16, slice.len() / 2)
            };
            String::from_utf16_lossy(slice)
        }
    }

    pub fn read_bool(&mut self) -> bool {
        // A bool is specified as "a single byte with value 0 (false) or 1 (true)".
        match self.read_u8() {
            0 => false,
            1 => true,
            _ => panic!(),
        }
    }

    pub fn read_i8(&mut self) -> i8 {
        let value = i8::from_le_bytes(self[..1].try_into().unwrap());
        self.offset(1);
        value
    }

    pub fn read_u8(&mut self) -> u8 {
        let value = u8::from_le_bytes(self[..1].try_into().unwrap());
        self.offset(1);
        value
    }

    pub fn read_i16(&mut self) -> i16 {
        let value = i16::from_le_bytes(self[..2].try_into().unwrap());
        self.offset(2);
        value
    }

    pub fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(self[..2].try_into().unwrap());
        self.offset(2);
        value
    }

    pub fn read_i32(&mut self) -> i32 {
        let value = i32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }

    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }

    pub fn read_i64(&mut self) -> i64 {
        let value = i64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }

    pub fn read_u64(&mut self) -> u64 {
        let value = u64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }

    pub fn read_f32(&mut self) -> f32 {
        let value = f32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }

    pub fn read_f64(&mut self) -> f64 {
        let value = f64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }

    pub fn read_integer(&mut self, ty: Type) -> Value {
        match ty {
            Type::I8 => Value::I8(self.read_i8()),
            Type::U8 => Value::U8(self.read_u8()),
            Type::I16 => Value::I16(self.read_i16()),
            Type::U16 => Value::U16(self.read_u16()),
            Type::I32 => Value::I32(self.read_i32()),
            Type::U32 => Value::U32(self.read_u32()),
            Type::I64 => Value::I64(self.read_i64()),
            Type::U64 => Value::U64(self.read_u64()),
            _ => panic!(),
        }
    }

    fn offset(&mut self, offset: usize) {
        self.slice = &self.slice[offset..];
    }
}
