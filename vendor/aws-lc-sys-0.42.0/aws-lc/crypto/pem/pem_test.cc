// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/pem.h>

#include <gtest/gtest.h>

#include <openssl/asn1.h>
#include <openssl/bio.h>
#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/rand.h>
#include <openssl/rsa.h>

#include "../test/test_util.h"

const char* SECRET = "test";

static int pem_password_callback(char *buf, int size, int rwflag, void *userdata) {
  char* data = (char *)userdata;
  if(size <= 0) {
    return 0;
  }
  return (int)BUF_strlcpy(buf, data, size);
}

// Test that implausible ciphers, notably an IV-less RC4, aren't allowed in PEM.
// This is a regression test for https://github.com/openssl/openssl/issues/6347,
// though our fix differs from upstream.
TEST(PEMTest, NoRC4) {
  static const char kPEM[] =
      "-----BEGIN RSA PUBLIC KEY-----\n"
      "Proc-Type: 4,ENCRYPTED\n"
      "DEK-Info: RC4 -\n"
      "extra-info\n"
      "router-signature\n"
      "\n"
      "Z1w=\n"
      "-----END RSA PUBLIC KEY-----\n";
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(kPEM, sizeof(kPEM) - 1));
  ASSERT_TRUE(bio);
  bssl::UniquePtr<RSA> rsa(PEM_read_bio_RSAPublicKey(
      bio.get(), nullptr, nullptr, const_cast<char *>("password")));
  EXPECT_FALSE(rsa);
  EXPECT_TRUE(
      ErrorEquals(ERR_get_error(), ERR_LIB_PEM, PEM_R_UNSUPPORTED_ENCRYPTION));
}

static void* d2i_ASN1_INTEGER_void(void ** out, const unsigned char **inp, long len) {
  return d2i_ASN1_INTEGER((ASN1_INTEGER **)out, inp, len);
}

static int i2d_ASN1_INTEGER_void(const void *a, unsigned char **out) {
  return i2d_ASN1_INTEGER((ASN1_INTEGER *)a, out);
}

TEST(PEMTest, WriteReadASN1IntegerPem) {
#if defined(OPENSSL_ANDROID)
  // On Android, when running from an APK, |tmpfile| does not work. See
  // b/36991167#comment8.
  GTEST_SKIP();
#endif
  // Numbers for testing
  std::vector<long> nums = {
      0x00000001L,
      0x00000100L,
      0x00010000L,
      0x01000000L,
      -2L};

  for(long original_value: nums) {
    // Create an ASN1_INTEGER with value
    bssl::UniquePtr<ASN1_INTEGER> asn1_int(ASN1_INTEGER_new());
    ASSERT_TRUE(asn1_int);
    ASSERT_TRUE(ASN1_INTEGER_set(asn1_int.get(), original_value));

    // Create buffer for writing
    TempFILE pem_file = createTempFILE();
    ASSERT_TRUE(pem_file);

    // Write the ASN1_INTEGER to a PEM-formatted string
    ASSERT_TRUE(PEM_ASN1_write(i2d_ASN1_INTEGER_void, "ASN1 INTEGER",
                               pem_file.get(), asn1_int.get(), nullptr, nullptr,
                               0, nullptr, nullptr));

    rewind(pem_file.get());
    // Read the ASN1_INTEGER back from the PEM-formatted string
    bssl::UniquePtr<ASN1_INTEGER> read_integer((ASN1_INTEGER *)PEM_ASN1_read(
        d2i_ASN1_INTEGER_void, "ASN1 INTEGER", pem_file.get(), nullptr,
        nullptr, nullptr));
    ASSERT_TRUE(read_integer);

    // Check if the read ASN1_INTEGER has the same value as the original
    long read_value = ASN1_INTEGER_get(read_integer.get());
    ASSERT_EQ(original_value, read_value);
  }
}

