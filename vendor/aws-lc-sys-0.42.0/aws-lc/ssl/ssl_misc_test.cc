// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>

#include "../crypto/test/file_util.h"
#include "../crypto/test/test_util.h"
#include "internal.h"
#include "ssl_common_test.h"

BSSL_NAMESPACE_BEGIN

TEST(GrowableArrayTest, Resize) {
  GrowableArray<size_t> array;
  ASSERT_TRUE(array.empty());
  EXPECT_EQ(array.size(), 0u);

  ASSERT_TRUE(array.Push(42));
  ASSERT_TRUE(!array.empty());
  EXPECT_EQ(array.size(), 1u);

  // Force a resize operation to occur
  for (size_t i = 0; i < 16; i++) {
    ASSERT_TRUE(array.Push(i + 1));
  }

  EXPECT_EQ(array.size(), 17u);

  // Verify that expected values are still contained in array
  for (size_t i = 0; i < array.size(); i++) {
    EXPECT_EQ(array[i], i == 0 ? 42 : i);
  }
}

TEST(GrowableArrayTest, MoveConstructor) {
  GrowableArray<size_t> array;
  for (size_t i = 0; i < 100; i++) {
    ASSERT_TRUE(array.Push(i));
  }

  GrowableArray<size_t> array_moved(std::move(array));
  for (size_t i = 0; i < 100; i++) {
    EXPECT_EQ(array_moved[i], i);
  }
}

TEST(GrowableArrayTest, GrowableArrayContainingGrowableArrays) {
  // Representative example of a struct that contains a GrowableArray.
  struct TagAndArray {
    size_t tag;
    GrowableArray<size_t> array;
  };

  GrowableArray<TagAndArray> array;
  for (size_t i = 0; i < 100; i++) {
    TagAndArray elem;
    elem.tag = i;
    for (size_t j = 0; j < i; j++) {
      ASSERT_TRUE(elem.array.Push(j));
    }
    ASSERT_TRUE(array.Push(std::move(elem)));
  }
  EXPECT_EQ(array.size(), static_cast<size_t>(100));

  GrowableArray<TagAndArray> array_moved(std::move(array));
  EXPECT_EQ(array_moved.size(), static_cast<size_t>(100));
  size_t count = 0;
  for (const TagAndArray &elem : array_moved) {
    // Test the square bracket operator returns the same value as iteration.
    EXPECT_EQ(&elem, &array_moved[count]);

    EXPECT_EQ(elem.tag, count);
    EXPECT_EQ(elem.array.size(), count);
    for (size_t j = 0; j < count; j++) {
      EXPECT_EQ(elem.array[j], j);
    }
    count++;
  }
}


struct EncodeDecodeKATTestParam {
  const char *input;
  const char *output;
};

