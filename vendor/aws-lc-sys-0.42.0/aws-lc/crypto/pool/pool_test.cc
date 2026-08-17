// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <gtest/gtest.h>

#include <openssl/pool.h>

#include "internal.h"
#include "../test/test_util.h"

#if defined(OPENSSL_THREADS)
#include <chrono>
#include <thread>
#endif


TEST(PoolTest, Unpooled) {
  static const uint8_t kData[4] = {1, 2, 3, 4};
  bssl::UniquePtr<CRYPTO_BUFFER> buf(
      CRYPTO_BUFFER_new(kData, sizeof(kData), nullptr));
  ASSERT_TRUE(buf);

  EXPECT_EQ(Bytes(kData),
            Bytes(CRYPTO_BUFFER_data(buf.get()), CRYPTO_BUFFER_len(buf.get())));

  // Test that reference-counting works properly.
  bssl::UniquePtr<CRYPTO_BUFFER> buf2 = bssl::UpRef(buf);

  bssl::UniquePtr<CRYPTO_BUFFER> buf_static(
    CRYPTO_BUFFER_new_from_static_data_unsafe(kData, sizeof(kData), nullptr));
  ASSERT_TRUE(buf_static);
  EXPECT_EQ(kData, CRYPTO_BUFFER_data(buf_static.get()));
  EXPECT_EQ(sizeof(kData), CRYPTO_BUFFER_len(buf_static.get()));

  // Test that reference-counting works properly.
  bssl::UniquePtr<CRYPTO_BUFFER> buf_static2 = bssl::UpRef(buf_static);
}

TEST(PoolTest, Empty) {
  bssl::UniquePtr<CRYPTO_BUFFER> buf(CRYPTO_BUFFER_new(nullptr, 0, nullptr));
  ASSERT_TRUE(buf);

  EXPECT_EQ(Bytes(""),
            Bytes(CRYPTO_BUFFER_data(buf.get()), CRYPTO_BUFFER_len(buf.get())));

  bssl::UniquePtr<CRYPTO_BUFFER> buf_static(
      CRYPTO_BUFFER_new_from_static_data_unsafe(nullptr, 0, nullptr));
  ASSERT_TRUE(buf_static);

  EXPECT_EQ(nullptr, CRYPTO_BUFFER_data(buf_static.get()));
  EXPECT_EQ(0u, CRYPTO_BUFFER_len(buf_static.get()));
}

TEST(PoolTest, Pooled) {
  bssl::UniquePtr<CRYPTO_BUFFER_POOL> pool(CRYPTO_BUFFER_POOL_new());
  ASSERT_TRUE(pool);

  static const uint8_t kData1[4] = {1, 2, 3, 4};
  bssl::UniquePtr<CRYPTO_BUFFER> buf(
      CRYPTO_BUFFER_new(kData1, sizeof(kData1), pool.get()));
  ASSERT_TRUE(buf);
  EXPECT_EQ(Bytes(kData1),
            Bytes(CRYPTO_BUFFER_data(buf.get()), CRYPTO_BUFFER_len(buf.get())));

  bssl::UniquePtr<CRYPTO_BUFFER> buf2(
      CRYPTO_BUFFER_new(kData1, sizeof(kData1), pool.get()));
  ASSERT_TRUE(buf2);
  EXPECT_EQ(Bytes(kData1), Bytes(CRYPTO_BUFFER_data(buf2.get()),
                                 CRYPTO_BUFFER_len(buf2.get())));

  EXPECT_EQ(buf.get(), buf2.get()) << "CRYPTO_BUFFER_POOL did not dedup data.";

  // Different inputs do not get deduped.
  static const uint8_t kData2[4] = {5, 6, 7, 8};
  bssl::UniquePtr<CRYPTO_BUFFER> buf3(
      CRYPTO_BUFFER_new(kData2, sizeof(kData2), pool.get()));
  ASSERT_TRUE(buf3);
  EXPECT_EQ(Bytes(kData2), Bytes(CRYPTO_BUFFER_data(buf3.get()),
                                 CRYPTO_BUFFER_len(buf3.get())));
  EXPECT_NE(buf.get(), buf3.get());

  // When the last refcount on |buf3| is dropped, it is removed from the pool.
  buf3 = nullptr;
  EXPECT_EQ(1u, lh_CRYPTO_BUFFER_num_items(pool->bufs));

  // Static buffers participate in pooling.
  buf3.reset(CRYPTO_BUFFER_new_from_static_data_unsafe(kData2, sizeof(kData2),
                                                       pool.get()));
  ASSERT_TRUE(buf3);
  EXPECT_EQ(kData2, CRYPTO_BUFFER_data(buf3.get()));
  EXPECT_EQ(sizeof(kData2), CRYPTO_BUFFER_len(buf3.get()));
  EXPECT_NE(buf.get(), buf3.get());

  bssl::UniquePtr<CRYPTO_BUFFER> buf4(
      CRYPTO_BUFFER_new(kData2, sizeof(kData2), pool.get()));
  EXPECT_EQ(buf4.get(), buf3.get());

  bssl::UniquePtr<CRYPTO_BUFFER> buf5(CRYPTO_BUFFER_new_from_static_data_unsafe(
      kData2, sizeof(kData2), pool.get()));
  EXPECT_EQ(buf5.get(), buf3.get());

  // When creating a static buffer, if there is already a non-static buffer, it
  // replaces the old buffer.
  bssl::UniquePtr<CRYPTO_BUFFER> buf6(CRYPTO_BUFFER_new_from_static_data_unsafe(
      kData1, sizeof(kData1), pool.get()));
  ASSERT_TRUE(buf6);
  EXPECT_EQ(kData1, CRYPTO_BUFFER_data(buf6.get()));
  EXPECT_EQ(sizeof(kData1), CRYPTO_BUFFER_len(buf6.get()));
  EXPECT_NE(buf.get(), buf6.get());

  // Subsequent lookups of |kData1| should return |buf6|.
  bssl::UniquePtr<CRYPTO_BUFFER> buf7(
      CRYPTO_BUFFER_new(kData1, sizeof(kData1), pool.get()));
  EXPECT_EQ(buf7.get(), buf6.get());
}