const char* kPemRsaPrivateKey = "-----BEGIN ENCRYPTED PRIVATE KEY-----\n"
    "MIHsMFcGCSqGSIb3DQEFDTBKMCkGCSqGSIb3DQEFDDAcBAhz3vU103jx3wICCAAw\n"
    "DAYIKoZIhvcNAgkFADAdBglghkgBZQMEASoEEA6vMhRLgHZuHFa+eiecYCgEgZDB\n"
    "E8EOzjGQuu4D0TVAjOa3Peb9/MzQz3t09m5pvNBFKrEl96gefpZdni5qQk34ukj9\n"
    "/kryXymP72m4Ch7vbmew08x5x9L69BpfsQLF1yyvAJVtEZ0a1Zqcn5veuoH2WtJT\n"
    "ZTrZtc5Eb+tAjMVzRXPZ80cUwCbbpb9KHUX8spwtG10I1VxJ18X31FVpGORdr0A=\n"
    "-----END ENCRYPTED PRIVATE KEY-----";


TEST(PEMTest, ReadPrivateKeyPem) {
  bssl::UniquePtr<BIO> read_bio(BIO_new_mem_buf(kPemRsaPrivateKey, BUF_strnlen(kPemRsaPrivateKey, 2048)) );
  ASSERT_TRUE(read_bio);
  bssl::UniquePtr<EC_KEY> ec_key(PEM_read_bio_ECPrivateKey(read_bio.get(), nullptr, pem_password_callback, (void*)SECRET));
  ASSERT_TRUE(ec_key);
  const EC_GROUP* p256 = EC_GROUP_new_by_curve_name(NID_X9_62_prime256v1);
  ASSERT_EQ(p256, EC_KEY_get0_group(ec_key.get()));
}


TEST(PEMTest, WriteReadRSAPem) {
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);
  bssl::UniquePtr<BIGNUM> bn(BN_new());
  ASSERT_TRUE(bn);
  BN_set_u64(bn.get(), RSA_F4);
#if defined(BORINGSSL_FIPS)
  ASSERT_TRUE(RSA_generate_key_fips(rsa.get(), 2048, nullptr));
#else
  ASSERT_TRUE(RSA_generate_key_ex(rsa.get(), 2048, bn.get(), nullptr));
#endif

  bssl::UniquePtr<BIO> write_bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(write_bio);
  const EVP_CIPHER* cipher = EVP_get_cipherbynid(NID_aes_256_cbc);
  ASSERT_TRUE(cipher);
  EXPECT_FALSE(PEM_write_bio_RSAPrivateKey(write_bio.get(), rsa.get(), cipher, (unsigned char*)SECRET, -1, nullptr, nullptr));
  ASSERT_TRUE(PEM_write_bio_RSAPrivateKey(write_bio.get(), rsa.get(), cipher, (unsigned char*)SECRET, (int)BUF_strnlen(SECRET, 256), nullptr, nullptr));

  const uint8_t* content;
  size_t content_len;
  BIO_mem_contents(write_bio.get(), &content, &content_len);

  bssl::UniquePtr<BIO> read_bio(BIO_new_mem_buf(content, content_len) );
  ASSERT_TRUE(read_bio);
  bssl::UniquePtr<RSA> rsa_read(PEM_read_bio_RSAPrivateKey(read_bio.get(), nullptr, pem_password_callback, (void*)SECRET));
  ASSERT_TRUE(rsa_read);
  ASSERT_EQ(0, BN_cmp(RSA_get0_n(rsa.get()), RSA_get0_n(rsa_read.get())));
}

TEST(PEMTest, WriteReadECPem) {
  bssl::UniquePtr<EC_KEY> ec_key(EC_KEY_new());
  ASSERT_TRUE(ec_key);
  bssl::UniquePtr<EC_GROUP> ec_group(EC_GROUP_new_by_curve_name(NID_X9_62_prime256v1));
  ASSERT_TRUE(ec_group);
  ASSERT_TRUE(EC_KEY_set_group(ec_key.get(), ec_group.get()));

#if defined(BORINGSSL_FIPS)
  ASSERT_TRUE(EC_KEY_generate_key_fips(ec_key.get()));
#else
  ASSERT_TRUE(EC_KEY_generate_key(ec_key.get()));
#endif

  bssl::UniquePtr<BIO> write_bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(write_bio);
  const EVP_CIPHER* cipher = EVP_get_cipherbynid(NID_aes_256_cbc);
  ASSERT_TRUE(cipher);
  ASSERT_TRUE(PEM_write_bio_ECPrivateKey(write_bio.get(), ec_key.get(), cipher, nullptr, 0, pem_password_callback, (void*)SECRET));

  const uint8_t* content;
  size_t content_len;
  BIO_mem_contents(write_bio.get(), &content, &content_len);

  bssl::UniquePtr<BIO> read_bio(BIO_new_mem_buf(content, content_len) );
  ASSERT_TRUE(read_bio);
  bssl::UniquePtr<EC_KEY> ec_key_read(PEM_read_bio_ECPrivateKey(read_bio.get(), nullptr, pem_password_callback, (void*)"test"));
  ASSERT_TRUE(ec_key_read);
  const BIGNUM* orig_priv_key = EC_KEY_get0_private_key(ec_key.get());
  const BIGNUM* read_priv_key = EC_KEY_get0_private_key(ec_key_read.get());
  ASSERT_EQ(0, BN_cmp(orig_priv_key, read_priv_key));
}