static const EncodeDecodeKATTestParam kEncodeDecodeKATs[] = {
    // V1 input round-trips as V2 output
    {"308201173082011302010102020303020240003081fa0201010408000000000000000104"
     "0800000000000000010420000004d29e62f41ded4bb33d0faa6ffada380e2c489dfbfb44"
     "4f574e475244010420cf3926d1ec5a562a642935a8050222b0aed93ffd9d1cac682274d9"
     "42e99e42a604020000020100020103040cb9b409f5129440622f87f84402010c040c1f49"
     "e2e989c66a263e9c227502010c020100020100020100a05b3059020101020203030402cc"
     "a80400043085668dcf9f0921094ebd7f91bf2a8c60d276e4c279fd85a989402f67868232"
     "4fd8098dc19d900b856d0a77e048e3ced2a104020204d2a20402021c20a4020400b10301"
     "01ffb20302011da206040474657374a7030101ff020108020100a0030101ff",
     "3082011c3082011802010102020303020240003081fa02010304080000000000000001040"
     "800000000000000010420000004d29e62f41ded4bb33d0faa6ffada380e2c489dfbfb444f"
     "574e475244010420cf3926d1ec5a562a642935a8050222b0aed93ffd9d1cac682274d942e"
     "99e42a604020000020100020103040cb9b409f5129440622f87f84402010c040c1f49e2e9"
     "89c66a263e9c227502010c020100020100020100a05b3059020101020203030402cca8040"
     "0043085668dcf9f0921094ebd7f91bf2a8c60d276e4c279fd85a989402f678682324fd809"
     "8dc19d900b856d0a77e048e3ced2a104020204d2a20402021c20a4020400b1030101ffb20"
     "302011da206040474657374a7030101ff020108020100a0030101ffa203020100"},
    // In runner.go, the test case "Basic-Server-TLS-Sync-SSL_Transfer" is used
    // to generate below bytes by adding print statement on the output of
    // |SSL_to_bytes| in bssl_shim.cc.
    // We've bumped the buffer size in the |previous_client/server_finished|
    // fields. This verifies that the original size is parsable and reencoded
    // with the new size.
    {"308201173082011302010102020303020240003081fa0201020408000000000000000104"
     "0800000000000000010420000004d29e62f41ded4bb33d0faa6ffada380e2c489dfbfb44"
     "4f574e475244010420cf3926d1ec5a562a642935a8050222b0aed93ffd9d1cac682274d9"
     "42e99e42a604020000020100020103040cb9b409f5129440622f87f84402010c040c1f49"
     "e2e989c66a263e9c227502010c020100020100020100a05b3059020101020203030402cc"
     "a80400043085668dcf9f0921094ebd7f91bf2a8c60d276e4c279fd85a989402f67868232"
     "4fd8098dc19d900b856d0a77e048e3ced2a104020204d2a20402021c20a4020400b10301"
     "01ffb20302011da206040474657374a7030101ff020108020100a0030101ff",
     "3082011c3082011802010102020303020240003081fa02010304080000000000000001040"
     "800000000000000010420000004d29e62f41ded4bb33d0faa6ffada380e2c489dfbfb444f"
     "574e475244010420cf3926d1ec5a562a642935a8050222b0aed93ffd9d1cac682274d942e"
     "99e42a604020000020100020103040cb9b409f5129440622f87f84402010c040c1f49e2e9"
     "89c66a263e9c227502010c020100020100020100a05b3059020101020203030402cca8040"
     "0043085668dcf9f0921094ebd7f91bf2a8c60d276e4c279fd85a989402f678682324fd809"
     "8dc19d900b856d0a77e048e3ced2a104020204d2a20402021c20a4020400b1030101ffb20"
     "302011da206040474657374a7030101ff020108020100a0030101ffa203020100"},
    // In runner.go, the test case
    // "TLS-TLS13-AES_128_GCM_SHA256-server-SSL_Transfer" is used to generate
    // below bytes by adding print statement on the output of |SSL_to_bytes| in
    // bssl_shim.cc.
    // We've bumped the buffer size in the |previous_client/server_finished|
    // fields. This verifies that the original size is parsable and reencoded
    // with the new size.
    {"308203883082038402010102020304020240003082036a020102040800000000000000000"
     "408000000000000000004206beca5c14aff6b92757545948b883c6c175327814bedcf38a6"
     "b2e4c43bc02d180420a32aee5b7705a19e4bb2b47f4918199c76cee7245f1311bc4ba3888"
     "3d33f236a04020000020100020101040c000000000000000000000000020100040c000000"
     "000000000000000000020100020100020100020100a04e304c02010102020304040213010"
     "40004200b66320d38c8fa1b0dfe9e37fcf2bf0bafb43077fa31ed2f1220dd245cef4c4da1"
     "04020204d2a205020302a300a4020400b20302011db9050203093a80a206040474657374a"
     "b03020100ac03010100ad03010100ae03010100af03020100b032043034c0893be938bade"
     "e7029ca3cfea4c821dde48e03f0d07641cba33b247bc161c0000000000000000000000000"
     "0000000b103020120b232043094b319ed2f41ee11aa73e141a238e5724c04f2aa8298c16b"
     "43c910c40cc98d1500000000000000000000000000000000b303020120b432043015a178c"
     "e69c0110ad36da8d58ca8428d9615ff07fc6a4e1bbab026c1bb0c02180000000000000000"
     "0000000000000000b503020120b88201700482016c040000b20002a30056355452010000a"
     "027abfd1f1aa28cee6e8e2396112e8285f150768898158dbce97a1aef0a63fa6dda1002a4"
     "d75942a3739c11e4b25827f529ab59d22e34e0cf0b59b9336eb60edbb1f686c072ab33c30"
     "e784f876da5b4c7fddd67f4a2ffa995f8c9ccf2128200ae9668d626866b1b7c6bb111867a"
     "87ed2a96122736595374f8fe5343e6ca492b278b67b1571423f2c1bcb673922e9044e9094"
     "9975ff72ab4a0eb659d8de664cac600042a2a0000040000b20002a3009e8c6738010100a0"
     "27abfd1f1aa28cee6e8e2396112e82851f15c84668b2f1d717681d1a3c6d2ea52d3401d31"
     "10a04498246480b96a7e5b3c39ea6cef3a2a86b81896f1621950472d858d18796c97e8320"
     "4daf94c1f30dfe763cd282fbee718a679dca8bff3cc8e11724062232e573bcf0252dc4d39"
     "0baa2b7f49a164b46d2d685e9fe826465cc135130f3e2e47838658af57173f864070fdce2"
     "41be58ecbd60d18128dfa28f4b1a00042a2a0000ba2330210201010204030013013016020"
     "101020117040e300c0201010201000201000101ffbb233021020101020403001301301602"
     "0101020117040e300c0201010201000201000101ff020108020100a0030101ff",
     "3082034a30820346020101020203040202400030820327020103040800000000000000000"
     "408000000000000000004206beca5c14aff6b92757545948b883c6c175327814bedcf38a6"
     "b2e4c43bc02d180420a32aee5b7705a19e4bb2b47f4918199c76cee7245f1311bc4ba3888"
     "3d33f236a0402000002010002010104000201000400020100020100020100020100a04e30"
     "4c0201010202030404021301040004200b66320d38c8fa1b0dfe9e37fcf2bf0bafb43077f"
     "a31ed2f1220dd245cef4c4da104020204d2a205020302a300a4020400b20302011db90502"
     "03093a80a206040474657374ab03020100ac03010100ad03010100ae03010100af0302010"
     "0b022042034c0893be938badee7029ca3cfea4c821dde48e03f0d07641cba33b247bc161c"
     "b103020120b222042094b319ed2f41ee11aa73e141a238e5724c04f2aa8298c16b43c910c"
     "40cc98d15b303020120b422042015a178ce69c0110ad36da8d58ca8428d9615ff07fc6a4e"
     "1bbab026c1bb0c0218b503020120b88201700482016c040000b20002a3005635545201000"
     "0a027abfd1f1aa28cee6e8e2396112e8285f150768898158dbce97a1aef0a63fa6dda1002"
     "a4d75942a3739c11e4b25827f529ab59d22e34e0cf0b59b9336eb60edbb1f686c072ab33c"
     "30e784f876da5b4c7fddd67f4a2ffa995f8c9ccf2128200ae9668d626866b1b7c6bb11186"
     "7a87ed2a96122736595374f8fe5343e6ca492b278b67b1571423f2c1bcb673922e9044e90"
     "949975ff72ab4a0eb659d8de664cac600042a2a0000040000b20002a3009e8c6738010100"
     "a027abfd1f1aa28cee6e8e2396112e82851f15c84668b2f1d717681d1a3c6d2ea52d3401d"
     "3110a04498246480b96a7e5b3c39ea6cef3a2a86b81896f1621950472d858d18796c97e83"
     "204daf94c1f30dfe763cd282fbee718a679dca8bff3cc8e11724062232e573bcf0252dc4d"
     "390baa2b7f49a164b46d2d685e9fe826465cc135130f3e2e47838658af57173f864070fdc"
     "e241be58ecbd60d18128dfa28f4b1a00042a2a0000ba23302102010102040300130130160"
     "20101020117040e300c0201010201000201000101ffbb2330210201010204030013013016"
     "020101020117040e300c0201010201000201000101ffbc030201ff020108020100a003010"
     "1ffa203020100"},
    // In runner.go, the test case
    // "TLS-ECH-Server-Cipher-HKDF-SHA256-AES-256-GCM-SSL_Transfer" is used
    // to generate below bytes by adding print statement on the output of
    // |SSL_to_bytes| in bssl_shim.cc.
    {"308203e8308203e40201010202030402024000308203c5020102040800000000000000000"
     "4080000000000000000042028431b914ffdb44ea92ca53d5734976c6a16f141d44f180b08"
     "16a5cb2b8e79030420bdaf544fa82d833d58c92213e44e850cc0b8147699b0b410d4aa2a2"
     "77030f3220402000002010002010104409e155007d04cd03cf4d8a95ce244dc978a87e180"
     "8f0f6c6acb51ad7bf8063ae00000000000000000000000000000000000000000000000000"
     "00000000000000002012004406680e8c36429d465ea520ae74a2062a5e07c39f34b688024"
     "ae2edfab28986707000000000000000000000000000000000000000000000000000000000"
     "0000000020120020100020100020100a04e304c020101020203040402130304000420df74"
     "ecd172087ad53083d505145ec4f6cf0ec5ed64b67ba526d55c918a0f8936a104020204d2a"
     "205020302a300a4020400b20302011db9050203093a80a210040e7365637265742e657861"
     "6d706c65ab03020100ac03010100ad03010100ae03010100af03020100b0320430c40f9f9"
     "5646fa700d58934e79c36b84ba3502d33df04248d56cded3444927e300000000000000000"
     "0000000000000000b103020120b23204307a1a99bf276b5e5be57dd68968411594e77b1a4"
     "8cf2c03cc5c143985aa40b32e00000000000000000000000000000000b303020120b43204"
     "30cbf50af88bc5a610910139172a468663675882caacaf176aa961b12a38a0df2a0000000"
     "0000000000000000000000000b503020120b703020101b88201700482016c040000b20002"
     "a300bbccf972010000a041e0b13ecd71dfb3d9e3cb451e37cfde81973a1b73106b6669b53"
     "475781f0203a3f32f45cef7742cf0efb86d850081254f20d3b6bd8330bc70331464905bcd"
     "99383c33e42c7d34bfeb47b387bf43b5c796daa4581f8b0043b7eb216911f8eebaf1e8bd5"
     "d05277943d5a319cc03d9555e414990099f56ee887145f34e8bff27f06d1865aa64d548a2"
     "2208318566959a097c080fa3e5e0d4b1d933132ef329299500045a5a0000040000b20002a"
     "3002ecba343010100a041e0b13ecd71dfb3d9e3cb451e37cfde289f90201519fb0dff08aa"
     "9e14a9f4ee1434edce481e49d22f061529bb4d230258f3dac886c2c1100bee2ccc7be889a"
     "90b417270c30b3b770558ef6f3c444ddefd08e673f788931d86542c4a1e7ec44b0957bb31"
     "5c17851bd8498b1d1131a79e19c66463e0566985ef55deb548fe370058ba83566278d01b3"
     "a565075b8ef2a82bea17ae95fa91b7b3ffa611a7d8a633100045a5a0000ba153013020101"
     "02040300130330080201010201050400bb153013020101020403001303300802010102010"
     "50400020108020100a0030101ffa203020100",
     "3082037d3082037902010102020304020240003082035a020103040800000000000000000"
     "4080000000000000000042028431b914ffdb44ea92ca53d5734976c6a16f141d44f180b08"
     "16a5cb2b8e79030420bdaf544fa82d833d58c92213e44e850cc0b8147699b0b410d4aa2a2"
     "77030f3220402000002010002010104209e155007d04cd03cf4d8a95ce244dc978a87e180"
     "8f0f6c6acb51ad7bf8063ae002012004206680e8c36429d465ea520ae74a2062a5e07c39f"
     "34b688024ae2edfab28986707020120020100020100020100a04e304c0201010202030404"
     "02130304000420df74ecd172087ad53083d505145ec4f6cf0ec5ed64b67ba526d55c918a0"
     "f8936a104020204d2a205020302a300a4020400b20302011db9050203093a80a210040e73"
     "65637265742e6578616d706c65ab03020100ac03010100ad03010100ae03010100af03020"
     "100b0220420c40f9f95646fa700d58934e79c36b84ba3502d33df04248d56cded3444927e"
     "30b103020120b22204207a1a99bf276b5e5be57dd68968411594e77b1a48cf2c03cc5c143"
     "985aa40b32eb303020120b4220420cbf50af88bc5a610910139172a468663675882caacaf"
     "176aa961b12a38a0df2ab503020120b703020101b88201700482016c040000b20002a300b"
     "bccf972010000a041e0b13ecd71dfb3d9e3cb451e37cfde81973a1b73106b6669b5347578"
     "1f0203a3f32f45cef7742cf0efb86d850081254f20d3b6bd8330bc70331464905bcd99383"
     "c33e42c7d34bfeb47b387bf43b5c796daa4581f8b0043b7eb216911f8eebaf1e8bd5d0527"
     "7943d5a319cc03d9555e414990099f56ee887145f34e8bff27f06d1865aa64d548a222083"
     "18566959a097c080fa3e5e0d4b1d933132ef329299500045a5a0000040000b20002a3002e"
     "cba343010100a041e0b13ecd71dfb3d9e3cb451e37cfde289f90201519fb0dff08aa9e14a"
     "9f4ee1434edce481e49d22f061529bb4d230258f3dac886c2c1100bee2ccc7be889a90b41"
     "7270c30b3b770558ef6f3c444ddefd08e673f788931d86542c4a1e7ec44b0957bb315c178"
     "51bd8498b1d1131a79e19c66463e0566985ef55deb548fe370058ba83566278d01b3a5650"
     "75b8ef2a82bea17ae95fa91b7b3ffa611a7d8a633100045a5a0000ba15301302010102040"
     "300130330080201010201050400bb15301302010102040300130330080201010201050400"
     "bc030201ff020108020100a0030101ffa203020100"},
    {"3082038a30820386020101020203040202400030820367020103040800000000000000000"
     "408000000000000000004208a0ac796db0e095391eeecd8023859d7c3681dbad2cc2aa74c"
     "096898756b804804204031c03eb662248ec9fa4e2ed49d030b00dbdb01a917a5978170f2d"
     "57f8f847a04020000020100020101042009a7a3b071b47296724aa98a0032b64d214f0321"
     "261ad553a1e1d915531909700201200420679419e0cedc3fb1d3c4b86844f028a120b6a43"
     "dca0602998d3456b96a4b433b020120020100020100020100a04e304c0201010202030404"
     "021301040004204aa6fac8dfe556ac9e00d748bb808c558b1119bea2ca8c272722c73be63"
     "d7b75a104020204d2a205020302a300a4020400b20302011db9050203093a80a206040474"
     "657374ab03020100ac03010100ad03010100ae03010100af03020100b022042041be9ecd2"
     "da1f2698bc89eb8259aba83f3810c8c1c4fba3fc2c557503bf79acdb103020120b2220420"
     "920ee9ed795ecd4db21dab3b9e207cf60ede7f1874d9a40fd419b7e5dcfd760fb30302012"
     "0b422042084307e1c1e54eb4e4b082546c46abd418ba09b0702b2979500dd8a38187f5204"
     "b503020120b88201700482016c040000b20002a3001d38486f010000a0f602f344b7cac7c"
     "d9ec544f5dcd4e59b74c4f91448d2b01ea5eaf485132448c18f508e5b07587bf147117bff"
     "3fe4f925c4502fd3e43ba6021dbf0bbd01c69556e3779ce6ad9e70c61158011fad4ee5549"
     "665a98365f8b345e3d589afc2f23a5404f05282d9b8b7d858a463f6460d64f8ae75ce6c23"
     "680831e44e014c69f7763f53295f4180c2949f0de73c653d244b10bd08dd9a72e3b2929ee"
     "9cdcacb687aef0004caca0000040000b20002a300f0ddbd9c010100a0f602f344b7cac7cd"
     "9ec544f5dcd4e59b00e726b096faa8169217e8d11bbbcf759075b4545aa2ee20a369a1955"
     "454d6e132a22840878ffb646b282f40d5f8b62d2a1ee6498cce66712c969aa4008a0f54f2"
     "037e12f90e62a9edcb13d03b96dc3ec9d79f82bf0ba43987e88223f6d2ba6f9b216bbc850"
     "d68aaeb63397124769fe0e4e238f2b38e7bb670a6fa0c070669228e94644071d564231349"
     "0c5d54435ece0004caca0000ba2330210201010204030013013016020101020117040e300"
     "c0201010201000201000101ffbb2330210201010204030013013016020101020117040e30"
     "0c0201010201000201000101ffbc030201ff020108020100a0030101ffa203020100",
     nullptr}};