#if defined(OPENSSL_THREADS)
TEST(PoolTest, Threads) {
  bssl::UniquePtr<CRYPTO_BUFFER_POOL> pool(CRYPTO_BUFFER_POOL_new());
  ASSERT_TRUE(pool);

  // Race threads making pooled |CRYPTO_BUFFER|s.
  static const uint8_t kData[4] = {1, 2, 3, 4};
  static const uint8_t kData2[3] = {4, 5, 6};
  bssl::UniquePtr<CRYPTO_BUFFER> buf, buf2, buf3;
  {
    std::thread thread([&] {
      buf.reset(CRYPTO_BUFFER_new(kData, sizeof(kData), pool.get()));
    });
    std::thread thread2([&] {
      buf2.reset(CRYPTO_BUFFER_new(kData, sizeof(kData), pool.get()));
    });
    buf3.reset(CRYPTO_BUFFER_new(kData2, sizeof(kData2), pool.get()));
    thread.join();
    thread2.join();
  }

  ASSERT_TRUE(buf);
  ASSERT_TRUE(buf2);
  ASSERT_TRUE(buf3);
  EXPECT_EQ(buf.get(), buf2.get()) << "CRYPTO_BUFFER_POOL did not dedup data.";
  EXPECT_NE(buf.get(), buf3.get())
      << "CRYPTO_BUFFER_POOL incorrectly deduped data.";
  EXPECT_EQ(Bytes(kData),
            Bytes(CRYPTO_BUFFER_data(buf.get()), CRYPTO_BUFFER_len(buf.get())));
  EXPECT_EQ(Bytes(kData2), Bytes(CRYPTO_BUFFER_data(buf3.get()),
                                 CRYPTO_BUFFER_len(buf3.get())));

  // Reference-counting of |CRYPTO_BUFFER| interacts with pooling. Race an
  // increment and free.
  {
    bssl::UniquePtr<CRYPTO_BUFFER> buf_ref;
    std::thread thread([&] { buf_ref = bssl::UpRef(buf); });
    buf2.reset();
    thread.join();
  }

  // |buf|'s data is still valid.
  EXPECT_EQ(Bytes(kData), Bytes(CRYPTO_BUFFER_data(buf.get()),
                                CRYPTO_BUFFER_len(buf.get())));

  // Race a thread re-creating the |CRYPTO_BUFFER| with another thread freeing
  // it. Do this twice with sleeps so ThreadSanitizer can observe two different
  // interleavings. Ideally we would run this test under a tool that could
  // search all interleavings.
  {
    std::thread thread([&] {
      std::this_thread::sleep_for(std::chrono::milliseconds(1));
      buf.reset();
    });
    buf2.reset(CRYPTO_BUFFER_new(kData, sizeof(kData), pool.get()));
    thread.join();

    ASSERT_TRUE(buf2);
    EXPECT_EQ(Bytes(kData), Bytes(CRYPTO_BUFFER_data(buf2.get()),
                                  CRYPTO_BUFFER_len(buf2.get())));
    buf = std::move(buf2);
  }

  {
    std::thread thread([&] { buf.reset(); });
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
    buf2.reset(CRYPTO_BUFFER_new(kData, sizeof(kData), pool.get()));
    thread.join();

    ASSERT_TRUE(buf2);
    EXPECT_EQ(Bytes(kData), Bytes(CRYPTO_BUFFER_data(buf2.get()),
                                  CRYPTO_BUFFER_len(buf2.get())));
    buf = std::move(buf2);
  }

  // Finally, race the frees.
  {
    buf2 = bssl::UpRef(buf);
    std::thread thread([&] { buf.reset(); });
    std::thread thread2([&] { buf3.reset(); });
    buf2.reset();
    thread.join();
    thread2.join();
  }
}
#endif