const char *kPemECPARAMETERS =
    "-----BEGIN EC PARAMETERS-----\n"
    "BgUrgQQAIw==\n"
    "-----END EC PARAMETERS-----\n";

const char *kPemExplictECPARAMETERS =
    "-----BEGIN EC PARAMETERS-----\n"
    "MIH3AgEBMCwGByqGSM49AQECIQD/////AAAAAQAAAAAAAAAAAAAAAP//////////"
    "/////zBbBCD/////AAAAAQAAAAAAAAAAAAAAAP///////////////AQgWsY12Ko6"
    "k+ez671VdpiGvGUdBrDMU7D2O848PifSYEsDFQDEnTYIhucEk2pmeOETnSa3gZ9+"
    "kARBBGsX0fLhLEJH+Lzm5WOkQPJ3A32BLeszoPShOUXYmMKWT+NC4v4af5uO5+tK"
    "fA+eFivOM1drMV7Oy7ZAaDe/UfUCIQD/////AAAAAP//////////vOb6racXnoTz"
    "ucrC/GMlUQIBAQ==\n"
    "-----END EC PARAMETERS-----\n";

TEST(PEMTest, WriteReadECPKPem) {
  // Check named curve can be outputted to a PEM file.
  bssl::UniquePtr<EC_GROUP> group(EC_GROUP_new_by_curve_name(NID_secp521r1));
  ASSERT_TRUE(group);
  bssl::UniquePtr<BIO> write_bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(write_bio);
  ASSERT_TRUE(PEM_write_bio_ECPKParameters(write_bio.get(), group.get()));

  const uint8_t *content;
  size_t content_len;
  BIO_mem_contents(write_bio.get(), &content, &content_len);
  EXPECT_EQ(Bytes(content, content_len), Bytes(kPemECPARAMETERS));

  // Check named curve of a PEM file can be parsed.
  bssl::UniquePtr<BIO> read_bio(
      BIO_new_mem_buf(kPemECPARAMETERS, strlen(kPemECPARAMETERS)));
  bssl::UniquePtr<EC_GROUP> read_group(
      PEM_read_bio_ECPKParameters(read_bio.get(), nullptr, nullptr, nullptr));
  ASSERT_TRUE(read_group);
  ASSERT_EQ(EC_GROUP_cmp(EC_group_p521(), read_group.get(), nullptr), 0);


  // Make an arbitrary curve which is identical to P-256.
  static const uint8_t kP[] = {
      0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
  };
  static const uint8_t kA[] = {
      0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc,
  };
  static const uint8_t kB[] = {
      0x5a, 0xc6, 0x35, 0xd8, 0xaa, 0x3a, 0x93, 0xe7, 0xb3, 0xeb, 0xbd,
      0x55, 0x76, 0x98, 0x86, 0xbc, 0x65, 0x1d, 0x06, 0xb0, 0xcc, 0x53,
      0xb0, 0xf6, 0x3b, 0xce, 0x3c, 0x3e, 0x27, 0xd2, 0x60, 0x4b,
  };
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(kP, sizeof(kP), nullptr)),
      a(BN_bin2bn(kA, sizeof(kA), nullptr)),
      b(BN_bin2bn(kB, sizeof(kB), nullptr));
  ASSERT_TRUE(p && a && b);

  // Writing custom curves, even if the parameters are identical to a named
  // curve, will result in an error
  bssl::UniquePtr<EC_GROUP> custom_group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), nullptr));
  write_bio.reset(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(write_bio);
  EXPECT_FALSE(
      PEM_write_bio_ECPKParameters(write_bio.get(), custom_group.get()));

  // Check that explicitly-encoded versions of namedCurves can be correctly
  // parsed from a PEM file.
  read_bio.reset(BIO_new_mem_buf(
      kPemExplictECPARAMETERS, strlen(kPemExplictECPARAMETERS)));
  read_group.reset(
      PEM_read_bio_ECPKParameters(read_bio.get(), nullptr, nullptr, nullptr));
  ASSERT_TRUE(read_group);
  ASSERT_EQ(EC_GROUP_cmp(EC_group_p256(), read_group.get(), nullptr), 0);
}