class EncodeDecodeKATTest
    : public testing::TestWithParam<EncodeDecodeKATTestParam> {};

INSTANTIATE_TEST_SUITE_P(EncodeAndDecodeKATTests, EncodeDecodeKATTest,
                         testing::ValuesIn(kEncodeDecodeKATs));

TEST_P(EncodeDecodeKATTest, RoundTrips) {
  std::string input(GetParam().input);
  std::string output;
  if (GetParam().output) {
    output = std::string(GetParam().output);
  } else {
    output = std::string(GetParam().input);
  }

  std::vector<uint8_t> input_bytes;
  ASSERT_TRUE(DecodeHex(&input_bytes, input));
  std::vector<uint8_t> output_bytes;
  ASSERT_TRUE(DecodeHex(&output_bytes, output));

  bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
  // Check the bytes are decoded successfully.
  bssl::UniquePtr<SSL> ssl(
      SSL_from_bytes(input_bytes.data(), input_bytes.size(), server_ctx.get()));
  ASSERT_TRUE(ssl);
  // Check the ssl can be encoded successfully.
  size_t encoded_len = 0;
  uint8_t *encoded = nullptr;
  ASSERT_TRUE(SSL_to_bytes(ssl.get(), &encoded, &encoded_len));
  bssl::UniquePtr<uint8_t> encoded_ptr;
  encoded_ptr.reset(encoded);
  // Check the encoded bytes are the same as the test input.
  ASSERT_EQ(output_bytes.size(), encoded_len);
  ASSERT_EQ(OPENSSL_memcmp(output_bytes.data(), encoded, encoded_len), 0);
}

// Test that |SSL_shutdown|, when quiet shutdown is enabled, simulates receiving
// a close_notify, down to |SSL_read| reporting |SSL_ERROR_ZERO_RETURN|.
TEST(SSLTest, QuietShutdown) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  SSL_CTX_set_quiet_shutdown(server_ctx.get(), 1);
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  // Quiet shutdown is enabled, so |SSL_shutdown| on the server should
  // immediately return that bidirectional shutdown "completed".
  EXPECT_EQ(SSL_shutdown(server.get()), 1);

  // Shut down writes so the client gets an EOF.
  EXPECT_TRUE(BIO_shutdown_wr(SSL_get_wbio(server.get())));

  // Confirm no close notify was actually sent. Client reads should report a
  // transport EOF, not a close_notify. (Both have zero return, but
  // |SSL_get_error| is different.)
  char buf[1];
  int ret = SSL_read(client.get(), buf, sizeof(buf));
  EXPECT_EQ(ret, 0);
  EXPECT_EQ(SSL_get_error(client.get(), ret), SSL_ERROR_SYSCALL);

  // The server believes bidirectional shutdown completed, so reads should
  // replay the (simulated) close_notify.
  ret = SSL_read(server.get(), buf, sizeof(buf));
  EXPECT_EQ(ret, 0);
  EXPECT_EQ(SSL_get_error(server.get(), ret), SSL_ERROR_ZERO_RETURN);
}

// Test that |SSL_OP_IGNORE_UNEXPECTED_EOF| causes an unexpected transport EOF
// when the server closes without a close_notify to be reported as a clean shutdown,
// |SSL_ERROR_ZERO_RETURN|, instead of |SSL_ERROR_SYSCALL|.
TEST(SSLTest, IgnoreUnexpectedEOFClient) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  SSL_CTX_set_options(client_ctx.get(), SSL_OP_IGNORE_UNEXPECTED_EOF);
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  // Close the server's write side without sending a close_notify, so the client
  // sees an unexpected transport EOF.
  EXPECT_TRUE(BIO_shutdown_wr(SSL_get_wbio(server.get())));

  // With the option set, the client reports the EOF as a clean shutdown rather
  // than |SSL_ERROR_SYSCALL|.
  char buf[1];
  int ret = SSL_read(client.get(), buf, sizeof(buf));
  EXPECT_EQ(ret, 0);
  EXPECT_EQ(SSL_get_error(client.get(), ret), SSL_ERROR_ZERO_RETURN);
}

// Test that |SSL_OP_IGNORE_UNEXPECTED_EOF| behaves symmetrically on the server:
// when the client closes without a close_notify, the server's |SSL_read| reports
// the unexpected transport EOF as a clean shutdown with |SSL_ERROR_ZERO_RETURN|.
TEST(SSLTest, IgnoreUnexpectedEOFServer) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  SSL_CTX_set_options(server_ctx.get(), SSL_OP_IGNORE_UNEXPECTED_EOF);
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  // Close the client's write side without sending a close_notify, so the server
  // sees an unexpected transport EOF.
  EXPECT_TRUE(BIO_shutdown_wr(SSL_get_wbio(client.get())));

  // With the option set, the server reports the EOF as a clean shutdown rather
  // than |SSL_ERROR_SYSCALL|.
  char buf[1];
  int ret = SSL_read(server.get(), buf, sizeof(buf));
  EXPECT_EQ(ret, 0);
  EXPECT_EQ(SSL_get_error(server.get(), ret), SSL_ERROR_ZERO_RETURN);
}


TEST(SSLTest, InvalidSignatureAlgorithm) {
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);

  static const uint16_t kInvalidPrefs[] = {1234};
  EXPECT_FALSE(SSL_CTX_set_signing_algorithm_prefs(
      ctx.get(), kInvalidPrefs, OPENSSL_ARRAY_SIZE(kInvalidPrefs)));
  EXPECT_FALSE(SSL_CTX_set_verify_algorithm_prefs(
      ctx.get(), kInvalidPrefs, OPENSSL_ARRAY_SIZE(kInvalidPrefs)));

  static const uint16_t kDuplicatePrefs[] = {SSL_SIGN_RSA_PKCS1_SHA256,
                                             SSL_SIGN_RSA_PKCS1_SHA256};
  EXPECT_FALSE(SSL_CTX_set_signing_algorithm_prefs(
      ctx.get(), kDuplicatePrefs, OPENSSL_ARRAY_SIZE(kDuplicatePrefs)));
  EXPECT_FALSE(SSL_CTX_set_verify_algorithm_prefs(
      ctx.get(), kDuplicatePrefs, OPENSSL_ARRAY_SIZE(kDuplicatePrefs)));
}

TEST(SSLTest, ErrorStrings) {
  int warning_value = SSL3_AD_CLOSE_NOTIFY | (SSL3_AL_WARNING << 8);
  int fatal_value = SSL3_AD_UNEXPECTED_MESSAGE | (SSL3_AL_FATAL << 8);
  int unknown_value = 99999;

  EXPECT_EQ(Bytes(SSL_alert_desc_string(warning_value)), Bytes("CN"));
  EXPECT_EQ(Bytes(SSL_alert_desc_string(fatal_value)), Bytes("UM"));
  EXPECT_EQ(Bytes(SSL_alert_desc_string(unknown_value)), Bytes("UK"));

  EXPECT_EQ(Bytes(SSL_alert_type_string(warning_value)), Bytes("W"));
  EXPECT_EQ(Bytes(SSL_alert_type_string(fatal_value)), Bytes("F"));
  EXPECT_EQ(Bytes(SSL_alert_type_string(unknown_value)), Bytes("U"));
}

TEST(SSLTest, NameLists) {
  struct {
    size_t (*func)(const char **, size_t);
    std::vector<std::string> expected;
  } kTests[] = {
      {SSL_get_all_version_names, {"TLSv1.3", "DTLSv1.2", "unknown"}},
      {SSL_get_all_standard_cipher_names,
       {"TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256", "TLS_AES_128_GCM_SHA256"}},
      {SSL_get_all_cipher_names,
       {"ECDHE-ECDSA-AES128-GCM-SHA256", "TLS_AES_128_GCM_SHA256", "(NONE)"}},
      {SSL_get_all_group_names, {"P-256", "X25519"}},
      {SSL_get_all_signature_algorithm_names,
       {"rsa_pkcs1_sha256", "ecdsa_secp256r1_sha256", "ecdsa_sha256"}},
  };
  for (const auto &t : kTests) {
    size_t num = t.func(nullptr, 0);
    EXPECT_GT(num, 0u);

    std::vector<const char *> list(num);
    EXPECT_EQ(num, t.func(list.data(), list.size()));

    // Check the expected values are in the list.
    for (const auto &s : t.expected) {
      EXPECT_NE(list.end(), std::find(list.begin(), list.end(), s))
          << "Could not find " << s;
    }

    // Passing in a larger buffer should leave excess space alone.
    std::vector<const char *> list2(num + 1, "placeholder");
    EXPECT_EQ(num, t.func(list2.data(), list2.size()));
    for (size_t i = 0; i < num; i++) {
      EXPECT_STREQ(list[i], list2[i]);
    }
    EXPECT_STREQ(list2.back(), "placeholder");

    // Passing in a shorter buffer should truncate the list.
    for (size_t l = 0; l < num; l++) {
      SCOPED_TRACE(l);
      list2.resize(l);
      EXPECT_EQ(num, t.func(list2.data(), list2.size()));
      for (size_t i = 0; i < l; i++) {
        EXPECT_STREQ(list[i], list2[i]);
      }
    }
  }
}



