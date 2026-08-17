use elliptic_curve::bigint::{ArrayEncoding, U256};
use elliptic_curve::consts::U48;
use elliptic_curve::generic_array::GenericArray;
use elliptic_curve::group::cofactor::CofactorGroup;
use elliptic_curve::hash2curve::OsswuMap;
use elliptic_curve::hash2curve::{FromOkm, GroupDigest, MapToCurve, OsswuMapParams, Sgn0};
use elliptic_curve::subtle::{Choice, CtOption};

use crate::{AffinePoint, NistP256, ProjectivePoint, Scalar};

use super::FieldElement;

impl GroupDigest for NistP256 {
    type FieldElement = FieldElement;
}

impl FromOkm for FieldElement {
    type Length = U48;

    fn from_okm(data: &GenericArray<u8, Self::Length>) -> Self {
        const F_2_192: FieldElement = FieldElement([
            0xffff_fffe_ffff_ffffu64,
            0xffff_ffff_ffff_fffeu64,
            0x0000_0002_0000_0000u64,
            0x0000_0000_0000_0003u64,
        ]);
        let d0 = FieldElement([
            u64::from_be_bytes(data[16..24].try_into().unwrap()),
            u64::from_be_bytes(data[8..16].try_into().unwrap()),
            u64::from_be_bytes(data[0..8].try_into().unwrap()),
            0,
        ])
        .to_montgomery();
        let d1 = FieldElement([
            u64::from_be_bytes(data[40..48].try_into().unwrap()),
            u64::from_be_bytes(data[32..40].try_into().unwrap()),
            u64::from_be_bytes(data[24..32].try_into().unwrap()),
            0,
        ])
        .to_montgomery();
        d0 * F_2_192 + d1
    }
}

impl Sgn0 for FieldElement {
    fn sgn0(&self) -> Choice {
        self.is_odd()
    }
}

impl OsswuMap for FieldElement {
    const PARAMS: OsswuMapParams<Self> = OsswuMapParams {
        c1: [
            0xffff_ffff_ffff_ffff,
            0x0000_0000_3fff_ffff,
            0x4000_0000_0000_0000,
            0x3fff_ffff_c000_0000,
        ],
        c2: FieldElement([
            0x53e4_3951_f64f_dbe7,
            0xb280_6c63_966a_1a66,
            0x1ac5_d59c_3298_bf50,
            0xa332_3851_ba99_7e27,
        ]),
        map_a: FieldElement([
            0xffff_ffff_ffff_fffc,
            0x0000_0003_ffff_ffff,
            0x0000_0000_0000_0000,
            0xffff_fffc_0000_0004,
        ]),
        map_b: FieldElement([
            0xd89c_df62_29c4_bddf,
            0xacf0_05cd_7884_3090,
            0xe5a2_20ab_f721_2ed6,
            0xdc30_061d_0487_4834,
        ]),
        z: FieldElement([
            0xffff_ffff_ffff_fff5,
            0x0000_000a_ffff_ffff,
            0x0000_0000_0000_0000,
            0xffff_fff5_0000_000b,
        ]),
    };
}

impl MapToCurve for FieldElement {
    type Output = ProjectivePoint;

    fn map_to_curve(&self) -> Self::Output {
        let (qx, qy) = self.osswu();

        AffinePoint {
            x: qx,
            y: qy,
            infinity: 0,
        }
        .into()
    }
}

impl CofactorGroup for ProjectivePoint {
    type Subgroup = ProjectivePoint;

    fn clear_cofactor(&self) -> Self::Subgroup {
        *self
    }

    fn into_subgroup(self) -> CtOption<Self::Subgroup> {
        CtOption::new(self, 1.into())
    }

    fn is_torsion_free(&self) -> Choice {
        1.into()
    }
}

impl FromOkm for Scalar {
    type Length = U48;

    fn from_okm(data: &GenericArray<u8, Self::Length>) -> Self {
        const F_2_192: Scalar = Scalar(U256::from_be_hex(
            "0000000000000001000000000000000000000000000000000000000000000000",
        ));

        let mut d0 = GenericArray::default();
        d0[8..].copy_from_slice(&data[0..24]);
        let d0 = Scalar(U256::from_be_byte_array(d0));

        let mut d1 = GenericArray::default();
        d1[8..].copy_from_slice(&data[24..]);
        let d1 = Scalar(U256::from_be_byte_array(d1));

        d0 * F_2_192 + d1
    }
}