TEST(ParametersTest, PEMReadwrite) {
  // Test |PEM_read/write_bio_Parameters| with |EC_KEY|.
  bssl::UniquePtr<EC_KEY> ec_key(EC_KEY_new());
  ASSERT_TRUE(ec_key);
  bssl::UniquePtr<EC_GROUP> ec_group(EC_GROUP_new_by_curve_name(NID_secp384r1));
  ASSERT_TRUE(ec_group);
  ASSERT_TRUE(EC_KEY_set_group(ec_key.get(), ec_group.get()));
  ASSERT_TRUE(EC_KEY_generate_key(ec_key.get()));

  bssl::UniquePtr<BIO> write_bio(BIO_new(BIO_s_mem()));
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), ec_key.get()));
  EXPECT_TRUE(PEM_write_bio_Parameters(write_bio.get(), pkey.get()));

  const uint8_t *content;
  size_t content_len;
  BIO_mem_contents(write_bio.get(), &content, &content_len);

  bssl::UniquePtr<BIO> read_bio(BIO_new_mem_buf(content, content_len));
  ASSERT_TRUE(read_bio);
  bssl::UniquePtr<EVP_PKEY> pkey_read(
      PEM_read_bio_Parameters(read_bio.get(), nullptr));
  ASSERT_TRUE(pkey_read);

  EC_KEY *pkey_eckey = EVP_PKEY_get0_EC_KEY(pkey.get());
  ASSERT_TRUE(pkey_eckey);
  const EC_GROUP *orig_params = EC_KEY_get0_group(pkey_eckey);
  ASSERT_TRUE(orig_params);
  const EC_GROUP *read_params = EC_KEY_get0_group(pkey_eckey);
  ASSERT_TRUE(read_params);
  ASSERT_EQ(0, EC_GROUP_cmp(orig_params, read_params, nullptr));

  // Test |PEM_read/write_bio_Parameters| with |DH|.
  bssl::UniquePtr<BIGNUM> p(BN_get_rfc3526_prime_1536(nullptr));
  ASSERT_TRUE(p);
  bssl::UniquePtr<BIGNUM> g(BN_new());
  ASSERT_TRUE(g);
  ASSERT_TRUE(BN_set_u64(g.get(), 2));
  bssl::UniquePtr<DH> dh(DH_new());
  ASSERT_TRUE(dh);
  ASSERT_TRUE(DH_set0_pqg(dh.get(), p.release(), nullptr, g.release()));
  write_bio.reset(BIO_new(BIO_s_mem()));
  pkey.reset(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_DH(pkey.get(), dh.get()));
  EXPECT_TRUE(PEM_write_bio_Parameters(write_bio.get(), pkey.get()));

  BIO_mem_contents(write_bio.get(), &content, &content_len);
  read_bio.reset(BIO_new_mem_buf(content, content_len));
  ASSERT_TRUE(read_bio);
  pkey_read.reset(PEM_read_bio_Parameters(read_bio.get(), nullptr));
  ASSERT_TRUE(pkey_read);

  DH *pkey_dh = EVP_PKEY_get0_DH(pkey.get());
  ASSERT_TRUE(pkey_dh);
  EXPECT_EQ(0, BN_cmp(DH_get0_p(pkey_dh), DH_get0_p(dh.get())));
  EXPECT_EQ(0, BN_cmp(DH_get0_g(pkey_dh), DH_get0_g(dh.get())));

  // Test |PEM_read/write_bio_Parameters| with |DSA|.
  bssl::UniquePtr<DSA> dsa(DSA_new());
  ASSERT_TRUE(dsa);
  uint8_t seed[20];
  ASSERT_TRUE(RAND_bytes(seed, sizeof(seed)));
  ASSERT_TRUE(DSA_generate_parameters_ex(dsa.get(), 512, seed, sizeof(seed),
                                         nullptr, nullptr, nullptr));
  ASSERT_TRUE(DSA_generate_key(dsa.get()));

  write_bio.reset(BIO_new(BIO_s_mem()));
  pkey.reset(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_DSA(pkey.get(), dsa.get()));
  EXPECT_TRUE(PEM_write_bio_Parameters(write_bio.get(), pkey.get()));

  BIO_mem_contents(write_bio.get(), &content, &content_len);
  read_bio.reset(BIO_new_mem_buf(content, content_len));
  ASSERT_TRUE(read_bio);
  pkey_read.reset(PEM_read_bio_Parameters(read_bio.get(), nullptr));
  ASSERT_TRUE(pkey_read);

  DSA *pkey_dsa = EVP_PKEY_get0_DSA(pkey.get());
  EXPECT_EQ(0, BN_cmp(DSA_get0_p(pkey_dsa), DSA_get0_p(dsa.get())));
  EXPECT_EQ(0, BN_cmp(DSA_get0_g(pkey_dsa), DSA_get0_g(dsa.get())));
}