TEST(SSLTest, SSLFileTests) {
#if defined(OPENSSL_ANDROID)
  // On Android, when running from an APK, temporary file creations do not work.
  // See b/36991167#comment8.
  GTEST_SKIP();
#endif

  char rsa_pem_filename[PATH_MAX];
  char ecdsa_pem_filename[PATH_MAX];
  ASSERT_TRUE(createTempFILEpath(rsa_pem_filename));
  ASSERT_TRUE(createTempFILEpath(ecdsa_pem_filename));

  ScopedFILE rsa_pem(fopen(rsa_pem_filename, "w"));
  ScopedFILE ecdsa_pem(fopen(ecdsa_pem_filename, "w"));
  ASSERT_TRUE(rsa_pem);
  ASSERT_TRUE(ecdsa_pem);

  bssl::UniquePtr<X509> rsa_leaf = GetChainTestCertificate();
  bssl::UniquePtr<X509> rsa_intermediate = GetChainTestIntermediate();
  bssl::UniquePtr<X509> ecdsa_leaf = GetECDSATestCertificate();
  ASSERT_TRUE(PEM_write_X509(rsa_pem.get(), rsa_leaf.get()));
  ASSERT_TRUE(PEM_write_X509(rsa_pem.get(), rsa_intermediate.get()));
  ASSERT_TRUE(PEM_write_X509(ecdsa_pem.get(), ecdsa_leaf.get()));
  rsa_pem.reset();
  ecdsa_pem.reset();

  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);
  // Load a certificate into |ctx| and verify that |ssl| inherits it.
  EXPECT_TRUE(SSL_CTX_use_certificate_chain_file(ctx.get(), rsa_pem_filename));
  bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
  ASSERT_TRUE(ssl);
  EXPECT_EQ(X509_cmp(SSL_get_certificate(ssl.get()), rsa_leaf.get()), 0);

  // Load a new cert into |ssl| and verify that it's correctly loaded.
  EXPECT_TRUE(SSL_use_certificate_chain_file(ssl.get(), ecdsa_pem_filename));
  EXPECT_EQ(X509_cmp(SSL_get_certificate(ssl.get()), ecdsa_leaf.get()), 0);

  ASSERT_EQ(remove(rsa_pem_filename), 0);
  ASSERT_EQ(remove(ecdsa_pem_filename), 0);
}

TEST(SSLTest, IncompatibleTLSVersionState) {
  // Using the following ASN.1 DER Sequence where 42 is the serialization
  // format version number of some future version not currently supported:
  // SEQUENCE {
  //   SEQUENCE {
  //     INTEGER { 42 }
  //   }
  // }
  static constexpr size_t INCOMPATIBLE_DER_LEN = 7;
  static const uint8_t INCOMPATIBLE_DER[INCOMPATIBLE_DER_LEN] = {
      0x30, 0x05, 0x30, 0x03, 0x02, 0x01, 0x2a};

  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);

  ASSERT_FALSE(
      SSL_from_bytes(INCOMPATIBLE_DER, INCOMPATIBLE_DER_LEN, ctx.get()));
  ASSERT_EQ(ERR_GET_LIB(ERR_peek_error()), ERR_LIB_SSL);
  ASSERT_EQ(ERR_GET_REASON(ERR_peek_error()),
            SSL_R_SERIALIZATION_INVALID_SERDE_VERSION);
}

// Test that it is possible for the certificate to be configured on a mix of
// SSL_CTX and SSL. This ensures that we do not inadvertently overshare objects
// in SSL_new.
TEST(SSLTest, MixContextAndConnection) {
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<X509> cert = GetTestCertificate();
  ASSERT_TRUE(cert);
  bssl::UniquePtr<EVP_PKEY> key = GetTestKey();
  ASSERT_TRUE(key);

  // Configure the certificate, but not the private key, on the context.
  ASSERT_TRUE(SSL_CTX_use_certificate(ctx.get(), cert.get()));

  bssl::UniquePtr<SSL> ssl1(SSL_new(ctx.get()));
  ASSERT_TRUE(ssl1.get());
  bssl::UniquePtr<SSL> ssl2(SSL_new(ctx.get()));
  ASSERT_TRUE(ssl2.get());

  // There is no private key configured yet.
  EXPECT_FALSE(SSL_CTX_get0_privatekey(ctx.get()));
  EXPECT_FALSE(SSL_get_privatekey(ssl1.get()));
  EXPECT_FALSE(SSL_get_privatekey(ssl2.get()));

  // Configuring the private key on |ssl1| works.
  ASSERT_TRUE(SSL_use_PrivateKey(ssl1.get(), key.get()));
  EXPECT_TRUE(SSL_get_privatekey(ssl1.get()));

  // It does not impact the other connection or the context.
  EXPECT_FALSE(SSL_CTX_get0_privatekey(ctx.get()));
  EXPECT_FALSE(SSL_get_privatekey(ssl2.get()));
}


static int XORCompressFunc(SSL *ssl, CBB *out, const uint8_t *in,
                           size_t in_len) {
  for (size_t i = 0; i < in_len; i++) {
    if (!CBB_add_u8(out, in[i] ^ 0x55)) {
      return 0;
    }
  }

  SSL_set_app_data(ssl, XORCompressFunc);

  return 1;
}

static int XORDecompressFunc(SSL *ssl, CRYPTO_BUFFER **out,
                             size_t uncompressed_len, const uint8_t *in,
                             size_t in_len) {
  if (in_len != uncompressed_len) {
    return 0;
  }

  uint8_t *data = nullptr;
  *out = CRYPTO_BUFFER_alloc(&data, uncompressed_len);
  if (*out == nullptr) {
    return 0;
  }

  for (size_t i = 0; i < in_len; i++) {
    data[i] = in[i] ^ 0x55;
  }

  SSL_set_app_data(ssl, XORDecompressFunc);

  return 1;
}

TEST(SSLTest, CertCompression) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx(
      CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_add_cert_compression_alg(
      client_ctx.get(), 0x1234, XORCompressFunc, XORDecompressFunc));
  ASSERT_TRUE(SSL_CTX_add_cert_compression_alg(
      server_ctx.get(), 0x1234, XORCompressFunc, XORDecompressFunc));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  EXPECT_TRUE(SSL_get_app_data(client.get()) == XORDecompressFunc);
  EXPECT_TRUE(SSL_get_app_data(server.get()) == XORCompressFunc);
}

static size_t test_ecc_privkey_calls = 0;

static enum ssl_private_key_result_t test_ecc_privkey_complete(SSL *ssl,
                                                               uint8_t *out,
                                                               size_t *out_len,
                                                               size_t max_out) {
  test_ecc_privkey_calls += 1;
  return ssl_private_key_success;
}

static enum ssl_private_key_result_t test_ecc_privkey_sign(
    SSL *ssl, uint8_t *out, size_t *out_len, size_t max_out,
    uint16_t signature_algorithm, const uint8_t *in, size_t in_len) {
  bssl::UniquePtr<EVP_PKEY> pkey(GetECDSATestKey());

  if (EVP_PKEY_id(pkey.get()) !=
      SSL_get_signature_algorithm_key_type(signature_algorithm)) {
    return ssl_private_key_failure;
  }

  const EVP_MD *md = SSL_get_signature_algorithm_digest(signature_algorithm);
  bssl::ScopedEVP_MD_CTX ctx;
  EVP_PKEY_CTX *pctx = nullptr;
  if (!EVP_DigestSignInit(ctx.get(), &pctx, md, nullptr, pkey.get())) {
    return ssl_private_key_failure;
  }

  size_t len = 0;
  if (!EVP_DigestSign(ctx.get(), nullptr, &len, in, in_len) || len > max_out) {
    return ssl_private_key_failure;
  }

  *out_len = max_out;

  if (!EVP_DigestSign(ctx.get(), out, out_len, in, in_len)) {
    return ssl_private_key_failure;
  }

  return test_ecc_privkey_complete(ssl, out, out_len, max_out);
}

static enum ssl_private_key_result_t test_ecc_privkey_decrypt(
    SSL *ssl, uint8_t *out, size_t *out_len, size_t max_out, const uint8_t *in,
    size_t in_len) {
  return ssl_private_key_failure;
}

static const SSL_PRIVATE_KEY_METHOD test_ecc_private_key_method = {
    test_ecc_privkey_sign,
    test_ecc_privkey_decrypt,
    test_ecc_privkey_complete,
};

