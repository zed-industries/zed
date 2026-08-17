//! ECDSA/secp256r1 test vectors

use ecdsa_core::dev::TestVector;
use hex_literal::hex;

/// ECDSA/P-256 test vectors.
///
/// Adapted from the FIPS 186-4 ECDSA test vectors
/// (P-256, SHA-256, from `SigGen.txt` in `186-4ecdsatestvectors.zip`)
/// <https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program/digital-signatures>
///
/// The `m` field contains a SHA-256 prehash of the `Msg` field in the
/// original `SigTen.txt`.
pub const ECDSA_TEST_VECTORS: &[TestVector] = &[
    TestVector {
        d: &hex!("519b423d715f8b581f4fa8ee59f4771a5b44c8130b4e3eacca54a56dda72b464"),
        q_x: &hex!("1ccbe91c075fc7f4f033bfa248db8fccd3565de94bbfb12f3c59ff46c271bf83"),
        q_y: &hex!("ce4014c68811f9a21a1fdb2c0e6113e06db7ca93b7404e78dc7ccd5ca89a4ca9"),
        k: &hex!("94a1bbb14b906a61a280f245f9e93c7f3b4a6247824f5d33b9670787642a68de"),
        m: &hex!("44acf6b7e36c1342c2c5897204fe09504e1e2efb1a900377dbc4e7a6a133ec56"),
        r: &hex!("f3ac8061b514795b8843e3d6629527ed2afd6b1f6a555a7acabb5e6f79c8c2ac"),
        s: &hex!("8bf77819ca05a6b2786c76262bf7371cef97b218e96f175a3ccdda2acc058903"),
    },
    TestVector {
        d: &hex!("0f56db78ca460b055c500064824bed999a25aaf48ebb519ac201537b85479813"),
        q_x: &hex!("e266ddfdc12668db30d4ca3e8f7749432c416044f2d2b8c10bf3d4012aeffa8a"),
        q_y: &hex!("bfa86404a2e9ffe67d47c587ef7a97a7f456b863b4d02cfc6928973ab5b1cb39"),
        k: &hex!("6d3e71882c3b83b156bb14e0ab184aa9fb728068d3ae9fac421187ae0b2f34c6"),
        m: &hex!("9b2db89cb0e8fa3cc7608b4d6cc1dec0114e0b9ff4080bea12b134f489ab2bbc"),
        r: &hex!("976d3a4e9d23326dc0baa9fa560b7c4e53f42864f508483a6473b6a11079b2db"),
        s: &hex!("1b766e9ceb71ba6c01dcd46e0af462cd4cfa652ae5017d4555b8eeefe36e1932"),
    },
    TestVector {
        d: &hex!("e283871239837e13b95f789e6e1af63bf61c918c992e62bca040d64cad1fc2ef"),
        q_x: &hex!("74ccd8a62fba0e667c50929a53f78c21b8ff0c3c737b0b40b1750b2302b0bde8"),
        q_y: &hex!("29074e21f3a0ef88b9efdf10d06aa4c295cc1671f758ca0e4cd108803d0f2614"),
        k: &hex!("ad5e887eb2b380b8d8280ad6e5ff8a60f4d26243e0124c2f31a297b5d0835de2"),
        m: &hex!("b804cf88af0c2eff8bbbfb3660ebb3294138e9d3ebd458884e19818061dacff0"),
        r: &hex!("35fb60f5ca0f3ca08542fb3cc641c8263a2cab7a90ee6a5e1583fac2bb6f6bd1"),
        s: &hex!("ee59d81bc9db1055cc0ed97b159d8784af04e98511d0a9a407b99bb292572e96"),
    },
    TestVector {
        d: &hex!("a3d2d3b7596f6592ce98b4bfe10d41837f10027a90d7bb75349490018cf72d07"),
        q_x: &hex!("322f80371bf6e044bc49391d97c1714ab87f990b949bc178cb7c43b7c22d89e1"),
        q_y: &hex!("3c15d54a5cc6b9f09de8457e873eb3deb1fceb54b0b295da6050294fae7fd999"),
        k: &hex!("24fc90e1da13f17ef9fe84cc96b9471ed1aaac17e3a4bae33a115df4e5834f18"),
        m: &hex!("85b957d92766235e7c880ac5447cfbe97f3cb499f486d1e43bcb5c2ff9608a1a"),
        r: &hex!("d7c562370af617b581c84a2468cc8bd50bb1cbf322de41b7887ce07c0e5884ca"),
        s: &hex!("b46d9f2d8c4bf83546ff178f1d78937c008d64e8ecc5cbb825cb21d94d670d89"),
    },
    TestVector {
        d: &hex!("53a0e8a8fe93db01e7ae94e1a9882a102ebd079b3a535827d583626c272d280d"),
        q_x: &hex!("1bcec4570e1ec2436596b8ded58f60c3b1ebc6a403bc5543040ba82963057244"),
        q_y: &hex!("8af62a4c683f096b28558320737bf83b9959a46ad2521004ef74cf85e67494e1"),
        k: &hex!("5d833e8d24cc7a402d7ee7ec852a3587cddeb48358cea71b0bedb8fabe84e0c4"),
        m: &hex!("3360d699222f21840827cf698d7cb635bee57dc80cd7733b682d41b55b666e22"),
        r: &hex!("18caaf7b663507a8bcd992b836dec9dc5703c080af5e51dfa3a9a7c387182604"),
        s: &hex!("77c68928ac3b88d985fb43fb615fb7ff45c18ba5c81af796c613dfa98352d29c"),
    },
    TestVector {
        d: &hex!("4af107e8e2194c830ffb712a65511bc9186a133007855b49ab4b3833aefc4a1d"),
        q_x: &hex!("a32e50be3dae2c8ba3f5e4bdae14cf7645420d425ead94036c22dd6c4fc59e00"),
        q_y: &hex!("d623bf641160c289d6742c6257ae6ba574446dd1d0e74db3aaa80900b78d4ae9"),
        k: &hex!("e18f96f84dfa2fd3cdfaec9159d4c338cd54ad314134f0b31e20591fc238d0ab"),
        m: &hex!("c413c4908cd0bc6d8e32001aa103043b2cf5be7fcbd61a5cec9488c3a577ca57"),
        r: &hex!("8524c5024e2d9a73bde8c72d9129f57873bbad0ed05215a372a84fdbc78f2e68"),
        s: &hex!("d18c2caf3b1072f87064ec5e8953f51301cada03469c640244760328eb5a05cb"),
    },
    TestVector {
        d: &hex!("78dfaa09f1076850b3e206e477494cddcfb822aaa0128475053592c48ebaf4ab"),
        q_x: &hex!("8bcfe2a721ca6d753968f564ec4315be4857e28bef1908f61a366b1f03c97479"),
        q_y: &hex!("0f67576a30b8e20d4232d8530b52fb4c89cbc589ede291e499ddd15fe870ab96"),
        k: &hex!("295544dbb2da3da170741c9b2c6551d40af7ed4e891445f11a02b66a5c258a77"),
        m: &hex!("88fc1e7d849794fc51b135fa135deec0db02b86c3cd8cebdaa79e8689e5b2898"),
        r: &hex!("c5a186d72df452015480f7f338970bfe825087f05c0088d95305f87aacc9b254"),
        s: &hex!("84a58f9e9d9e735344b316b1aa1ab5185665b85147dc82d92e969d7bee31ca30"),
    },
    TestVector {
        d: &hex!("80e692e3eb9fcd8c7d44e7de9f7a5952686407f90025a1d87e52c7096a62618a"),
        q_x: &hex!("a88bc8430279c8c0400a77d751f26c0abc93e5de4ad9a4166357952fe041e767"),
        q_y: &hex!("2d365a1eef25ead579cc9a069b6abc1b16b81c35f18785ce26a10ba6d1381185"),
        k: &hex!("7c80fd66d62cc076cef2d030c17c0a69c99611549cb32c4ff662475adbe84b22"),
        m: &hex!("41fa8d8b4cd0a5fdf021f4e4829d6d1e996bab6b4a19dcb85585fe76c582d2bc"),
        r: &hex!("9d0c6afb6df3bced455b459cc21387e14929392664bb8741a3693a1795ca6902"),
        s: &hex!("d7f9ddd191f1f412869429209ee3814c75c72fa46a9cccf804a2f5cc0b7e739f"),
    },
    TestVector {
        d: &hex!("5e666c0db0214c3b627a8e48541cc84a8b6fd15f300da4dff5d18aec6c55b881"),
        q_x: &hex!("1bc487570f040dc94196c9befe8ab2b6de77208b1f38bdaae28f9645c4d2bc3a"),
        q_y: &hex!("ec81602abd8345e71867c8210313737865b8aa186851e1b48eaca140320f5d8f"),
        k: &hex!("2e7625a48874d86c9e467f890aaa7cd6ebdf71c0102bfdcfa24565d6af3fdce9"),
        m: &hex!("2d72947c1731543b3d62490866a893952736757746d9bae13e719079299ae192"),
        r: &hex!("2f9e2b4e9f747c657f705bffd124ee178bbc5391c86d056717b140c153570fd9"),
        s: &hex!("f5413bfd85949da8d83de83ab0d19b2986613e224d1901d76919de23ccd03199"),
    },
    TestVector {
        d: &hex!("f73f455271c877c4d5334627e37c278f68d143014b0a05aa62f308b2101c5308"),
        q_x: &hex!("b8188bd68701fc396dab53125d4d28ea33a91daf6d21485f4770f6ea8c565dde"),
        q_y: &hex!("423f058810f277f8fe076f6db56e9285a1bf2c2a1dae145095edd9c04970bc4a"),
        k: &hex!("62f8665fd6e26b3fa069e85281777a9b1f0dfd2c0b9f54a086d0c109ff9fd615"),
        m: &hex!("e138bd577c3729d0e24a98a82478bcc7482499c4cdf734a874f7208ddbc3c116"),
        r: &hex!("1cc628533d0004b2b20e7f4baad0b8bb5e0673db159bbccf92491aef61fc9620"),
        s: &hex!("880e0bbf82a8cf818ed46ba03cf0fc6c898e36fca36cc7fdb1d2db7503634430"),
    },
    TestVector {
        d: &hex!("b20d705d9bd7c2b8dc60393a5357f632990e599a0975573ac67fd89b49187906"),
        q_x: &hex!("51f99d2d52d4a6e734484a018b7ca2f895c2929b6754a3a03224d07ae61166ce"),
        q_y: &hex!("4737da963c6ef7247fb88d19f9b0c667cac7fe12837fdab88c66f10d3c14cad1"),
        k: &hex!("72b656f6b35b9ccbc712c9f1f3b1a14cbbebaec41c4bca8da18f492a062d6f6f"),
        m: &hex!("17b03f9f00f6692ccdde485fc63c4530751ef35da6f71336610944b0894fcfb8"),
        r: &hex!("9886ae46c1415c3bc959e82b760ad760aab66885a84e620aa339fdf102465c42"),
        s: &hex!("2bf3a80bc04faa35ebecc0f4864ac02d349f6f126e0f988501b8d3075409a26c"),
    },
    TestVector {
        d: &hex!("d4234bebfbc821050341a37e1240efe5e33763cbbb2ef76a1c79e24724e5a5e7"),
        q_x: &hex!("8fb287f0202ad57ae841aea35f29b2e1d53e196d0ddd9aec24813d64c0922fb7"),
        q_y: &hex!("1f6daff1aa2dd2d6d3741623eecb5e7b612997a1039aab2e5cf2de969cfea573"),
        k: &hex!("d926fe10f1bfd9855610f4f5a3d666b1a149344057e35537373372ead8b1a778"),
        m: &hex!("c25beae638ff8dcd370e03a6f89c594c55bed1277ee14d83bbb0ef783a0517c7"),
        r: &hex!("490efd106be11fc365c7467eb89b8d39e15d65175356775deab211163c2504cb"),
        s: &hex!("644300fc0da4d40fb8c6ead510d14f0bd4e1321a469e9c0a581464c7186b7aa7"),
    },
    TestVector {
        d: &hex!("b58f5211dff440626bb56d0ad483193d606cf21f36d9830543327292f4d25d8c"),
        q_x: &hex!("68229b48c2fe19d3db034e4c15077eb7471a66031f28a980821873915298ba76"),
        q_y: &hex!("303e8ee3742a893f78b810991da697083dd8f11128c47651c27a56740a80c24c"),
        k: &hex!("e158bf4a2d19a99149d9cdb879294ccb7aaeae03d75ddd616ef8ae51a6dc1071"),
        m: &hex!("5eb28029ebf3c7025ff2fc2f6de6f62aecf6a72139e1cba5f20d11bbef036a7f"),
        r: &hex!("e67a9717ccf96841489d6541f4f6adb12d17b59a6bef847b6183b8fcf16a32eb"),
        s: &hex!("9ae6ba6d637706849a6a9fc388cf0232d85c26ea0d1fe7437adb48de58364333"),
    },
    TestVector {
        d: &hex!("54c066711cdb061eda07e5275f7e95a9962c6764b84f6f1f3ab5a588e0a2afb1"),
        q_x: &hex!("0a7dbb8bf50cb605eb2268b081f26d6b08e012f952c4b70a5a1e6e7d46af98bb"),
        q_y: &hex!("f26dd7d799930062480849962ccf5004edcfd307c044f4e8f667c9baa834eeae"),
        k: &hex!("646fe933e96c3b8f9f507498e907fdd201f08478d0202c752a7c2cfebf4d061a"),
        m: &hex!("12135386c09e0bf6fd5c454a95bcfe9b3edb25c71e455c73a212405694b29002"),
        r: &hex!("b53ce4da1aa7c0dc77a1896ab716b921499aed78df725b1504aba1597ba0c64b"),
        s: &hex!("d7c246dc7ad0e67700c373edcfdd1c0a0495fc954549ad579df6ed1438840851"),
    },
    TestVector {
        d: &hex!("34fa4682bf6cb5b16783adcd18f0e6879b92185f76d7c920409f904f522db4b1"),
        q_x: &hex!("105d22d9c626520faca13e7ced382dcbe93498315f00cc0ac39c4821d0d73737"),
        q_y: &hex!("6c47f3cbbfa97dfcebe16270b8c7d5d3a5900b888c42520d751e8faf3b401ef4"),
        k: &hex!("a6f463ee72c9492bc792fe98163112837aebd07bab7a84aaed05be64db3086f4"),
        m: &hex!("aea3e069e03c0ff4d6b3fa2235e0053bbedc4c7e40efbc686d4dfb5efba4cfed"),
        r: &hex!("542c40a18140a6266d6f0286e24e9a7bad7650e72ef0e2131e629c076d962663"),
        s: &hex!("4f7f65305e24a6bbb5cff714ba8f5a2cee5bdc89ba8d75dcbf21966ce38eb66f"),
    },
];