const char *kRubyPemDHPARAMETERS =
    "-----BEGIN DH PARAMETERS-----\n"
    "MIIBCAKCAQEA7E6kBrYiyvmKAMzQ7i8WvwVk9Y/+f8S7sCTN712KkK3cqd1jhJDY"
    "JbrYeNV3kUIKhPxWHhObHKpD1R84UpL+s2b55+iMd6GmL7OYmNIT/FccKhTcveab"
    "VBmZT86BZKYyf45hUF9FOuUM9xPzuK3Vd8oJQvfYMCd7LPC0taAEljQLR4Edf8E6"
    "YoaOffgTf5qxiwkjnlVZQc3whgnEt9FpVMvQ9eknyeGB5KHfayAc3+hUAvI3/Cr3"
    "1bNveX5wInh5GDx1FGhKBZ+s1H+aedudCm7sCgRwv8lKWYGiHzObSma8A86KG+MD"
    "7Lo5JquQ3DlBodj3IDyPrxIv96lvRPFtAwIBAg==\n"
    "-----END DH PARAMETERS-----\n";

TEST(ParametersTest, RubyDHFile) {
  bssl::UniquePtr<BIO> read_bio(
      BIO_new_mem_buf(kRubyPemDHPARAMETERS, strlen(kRubyPemDHPARAMETERS)));
  ASSERT_TRUE(read_bio);
  bssl::UniquePtr<EVP_PKEY> pkey_read(
      PEM_read_bio_Parameters(read_bio.get(), nullptr));
  ASSERT_TRUE(pkey_read);
  bssl::UniquePtr<DH> dh(EVP_PKEY_get1_DH(pkey_read.get()));
  EXPECT_TRUE(dh);
  EXPECT_EQ(DH_num_bits(dh.get()), 2048u);
}