TEST(SSLTest, SSLPrivateKeyMethod) {
  {
    bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
    bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));

    bssl::UniquePtr<X509> ecdsa_cert(GetECDSATestCertificate());
    bssl::UniquePtr<CRYPTO_BUFFER> ecdsa_leaf =
        x509_to_buffer(ecdsa_cert.get());
    std::vector<CRYPTO_BUFFER *> chain = {
        ecdsa_leaf.get(),
    };

    // Index should be have been set to default, 0, but no key loaded
    EXPECT_EQ(server_ctx->cert->cert_private_key_idx, SSL_PKEY_RSA);
    EXPECT_EQ(
        server_ctx->cert->cert_private_keys[SSL_PKEY_RSA].privatekey.get(),
        nullptr);
    EXPECT_EQ(server_ctx->cert->key_method, nullptr);


    // Load a certificate chain containg the leaf but set private key method
    ASSERT_TRUE(SSL_CTX_set_chain_and_key(server_ctx.get(), &chain[0],
                                          chain.size(), nullptr,
                                          &test_ecc_private_key_method));

    // Should be initiall zero
    ASSERT_EQ(test_ecc_privkey_calls, (size_t)0);

    // Index must be ECC key now, but key_method must be set.
    ASSERT_EQ(server_ctx->cert->cert_private_key_idx, SSL_PKEY_ECC);
    ASSERT_EQ(server_ctx->cert->key_method, &test_ecc_private_key_method);

    bssl::UniquePtr<SSL> client, server;
    ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                       server_ctx.get(), ClientConfig(),
                                       false));

    ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

    ASSERT_EQ(test_ecc_privkey_calls, (size_t)1);

    // Check the internal slot index to verify that the correct slot was used
    // during the handshake.
    ASSERT_EQ(server->config->cert->cert_private_key_idx, SSL_PKEY_ECC);
    ASSERT_EQ(server->config->cert->key_method, &test_ecc_private_key_method);
  }

  {
    size_t current_invoke_count = test_ecc_privkey_calls;

    bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
    bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));

    // Index should be have been set to default, 0, but no key loaded
    EXPECT_EQ(server_ctx->cert->cert_private_key_idx, SSL_PKEY_RSA);
    EXPECT_EQ(
        server_ctx->cert->cert_private_keys[SSL_PKEY_RSA].privatekey.get(),
        nullptr);
    EXPECT_EQ(server_ctx->cert->key_method, nullptr);

    bssl::UniquePtr<X509> ed_cert(GetED25519TestCertificate());
    bssl::UniquePtr<EVP_PKEY> ed_key(GetED25519TestKey());
    bssl::UniquePtr<CRYPTO_BUFFER> ed_leaf = x509_to_buffer(ed_cert.get());
    std::vector<CRYPTO_BUFFER *> ed_chain = {
        ed_leaf.get(),
    };

    // Load a certificate chain containg the leaf but set private key method
    ASSERT_TRUE(SSL_CTX_set_chain_and_key(server_ctx.get(), &ed_chain[0],
                                          ed_chain.size(), ed_key.get(),
                                          nullptr));

    // Index must be ECC key now, but key_method must not be set.
    ASSERT_EQ(server_ctx->cert->cert_private_key_idx, SSL_PKEY_ED25519);
    ASSERT_EQ(server_ctx->cert->key_method, nullptr);

    std::vector<uint16_t> sigalgs = {SSL_SIGN_ED25519};

    ASSERT_TRUE(SSL_CTX_set_signing_algorithm_prefs(
        client_ctx.get(), sigalgs.data(), sigalgs.size()));
    ASSERT_TRUE(SSL_CTX_set_verify_algorithm_prefs(
        client_ctx.get(), sigalgs.data(), sigalgs.size()));

    bssl::UniquePtr<SSL> client, server;
    ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                       server_ctx.get(), ClientConfig(),
                                       false));

    ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

    // This should still be the same, as we didn't use the private key method
    // functionality, so it shouldn't have incremented.
    ASSERT_EQ(test_ecc_privkey_calls, current_invoke_count);

    // Check the internal slot index to verify that the correct slot was used
    // during the handshake and that key_method was not set.
    ASSERT_EQ(server->config->cert->cert_private_key_idx, SSL_PKEY_ED25519);
    ASSERT_EQ(server->config->cert->key_method, nullptr);
  }
}

// Test that |key_method| and per-slot |privatekey| maintain mutual exclusivity
// across all mutation paths. Setting one must clear the other so that stale
// values never silently take effect.
TEST(SSLTest, KeyMethodPrivateKeyMutualExclusivity) {
  // Test 1: SSL_CTX_set_private_key_method clears the active slot's
  // privatekey.
  {
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    bssl::UniquePtr<X509> cert(GetECDSATestCertificate());
    bssl::UniquePtr<EVP_PKEY> key(GetECDSATestKey());
    ASSERT_TRUE(cert);
    ASSERT_TRUE(key);

    // Set a real private key first.
    ASSERT_TRUE(SSL_CTX_use_certificate(ctx.get(), cert.get()));
    ASSERT_TRUE(SSL_CTX_use_PrivateKey(ctx.get(), key.get()));
    EXPECT_EQ(ctx->cert->cert_private_key_idx, SSL_PKEY_ECC);
    EXPECT_NE(ctx->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
              nullptr);
    EXPECT_EQ(ctx->cert->key_method, nullptr);

    // Switch to key_method. The per-slot privatekey must be cleared.
    SSL_CTX_set_private_key_method(ctx.get(), &test_ecc_private_key_method);
    EXPECT_EQ(ctx->cert->key_method, &test_ecc_private_key_method);
    EXPECT_EQ(ctx->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
              nullptr);
  }

  // Test 2: SSL_use_PrivateKey clears key_method.
  {
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    bssl::UniquePtr<X509> cert(GetECDSATestCertificate());
    bssl::UniquePtr<EVP_PKEY> key(GetECDSATestKey());
    ASSERT_TRUE(cert);
    ASSERT_TRUE(key);

    // Set key_method first via set_chain_and_key.
    bssl::UniquePtr<CRYPTO_BUFFER> leaf = x509_to_buffer(cert.get());
    ASSERT_TRUE(leaf);
    std::vector<CRYPTO_BUFFER *> chain = {leaf.get()};
    ASSERT_TRUE(SSL_CTX_set_chain_and_key(ctx.get(), &chain[0], chain.size(),
                                          nullptr,
                                          &test_ecc_private_key_method));
    EXPECT_EQ(ctx->cert->key_method, &test_ecc_private_key_method);
    EXPECT_EQ(ctx->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
              nullptr);

    // Now create an SSL and switch to a real private key. key_method must be
    // cleared.
    bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
    ASSERT_TRUE(ssl);
    ASSERT_TRUE(SSL_use_PrivateKey(ssl.get(), key.get()));
    EXPECT_EQ(ssl->config->cert->key_method, nullptr);
    EXPECT_NE(
        ssl->config->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
        nullptr);
  }

  // Test 3: SSL_CTX_set_chain_and_key with a real privkey clears a
  // pre-existing key_method.
  {
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    bssl::UniquePtr<X509> cert(GetECDSATestCertificate());
    bssl::UniquePtr<EVP_PKEY> key(GetECDSATestKey());
    ASSERT_TRUE(cert);
    ASSERT_TRUE(key);

    // Set key_method first.
    SSL_CTX_set_private_key_method(ctx.get(), &test_ecc_private_key_method);
    EXPECT_EQ(ctx->cert->key_method, &test_ecc_private_key_method);

    // Now set chain and key with a real privkey. key_method must be cleared.
    bssl::UniquePtr<CRYPTO_BUFFER> leaf = x509_to_buffer(cert.get());
    ASSERT_TRUE(leaf);
    std::vector<CRYPTO_BUFFER *> chain = {leaf.get()};
    ASSERT_TRUE(SSL_CTX_set_chain_and_key(ctx.get(), &chain[0], chain.size(),
                                          key.get(), nullptr));
    EXPECT_EQ(ctx->cert->key_method, nullptr);
    EXPECT_NE(ctx->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
              nullptr);
  }

  // Test 4: SSL_CTX_set_chain_and_key with key_method clears a pre-existing
  // per-slot privatekey.
  {
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    bssl::UniquePtr<X509> cert(GetECDSATestCertificate());
    bssl::UniquePtr<EVP_PKEY> key(GetECDSATestKey());
    ASSERT_TRUE(cert);
    ASSERT_TRUE(key);

    // Set a real private key first.
    ASSERT_TRUE(SSL_CTX_use_certificate(ctx.get(), cert.get()));
    ASSERT_TRUE(SSL_CTX_use_PrivateKey(ctx.get(), key.get()));
    EXPECT_NE(ctx->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
              nullptr);
    EXPECT_EQ(ctx->cert->key_method, nullptr);

    // Now set chain and key with key_method. The per-slot privatekey must be
    // cleared.
    bssl::UniquePtr<CRYPTO_BUFFER> leaf = x509_to_buffer(cert.get());
    ASSERT_TRUE(leaf);
    std::vector<CRYPTO_BUFFER *> chain = {leaf.get()};
    ASSERT_TRUE(SSL_CTX_set_chain_and_key(ctx.get(), &chain[0], chain.size(),
                                          nullptr,
                                          &test_ecc_private_key_method));
    EXPECT_EQ(ctx->cert->key_method, &test_ecc_private_key_method);
    EXPECT_EQ(ctx->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
              nullptr);
  }

  // Test 5: Handshake succeeds after switching from key_method to a real
  // private key. Verifies the stale key_method does not interfere.
  {
    size_t calls_before = test_ecc_privkey_calls;

    bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
    bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(client_ctx);
    ASSERT_TRUE(server_ctx);

    bssl::UniquePtr<X509> cert(GetECDSATestCertificate());
    bssl::UniquePtr<EVP_PKEY> key(GetECDSATestKey());
    ASSERT_TRUE(cert);
    ASSERT_TRUE(key);

    // Configure with key_method first.
    bssl::UniquePtr<CRYPTO_BUFFER> leaf = x509_to_buffer(cert.get());
    ASSERT_TRUE(leaf);
    std::vector<CRYPTO_BUFFER *> chain = {leaf.get()};
    ASSERT_TRUE(SSL_CTX_set_chain_and_key(server_ctx.get(), &chain[0],
                                          chain.size(), nullptr,
                                          &test_ecc_private_key_method));
    ASSERT_EQ(server_ctx->cert->key_method, &test_ecc_private_key_method);

    // Switch to a real private key. key_method must be cleared.
    ASSERT_TRUE(SSL_CTX_use_PrivateKey(server_ctx.get(), key.get()));
    ASSERT_EQ(server_ctx->cert->key_method, nullptr);
    ASSERT_NE(
        server_ctx->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
        nullptr);

    // Handshake must succeed using the real key, not the old key_method.
    bssl::UniquePtr<SSL> client, server;
    ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                       server_ctx.get(), ClientConfig(),
                                       false));
    ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

    // The key_method callbacks must not have been invoked.
    ASSERT_EQ(test_ecc_privkey_calls, calls_before);
  }

  // Test 6: Handshake succeeds after switching from a real private key to
  // key_method. Verifies the key_method callbacks are used, not the stale
  // per-slot privatekey.
  {
    size_t calls_before = test_ecc_privkey_calls;

    bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
    bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(client_ctx);
    ASSERT_TRUE(server_ctx);

    bssl::UniquePtr<X509> cert(GetECDSATestCertificate());
    bssl::UniquePtr<EVP_PKEY> key(GetECDSATestKey());
    ASSERT_TRUE(cert);
    ASSERT_TRUE(key);

    // Configure with a real private key first.
    ASSERT_TRUE(SSL_CTX_use_certificate(server_ctx.get(), cert.get()));
    ASSERT_TRUE(SSL_CTX_use_PrivateKey(server_ctx.get(), key.get()));
    ASSERT_EQ(server_ctx->cert->key_method, nullptr);

    // Switch to key_method. The per-slot privatekey must be cleared.
    SSL_CTX_set_private_key_method(server_ctx.get(),
                                   &test_ecc_private_key_method);
    ASSERT_EQ(server_ctx->cert->key_method, &test_ecc_private_key_method);
    ASSERT_EQ(
        server_ctx->cert->cert_private_keys[SSL_PKEY_ECC].privatekey.get(),
        nullptr);

    // Handshake must succeed using the key_method callbacks.
    bssl::UniquePtr<SSL> client, server;
    ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                       server_ctx.get(), ClientConfig(),
                                       false));
    ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

    // The key_method callbacks must have been invoked.
    ASSERT_GT(test_ecc_privkey_calls, calls_before);
  }
}


