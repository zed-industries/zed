// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/lhash.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <memory>
#include <map>
#include <string>
#include <utility>
#include <vector>

#include <openssl/mem.h>

#include <gtest/gtest.h>

#include "internal.h"


DEFINE_LHASH_OF(char)

static std::unique_ptr<char[]> RandString(void) {
  unsigned len = 1 + (rand() % 3); // NOLINT(clang-analyzer-security.insecureAPI.rand)
  std::unique_ptr<char[]> ret(new char[len + 1]);

  for (unsigned i = 0; i < len; i++) {
    ret[i] = '0' + (rand() & 7); // NOLINT(clang-analyzer-security.insecureAPI.rand)
  }
  ret[len] = 0;

  return ret;
}

struct FreeLHASH_OF_char {
  void operator()(LHASH_OF(char) *lh) { lh_char_free(lh); }
};

static const char *Lookup(
    std::map<std::string, std::unique_ptr<char[]>> *dummy_lh, const char *key) {
  // Using operator[] implicitly inserts into the map.
  auto iter = dummy_lh->find(key);
  if (iter == dummy_lh->end()) {
    return nullptr;
  }
  return iter->second.get();
}

TEST(LHashTest, Basic) {
  std::unique_ptr<LHASH_OF(char), FreeLHASH_OF_char> lh(
      lh_char_new(OPENSSL_strhash, strcmp));
  ASSERT_TRUE(lh);

  // lh is expected to store a canonical instance of each string. dummy_lh
  // mirrors what it stores for comparison. It also manages ownership of the
  // pointers.
  std::map<std::string, std::unique_ptr<char[]>> dummy_lh;

  for (unsigned i = 0; i < 100000; i++) {
    EXPECT_EQ(dummy_lh.size(), lh_char_num_items(lh.get()));

    // Check the entire contents and test |lh_*_doall_arg|. This takes O(N)
    // time, so only do it every few iterations.
    //
    // TODO(davidben): |lh_*_doall_arg| also supports modifying the hash in the
    // callback. Test this.
    if (i % 1000 == 0) {
      using ValueList = std::vector<const char *>;
      ValueList expected, actual;
      for (const auto &pair : dummy_lh) {
        expected.push_back(pair.second.get());
      }
      std::sort(expected.begin(), expected.end());

      lh_char_doall_arg(lh.get(),
                        [](char *ptr, void *arg) {
                          ValueList *out = reinterpret_cast<ValueList *>(arg);
                          out->push_back(ptr);
                        },
                        &actual);
      std::sort(actual.begin(), actual.end());
      EXPECT_EQ(expected, actual);
    }

    enum Action {
      kRetrieve = 0,
      kInsert,
      kDelete,
    };

    Action action = static_cast<Action>(rand() % 3); // NOLINT(clang-analyzer-security.insecureAPI.rand)
    switch (action) {
      case kRetrieve: {
        std::unique_ptr<char[]> key = RandString();
        char *value = lh_char_retrieve(lh.get(), key.get());
        EXPECT_EQ(Lookup(&dummy_lh, key.get()), value);

        // Do the same lookup with |lh_char_retrieve_key|.
        value = lh_char_retrieve_key(
            lh.get(), &key, OPENSSL_strhash(key.get()),
            [](const void *key_ptr, const char *data) -> int {
              const char *key_data =
                  reinterpret_cast<const std::unique_ptr<char[]> *>(key_ptr)
                      ->get();
              return strcmp(key_data, data);
            });
        EXPECT_EQ(Lookup(&dummy_lh, key.get()), value);
        break;
      }

      case kInsert: {
        std::unique_ptr<char[]> key = RandString();
        char *previous;
        ASSERT_TRUE(lh_char_insert(lh.get(), &previous, key.get()));
        EXPECT_EQ(Lookup(&dummy_lh, key.get()), previous);
        dummy_lh[key.get()] = std::move(key);
        break;
      }

      case kDelete: {
        std::unique_ptr<char[]> key = RandString();
        char *value = lh_char_delete(lh.get(), key.get());
        EXPECT_EQ(Lookup(&dummy_lh, key.get()), value);
        dummy_lh.erase(key.get());
        break;
      }
    }
  }
}