TEST(PEMTest, WriteReadTraditionalPem) {
  // Test |PEM_write_bio_PrivateKey_traditional| with |EC_KEY|.
  bssl::UniquePtr<EC_KEY> ec_key(EC_KEY_new());
  ASSERT_TRUE(ec_key);
  bssl::UniquePtr<EC_GROUP> ec_group(EC_GROUP_new_by_curve_name(NID_secp256k1));
  ASSERT_TRUE(ec_group);
  ASSERT_TRUE(EC_KEY_set_group(ec_key.get(), ec_group.get()));
  ASSERT_TRUE(EC_KEY_generate_key(ec_key.get()));

  bssl::UniquePtr<BIO> write_bio(BIO_new(BIO_s_mem()));
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), ec_key.get()));
  EXPECT_TRUE(PEM_write_bio_PrivateKey_traditional(
      write_bio.get(), pkey.get(), nullptr, nullptr, 0, nullptr, nullptr));

  const uint8_t *content;
  size_t content_len;
  BIO_mem_contents(write_bio.get(), &content, &content_len);

  bssl::UniquePtr<BIO> read_bio(BIO_new_mem_buf(content, content_len));
  ASSERT_TRUE(read_bio);
  bssl::UniquePtr<EVP_PKEY> pkey_read(
      PEM_read_bio_PrivateKey(read_bio.get(), nullptr, nullptr, nullptr));
  ASSERT_TRUE(pkey_read);

  EC_KEY *pkey_eckey = EVP_PKEY_get0_EC_KEY(pkey.get());
  ASSERT_TRUE(pkey_eckey);
  const BIGNUM *orig_priv_key = EC_KEY_get0_private_key(ec_key.get());
  const BIGNUM *read_priv_key = EC_KEY_get0_private_key(pkey_eckey);
  ASSERT_EQ(0, BN_cmp(orig_priv_key, read_priv_key));

  // Test |PEM_write_bio_PrivateKey_traditional| with |RSA|.
  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);
  ASSERT_TRUE(RSA_generate_key_ex(rsa.get(), 1024, e.get(), nullptr));

  write_bio.reset(BIO_new(BIO_s_mem()));
  pkey.reset(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_RSA(pkey.get(), rsa.get()));
  EXPECT_TRUE(PEM_write_bio_PrivateKey_traditional(
      write_bio.get(), pkey.get(), nullptr, nullptr, 0, nullptr, nullptr));

  BIO_mem_contents(write_bio.get(), &content, &content_len);
  read_bio.reset(BIO_new_mem_buf(content, content_len));
  ASSERT_TRUE(read_bio);
  pkey_read.reset(
      PEM_read_bio_PrivateKey(read_bio.get(), nullptr, nullptr, nullptr));
  ASSERT_TRUE(pkey_read);

  RSA *pkey_rsa = EVP_PKEY_get0_RSA(pkey.get());
  ASSERT_TRUE(pkey_rsa);
  EXPECT_EQ(0, BN_cmp(RSA_get0_d(pkey_rsa), RSA_get0_d(rsa.get())));
  EXPECT_EQ(0, BN_cmp(RSA_get0_d(pkey_rsa), RSA_get0_d(rsa.get())));

  // Test |PEM_write_bio_PrivateKey_traditional| with |DSA|.
  bssl::UniquePtr<DSA> dsa(DSA_new());
  ASSERT_TRUE(dsa);
  uint8_t seed[20];
  ASSERT_TRUE(RAND_bytes(seed, sizeof(seed)));
  ASSERT_TRUE(DSA_generate_parameters_ex(dsa.get(), 512, seed, sizeof(seed),
                                         nullptr, nullptr, nullptr));
  ASSERT_TRUE(DSA_generate_key(dsa.get()));

  write_bio.reset(BIO_new(BIO_s_mem()));
  pkey.reset(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_DSA(pkey.get(), dsa.get()));
  EXPECT_TRUE(PEM_write_bio_PrivateKey_traditional(
      write_bio.get(), pkey.get(), nullptr, nullptr, 0, nullptr, nullptr));

  BIO_mem_contents(write_bio.get(), &content, &content_len);
  read_bio.reset(BIO_new_mem_buf(content, content_len));
  ASSERT_TRUE(read_bio);
  pkey_read.reset(
      PEM_read_bio_PrivateKey(read_bio.get(), nullptr, nullptr, nullptr));
  ASSERT_TRUE(pkey_read);

  DSA *pkey_dsa = EVP_PKEY_get0_DSA(pkey.get());
  EXPECT_EQ(0, BN_cmp(DSA_get0_priv_key(pkey_dsa), DSA_get0_priv_key(dsa.get())));

  // Test |PEM_write_bio_PrivateKey_traditional| with |DH|. This should fail,
  // since it's not supported by the API.
  bssl::UniquePtr<BIGNUM> p(BN_get_rfc3526_prime_1536(nullptr));
  ASSERT_TRUE(p);
  bssl::UniquePtr<BIGNUM> g(BN_new());
  ASSERT_TRUE(g);
  ASSERT_TRUE(BN_set_u64(g.get(), 2));
  bssl::UniquePtr<DH> dh(DH_new());
  ASSERT_TRUE(dh);
  ASSERT_TRUE(DH_set0_pqg(dh.get(), p.release(), nullptr, g.release()));
  write_bio.reset(BIO_new(BIO_s_mem()));
  pkey.reset(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_DH(pkey.get(), dh.get()));
  EXPECT_FALSE(PEM_write_bio_PrivateKey_traditional(
      write_bio.get(), pkey.get(), nullptr, nullptr, 0, nullptr, nullptr));
}

