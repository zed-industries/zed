use crate::CFUUIDBytes;

impl From<[u8; 16]> for CFUUIDBytes {
    #[inline]
    fn from(value: [u8; 16]) -> Self {
        Self {
            byte0: value[0],
            byte1: value[1],
            byte2: value[2],
            byte3: value[3],
            byte4: value[4],
            byte5: value[5],
            byte6: value[6],
            byte7: value[7],
            byte8: value[8],
            byte9: value[9],
            byte10: value[10],
            byte11: value[11],
            byte12: value[12],
            byte13: value[13],
            byte14: value[14],
            byte15: value[15],
        }
    }
}

impl From<CFUUIDBytes> for [u8; 16] {
    #[inline]
    fn from(value: CFUUIDBytes) -> Self {
        [
            value.byte0,
            value.byte1,
            value.byte2,
            value.byte3,
            value.byte4,
            value.byte5,
            value.byte6,
            value.byte7,
            value.byte8,
            value.byte9,
            value.byte10,
            value.byte11,
            value.byte12,
            value.byte13,
            value.byte14,
            value.byte15,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::CFUUID;

    #[test]
    fn eq() {
        let uuid0 = CFUUID::from_uuid_bytes(None, [0; 16].into()).unwrap();
        let uuid1 = CFUUID::from_uuid_bytes(None, [1; 16].into()).unwrap();
        assert_eq!(uuid0, uuid0);
        assert_ne!(uuid0, uuid1);
    }

    #[test]
    fn roundtrip() {
        let uuid = CFUUID::new(None).unwrap();
        assert_eq!(uuid, uuid);

        let bytes = uuid.uuid_bytes();
        let same_uuid = CFUUID::from_uuid_bytes(None, bytes).unwrap();
        assert_eq!(uuid, same_uuid);
    }
}