// Test that failures are supressed on (potentially)
// transient empty reads.
TEST(SSLTest, IntermittentEmptyRead) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  // Create a fake read BIO that returns 0 on read to simulate empty read
  bssl::UniquePtr<BIO_METHOD> method(BIO_meth_new(0, nullptr));
  ASSERT_TRUE(method);
  ASSERT_TRUE(BIO_meth_set_create(method.get(), [](BIO *b) -> int {
    BIO_set_init(b, 1);
    return 1;
  }));
  ASSERT_TRUE(BIO_meth_set_read(method.get(),
                                [](BIO *, char *, int) -> int { return 0; }));
  bssl::UniquePtr<BIO> rbio_empty(BIO_new(method.get()));
  ASSERT_TRUE(rbio_empty);
  BIO_set_flags(rbio_empty.get(), BIO_FLAGS_READ);

  // Save off client rbio and use empty read BIO
  bssl::UniquePtr<BIO> client_rbio(SSL_get_rbio(client.get()));
  ASSERT_TRUE(client_rbio);
  // Up-ref |client_rbio| as SSL_CTX dtor will also attempt to free it
  ASSERT_TRUE(BIO_up_ref(client_rbio.get()));
  SSL_set0_rbio(client.get(), rbio_empty.release());

  // Server writes some data to the client
  const uint8_t write_data[] = {1, 2, 3};
  int ret = SSL_write(server.get(), write_data, (int)sizeof(write_data));
  EXPECT_EQ(ret, (int)sizeof(write_data));
  EXPECT_EQ(SSL_get_error(server.get(), ret), SSL_ERROR_NONE);

  uint8_t read_data[] = {0, 0, 0};
  ret = SSL_read(client.get(), read_data, sizeof(read_data));
  EXPECT_EQ(ret, 0);
  // On empty read, client should still want a read so caller will retry.
  // This would have returned |SSL_ERROR_SYSCALL| in OpenSSL 1.0.2.
  EXPECT_EQ(SSL_get_error(client.get(), ret), SSL_ERROR_WANT_READ);

  // Reset client rbio, read should succeed
  SSL_set0_rbio(client.get(), client_rbio.release());
  ret = SSL_read(client.get(), read_data, sizeof(read_data));
  EXPECT_EQ(ret, (int)sizeof(write_data));
  EXPECT_EQ(OPENSSL_memcmp(read_data, write_data, sizeof(write_data)), 0);
  EXPECT_EQ(SSL_get_error(client.get(), ret), SSL_ERROR_NONE);

  // Subsequent attempts to read should fail
  ret = SSL_read(client.get(), read_data, sizeof(read_data));
  EXPECT_LT(ret, 0);
  EXPECT_EQ(SSL_get_error(client.get(), ret), SSL_ERROR_WANT_READ);
}

enum ssl_test_ticket_aead_failure_mode {
  ssl_test_ticket_aead_ok = 0,
  ssl_test_ticket_aead_seal_fail,
  ssl_test_ticket_aead_open_soft_fail,
  ssl_test_ticket_aead_open_hard_fail,
};

struct ssl_test_ticket_aead_state {
  unsigned retry_count = 0;
  ssl_test_ticket_aead_failure_mode failure_mode = ssl_test_ticket_aead_ok;
};

static int ssl_test_ticket_aead_ex_index_dup(CRYPTO_EX_DATA *to,
                                             const CRYPTO_EX_DATA *from,
                                             void **from_d, int index,
                                             long argl, void *argp) {
  abort();
}

static void ssl_test_ticket_aead_ex_index_free(void *parent, void *ptr,
                                               CRYPTO_EX_DATA *ad, int index,
                                               long argl, void *argp) {
  delete reinterpret_cast<ssl_test_ticket_aead_state *>(ptr);
}

static CRYPTO_once_t g_ssl_test_ticket_aead_ex_index_once = CRYPTO_ONCE_INIT;
static int g_ssl_test_ticket_aead_ex_index;

static int ssl_test_ticket_aead_get_ex_index() {
  CRYPTO_once(&g_ssl_test_ticket_aead_ex_index_once, [] {
    g_ssl_test_ticket_aead_ex_index = SSL_get_ex_new_index(
        0, nullptr, nullptr, ssl_test_ticket_aead_ex_index_dup,
        ssl_test_ticket_aead_ex_index_free);
  });
  return g_ssl_test_ticket_aead_ex_index;
}

static size_t ssl_test_ticket_aead_max_overhead(SSL *ssl) { return 1; }

static int ssl_test_ticket_aead_seal(SSL *ssl, uint8_t *out, size_t *out_len,
                                     size_t max_out_len, const uint8_t *in,
                                     size_t in_len) {
  auto state = reinterpret_cast<ssl_test_ticket_aead_state *>(
      SSL_get_ex_data(ssl, ssl_test_ticket_aead_get_ex_index()));

  if (state->failure_mode == ssl_test_ticket_aead_seal_fail ||
      max_out_len < in_len + 1) {
    return 0;
  }

  OPENSSL_memmove(out, in, in_len);
  out[in_len] = 0xff;
  *out_len = in_len + 1;

  return 1;
}

static ssl_ticket_aead_result_t ssl_test_ticket_aead_open(
    SSL *ssl, uint8_t *out, size_t *out_len, size_t max_out_len,
    const uint8_t *in, size_t in_len) {
  auto state = reinterpret_cast<ssl_test_ticket_aead_state *>(
      SSL_get_ex_data(ssl, ssl_test_ticket_aead_get_ex_index()));

  if (state->retry_count > 0) {
    state->retry_count--;
    return ssl_ticket_aead_retry;
  }

  switch (state->failure_mode) {
    case ssl_test_ticket_aead_ok:
      break;
    case ssl_test_ticket_aead_seal_fail:
      // If |seal| failed then there shouldn't be any ticket to try and
      // decrypt.
      abort();
      break;
    case ssl_test_ticket_aead_open_soft_fail:
      return ssl_ticket_aead_ignore_ticket;
    case ssl_test_ticket_aead_open_hard_fail:
      return ssl_ticket_aead_error;
  }

  if (in_len == 0 || in[in_len - 1] != 0xff) {
    return ssl_ticket_aead_ignore_ticket;
  }

  if (max_out_len < in_len - 1) {
    return ssl_ticket_aead_error;
  }

  OPENSSL_memmove(out, in, in_len - 1);
  *out_len = in_len - 1;
  return ssl_ticket_aead_success;
}

static const SSL_TICKET_AEAD_METHOD kSSLTestTicketMethod = {
    ssl_test_ticket_aead_max_overhead,
    ssl_test_ticket_aead_seal,
    ssl_test_ticket_aead_open,
};

static void ConnectClientAndServerWithTicketMethod(
    bssl::UniquePtr<SSL> *out_client, bssl::UniquePtr<SSL> *out_server,
    SSL_CTX *client_ctx, SSL_CTX *server_ctx, unsigned retry_count,
    ssl_test_ticket_aead_failure_mode failure_mode, SSL_SESSION *session) {
  bssl::UniquePtr<SSL> client(SSL_new(client_ctx)), server(SSL_new(server_ctx));
  ASSERT_TRUE(client);
  ASSERT_TRUE(server);
  SSL_set_connect_state(client.get());
  SSL_set_accept_state(server.get());

  auto state = new ssl_test_ticket_aead_state;
  state->retry_count = retry_count;
  state->failure_mode = failure_mode;

  ASSERT_GE(ssl_test_ticket_aead_get_ex_index(), 0);
  ASSERT_TRUE(SSL_set_ex_data(server.get(), ssl_test_ticket_aead_get_ex_index(),
                              state));

  SSL_set_session(client.get(), session);

  BIO *bio1 = nullptr, *bio2 = nullptr;
  ASSERT_TRUE(BIO_new_bio_pair(&bio1, 0, &bio2, 0));

  // SSL_set_bio takes ownership.
  SSL_set_bio(client.get(), bio1, bio1);
  SSL_set_bio(server.get(), bio2, bio2);

  if (CompleteHandshakes(client.get(), server.get())) {
    *out_client = std::move(client);
    *out_server = std::move(server);
  } else {
    out_client->reset();
    out_server->reset();
  }
}

using TicketAEADMethodParam =
    testing::tuple<uint16_t, unsigned, ssl_test_ticket_aead_failure_mode, bool>;

class TicketAEADMethodTest
    : public ::testing::TestWithParam<TicketAEADMethodParam> {};