#[test]
fn hash_to_curve() {
    use elliptic_curve::hash2curve::{self, ExpandMsgXmd};
    use hex_literal::hex;
    use sha2::Sha256;

    struct TestVector {
        msg: &'static [u8],
        p_x: [u8; 32],
        p_y: [u8; 32],
        u_0: [u8; 32],
        u_1: [u8; 32],
        q0_x: [u8; 32],
        q0_y: [u8; 32],
        q1_x: [u8; 32],
        q1_y: [u8; 32],
    }

    const DST: &[u8] = b"QUUX-V01-CS02-with-P256_XMD:SHA-256_SSWU_RO_";

    const TEST_VECTORS: &[TestVector] = &[
        TestVector {
            msg: b"",
            p_x: hex!("2c15230b26dbc6fc9a37051158c95b79656e17a1a920b11394ca91c44247d3e4"),
            p_y: hex!("8a7a74985cc5c776cdfe4b1f19884970453912e9d31528c060be9ab5c43e8415"),
            u_0: hex!("ad5342c66a6dd0ff080df1da0ea1c04b96e0330dd89406465eeba11582515009"),
            u_1: hex!("8c0f1d43204bd6f6ea70ae8013070a1518b43873bcd850aafa0a9e220e2eea5a"),
            q0_x: hex!("ab640a12220d3ff283510ff3f4b1953d09fad35795140b1c5d64f313967934d5"),
            q0_y: hex!("dccb558863804a881d4fff3455716c836cef230e5209594ddd33d85c565b19b1"),
            q1_x: hex!("51cce63c50d972a6e51c61334f0f4875c9ac1cd2d3238412f84e31da7d980ef5"),
            q1_y: hex!("b45d1a36d00ad90e5ec7840a60a4de411917fbe7c82c3949a6e699e5a1b66aac"),
        },
        TestVector {
            msg: b"abc",
            p_x: hex!("0bb8b87485551aa43ed54f009230450b492fead5f1cc91658775dac4a3388a0f"),
            p_y: hex!("5c41b3d0731a27a7b14bc0bf0ccded2d8751f83493404c84a88e71ffd424212e"),
            u_0: hex!("afe47f2ea2b10465cc26ac403194dfb68b7f5ee865cda61e9f3e07a537220af1"),
            u_1: hex!("379a27833b0bfe6f7bdca08e1e83c760bf9a338ab335542704edcd69ce9e46e0"),
            q0_x: hex!("5219ad0ddef3cc49b714145e91b2f7de6ce0a7a7dc7406c7726c7e373c58cb48"),
            q0_y: hex!("7950144e52d30acbec7b624c203b1996c99617d0b61c2442354301b191d93ecf"),
            q1_x: hex!("019b7cb4efcfeaf39f738fe638e31d375ad6837f58a852d032ff60c69ee3875f"),
            q1_y: hex!("589a62d2b22357fed5449bc38065b760095ebe6aeac84b01156ee4252715446e"),
        },
        TestVector {
            msg: b"abcdef0123456789",
            p_x: hex!("65038ac8f2b1def042a5df0b33b1f4eca6bff7cb0f9c6c1526811864e544ed80"),
            p_y: hex!("cad44d40a656e7aff4002a8de287abc8ae0482b5ae825822bb870d6df9b56ca3"),
            u_0: hex!("0fad9d125a9477d55cf9357105b0eb3a5c4259809bf87180aa01d651f53d312c"),
            u_1: hex!("b68597377392cd3419d8fcc7d7660948c8403b19ea78bbca4b133c9d2196c0fb"),
            q0_x: hex!("a17bdf2965eb88074bc01157e644ed409dac97cfcf0c61c998ed0fa45e79e4a2"),
            q0_y: hex!("4f1bc80c70d411a3cc1d67aeae6e726f0f311639fee560c7f5a664554e3c9c2e"),
            q1_x: hex!("7da48bb67225c1a17d452c983798113f47e438e4202219dd0715f8419b274d66"),
            q1_y: hex!("b765696b2913e36db3016c47edb99e24b1da30e761a8a3215dc0ec4d8f96e6f9"),
        },
        TestVector {
            msg: b"q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
            p_x: hex!("4be61ee205094282ba8a2042bcb48d88dfbb609301c49aa8b078533dc65a0b5d"),
            p_y: hex!("98f8df449a072c4721d241a3b1236d3caccba603f916ca680f4539d2bfb3c29e"),
            u_0: hex!("3bbc30446f39a7befad080f4d5f32ed116b9534626993d2cc5033f6f8d805919"),
            u_1: hex!("76bb02db019ca9d3c1e02f0c17f8baf617bbdae5c393a81d9ce11e3be1bf1d33"),
            q0_x: hex!("c76aaa823aeadeb3f356909cb08f97eee46ecb157c1f56699b5efebddf0e6398"),
            q0_y: hex!("776a6f45f528a0e8d289a4be12c4fab80762386ec644abf2bffb9b627e4352b1"),
            q1_x: hex!("418ac3d85a5ccc4ea8dec14f750a3a9ec8b85176c95a7022f391826794eb5a75"),
            q1_y: hex!("fd6604f69e9d9d2b74b072d14ea13050db72c932815523305cb9e807cc900aff"),
        },
        TestVector {
            msg: b"a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            p_x: hex!("457ae2981f70ca85d8e24c308b14db22f3e3862c5ea0f652ca38b5e49cd64bc5"),
            p_y: hex!("ecb9f0eadc9aeed232dabc53235368c1394c78de05dd96893eefa62b0f4757dc"),
            u_0: hex!("4ebc95a6e839b1ae3c63b847798e85cb3c12d3817ec6ebc10af6ee51adb29fec"),
            u_1: hex!("4e21af88e22ea80156aff790750121035b3eefaa96b425a8716e0d20b4e269ee"),
            q0_x: hex!("d88b989ee9d1295df413d4456c5c850b8b2fb0f5402cc5c4c7e815412e926db8"),
            q0_y: hex!("bb4a1edeff506cf16def96afff41b16fc74f6dbd55c2210e5b8f011ba32f4f40"),
            q1_x: hex!("a281e34e628f3a4d2a53fa87ff973537d68ad4fbc28d3be5e8d9f6a2571c5a4b"),
            q1_y: hex!("f6ed88a7aab56a488100e6f1174fa9810b47db13e86be999644922961206e184"),
        },
    ];

    for test_vector in TEST_VECTORS {
        // in parts
        let mut u = [FieldElement::default(), FieldElement::default()];
        hash2curve::hash_to_field::<ExpandMsgXmd<Sha256>, FieldElement>(
            &[test_vector.msg],
            DST,
            &mut u,
        )
        .unwrap();
        assert_eq!(u[0].to_bytes().as_slice(), test_vector.u_0);
        assert_eq!(u[1].to_bytes().as_slice(), test_vector.u_1);

        let q0 = u[0].map_to_curve();
        let aq0 = q0.to_affine();
        assert_eq!(aq0.x.to_bytes().as_slice(), test_vector.q0_x);
        assert_eq!(aq0.y.to_bytes().as_slice(), test_vector.q0_y);

        let q1 = u[1].map_to_curve();
        let aq1 = q1.to_affine();
        assert_eq!(aq1.x.to_bytes().as_slice(), test_vector.q1_x);
        assert_eq!(aq1.y.to_bytes().as_slice(), test_vector.q1_y);

        let p = q0.clear_cofactor() + q1.clear_cofactor();
        let ap = p.to_affine();
        assert_eq!(ap.x.to_bytes().as_slice(), test_vector.p_x);
        assert_eq!(ap.y.to_bytes().as_slice(), test_vector.p_y);

        // complete run
        let pt =
            NistP256::hash_from_bytes::<ExpandMsgXmd<Sha256>>(&[test_vector.msg], DST).unwrap();
        let apt = pt.to_affine();
        assert_eq!(apt.x.to_bytes().as_slice(), test_vector.p_x);
        assert_eq!(apt.y.to_bytes().as_slice(), test_vector.p_y);
    }
}