TEST(PEMTest, WriteReadECPemEmptyPassword) {
  bssl::UniquePtr<EC_KEY> ec_key(EC_KEY_new());
  ASSERT_TRUE(ec_key);
  bssl::UniquePtr<EC_GROUP> ec_group(EC_GROUP_new_by_curve_name(NID_X9_62_prime256v1));
  ASSERT_TRUE(ec_group);
  ASSERT_TRUE(EC_KEY_set_group(ec_key.get(), ec_group.get()));

#if defined(BORINGSSL_FIPS)
  ASSERT_TRUE(EC_KEY_generate_key_fips(ec_key.get()));
#else
  ASSERT_TRUE(EC_KEY_generate_key(ec_key.get()));
#endif

  bssl::UniquePtr<BIO> write_bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(write_bio);
  const EVP_CIPHER* cipher = EVP_get_cipherbynid(NID_aes_256_cbc);
  ASSERT_TRUE(cipher);
  ASSERT_TRUE(PEM_write_bio_ECPrivateKey(write_bio.get(), ec_key.get(), cipher, nullptr, 0, pem_password_callback, (void*)""));

  const uint8_t* content = nullptr;
  size_t content_len = 0;
  BIO_mem_contents(write_bio.get(), &content, &content_len);

  bssl::UniquePtr<BIO> read_bio(BIO_new_mem_buf(content, content_len) );
  ASSERT_TRUE(read_bio);
  bssl::UniquePtr<EC_KEY> ec_key_read(PEM_read_bio_ECPrivateKey(read_bio.get(), nullptr, pem_password_callback, (void*)""));
  ASSERT_TRUE(ec_key_read);
  const BIGNUM* orig_priv_key = EC_KEY_get0_private_key(ec_key.get());
  const BIGNUM* read_priv_key = EC_KEY_get0_private_key(ec_key_read.get());
  ASSERT_EQ(0, BN_cmp(orig_priv_key, read_priv_key));
}

TEST(PEMTest, WriteReadPKCS8DerEmptyPassword) {
  bssl::UniquePtr<EC_KEY> ec_key(EC_KEY_new_by_curve_name(NID_X9_62_prime256v1));
  ASSERT_TRUE(ec_key);
  ASSERT_TRUE(EC_KEY_generate_key(ec_key.get()));
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  ASSERT_TRUE(pkey);
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), ec_key.get()));

  bssl::UniquePtr<BIO> write_bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(write_bio);
  ASSERT_TRUE(i2d_PKCS8PrivateKey_bio(write_bio.get(), pkey.get(), EVP_aes_256_cbc(), nullptr, 0, pem_password_callback, (void*)""));

  const uint8_t* content = nullptr;
  size_t content_len = 0;
  BIO_mem_contents(write_bio.get(), &content, &content_len);

  bssl::UniquePtr<BIO> read_bio(BIO_new_mem_buf(content, content_len) );
  ASSERT_TRUE(read_bio);
  bssl::UniquePtr<EVP_PKEY> pkey_read(d2i_PKCS8PrivateKey_bio(read_bio.get(), nullptr, pem_password_callback, (void*)""));
  ASSERT_TRUE(pkey_read);

  const EC_KEY* read_ec = EVP_PKEY_get0_EC_KEY(pkey_read.get());
  ASSERT_TRUE(read_ec);
  ASSERT_EQ(0, BN_cmp(EC_KEY_get0_private_key(ec_key.get()), EC_KEY_get0_private_key(read_ec)));
}