TEST_P(TicketAEADMethodTest, Resume) {
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(server_ctx);
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);

  const uint16_t version = testing::get<0>(GetParam());
  const unsigned retry_count = testing::get<1>(GetParam());
  const ssl_test_ticket_aead_failure_mode failure_mode =
      testing::get<2>(GetParam());
  const bool transfer_ssl = testing::get<3>(GetParam());

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), version));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), version));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), version));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), version));

  SSL_CTX_set_session_cache_mode(client_ctx.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_current_time_cb(client_ctx.get(), FrozenTimeCallback);
  SSL_CTX_set_current_time_cb(server_ctx.get(), FrozenTimeCallback);
  SSL_CTX_sess_set_new_cb(client_ctx.get(), SaveLastSession);

  SSL_CTX_set_ticket_aead_method(server_ctx.get(), &kSSLTestTicketMethod);

  bssl::UniquePtr<SSL> client, server;
  ASSERT_NO_FATAL_FAILURE(ConnectClientAndServerWithTicketMethod(
      &client, &server, client_ctx.get(), server_ctx.get(), retry_count,
      failure_mode, nullptr));
  // Only transfer when the code is to test SSL transfer and the connection is
  // finished successuflly.
  if (transfer_ssl && server) {
    // |server| is reset to hold the transferred SSL.
    TransferSSL(&server, server_ctx.get(), nullptr);
  }
  switch (failure_mode) {
    case ssl_test_ticket_aead_ok:
    case ssl_test_ticket_aead_open_hard_fail:
    case ssl_test_ticket_aead_open_soft_fail:
      ASSERT_TRUE(client);
      break;
    case ssl_test_ticket_aead_seal_fail:
      EXPECT_FALSE(client);
      return;
  }
  EXPECT_FALSE(SSL_session_reused(client.get()));
  EXPECT_FALSE(SSL_session_reused(server.get()));

  ASSERT_TRUE(FlushNewSessionTickets(client.get(), server.get()));
  bssl::UniquePtr<SSL_SESSION> session = std::move(g_last_session);
  ASSERT_NO_FATAL_FAILURE(ConnectClientAndServerWithTicketMethod(
      &client, &server, client_ctx.get(), server_ctx.get(), retry_count,
      failure_mode, session.get()));
  // Do SSL transfer again.
  // Only transfer when the code is to test SSL transfer and the connection is
  // finished successuflly.
  if (transfer_ssl && server) {
    // |server| is reset to hold the transferred SSL.
    TransferSSL(&server, server_ctx.get(), nullptr);
  }
  switch (failure_mode) {
    case ssl_test_ticket_aead_ok:
      ASSERT_TRUE(client);
      EXPECT_TRUE(SSL_session_reused(client.get()));
      EXPECT_TRUE(SSL_session_reused(server.get()));
      break;
    case ssl_test_ticket_aead_seal_fail:
      abort();
      break;
    case ssl_test_ticket_aead_open_hard_fail:
      EXPECT_FALSE(client);
      break;
    case ssl_test_ticket_aead_open_soft_fail:
      ASSERT_TRUE(client);
      EXPECT_FALSE(SSL_session_reused(client.get()));
      EXPECT_FALSE(SSL_session_reused(server.get()));
  }
}

static std::string TicketAEADMethodParamToString(
    const testing::TestParamInfo<TicketAEADMethodParam> &params) {
  std::string ret = GetVersionName(std::get<0>(params.param));
  // GTest only allows alphanumeric characters and '_' in the parameter
  // string. Additionally filter out the 'v' to get "TLS13" over "TLSv13".
  for (auto it = ret.begin(); it != ret.end();) {
    if (*it == '.' || *it == 'v') {
      it = ret.erase(it);
    } else {
      ++it;
    }
  }
  char retry_count[256];
  snprintf(retry_count, sizeof(retry_count), "%u", std::get<1>(params.param));
  ret += "_";
  ret += retry_count;
  ret += "Retries_";
  switch (std::get<2>(params.param)) {
    case ssl_test_ticket_aead_ok:
      ret += "OK";
      break;
    case ssl_test_ticket_aead_seal_fail:
      ret += "SealFail";
      break;
    case ssl_test_ticket_aead_open_soft_fail:
      ret += "OpenSoftFail";
      break;
    case ssl_test_ticket_aead_open_hard_fail:
      ret += "OpenHardFail";
      break;
  }
  if (std::get<3>(params.param)) {
    ret += "_SSLTransfer";
  }
  return ret;
}

INSTANTIATE_TEST_SUITE_P(
    TicketAEADMethodTests, TicketAEADMethodTest,
    testing::Combine(testing::Values(TLS1_2_VERSION, TLS1_3_VERSION),
                     testing::Values(0, 1, 2),
                     testing::Values(ssl_test_ticket_aead_ok,
                                     ssl_test_ticket_aead_seal_fail,
                                     ssl_test_ticket_aead_open_soft_fail,
                                     ssl_test_ticket_aead_open_hard_fail),
                     testing::Values(TRANSFER_SSL, !TRANSFER_SSL)),
    TicketAEADMethodParamToString);

TEST(SSLTest, GetTrafficSecrets) {
  // Set up client and server contexts with TLS 1.3
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  // Ensure TLS 1.3 is used
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));

  // Connect client and server
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  // Test getting traffic secrets
  uint8_t client_read_secret[SSL_MAX_MD_SIZE] = {0};
  uint8_t client_write_secret[SSL_MAX_MD_SIZE] = {0};
  uint8_t server_read_secret[SSL_MAX_MD_SIZE] = {0};
  uint8_t server_write_secret[SSL_MAX_MD_SIZE] = {0};
  size_t client_read_len = 0, client_write_len = 0, server_read_len = 0,
         server_write_len = 0;

  // First check the lengths
  ASSERT_TRUE(
      SSL_get_read_traffic_secret(client.get(), nullptr, &client_read_len));
  ASSERT_TRUE(
      SSL_get_write_traffic_secret(client.get(), nullptr, &client_write_len));
  ASSERT_TRUE(
      SSL_get_read_traffic_secret(server.get(), nullptr, &server_read_len));
  ASSERT_TRUE(
      SSL_get_write_traffic_secret(server.get(), nullptr, &server_write_len));

  ASSERT_EQ(client_read_len, server_write_len);
  ASSERT_EQ(client_write_len, server_read_len);

  // Get the actual secrets
  ASSERT_TRUE(SSL_get_read_traffic_secret(client.get(), client_read_secret,
                                          &client_read_len));
  ASSERT_TRUE(SSL_get_write_traffic_secret(client.get(), client_write_secret,
                                           &client_write_len));
  ASSERT_TRUE(SSL_get_read_traffic_secret(server.get(), server_read_secret,
                                          &server_read_len));
  ASSERT_TRUE(SSL_get_write_traffic_secret(server.get(), server_write_secret,
                                           &server_write_len));

  // Client's read secret should match server's write secret
  ASSERT_EQ(0, OPENSSL_memcmp(client_read_secret, server_write_secret,
                              client_read_len));
  // Client's write secret should match server's read secret
  ASSERT_EQ(0, OPENSSL_memcmp(client_write_secret, server_read_secret,
                              client_write_len));

  // Test error cases
  bssl::UniquePtr<SSL> unconnected(SSL_new(client_ctx.get()));
  ASSERT_TRUE(unconnected);
  size_t unused = 0;
  ASSERT_FALSE(
      SSL_get_read_traffic_secret(unconnected.get(), nullptr, &unused));
  ASSERT_FALSE(
      SSL_get_write_traffic_secret(unconnected.get(), nullptr, &unused));

  // Test buffer too small
  uint8_t small_buffer[1];
  size_t actual_size = sizeof(small_buffer);
  ASSERT_FALSE(
      SSL_get_read_traffic_secret(client.get(), small_buffer, &actual_size));
  ASSERT_FALSE(
      SSL_get_write_traffic_secret(client.get(), small_buffer, &actual_size));

  // Passing null buffers and null size
  ASSERT_FALSE(SSL_get_read_traffic_secret(client.get(), nullptr, nullptr));
  ASSERT_FALSE(SSL_get_write_traffic_secret(client.get(), nullptr, nullptr));
}

struct InvalidTransferEncodingTestParam {
  const char *input;
};