#[test]
fn hash_to_scalar_voprf() {
    use elliptic_curve::hash2curve::ExpandMsgXmd;
    use hex_literal::hex;
    use sha2::Sha256;

    struct TestVector {
        dst: &'static [u8],
        seed: &'static [u8],
        sk_sm: &'static [u8],
    }

    const TEST_VECTORS: &[TestVector] = &[
        TestVector {
            dst: b"HashToScalar-VOPRF08-\x00\x00\x03",
            seed: &hex!("a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3"),
            sk_sm: &hex!("c15d9e9ab36d495d9d62954db6aafe06d3edabf41600d58f9be0737af2719e97"),
        },
        TestVector {
            dst: b"HashToScalar-VOPRF08-\x01\x00\x03",
            seed: &hex!("a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3"),
            sk_sm: &hex!("7f62054fcd598b5e023c08ef0f04e05e26867438d5e355e846c9d8788d5c7a12"),
        },
    ];

    for test_vector in TEST_VECTORS {
        assert_eq!(
            NistP256::hash_to_scalar::<ExpandMsgXmd<Sha256>>(&[test_vector.seed], test_vector.dst,)
                .unwrap()
                .to_bytes()
                .as_slice(),
            test_vector.sk_sm
        );
    }
}

#[test]
fn from_okm_fuzz() {
    use elliptic_curve::bigint::{NonZero, U384};
    use elliptic_curve::Curve;
    use proptest::num::u64::ANY;
    use proptest::{prelude::ProptestConfig, proptest};

    let mut wide_order = GenericArray::default();
    wide_order[16..].copy_from_slice(&NistP256::ORDER.to_be_byte_array());
    let wide_order = NonZero::new(U384::from_be_byte_array(wide_order)).unwrap();

    let simple_from_okm = move |data: GenericArray<u8, U48>| -> Scalar {
        let data = U384::from_be_slice(&data);

        let scalar = data % wide_order;
        let reduced_scalar = U256::from_be_slice(&scalar.to_be_byte_array()[16..]);

        Scalar(reduced_scalar)
    };

    proptest!(ProptestConfig::with_cases(1000), |(b0 in ANY, b1 in ANY, b2 in ANY, b3 in ANY, b4 in ANY, b5 in ANY)| {
        let mut data = GenericArray::default();
        data[..8].copy_from_slice(&b0.to_be_bytes());
        data[8..16].copy_from_slice(&b1.to_be_bytes());
        data[16..24].copy_from_slice(&b2.to_be_bytes());
        data[24..32].copy_from_slice(&b3.to_be_bytes());
        data[32..40].copy_from_slice(&b4.to_be_bytes());
        data[40..].copy_from_slice(&b5.to_be_bytes());

        let from_okm = Scalar::from_okm(&data);
        let simple_from_okm = simple_from_okm(data);
        assert_eq!(from_okm, simple_from_okm);
    });
}