static const InvalidTransferEncodingTestParam kInvalidTransferEncodings[] = {
    // The serialized read_buffer OCTET STRING has 8 extra bytes then it should
    {"3082026e3082026a020101020203040202400030820250020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a0030201ed020102a93630340201020101ff02011802010502011802010004201703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7e0001020304050607ab03020100ac0301"
     "0100ad03010100ae03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b"
     "1f3d2c95839fd9b380b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b4"
     "3f7857797569b0459c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e"
     "4d4d31da55f01129be4208c0f4a8bca3da25615289c1eba7674b503020120b91d041b1703"
     "030016835338b2dcab482315b8d92421d601c602b50f2041bbba233021020101020403001"
     "3013016020101020117040e300c0201010201000201000101ffbb23302102010102040300"
     "13013016020101020117040e300c0201010201000201000101ffbc0302010102010802010"
     "0a203020100"},
    // The serialized read_buffer OCTET STRING is larger then inline_buf_
    {"308202493082024502010102020304020240003082022b020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a91d30"
     "1b020102010100020105020105020100020100040700010203040506ab03020100ac03010"
     "100ad03010100ae03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1"
     "f3d2c95839fd9b380b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43"
     "f7857797569b0459c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4"
     "d4d31da55f01129be4208c0f4a8bca3da25615289c1eba7674b503020120b91d041b17030"
     "30016835338b2dcab482315b8d92421d601c602b50f2041bbba2330210201010204030013"
     "013016020101020117040e300c0201010201000201000101ffbb233021020101020403001"
     "3013016020101020117040e300c0201010201000201000101ffbc03020101020108020100"
     "a203020100"},
    // pending_app_data offset is smaller then what is representable as int64
    {"3082027530820271020101020203040202400030820257020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a81930"
     "17a0120210ffffffffffffffff7fffffffffffffff020102a92e302c0201020101ff02011"
     "802010502011802010004181703030013313917e5402dfef87b011cdaedf9fb56e0ce7eab"
     "03020100ac03010100ad03010100ae03010100af03020100b0220420ee4651b968739c5f6"
     "e10f16b49f8e4b1f3d2c95839fd9b380b90a2f327e8d8a1b103020120b22204202d2027d1"
     "5f143f516939b43f7857797569b0459c0b3f9e02c9d798e891d5a5a6b303020120b422042"
     "07d6420075c44e4d4d31da55f01129be4208c0f4a8bca3da25615289c1eba7674b5030201"
     "20b91d041b1703030016835338b2dcab482315b8d92421d601c602b50f2041bbba2330210"
     "201010204030013013016020101020117040e300c0201010201000201000101ffbb233021"
     "0201010204030013013016020101020117040e300c0201010201000201000101ffbc03020"
     "101020108020100a203020100"},
    // pending_app_data offset is larger then what is representable as int64
    {"3082027530820271020101020203040202400030820257020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a81930"
     "17a012021000000000000000008000000000000000020102a92e302c0201020101ff02011"
     "802010502011802010004181703030013313917e5402dfef87b011cdaedf9fb56e0ce7eab"
     "03020100ac03010100ad03010100ae03010100af03020100b0220420ee4651b968739c5f6"
     "e10f16b49f8e4b1f3d2c95839fd9b380b90a2f327e8d8a1b103020120b22204202d2027d1"
     "5f143f516939b43f7857797569b0459c0b3f9e02c9d798e891d5a5a6b303020120b422042"
     "07d6420075c44e4d4d31da55f01129be4208c0f4a8bca3da25615289c1eba7674b5030201"
     "20b91d041b1703030016835338b2dcab482315b8d92421d601c602b50f2041bbba2330210"
     "201010204030013013016020101020117040e300c0201010201000201000101ffbb233021"
     "0201010204030013013016020101020117040e300c0201010201000201000101ffbc03020"
     "101020108020100a203020100"},
    // pending_app_data offset falls outside of read_buffer
    {"3082026630820262020101020203040202400030820248020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a003020108020100a92e302c0201020101ff02011802010502011802010004181703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae"
     "03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b3"
     "80b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b045"
     "9c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129"
     "be4208c0f4a8bca3da25615289c1eba7674b503020120b91d041b1703030016835338b2dc"
     "ab482315b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020"
     "117040e300c0201010201000201000101ffbb233021020101020403001301301602010102"
     "0117040e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // pending_app_data offset+size falls outside of read_buffer
    {"3082026630820262020101020203040202400030820248020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a003020107020101a92e302c0201020101ff02011802010502011802010004181703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae"
     "03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b3"
     "80b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b045"
     "9c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129"
     "be4208c0f4a8bca3da25615289c1eba7674b503020120b91d041b1703030016835338b2dc"
     "ab482315b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020"
     "117040e300c0201010201000201000101ffbb233021020101020403001301301602010102"
     "0117040e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // pending_app_data offset falls before read_buffer start
    {"3082026630820262020101020203040202400030820248020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a0030201e0020100a92e302c0201020101ff02011802010502011802010004181703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae"
     "03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b3"
     "80b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b045"
     "9c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129"
     "be4208c0f4a8bca3da25615289c1eba7674b503020120b91d041b1703030016835338b2dc"
     "ab482315b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020"
     "117040e300c0201010201000201000101ffbb233021020101020403001301301602010102"
     "0117040e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // CBS_len(&previous_client_finished) != previous_client_finished_len
    {"3082026630820262020101020203040202400030820248020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201050420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a0030201ed020102a92e302c0201020101ff02011802010502011802010004181703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae"
     "03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b3"
     "80b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b045"
     "9c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129"
     "be4208c0f4a8bca3da25615289c1eba7674b503020120b91d041b1703030016835338b2dc"
     "ab482315b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020"
     "117040e300c0201010201000201000101ffbb233021020101020403001301301602010102"
     "0117040e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // previous_client_finished_len > PREV_FINISHED_MAX_SIZE
    {"3082028730820283020101020203040202400030820269020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010441da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d42120521da505fe678128954a65c747dee14514f160b3b3c670e10469"
     "c94e67d42120521ff0201410420b4a380d705da4a9138c941f97a95ff5029e0a91041d019"
     "c7f311126faf72e728020120020100020100020100a050304e02010102020304040213010"
     "4000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30df354a1"
     "06020468a38654a205020302a300a4020400b20302011db9050203093a80a80a3008a0030"
     "201ed020102a92e302c0201020101ff020118020105020118020100041817030300133139"
     "17e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae0301010"
     "0af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b380b90a2"
     "f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b0459c0b3f9"
     "e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129be4208c"
     "0f4a8bca3da25615289c1eba7674b503020120b91d041b1703030016835338b2dcab48231"
     "5b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020117040e"
     "300c0201010201000201000101ffbb2330210201010204030013013016020101020117040"
     "e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // CBS_len(&previous_server_finished) != previous_server_finished_len)
    {"3082026630820262020101020203040202400030820248020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020140020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a0030201ed020102a92e302c0201020101ff02011802010502011802010004181703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae"
     "03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b3"
     "80b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b045"
     "9c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129"
     "be4208c0f4a8bca3da25615289c1eba7674b503020120b91d041b1703030016835338b2dc"
     "ab482315b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020"
     "117040e300c0201010201000201000101ffbb233021020101020403001301301602010102"
     "0117040e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // previous_server_finished_len > PREV_FINISHED_MAX_SIZE
    {"3082028730820283020101020203040202400030820269020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200441b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728b4a380d705da4a9138c941f97a95ff5029e0a91041d019c7"
     "f311126faf72e728ff020141020100020100020100a050304e02010102020304040213010"
     "4000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30df354a1"
     "06020468a38654a205020302a300a4020400b20302011db9050203093a80a80a3008a0030"
     "201ed020102a92e302c0201020101ff020118020105020118020100041817030300133139"
     "17e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae0301010"
     "0af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b380b90a2"
     "f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b0459c0b3f9"
     "e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129be4208c"
     "0f4a8bca3da25615289c1eba7674b503020120b91d041b1703030016835338b2dcab48231"
     "5b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020117040e"
     "300c0201010201000201000101ffbb2330210201010204030013013016020101020117040"
     "e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // exporter_secret_len > UINT8_MAX
    {"3082026730820263020101020203040202400030820249020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a0030201ed020102a92e302c0201020101ff02011802010502011802010004181703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae"
     "03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b3"
     "80b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b045"
     "9c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129"
     "be4208c0f4a8bca3da25615289c1eba7674b50402020100b91d041b1703030016835338b2"
     "dcab482315b8d92421d601c602b50f2041bbba23302102010102040300130130160201010"
     "20117040e300c0201010201000201000101ffbb2330210201010204030013013016020101"
     "020117040e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // exporter_secret_len > SSL_MAX_MD_SIZE
    {"3082026630820262020101020203040202400030820248020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a0030201ed020102a92e302c0201020101ff02011802010502011802010004181703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae"
     "03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b3"
     "80b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b045"
     "9c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129"
     "be4208c0f4a8bca3da25615289c1eba7674b503020131b91d041b1703030016835338b2dc"
     "ab482315b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020"
     "117040e300c0201010201000201000101ffbb233021020101020403001301301602010102"
     "0117040e300c0201010201000201000101ffbc03020101020108020100a203020100"},
    // CBS_len(&exporter_secret) != exporter_secret_len)
    {"3082026630820262020101020203040202400030820248020103040800000000000000140"
     "40800000000000000000420b01de2070af16e7412a6fa4fd947248c2df0f1ebbaf3adc9a4"
     "b97f3b0be6ae360420a606b526d185e18a6eb1a2d88943d3ac4d2d14db0807af71d39f4f8"
     "77d3ec125040200000201000201010420da505fe678128954a65c747dee14514f160b3b3c"
     "670e10469c94e67d421205210201200420b4a380d705da4a9138c941f97a95ff5029e0a91"
     "041d019c7f311126faf72e728020120020100020100020100a050304e0201010202030404"
     "02130104000420ae1689a5645a728a77eb1869e3b2d7d67b79c12229e7027b2847215ca30"
     "df354a106020468a38654a205020302a300a4020400b20302011db9050203093a80a80a30"
     "08a0030201ed020102a92e302c0201020101ff02011802010502011802010004181703030"
     "013313917e5402dfef87b011cdaedf9fb56e0ce7eab03020100ac03010100ad03010100ae"
     "03010100af03020100b0220420ee4651b968739c5f6e10f16b49f8e4b1f3d2c95839fd9b3"
     "80b90a2f327e8d8a1b103020120b22204202d2027d15f143f516939b43f7857797569b045"
     "9c0b3f9e02c9d798e891d5a5a6b303020120b42204207d6420075c44e4d4d31da55f01129"
     "be4208c0f4a8bca3da25615289c1eba7674b503020130b91d041b1703030016835338b2dc"
     "ab482315b8d92421d601c602b50f2041bbba2330210201010204030013013016020101020"
     "117040e300c0201010201000201000101ffbb233021020101020403001301301602010102"
     "0117040e300c0201010201000201000101ffbc03020101020108020100a203020100"},
};

class InvalidTransferEncoding
    : public testing::TestWithParam<InvalidTransferEncodingTestParam> {};

INSTANTIATE_TEST_SUITE_P(InvalidTransferEncodings, InvalidTransferEncoding,
                         testing::ValuesIn(kInvalidTransferEncodings));

TEST_P(InvalidTransferEncoding, FailsDeserialization) {
  std::string input(GetParam().input);

  std::vector<uint8_t> input_bytes;
  ASSERT_TRUE(DecodeHex(&input_bytes, input));

  bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL> ssl(
      SSL_from_bytes(input_bytes.data(), input_bytes.size(), server_ctx.get()));
  ASSERT_FALSE(ssl);
}

BSSL_NAMESPACE_END
