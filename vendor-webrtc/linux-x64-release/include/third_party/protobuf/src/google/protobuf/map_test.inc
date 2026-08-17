// -*- c++ -*-
// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// A hack to include windows.h first, which ensures the GetMessage macro can
// be undefined when we include <google/protobuf/stubs/common.h>
#if defined(_MSC_VER)
#define _WINSOCKAPI_  // to avoid re-definition in WinSock2.h
#define NOMINMAX      // to avoid defining min/max macros
#include <windows.h>
#endif  // _WIN32

#include <algorithm>
#include <cstddef>
#include <memory>
#include <random>
#include <sstream>
#include <vector>

#include "google/protobuf/testing/file.h"
#include "google/protobuf/descriptor.pb.h"
#include <gmock/gmock.h>
#include "google/protobuf/testing/googletest.h"
#include <gtest/gtest.h>
#include "absl/base/casts.h"
#include "absl/cleanup/cleanup.h"
#include "absl/container/btree_set.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/strings/substitute.h"
#include "absl/time/time.h"
#include "google/protobuf/arena_test_util.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor_database.h"
#include "google/protobuf/dynamic_message.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/io/tokenizer.h"
#include "google/protobuf/io/zero_copy_stream_impl.h"
#include "google/protobuf/map.h"
#include "google/protobuf/map_field_inl.h"
#include "google/protobuf/map_proto2_unittest.pb.h"
#include "google/protobuf/map_proto3_unittest.pb.h"
#include "google/protobuf/message.h"
#include "google/protobuf/port.h"
#include "google/protobuf/reflection.h"
#include "google/protobuf/reflection_ops.h"
#include "google/protobuf/test_textproto.h"
#include "google/protobuf/test_util2.h"
#include "google/protobuf/text_format.h"
#include "google/protobuf/util/message_differencer.h"
#include "google/protobuf/wire_format.h"


// Must be included last.
#include "google/protobuf/port_def.inc"

using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::Pair;

namespace google {
namespace protobuf {

using UNITTEST::ForeignMessage;
using UNITTEST::TestAllTypes;
using UNITTEST::TestMap;
using UNITTEST::TestRecursiveMapMessage;

namespace internal {

void MapTestForceDeterministic() {
  io::CodedOutputStream::SetDefaultSerializationDeterministic();
}

struct MapTestPeer {
  template <typename T>
  static typename T::KeyMapBase& GetKeyMapBase(T& value) {
    return value;
  }

  template <typename T>
  static bool InsertOrReplaceNode(T& map, typename T::key_type key,
                                  typename T::mapped_type value) {
    using Node = typename T::Node;
    auto* node = static_cast<Node*>(map.AllocNode(sizeof(Node)));
    ::new (static_cast<void*>(&node->kv)) typename T::value_type{key, value};
    return GetKeyMapBase(map).InsertOrReplaceNode(node);
  }

  template <typename T>
  static size_t NumBuckets(T& map) {
    return map.num_buckets_;
  }

  template <typename T>
  static size_t BucketNumber(T& map, typename T::key_type key) {
    return map.BucketNumber(key);
  }

  template <typename T>
  static void Resize(T& map, size_t num_buckets) {
    map.Resize(num_buckets);
  }

  static size_t CalculateHiCutoff(size_t num_buckets) {
    return Map<int, int>::CalculateHiCutoff(num_buckets);
  }

  static size_t CalculateCapacityForSize(size_t size) {
    return Map<int, int>::CalculateCapacityForSize(size);
  }

  template <typename Map>
  static auto GetTypeInfo() {
    return Map::GetTypeInfo();
  }

  static constexpr size_t kMinTableSize = UntypedMapBase::kMinTableSize;
};

namespace {

// Map API Test =====================================================

class MapImplTest : public ::testing::Test {
 protected:
  MapImplTest()
      : map_ptr_(new Map<int32_t, int32_t>()),
        map_(*map_ptr_),
        const_map_(*map_ptr_) {
    EXPECT_TRUE(map_.empty());
    EXPECT_EQ(0, map_.size());
  }

  void ExpectSingleElement(int32_t key, int32_t value) {
    EXPECT_FALSE(map_.empty());
    EXPECT_EQ(1, map_.size());
    ExpectElement(key, value);
  }

  void ExpectElements(const absl::flat_hash_map<int32_t, int32_t>& map) {
    EXPECT_FALSE(map_.empty());
    EXPECT_EQ(map.size(), map_.size());
    for (const auto& e : map) {
      ExpectElement(e.first, e.second);
    }
  }

  void ExpectElement(int32_t key, int32_t value) {
    // Test map size is correct.
    EXPECT_EQ(value, map_[key]);
    EXPECT_EQ(1, map_.count(key));
    EXPECT_TRUE(map_.contains(key));

    // Check mutable at and find work correctly.
    EXPECT_EQ(value, map_.at(key));
    Map<int32_t, int32_t>::iterator it = map_.find(key);

    // iterator dereferenceable
    EXPECT_EQ(key, (*it).first);
    EXPECT_EQ(value, (*it).second);
    EXPECT_EQ(key, it->first);
    EXPECT_EQ(value, it->second);

    // iterator mutable
    ((*it).second) = value + 1;
    EXPECT_EQ(value + 1, map_[key]);
    ((*it).second) = value;
    EXPECT_EQ(value, map_[key]);

    it->second = value + 1;
    EXPECT_EQ(value + 1, map_[key]);
    it->second = value;
    EXPECT_EQ(value, map_[key]);

    // copy constructor
    Map<int32_t, int32_t>::iterator it_copy = it;
    EXPECT_EQ(key, it_copy->first);
    EXPECT_EQ(value, it_copy->second);

    // Immutable API ================================================

    // Check immutable at and find work correctly.
    EXPECT_EQ(value, const_map_.at(key));
    Map<int32_t, int32_t>::const_iterator const_it = const_map_.find(key);

    // iterator dereferenceable
    EXPECT_EQ(key, (*const_it).first);
    EXPECT_EQ(value, (*const_it).second);
    EXPECT_EQ(key, const_it->first);
    EXPECT_EQ(value, const_it->second);

    // copy constructor
    Map<int32_t, int32_t>::const_iterator const_it_copy = const_it;
    EXPECT_EQ(key, const_it_copy->first);
    EXPECT_EQ(value, const_it_copy->second);
  }

  std::unique_ptr<Map<int32_t, int32_t>> map_ptr_;
  Map<int32_t, int32_t>& map_;
  const Map<int32_t, int32_t>& const_map_;
};

TEST_F(MapImplTest, OperatorBracket) {
  int32_t key = 0;
  int32_t value1 = 100;
  int32_t value2 = 101;

  EXPECT_EQ(0, map_[key]);

  map_[key] = value1;
  ExpectSingleElement(key, value1);

  map_[key] = value2;
  ExpectSingleElement(key, value2);
}

}  // namespace
}  // namespace internal
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace internal {
namespace {

TEST_F(MapImplTest, OperatorBracketRValue) {
  Arena arena;
  for (Arena* arena_to_use : {&arena, static_cast<Arena*>(nullptr)}) {
    Map<std::string, int> map(arena_to_use);
    {
      std::string key(10000, 'a');
      const char* const data = key.data();
      map[key] = 0;
      EXPECT_EQ(data, key.data());
      EXPECT_NE(data, map.find(std::string(10000, 'a'))->first.data());
    }

    {
      std::string key(10000, 'b');
      const char* const data = key.data();
      map[std::move(key)] = 0;
      EXPECT_NE(data, key.data());  // NOLINT
      EXPECT_EQ(data, map.find(std::string(10000, 'b'))->first.data());
    }
  }
}

TEST_F(MapImplTest, OperatorBracketNonExist) {
  int32_t key = 0;
  int32_t default_value = 0;

  EXPECT_EQ(default_value, map_[key]);
  ExpectSingleElement(key, default_value);
}

TEST_F(MapImplTest, MutableAt) {
  int32_t key = 0;
  int32_t value1 = 100;
  int32_t value2 = 101;

  map_[key] = value1;
  ExpectSingleElement(key, value1);

  map_.at(key) = value2;
  ExpectSingleElement(key, value2);
}

#if GTEST_HAS_DEATH_TEST

TEST_F(MapImplTest, MutableAtNonExistDeathTest) {
  EXPECT_DEATH(map_.at(0), "");
}

TEST_F(MapImplTest, ImmutableAtNonExistDeathTest) {
  EXPECT_DEATH(const_map_.at(0), "");
}

TEST_F(MapImplTest, UsageErrors) {
  MapKey key;
  key.SetInt64Value(1);
  EXPECT_DEATH(key.GetUInt64Value(),
               "Protocol Buffer map usage error:\n"
               "MapKey::GetUInt64Value type does not match\n"
               "  Expected : uint64\n"
               "  Actual   : int64");

  MapValueRef value;
  EXPECT_DEATH(
      value.SetFloatValue(0.1),
      testing::AnyOf(
        testing::HasSubstr(
          "Protocol Buffer map usage error:\n"
          "MapValueRef::type MapValueRef is not initialized."),
        testing::HasSubstr(
          "Protocol Buffer map usage error:\n"
          "MapValueConstRef::type MapValueConstRef is not initialized.")));
}

#endif  // GTEST_HAS_DEATH_TEST

TEST_F(MapImplTest, MapKeyAssignment) {
  MapKey from, to;
  from.SetStringValue("abc");
  to = from;
  EXPECT_EQ("abc", to.GetStringValue());
}

TEST_F(MapImplTest, CountNonExist) { EXPECT_EQ(0, map_.count(0)); }

TEST_F(MapImplTest, ContainNotExist) { EXPECT_FALSE(map_.contains(0)); }

TEST_F(MapImplTest, ImmutableContainNotExist) {
  EXPECT_FALSE(const_map_.contains(0));
}

TEST_F(MapImplTest, MutableFindNonExist) {
  EXPECT_TRUE(map_.end() == map_.find(0));
}

TEST_F(MapImplTest, ImmutableFindNonExist) {
  EXPECT_TRUE(const_map_.end() == const_map_.find(0));
}

TEST_F(MapImplTest, ConstEnd) {
  EXPECT_TRUE(const_map_.end() == const_map_.cend());
}

TEST_F(MapImplTest, GetReferenceFromIterator) {
  for (int i = 0; i < 10; i++) {
    map_[i] = i;
  }

  for (Map<int32_t, int32_t>::const_iterator it = map_.cbegin();
       it != map_.cend();) {
    Map<int32_t, int32_t>::const_reference entry = *it++;
    EXPECT_EQ(entry.first, entry.second);
  }

  for (Map<int32_t, int32_t>::const_iterator it = const_map_.begin();
       it != const_map_.end();) {
    Map<int32_t, int32_t>::const_reference entry = *it++;
    EXPECT_EQ(entry.first, entry.second);
  }

  for (Map<int32_t, int32_t>::iterator it = map_.begin(); it != map_.end();) {
    Map<int32_t, int32_t>::reference entry = *it++;
    EXPECT_EQ(entry.first + 1, ++entry.second);
  }
}

TEST_F(MapImplTest, IteratorBasic) {
  map_[0] = 0;

  // Default constructible (per forward iterator requirements).
  Map<int, int>::const_iterator cit;
  Map<int, int>::iterator it;

  it = map_.begin();
  cit = it;  // Converts to const_iterator

  // Can compare between them.
  EXPECT_TRUE(it == cit);
  EXPECT_FALSE(cit != it);

  // Pre increment.
  EXPECT_FALSE(it == ++cit);

  // Post increment.
  EXPECT_FALSE(it++ == cit);
  EXPECT_TRUE(it == cit);
}

// Arbitrary odd integers for creating test data.
static int k0 = 812398771;
static int k1 = 1312938717;
static int k2 = 1321555333;

TEST_F(MapImplTest, CopyIteratorStressTest) {
  std::vector<Map<int32_t, int32_t>::iterator> v;
  const int kIters = 1e5;
  for (uint32_t i = 0; i < kIters; i++) {
    int32_t key = (3 + i * (5 + i * (-8 + i * (62 + i)))) & 0x77777777;
    map_[key] = i;
    v.push_back(map_.find(key));
  }
  for (auto it = v.begin(); it != v.end(); it++) {
    Map<int32_t, int32_t>::iterator i = *it;
    ASSERT_EQ(i->first, (*it)->first);
    ASSERT_EQ(i->second, (*it)->second);
  }
}

template <typename T, typename U>
static void TestValidityForAllKeysExcept(int key_to_avoid, const T& check_map,
                                         const U& map) {
  typedef typename U::value_type value_type;  // a key-value pair
  for (typename U::const_iterator it = map.begin(); it != map.end(); ++it) {
    const int key = it->first;
    if (key == key_to_avoid) continue;
    // All iterators relevant to this key, whether old (from check_map) or new,
    // must point to the same memory.  So, test pointer equality here.
    const value_type* check_val = &*check_map.find(key)->second;
    EXPECT_EQ(check_val, &*it);
    EXPECT_EQ(check_val, &*map.find(key));
  }
}

// EXPECT i0 and i1 to be the same.
template <typename Iter>
static void TestEqualIterators(Iter i0, Iter i1, Iter end) {
  EXPECT_EQ(i0 == end, i1 == end);
  if (i0 == end) return;
  EXPECT_EQ(&*i0, &*i1);
}

template <typename IteratorType>
static void TestOldVersusNewIterator(int skip, Map<int, int>* m) {
  const int initial_size = m->size();
  IteratorType it = m->begin();
  for (int i = 0; i < skip && it != m->end(); it++, i++) {
  }
  if (it == m->end()) return;
  const IteratorType old = it;
  ABSL_LOG(INFO) << "skip=" << skip << ", old->first=" << old->first;
  const int target_size =
      initial_size < 100 ? initial_size * 5 : initial_size * 5 / 4;
  for (int i = 0; m->size() <= target_size; i++) {
    (*m)[i] = 0;
  }
  // Iterator 'old' should still work just fine despite the growth of *m.
  const IteratorType after_growth = m->find(old->first);
  TestEqualIterators<IteratorType>(old, after_growth, m->end());

  // Now shrink the number of elements.  Do this with a mix of erases and
  // inserts to increase the chance that the hashtable will resize to a lower
  // number of buckets.  (But, in any case, the test is still useful.)
  for (int i = 0; i < 2 * (target_size - initial_size); i++) {
    if (i != old->first) {
      m->erase(i);
    }
    if (((i ^ m->begin()->first) & 15) == 0) {
      (*m)[i * 342] = i;
    }
  }
  // Now, the table has grown and shrunk; test again.
  TestEqualIterators<IteratorType>(old, m->find(old->first), m->end());
  TestEqualIterators<IteratorType>(old, after_growth, m->end());
}

// Create and test an n-element Map, with emphasis on iterator correctness.
static void StressTestIterators(int n) {
  ABSL_LOG(INFO) << "StressTestIterators " << n;
  ABSL_CHECK_GT(n, 0);
  // Create a random-looking map of size n.  Use non-negative integer keys.
  Map<int, int> m;
  uint32_t frog = 123987 + n;
  int last_key = 0;
  int counter = 0;
  while (m.size() < n) {
    frog *= static_cast<uint32_t>(k0);
    frog ^= frog >> 17;
    frog += counter++;
    last_key =
        static_cast<int>(frog) >= 0 ? static_cast<int>(frog) : last_key ^ 1;
    ABSL_DCHECK_GE(last_key, 0);
    m[last_key] = last_key ^ 1;
  }
  // Test it.
  ASSERT_EQ(n, m.size());
  // Create maps of pointers and iterators.
  // These should remain valid even if we modify m.
  absl::flat_hash_map<int, Map<int, int>::value_type*> mp(n);
  absl::flat_hash_map<int, Map<int, int>::iterator> mi(n);
  for (Map<int, int>::iterator it = m.begin(); it != m.end(); ++it) {
    mp[it->first] = &*it;
    mi[it->first] = it;
  }
  ASSERT_EQ(m.size(), mi.size());
  ASSERT_EQ(m.size(), mp.size());
  m.erase(last_key);
  ASSERT_EQ(n - 1, m.size());
  TestValidityForAllKeysExcept(last_key, mp, m);
  TestValidityForAllKeysExcept(last_key, mi, m);

  m[last_key] = 0;
  ASSERT_EQ(n, m.size());
  // Test old iterator vs new iterator, with table modification in between.
  TestOldVersusNewIterator<Map<int, int>::const_iterator>(n % 3, &m);
  TestOldVersusNewIterator<Map<int, int>::iterator>(n % (1 + (n / 40)), &m);
  // Finally, ensure erase(iterator) doesn't reorder anything, because that is
  // what its documentation says.
  m[last_key] = m[last_key ^ 999] = 0;
  std::vector<Map<int, int>::iterator> v;
  v.reserve(m.size());
  int position_of_last_key = 0;
  for (Map<int, int>::iterator it = m.begin(); it != m.end(); ++it) {
    if (it->first == last_key) {
      position_of_last_key = v.size();
    }
    v.push_back(it);
  }
  ASSERT_EQ(m.size(), v.size());
  const auto erase_result = m.erase(m.find(last_key));
  int index = 0;
  for (auto it = m.begin(); it != m.end(); ++it, ++index) {
    if (index == position_of_last_key) {
      EXPECT_EQ(&*erase_result, &*v[++index]);
    }
    ASSERT_EQ(&*it, &*v[index]);
  }
}

TEST_F(MapImplTest, IteratorInvalidation) {
  // Create a set of pseudo-random sizes to test.
#ifndef NDEBUG
  const int kMaxSizeToTest = 100 * 1000;
#else
  const int kMaxSizeToTest = 1000 * 1000;
#endif
  absl::btree_set<int> s;
  int n = kMaxSizeToTest;
  unsigned int frog = k1 + n;
  while (n > 1 && s.size() < 25) {
    s.insert(n);
    n = static_cast<int>(n * 100 / (101.0 + (frog & 63)));
    frog *= k2;
    frog ^= frog >> 17;
  }
  // Ensure we test a few small sizes.
  s.insert(1);
  s.insert(2);
  s.insert(3);
  // Now, the real work.
  for (int i : s) {
    StressTestIterators(i);
  }
}

// Test that erase() revalidates iterators.
TEST_F(MapImplTest, EraseRevalidates) {
  map_[3] = map_[13] = map_[20] = 0;
  const int initial_size = map_.size();
  EXPECT_EQ(3, initial_size);
  std::vector<Map<int, int>::iterator> v;
  for (Map<int, int>::iterator it = map_.begin(); it != map_.end(); ++it) {
    v.push_back(it);
  }
  EXPECT_EQ(initial_size, v.size());
  for (int i = 0; map_.size() <= initial_size * 20; i++) {
    map_[i] = 0;
  }
  const int larger_size = map_.size();
  // We've greatly increased the size of the map, so it is highly likely that
  // the following will corrupt m if erase() doesn't properly revalidate
  // iterators passed to it.  Finishing this routine without crashing indicates
  // success.
  for (int i = 0; i < v.size(); i++) {
    map_.erase(v[i]);
  }
  EXPECT_EQ(larger_size - v.size(), map_.size());
}

template <typename T>
bool IsConstHelper(T& /*t*/) {  // NOLINT. We want to catch non-const refs here.
  return false;
}
template <typename T>
bool IsConstHelper(const T& /*t*/) {
  return true;
}

TEST_F(MapImplTest, IteratorConstness) {
  map_[0] = 0;
  EXPECT_TRUE(IsConstHelper(*map_.cbegin()));
  EXPECT_TRUE(IsConstHelper(*const_map_.begin()));
  EXPECT_FALSE(IsConstHelper(*map_.begin()));
}

bool IsForwardIteratorHelper(std::forward_iterator_tag /*tag*/) { return true; }

TEST_F(MapImplTest, IteratorCategory) {
  EXPECT_TRUE(IsForwardIteratorHelper(
      std::iterator_traits<Map<int, int>::iterator>::iterator_category()));
  EXPECT_TRUE(IsForwardIteratorHelper(
      std::iterator_traits<
          Map<int, int>::const_iterator>::iterator_category()));
}

TEST_F(MapImplTest, InsertSingleLValue) {
  int32_t key = 0;
  int32_t value1 = 100;
  int32_t value2 = 101;

  // Insert a non-existing key.
  Map<int32_t, int32_t>::value_type v1(key, value1);
  std::pair<Map<int32_t, int32_t>::iterator, bool> result1 = map_.insert(v1);
  ExpectSingleElement(key, value1);

  Map<int32_t, int32_t>::iterator it1 = result1.first;
  EXPECT_EQ(key, it1->first);
  EXPECT_EQ(value1, it1->second);
  EXPECT_TRUE(result1.second);

  // Insert an existing key.
  Map<int32_t, int32_t>::value_type v2(key, value2);
  std::pair<Map<int32_t, int32_t>::iterator, bool> result2 = map_.insert(v2);
  ExpectSingleElement(key, value1);

  Map<int32_t, int32_t>::iterator it2 = result2.first;
  EXPECT_TRUE(it1 == it2);
  EXPECT_FALSE(result2.second);
}

TEST_F(MapImplTest, InsertSingleRValue) {
  int32_t key = 0;
  int32_t value1 = 100;
  int32_t value2 = 101;

  // Insert a non-existing key.
  std::pair<Map<int32_t, int32_t>::iterator, bool> result1 =
      map_.insert(Map<int32_t, int32_t>::value_type(key, value1));
  ExpectSingleElement(key, value1);

  Map<int32_t, int32_t>::iterator it1 = result1.first;
  EXPECT_EQ(key, it1->first);
  EXPECT_EQ(value1, it1->second);
  EXPECT_TRUE(result1.second);

  // Insert an existing key.
  std::pair<Map<int32_t, int32_t>::iterator, bool> result2 =
      map_.insert(Map<int32_t, int32_t>::value_type(key, value2));
  ExpectSingleElement(key, value1);

  Map<int32_t, int32_t>::iterator it2 = result2.first;
  EXPECT_TRUE(it1 == it2);
  EXPECT_FALSE(result2.second);
}

TEST_F(MapImplTest, InsertSingleBraceInitList) {
  int32_t key = 0;
  int32_t value1 = 100;
  int32_t value2 = 101;

  // Insert a non-existing key.
  auto result1 = map_.insert({key, value1});
  ExpectSingleElement(key, value1);

  auto it1 = result1.first;
  EXPECT_EQ(key, it1->first);
  EXPECT_EQ(value1, it1->second);
  EXPECT_TRUE(result1.second);

  // Insert an existing key.
  auto result2 = map_.insert({key, value2});
  ExpectSingleElement(key, value1);

  auto it2 = result2.first;
  EXPECT_TRUE(it1 == it2);
  EXPECT_FALSE(result2.second);
}

TEST_F(MapImplTest, InsertSingleBraceInitListTypeMismatch) {
  int32_t key = 0;
  int32_t value1 = 100;
  int32_t value2 = 101;
  Map<int64_t, int64_t> m;

  // Insert a non-existing key.
  auto result1 = m.insert({key, value1});
  EXPECT_TRUE(result1.second);

  // Insert an existing key.
  auto result2 = m.insert({key, value2});
  EXPECT_FALSE(result2.second);

  EXPECT_TRUE(result1.first == result2.first);
}

TEST_F(MapImplTest, TryEmplace) {
  using ::testing::Pair;
  using ::testing::UnorderedElementsAre;

  Map<int32_t, std::string> m;

  m.try_emplace(1, "one");
  EXPECT_EQ(m.size(), 1);

  const int32_t key = 42;
  m.try_emplace(key, 3, 'a');
  m.try_emplace(2, std::string("two"));
  EXPECT_THAT(
      m, UnorderedElementsAre(Pair(1, "one"), Pair(2, "two"), Pair(42, "aaa")));
}

TEST_F(MapImplTest, Emplace) {
  using ::testing::Pair;
  using ::testing::UnorderedElementsAre;

  Map<int32_t, std::string> m;

  m.emplace(1, "one");
  EXPECT_EQ(m.size(), 1);

  const int32_t key = 42;
  m.emplace(key, "aaa");
  m.emplace(2, std::string("two"));
  EXPECT_THAT(
      m, UnorderedElementsAre(Pair(1, "one"), Pair(2, "two"), Pair(42, "aaa")));
}

TEST_F(MapImplTest, TryEmplaceExisting) {
  Arena arena;
  Map<int32_t, int32_t>& m = *Arena::Create<Map<int32_t, int32_t>>(&arena);
  const size_t size1 = arena.SpaceUsed();

  m.try_emplace(1, 1);
  ASSERT_EQ(m.size(), 1);

  const size_t size2 = arena.SpaceUsed();
  ASSERT_GT(size2, size1);

  m.try_emplace(1, 123);
  EXPECT_EQ(m.size(), 1);

  EXPECT_EQ(size2, arena.SpaceUsed());
}

TEST_F(MapImplTest, EmplaceSingle) {
  int32_t key = 0;
  int32_t value1 = 100;
  int32_t value2 = 101;

  // Emplace a non-existing key.
  auto result1 = map_.emplace(key, value1);
  ExpectSingleElement(key, value1);

  Map<int32_t, int32_t>::iterator it1 = result1.first;
  EXPECT_EQ(key, it1->first);
  EXPECT_EQ(value1, it1->second);
  EXPECT_TRUE(result1.second);

  // Emplace an existing key.
  auto result2 = map_.emplace(key, value2);
  ExpectSingleElement(key, value1);

  Map<int32_t, int32_t>::iterator it2 = result2.first;
  EXPECT_TRUE(it1 == it2);
  EXPECT_FALSE(result2.second);
}

TEST_F(MapImplTest, InsertByIterator) {
  int32_t key1 = 0;
  int32_t key2 = 1;
  int32_t value1a = 100;
  int32_t value1b = 101;
  int32_t value2a = 200;
  int32_t value2b = 201;

  absl::flat_hash_map<int32_t, int32_t> map1;
  map1[key1] = value1a;
  map1[key2] = value2a;

  map_.insert(map1.begin(), map1.end());
  ExpectElements(map1);

  absl::flat_hash_map<int32_t, int32_t> map2;
  map2[key1] = value1b;
  map2[key2] = value2b;

  map_.insert(map2.begin(), map2.end());
  ExpectElements(map1);
}

TEST_F(MapImplTest, InsertByInitializerList) {
  map_.insert({{1, 100}, {2, 200}});
  ExpectElements({{1, 100}, {2, 200}});

  map_.insert({{2, 201}, {3, 301}});
  ExpectElements({{1, 100}, {2, 200}, {3, 301}});
}

TEST_F(MapImplTest, EraseSingleByKey) {
  int32_t key = 0;
  int32_t value = 100;

  map_[key] = value;
  ExpectSingleElement(key, value);

  // Erase an existing key.
  EXPECT_EQ(1, map_.erase(key));
  EXPECT_TRUE(map_.empty());
  EXPECT_EQ(0, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(key));
  EXPECT_TRUE(map_.begin() == map_.end());

  // Erase a non-existing key.
  EXPECT_EQ(0, map_.erase(key));
}

TEST_F(MapImplTest, EraseMultipleByKey) {
  // erase in one specific order to trigger corner cases
  for (int i = 0; i < 5; i++) {
    map_[i] = i;
  }

  map_.erase(0);
  EXPECT_EQ(4, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(0));

  map_.erase(1);
  EXPECT_EQ(3, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(1));

  map_.erase(3);
  EXPECT_EQ(2, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(3));

  map_.erase(4);
  EXPECT_EQ(1, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(4));

  map_.erase(2);
  EXPECT_EQ(0, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(2));
}

TEST_F(MapImplTest, EraseSingleByIterator) {
  int32_t key = 0;
  int32_t value = 100;

  map_[key] = value;
  ExpectSingleElement(key, value);

  Map<int32_t, int32_t>::iterator it = map_.find(key);
  map_.erase(it);
  EXPECT_TRUE(map_.empty());
  EXPECT_EQ(0, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(key));
  EXPECT_TRUE(map_.begin() == map_.end());
}

TEST_F(MapImplTest, ValidIteratorAfterErase) {
  for (int i = 0; i < 10; i++) {
    map_[i] = i;
  }

  int count = 0;

  for (Map<int32_t, int32_t>::iterator it = map_.begin(); it != map_.end();) {
    count++;
    if (it->first % 2 == 1) {
      map_.erase(it++);
    } else {
      ++it;
    }
  }

  EXPECT_EQ(10, count);
  EXPECT_EQ(5, map_.size());
}

TEST_F(MapImplTest, EraseByIterator) {
  int32_t key1 = 0;
  int32_t key2 = 1;
  int32_t value1 = 100;
  int32_t value2 = 101;

  absl::flat_hash_map<int32_t, int32_t> map;
  map[key1] = value1;
  map[key2] = value2;

  map_.insert(map.begin(), map.end());
  ExpectElements(map);

  map_.erase(map_.begin(), map_.end());
  EXPECT_TRUE(map_.empty());
  EXPECT_EQ(0, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(key1));
  EXPECT_TRUE(map_.end() == map_.find(key2));
  EXPECT_TRUE(map_.begin() == map_.end());
}

TEST_F(MapImplTest, Clear) {
  int32_t key = 0;
  int32_t value = 100;

  map_[key] = value;
  ExpectSingleElement(key, value);

  map_.clear();

  EXPECT_TRUE(map_.empty());
  EXPECT_EQ(0, map_.size());
  EXPECT_TRUE(map_.end() == map_.find(key));
  EXPECT_TRUE(map_.begin() == map_.end());
}

TEST_F(MapImplTest, IterConstructor) {
  int32_t key1 = 0;
  int32_t key2 = 1;
  int32_t value1 = 100;
  int32_t value2 = 101;

  absl::flat_hash_map<int32_t, int32_t> map;
  map[key1] = value1;
  map[key2] = value2;

  Map<int32_t, int32_t> new_map(map.begin(), map.end());

  EXPECT_EQ(2, new_map.size());
  EXPECT_EQ(value1, new_map.at(key1));
  EXPECT_EQ(value2, new_map.at(key2));
}

TEST_F(MapImplTest, Assigner) {
  int32_t key1 = 0;
  int32_t key2 = 1;
  int32_t value1 = 100;
  int32_t value2 = 101;

  absl::flat_hash_map<int32_t, int32_t> map;
  map[key1] = value1;
  map[key2] = value2;

  map_.insert(map.begin(), map.end());

  Map<int32_t, int32_t> other;
  int32_t key_other = 123;
  int32_t value_other = 321;
  other[key_other] = value_other;
  EXPECT_EQ(1, other.size());

  other = map_;

  EXPECT_EQ(2, other.size());
  EXPECT_EQ(value1, other.at(key1));
  EXPECT_EQ(value2, other.at(key2));
  EXPECT_TRUE(other.find(key_other) == other.end());

  // Self assign
  other = *&other;  // Avoid -Wself-assign.
  EXPECT_EQ(2, other.size());
  EXPECT_EQ(value1, other.at(key1));
  EXPECT_EQ(value2, other.at(key2));
}

TEST_F(MapImplTest, Rehash) {
  const int test_size = 50;
  absl::flat_hash_map<int32_t, int32_t> reference_map;
  for (int i = 0; i < test_size; i++) {
    reference_map[i] = i;
  }
  for (int i = 0; i < test_size; i++) {
    map_[i] = reference_map[i];
    EXPECT_EQ(reference_map[i], map_[i]);
  }
  for (int i = 0; i < test_size; i++) {
    map_.erase(i);
    EXPECT_TRUE(map_.end() == map_.find(i));
  }
  EXPECT_TRUE(map_.empty());
}

TEST_F(MapImplTest, EqualRange) {
  int key = 100, key_missing = 101;
  map_[key] = 100;

  std::pair<Map<int32_t, int32_t>::iterator, Map<int32_t, int32_t>::iterator>
      range = map_.equal_range(key);
  EXPECT_TRUE(map_.find(key) == range.first);
  EXPECT_TRUE(++map_.find(key) == range.second);

  range = map_.equal_range(key_missing);
  EXPECT_TRUE(map_.end() == range.first);
  EXPECT_TRUE(map_.end() == range.second);

  std::pair<Map<int32_t, int32_t>::const_iterator,
            Map<int32_t, int32_t>::const_iterator>
      const_range = const_map_.equal_range(key);
  EXPECT_TRUE(const_map_.find(key) == const_range.first);
  EXPECT_TRUE(++const_map_.find(key) == const_range.second);

  const_range = const_map_.equal_range(key_missing);
  EXPECT_TRUE(const_map_.end() == const_range.first);
  EXPECT_TRUE(const_map_.end() == const_range.second);
}

TEST_F(MapImplTest, ConvertToStdMap) {
  map_[100] = 101;
  absl::flat_hash_map<int32_t, int32_t> std_map(map_.begin(), map_.end());
  EXPECT_EQ(1, std_map.size());
  EXPECT_EQ(101, std_map[100]);
}

TEST_F(MapImplTest, ConvertToStdVectorOfPairs) {
  map_[100] = 101;
  std::vector<std::pair<int32_t, int32_t>> std_vec(map_.begin(), map_.end());
  EXPECT_EQ(1, std_vec.size());
  EXPECT_EQ(100, std_vec[0].first);
  EXPECT_EQ(101, std_vec[0].second);
}

TEST_F(MapImplTest, SwapBasic) {
  Map<int32_t, int32_t> another;
  map_[9398] = 41999;
  another[9398] = 41999;
  another[8070] = 42056;
  another.swap(map_);
  EXPECT_THAT(another,
              testing::UnorderedElementsAre(testing::Pair(9398, 41999)));
  EXPECT_THAT(map_, testing::UnorderedElementsAre(testing::Pair(8070, 42056),
                                                  testing::Pair(9398, 41999)));
}

TEST_F(MapImplTest, SwapArena) {
  Arena arena1, arena2;
  Map<int32_t, int32_t> m1(&arena1);
  Map<int32_t, int32_t> m2(&arena2);
  map_[9398] = 41999;
  m1[9398] = 41999;
  m1[8070] = 42056;
  m2[10244] = 10247;
  m2[8070] = 42056;
  m1.swap(map_);
  EXPECT_THAT(m1, testing::UnorderedElementsAre(testing::Pair(9398, 41999)));
  EXPECT_THAT(map_, testing::UnorderedElementsAre(testing::Pair(8070, 42056),
                                                  testing::Pair(9398, 41999)));
  m2.swap(m1);
  EXPECT_THAT(m1, testing::UnorderedElementsAre(testing::Pair(8070, 42056),
                                                testing::Pair(10244, 10247)));
  EXPECT_THAT(m2, testing::UnorderedElementsAre(testing::Pair(9398, 41999)));
}

TEST_F(MapImplTest, SwapFieldArenaReflection) {
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());

  {
    // Tests filled lfs and empty rhs.
    TestMap rhs;

    {
      // Use local_arena to allocate lhs to trigger use-after-free error.
      Arena local_arena;
      auto* lhs = Arena::Create<TestMap>(&local_arena);
      const auto* reflection = lhs->GetReflection();
      std::vector<const FieldDescriptor*> fields;

      reflection_tester.SetMapFieldsViaReflection(lhs);
      reflection->ListFields(*lhs, &fields);

      reflection->SwapFields(lhs, &rhs, fields);

      reflection_tester.ExpectClearViaReflection(*lhs);

      // Add an entry to make sure it is using the right arena.
      (*lhs->mutable_map_int32_int32())[1234] = 1234;
    }

    reflection_tester.ExpectMapFieldsSetViaReflection(rhs);

      // Add an entry to make sure it is using the right arena.
    (*rhs.mutable_map_int32_int32())[1234] = 1234;
  }
}

TEST_F(MapImplTest, RecursiveMapTypeDoesNotDeadlockOnDynamicMessage) {
  const auto* descriptor = UNITTEST::TestRecursiveMapMessage::descriptor();
  const auto* entry_descriptor =
      descriptor->FindFieldByNumber(1)->message_type();

  {
    // Start at the message
    DynamicMessageFactory factory;
    const Message* prototype = factory.GetPrototype(descriptor);
    std::unique_ptr<Message> msg(prototype->New());
  }
  {
    // Start at the map entry
    DynamicMessageFactory factory;
    const Message* prototype = factory.GetPrototype(entry_descriptor);
    std::unique_ptr<Message> msg(prototype->New());
  }
}

TEST_F(MapImplTest, DeleteMapValues) {
  auto* descriptor = UNITTEST::TestMap::descriptor();

  DynamicMessageFactory factory;
  std::unique_ptr<Message> dyn_message(factory.GetPrototype(descriptor)->New());
  std::unique_ptr<Message> gen_message = std::make_unique<TestMap>();
  Arena arena;

  for (auto* message : {gen_message.get(), dyn_message.get(),
                        gen_message->New(&arena), dyn_message->New(&arena)}) {
    MapReflectionTester reflection_tester(descriptor);
    reflection_tester.SetMapFieldsViaMapReflection(message);

    // Each maps has two elements. Let's remove the smallest one first, so that
    // we can test the result. Then remove the rest and test again.

    for (int i = 0; i < descriptor->field_count(); ++i) {
      auto* field = descriptor->field(i);
      if (!field->is_map()) continue;

      MapIterator it = reflection_tester.MapBegin(message, field->name());
      MapIterator end = reflection_tester.MapEnd(message, field->name());
      // It's unordered, so search for it.
      if (it == end) continue;
      auto next = it;
      ++next;
      ASSERT_NE(next, end);
      if (next.GetKey() < it.GetKey()) {
        it = next;
      }

      EXPECT_EQ(reflection_tester.MapSize(*message, field->name()), 2);
      reflection_tester.DeleteMapValueViaMapReflection(
          message, field->name(), it.GetKey());
      EXPECT_EQ(reflection_tester.MapSize(*message, field->name()), 1);
    }

    EXPECT_THAT(*message, EqualsProto(R"pb(
      map_int32_int32 { key: 1 value: 1 }
      map_int64_int64 { key: 1 value: 1 }
      map_uint32_uint32 { key: 1 value: 1 }
      map_uint64_uint64 { key: 1 value: 1 }
      map_sint32_sint32 { key: 1 value: 1 }
      map_sint64_sint64 { key: 1 value: 1 }
      map_fixed32_fixed32 { key: 1 value: 1 }
      map_fixed64_fixed64 { key: 1 value: 1 }
      map_sfixed32_sfixed32 { key: 1 value: 1 }
      map_sfixed64_sfixed64 { key: 1 value: 1 }
      map_int32_float { key: 1 value: 1 }
      map_int32_double { key: 1 value: 1 }
      map_bool_bool { key: true value: true }
      map_string_string {
        key: "This is another very long string that goes in the heap"
        value: "This is another very long string that goes in the heap"
      }
      map_int32_bytes {
        key: 1
        value: "This is another very long string that goes in the heap"
      }
      map_int32_enum { key: 1 value: MAP_ENUM_BAZ }
      map_int32_foreign_message {
        key: 1
        value { c: 1 }
      }
    )pb"));

    for (int i = 0; i < descriptor->field_count(); ++i) {
      auto* field = descriptor->field(i);
      if (!field->is_map()) continue;

      MapIterator it = reflection_tester.MapBegin(message, field->name());
      MapIterator end = reflection_tester.MapEnd(message, field->name());

      while (it != end) {
        auto next = it;
        ++next;
        reflection_tester.DeleteMapValueViaMapReflection(
            message, field->name(), it.GetKey());
        it = next;
      }
      EXPECT_EQ(reflection_tester.MapSize(*message, field->name()), 0);
    }
  }
}

TEST_F(MapImplTest, CopyAssignMapIterator) {
  TestMap message;
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaMapReflection(&message);
  MapIterator it1 = reflection_tester.MapBegin(&message, "map_int32_int32");
  MapIterator it2 = reflection_tester.MapEnd(&message, "map_int32_int32");
  it2 = it1;
  EXPECT_EQ(it1.GetKey().GetInt32Value(), it2.GetKey().GetInt32Value());
}

class MapTestWithParams
    : public testing::TestWithParam<std::tuple<bool, bool, bool>> {};

INSTANTIATE_TEST_SUITE_P(MapTestWithParamsSuite, MapTestWithParams,
                         testing::Combine(testing::Bool(), testing::Bool(),
                                          testing::Bool()));

TEST_P(MapTestWithParams, SwapMapsFieldsViaReflection) {
  auto* descriptor = UNITTEST::TestMap::descriptor();
  auto [use_dynamic, lhs_arena, rhs_arena] = GetParam();

  DynamicMessageFactory factory;
  std::unique_ptr<const Message> dyn_message(
      factory.GetPrototype(descriptor)->New());
  TestMap gen_message;
  Arena arena;

  std::vector<const FieldDescriptor*> fields;
  for (int i = 0; i < descriptor->field_count(); ++i) {
    fields.push_back(descriptor->field(i));
  }

  const auto make = [&, use_dynamic = use_dynamic](bool in_arena) {
    auto* prototype = use_dynamic ? dyn_message.get() : &gen_message;
    return in_arena ? prototype->New(&arena)
                    : Arena::Create<std::unique_ptr<Message>>(&arena,
                                                              prototype->New())
                          ->get();
  };

  auto* lhs = make(lhs_arena);
  auto* rhs = make(rhs_arena);
  MapReflectionTester reflection_tester(descriptor);
  reflection_tester.SetMapFieldsViaMapReflection(lhs);

  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(lhs);
  reflection_tester.ExpectClearViaReflectionIterator(rhs);

  lhs->GetReflection()->SwapFields(lhs, rhs, fields);

  reflection_tester.ExpectClearViaReflectionIterator(lhs);
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(rhs);

  if (lhs_arena == rhs_arena) {
    // If they are the same arena, we can try the unsafe swap.
    lhs->GetReflection()->UnsafeArenaSwapFields(lhs, rhs, fields);
    reflection_tester.ExpectClearViaReflectionIterator(rhs);
    reflection_tester.ExpectMapFieldsSetViaReflectionIterator(lhs);
  }
}

class MapTestCodegenOrDynamic : public testing::TestWithParam<bool> {};

INSTANTIATE_TEST_SUITE_P(MapTestCodegenOrDynamicSuite, MapTestCodegenOrDynamic,
                         testing::Bool());

TEST_F(MapImplTest, CopyConstructMapSyncsRepeatedFieldRep) {
  TestMap msg;

  const auto* reflection = msg.GetReflection();
  const auto* field = msg.GetDescriptor()->FindFieldByName("map_int32_int32");

  {
    auto* sub = reflection->AddMessage(&msg, field);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_key(), 10);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_value(), 100);
  }

  TestMap copy = msg;
  EXPECT_THAT(msg,
              EqualsProto(R"pb(map_int32_int32 { key: 10 value: 100 })pb"));
  EXPECT_THAT(copy,
              EqualsProto(R"pb(map_int32_int32 { key: 10 value: 100 })pb"));
}

TEST_F(MapImplTest, CopyAssignMapSyncsRepeatedFieldRep) {
  TestMap msg;

  const auto* reflection = msg.GetReflection();
  const auto* field = msg.GetDescriptor()->FindFieldByName("map_int32_int32");

  {
    auto* sub = reflection->AddMessage(&msg, field);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_key(), 10);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_value(), 100);
  }

  TestMap dst;
  dst = msg;
  EXPECT_THAT(msg,
              EqualsProto(R"pb(map_int32_int32 { key: 10 value: 100 })pb"));
  EXPECT_THAT(dst,
              EqualsProto(R"pb(map_int32_int32 { key: 10 value: 100 })pb"));
}

TEST_F(MapImplTest, MergeMapSyncsRepeatedFieldRep) {
  TestMap msg;

  const auto* reflection = msg.GetReflection();
  const auto* field = msg.GetDescriptor()->FindFieldByName("map_int32_int32");

  {
    auto* sub = reflection->AddMessage(&msg, field);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_key(), 10);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_value(), 100);
  }

  TestMap dst;
  {
    auto* sub = reflection->AddMessage(&dst, field);
    // A duplicate key
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_key(), 10);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_value(), 200);
    sub = reflection->AddMessage(&dst, field);
    // A unique key
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_key(), 11);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_value(), 17);
  }
  dst.MergeFrom(msg);
  EXPECT_THAT(msg,
              EqualsProto(R"pb(map_int32_int32 { key: 10 value: 100 })pb"));
  EXPECT_THAT(dst, EqualsProto(R"pb(
                map_int32_int32 { key: 10 value: 100 }
                map_int32_int32 { key: 11 value: 17 }
              )pb"));
}

TEST_P(MapTestCodegenOrDynamic, LookupMapValueSyncsRepeatedFieldRep) {
  auto* descriptor = UNITTEST::TestMap::descriptor();
  bool use_dynamic = GetParam();

  DynamicMessageFactory factory;
  std::unique_ptr<Message> dyn_message(
      factory.GetPrototype(descriptor)->New());
  TestMap gen_message;
  Message* msg = use_dynamic ? dyn_message.get() : &gen_message;

  const auto* reflection = msg->GetReflection();
  const auto* field = msg->GetDescriptor()->FindFieldByName("map_int32_int32");

  MapKey map_key;
  map_key.SetInt32Value(10);
  auto res =
      MapReflectionTester::LookupMapValue(*reflection, *msg, *field, map_key);
  // Check that we don't have it yet.
  ASSERT_FALSE(res.has_value());

  {
    auto* sub = reflection->AddMessage(msg, field);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_key(), 10);
    sub->GetReflection()->SetInt32(sub, sub->GetDescriptor()->map_value(), 100);
  }


  res = MapReflectionTester::LookupMapValue(*reflection, *msg, *field, map_key);
  // If this fails, LookupMapValue might have failed to sync from the repeated
  // field.
  ASSERT_TRUE(res.has_value());
  EXPECT_EQ(res->GetInt32Value(), 100);
}

TEST_F(MapImplTest, SpaceUsed) {
  constexpr size_t kMinCap = 16 / sizeof(void*);

  Map<int32_t, int32_t> m;
  // An newly constructed map should have no space used.
  EXPECT_EQ(m.SpaceUsedExcludingSelfLong(), 0);

  struct IntIntNode : internal::NodeBase {
    std::pair<int32_t, int32_t> kv;
  };

  for (int i = 0; i < 100; ++i) {
    m[i];
    EXPECT_EQ(m.SpaceUsedExcludingSelfLong(),
              sizeof(void*) * MapTestPeer::NumBuckets(m) +
                  m.size() * sizeof(IntIntNode));
  }

  // Test string, and non-scalar keys.
  Map<std::string, int32_t> m2;
  std::string str = "Some arbitrarily large string";
  m2[str] = 1;

  struct StringIntNode : internal::NodeBase {
    std::pair<std::string, int32_t> kv;
  };

  EXPECT_EQ(m2.SpaceUsedExcludingSelfLong(),
            sizeof(void*) * kMinCap + sizeof(StringIntNode) +
                internal::StringSpaceUsedExcludingSelfLong(str));

  struct IntAllTypesNode : internal::NodeBase {
    std::pair<int32_t, TestAllTypes> kv;
  };

  // Test messages, and non-scalar values.
  Map<int32_t, TestAllTypes> m3;
  m3[0].set_optional_string(str);
  EXPECT_EQ(m3.SpaceUsedExcludingSelfLong(),
            sizeof(void*) * kMinCap + sizeof(IntAllTypesNode) +
                m3[0].SpaceUsedLong() - sizeof(m3[0]));
}

// Attempts to verify that a map with keys a and b has a random ordering. This
// function returns true if it succeeds in observing both possible orderings.
bool MapOrderingIsRandom(int a, int b) {
  bool saw_a_first = false;
  bool saw_b_first = false;
  std::vector<Map<int32_t, int32_t>> v;
  while (v.size() < 100) {
    Map<int32_t, int32_t>& m = v.emplace_back();
    m[a] = 0;
    m[b] = 0;
    int32_t first_element = m.begin()->first;
    if (first_element == a) saw_a_first = true;
    if (first_element == b) saw_b_first = true;
    if (saw_a_first && saw_b_first) {
      return true;
    }
  }
  return false;
}

// This test verifies that the iteration order is reasonably random even for
// small maps.
TEST_F(MapImplTest, RandomOrdering) {
  for (int i = 0; i < 10; ++i) {
    for (int j = i + 1; j < 10; ++j) {
      EXPECT_TRUE(MapOrderingIsRandom(i, j))
          << "Map with keys " << i << " and " << j
          << " has deterministic ordering";
    }
  }
}

template <typename Key>
void TestTransparent(const Key& key, const Key& miss_key) {
  Map<std::string, int> m;
  const auto& cm = m;

  m.insert({"ABC", 1});
  m.insert({"DEF", 2});

  const auto abc_it = m.find("ABC");

  using testing::Pair;
  using testing::UnorderedElementsAre;

  EXPECT_EQ(m.at(key), 1);
  EXPECT_EQ(cm.at(key), 1);

#if GTEST_HAS_DEATH_TEST
  EXPECT_DEATH(m.at(miss_key), "");
  EXPECT_DEATH(cm.at(miss_key), "");
#endif  // GTEST_HAS_DEATH_TEST

  EXPECT_EQ(m.count(key), 1);
  EXPECT_EQ(cm.count(key), 1);
  EXPECT_EQ(m.count(miss_key), 0);
  EXPECT_EQ(cm.count(miss_key), 0);

  EXPECT_EQ(m.find(key), abc_it);
  EXPECT_EQ(cm.find(key), abc_it);
  EXPECT_EQ(m.find(miss_key), m.end());
  EXPECT_EQ(cm.find(miss_key), cm.end());

  EXPECT_TRUE(m.contains(key));
  EXPECT_TRUE(cm.contains(key));
  EXPECT_FALSE(m.contains(miss_key));
  EXPECT_FALSE(cm.contains(miss_key));

  EXPECT_THAT(m.equal_range(key), Pair(abc_it, std::next(abc_it)));
  EXPECT_THAT(cm.equal_range(key), Pair(abc_it, std::next(abc_it)));
  EXPECT_THAT(m.equal_range(miss_key), Pair(m.end(), m.end()));
  EXPECT_THAT(cm.equal_range(miss_key), Pair(m.end(), m.end()));

  EXPECT_THAT(m, UnorderedElementsAre(Pair("ABC", 1), Pair("DEF", 2)));
  EXPECT_EQ(m.erase(key), 1);
  EXPECT_THAT(m, UnorderedElementsAre(Pair("DEF", 2)));
  EXPECT_EQ(m.erase(key), 0);
  EXPECT_EQ(m.erase(miss_key), 0);
  EXPECT_THAT(m, UnorderedElementsAre(Pair("DEF", 2)));

  m[key];
  EXPECT_THAT(m, UnorderedElementsAre(Pair("ABC", 0), Pair("DEF", 2)));
  m[key] = 1;
  EXPECT_THAT(m, UnorderedElementsAre(Pair("ABC", 1), Pair("DEF", 2)));
}

// Emulate a non-Abseil string_view (e.g. STL when Abseil's alias is disabled).
class CustomStringView {
  public:
    CustomStringView(absl::string_view str) : str_(std::string(str)) {}

    const char* data() const { return str_.data(); }
    size_t size() const { return str_.size(); }

    bool operator==(const CustomStringView& other) const {
      return other.str_ == str_;
    }
    bool operator==(absl::string_view other) const { return other == str_; }
    explicit operator std::string() const { return str_; }
    friend std::ostream& operator<<(std::ostream& out, const CustomStringView& view) {
      return out << view.str_;
    }

  private:
    std::string str_;
};

TEST_F(MapImplTest, TransparentLookupForString) {
  TestTransparent("ABC", "LKJ");
  TestTransparent(std::string("ABC"), std::string("LKJ"));
  TestTransparent(absl::string_view("ABC"), absl::string_view("LKJ"));
  TestTransparent(CustomStringView("ABC"), CustomStringView("LKJ"));

  // std::reference_wrapper
  std::string abc = "ABC", lkj = "LKJ";
  TestTransparent(std::ref(abc), std::ref(lkj));
  TestTransparent(std::cref(abc), std::cref(lkj));
}

TEST_F(MapImplTest, ConstInit) {
  PROTOBUF_CONSTINIT static Map<int, int> map;  // NOLINT
  EXPECT_TRUE(map.empty());
}

// Map Field Reflection Test ========================================

static int Func(int i, int j) { return i * j; }

static std::string StrFunc(int i, int j) { return absl::StrCat(Func(i, j)); }

static int Int(const std::string& value) {
  int result = 0;
  std::istringstream(value) >> result;
  return result;
}

}  // namespace

// This class is a friend, so no anonymous namespace.
class MapFieldReflectionTest : public testing::Test {
 protected:
  typedef FieldDescriptor FD;

  int MapSize(const Reflection* reflection, const FieldDescriptor* field,
              const Message& message) {
    return reflection->MapSize(message, field);
  }
};

namespace {

TEST_F(MapFieldReflectionTest, RegularFields) {
  TestMap message;
  const Reflection* refl = message.GetReflection();
  const Descriptor* desc = message.GetDescriptor();

  Map<int32_t, int32_t>* map_int32_int32 = message.mutable_map_int32_int32();
  Map<int32_t, double>* map_int32_double = message.mutable_map_int32_double();
  Map<std::string, std::string>* map_string_string =
      message.mutable_map_string_string();
  Map<int32_t, ForeignMessage>* map_int32_foreign_message =
      message.mutable_map_int32_foreign_message();

  for (int i = 0; i < 10; ++i) {
    (*map_int32_int32)[i] = Func(i, 1);
    (*map_int32_double)[i] = Func(i, 2);
    (*map_string_string)[StrFunc(i, 1)] = StrFunc(i, 5);
    (*map_int32_foreign_message)[i].set_c(Func(i, 6));
  }

  // Get FieldDescriptors for all the fields of interest.
  const FieldDescriptor* fd_map_int32_int32 =
      desc->FindFieldByName("map_int32_int32");
  const FieldDescriptor* fd_map_int32_double =
      desc->FindFieldByName("map_int32_double");
  const FieldDescriptor* fd_map_string_string =
      desc->FindFieldByName("map_string_string");
  const FieldDescriptor* fd_map_int32_foreign_message =
      desc->FindFieldByName("map_int32_foreign_message");

  const FieldDescriptor* fd_map_int32_in32_key =
      fd_map_int32_int32->message_type()->map_key();
  const FieldDescriptor* fd_map_int32_in32_value =
      fd_map_int32_int32->message_type()->map_value();
  const FieldDescriptor* fd_map_int32_double_key =
      fd_map_int32_double->message_type()->map_key();
  const FieldDescriptor* fd_map_int32_double_value =
      fd_map_int32_double->message_type()->map_value();
  const FieldDescriptor* fd_map_string_string_key =
      fd_map_string_string->message_type()->map_key();
  const FieldDescriptor* fd_map_string_string_value =
      fd_map_string_string->message_type()->map_value();
  const FieldDescriptor* fd_map_int32_foreign_message_key =
      fd_map_int32_foreign_message->message_type()->map_key();
  const FieldDescriptor* fd_map_int32_foreign_message_value =
      fd_map_int32_foreign_message->message_type()->map_value();

  // Get RepeatedPtrField objects for all fields of interest.
  const RepeatedPtrField<Message>& mf_int32_int32 =
      refl->GetRepeatedPtrField<Message>(message, fd_map_int32_int32);
  const RepeatedPtrField<Message>& mf_int32_double =
      refl->GetRepeatedPtrField<Message>(message, fd_map_int32_double);
  const RepeatedPtrField<Message>& mf_string_string =
      refl->GetRepeatedPtrField<Message>(message, fd_map_string_string);
  const RepeatedPtrField<Message>& mf_int32_foreign_message =
      refl->GetRepeatedPtrField<Message>(message, fd_map_int32_foreign_message);

  // Get mutable RepeatedPtrField objects for all fields of interest.
  RepeatedPtrField<Message>* mmf_int32_int32 =
      refl->MutableRepeatedPtrField<Message>(&message, fd_map_int32_int32);
  RepeatedPtrField<Message>* mmf_int32_double =
      refl->MutableRepeatedPtrField<Message>(&message, fd_map_int32_double);
  RepeatedPtrField<Message>* mmf_string_string =
      refl->MutableRepeatedPtrField<Message>(&message, fd_map_string_string);
  RepeatedPtrField<Message>* mmf_int32_foreign_message =
      refl->MutableRepeatedPtrField<Message>(&message,
                                             fd_map_int32_foreign_message);

  // Make sure we can do gets through the RepeatedPtrField objects.
  for (int i = 0; i < 10; ++i) {
    {
      // Check gets through const objects.
      const Message& message_int32_int32 = mf_int32_int32.Get(i);
      int32_t key_int32_int32 = message_int32_int32.GetReflection()->GetInt32(
          message_int32_int32, fd_map_int32_in32_key);
      int32_t value_int32_int32 = message_int32_int32.GetReflection()->GetInt32(
          message_int32_int32, fd_map_int32_in32_value);
      EXPECT_EQ(value_int32_int32, Func(key_int32_int32, 1));

      const Message& message_int32_double = mf_int32_double.Get(i);
      int32_t key_int32_double = message_int32_double.GetReflection()->GetInt32(
          message_int32_double, fd_map_int32_double_key);
      double value_int32_double =
          message_int32_double.GetReflection()->GetDouble(
              message_int32_double, fd_map_int32_double_value);
      EXPECT_EQ(value_int32_double, Func(key_int32_double, 2));

      const Message& message_string_string = mf_string_string.Get(i);
      std::string key_string_string =
          message_string_string.GetReflection()->GetString(
              message_string_string, fd_map_string_string_key);
      std::string value_string_string =
          message_string_string.GetReflection()->GetString(
              message_string_string, fd_map_string_string_value);
      EXPECT_EQ(value_string_string, StrFunc(Int(key_string_string), 5));

      const Message& message_int32_message = mf_int32_foreign_message.Get(i);
      int32_t key_int32_message =
          message_int32_message.GetReflection()->GetInt32(
              message_int32_message, fd_map_int32_foreign_message_key);
      const ForeignMessage& value_int32_message =
          DownCastMessage<ForeignMessage>(
              message_int32_message.GetReflection()->GetMessage(
                  message_int32_message, fd_map_int32_foreign_message_value));
      EXPECT_EQ(value_int32_message.c(), Func(key_int32_message, 6));
    }

    {
      // Check gets through mutable objects.
      const Message& message_int32_int32 = mmf_int32_int32->Get(i);
      int32_t key_int32_int32 = message_int32_int32.GetReflection()->GetInt32(
          message_int32_int32, fd_map_int32_in32_key);
      int32_t value_int32_int32 = message_int32_int32.GetReflection()->GetInt32(
          message_int32_int32, fd_map_int32_in32_value);
      EXPECT_EQ(value_int32_int32, Func(key_int32_int32, 1));

      const Message& message_int32_double = mmf_int32_double->Get(i);
      int32_t key_int32_double = message_int32_double.GetReflection()->GetInt32(
          message_int32_double, fd_map_int32_double_key);
      double value_int32_double =
          message_int32_double.GetReflection()->GetDouble(
              message_int32_double, fd_map_int32_double_value);
      EXPECT_EQ(value_int32_double, Func(key_int32_double, 2));

      const Message& message_string_string = mmf_string_string->Get(i);
      std::string key_string_string =
          message_string_string.GetReflection()->GetString(
              message_string_string, fd_map_string_string_key);
      std::string value_string_string =
          message_string_string.GetReflection()->GetString(
              message_string_string, fd_map_string_string_value);
      EXPECT_EQ(value_string_string, StrFunc(Int(key_string_string), 5));

      const Message& message_int32_message = mmf_int32_foreign_message->Get(i);
      int32_t key_int32_message =
          message_int32_message.GetReflection()->GetInt32(
              message_int32_message, fd_map_int32_foreign_message_key);
      const ForeignMessage& value_int32_message =
          DownCastMessage<ForeignMessage>(
              message_int32_message.GetReflection()->GetMessage(
                  message_int32_message, fd_map_int32_foreign_message_value));
      EXPECT_EQ(value_int32_message.c(), Func(key_int32_message, 6));
    }
  }

  // Do sets through the RepeatedPtrField objects.
  for (int i = 0; i < 10; i++) {
    {
      Message* message_int32_int32 = mmf_int32_int32->Mutable(i);
      int32_t key_int32_int32 = message_int32_int32->GetReflection()->GetInt32(
          *message_int32_int32, fd_map_int32_in32_key);
      message_int32_int32->GetReflection()->SetInt32(message_int32_int32,
                                                     fd_map_int32_in32_value,
                                                     Func(key_int32_int32, -1));

      Message* message_int32_double = mmf_int32_double->Mutable(i);
      int32_t key_int32_double =
          message_int32_double->GetReflection()->GetInt32(
              *message_int32_double, fd_map_int32_double_key);
      message_int32_double->GetReflection()->SetDouble(
          message_int32_double, fd_map_int32_double_value,
          Func(key_int32_double, -2));

      Message* message_string_string = mmf_string_string->Mutable(i);
      std::string key_string_string =
          message_string_string->GetReflection()->GetString(
              *message_string_string, fd_map_string_string_key);
      message_string_string->GetReflection()->SetString(
          message_string_string, fd_map_string_string_value,
          StrFunc(Int(key_string_string), -5));

      Message* message_int32_message = mmf_int32_foreign_message->Mutable(i);
      int32_t key_int32_message =
          message_int32_message->GetReflection()->GetInt32(
              *message_int32_message, fd_map_int32_foreign_message_key);
      ForeignMessage* value_int32_message = DownCastMessage<ForeignMessage>(
          message_int32_message->GetReflection()->MutableMessage(
              message_int32_message, fd_map_int32_foreign_message_value));
      value_int32_message->set_c(Func(key_int32_message, -6));
    }
  }

  // Check gets through mutable objects.
  for (int i = 0; i < 10; i++) {
    EXPECT_EQ(Func(i, -1), message.map_int32_int32().at(i));
    EXPECT_EQ(Func(i, -2), message.map_int32_double().at(i));
    EXPECT_EQ(StrFunc(i, -5), message.map_string_string().at(StrFunc(i, 1)));
    EXPECT_EQ(Func(i, -6), message.map_int32_foreign_message().at(i).c());
  }
}

TEST_F(MapFieldReflectionTest, RepeatedFieldRefForRegularFields) {
  TestMap message;
  const Reflection* refl = message.GetReflection();
  const Descriptor* desc = message.GetDescriptor();

  Map<int32_t, int32_t>* map_int32_int32 = message.mutable_map_int32_int32();
  Map<int32_t, double>* map_int32_double = message.mutable_map_int32_double();
  Map<std::string, std::string>* map_string_string =
      message.mutable_map_string_string();
  Map<int32_t, ForeignMessage>* map_int32_foreign_message =
      message.mutable_map_int32_foreign_message();

  for (int i = 0; i < 10; ++i) {
    (*map_int32_int32)[i] = Func(i, 1);
    (*map_int32_double)[i] = Func(i, 2);
    (*map_string_string)[StrFunc(i, 1)] = StrFunc(i, 5);
    (*map_int32_foreign_message)[i].set_c(Func(i, 6));
  }

  // Get FieldDescriptors for all the fields of interest.
  const FieldDescriptor* fd_map_int32_int32 =
      desc->FindFieldByName("map_int32_int32");
  const FieldDescriptor* fd_map_int32_double =
      desc->FindFieldByName("map_int32_double");
  const FieldDescriptor* fd_map_string_string =
      desc->FindFieldByName("map_string_string");
  const FieldDescriptor* fd_map_int32_foreign_message =
      desc->FindFieldByName("map_int32_foreign_message");

  const FieldDescriptor* fd_map_int32_in32_key =
      fd_map_int32_int32->message_type()->map_key();
  const FieldDescriptor* fd_map_int32_in32_value =
      fd_map_int32_int32->message_type()->map_value();
  const FieldDescriptor* fd_map_int32_double_key =
      fd_map_int32_double->message_type()->map_key();
  const FieldDescriptor* fd_map_int32_double_value =
      fd_map_int32_double->message_type()->map_value();
  const FieldDescriptor* fd_map_string_string_key =
      fd_map_string_string->message_type()->map_key();
  const FieldDescriptor* fd_map_string_string_value =
      fd_map_string_string->message_type()->map_value();
  const FieldDescriptor* fd_map_int32_foreign_message_key =
      fd_map_int32_foreign_message->message_type()->map_key();
  const FieldDescriptor* fd_map_int32_foreign_message_value =
      fd_map_int32_foreign_message->message_type()->map_value();

  // Get RepeatedFieldRef objects for all fields of interest.
  const RepeatedFieldRef<Message> mf_int32_int32 =
      refl->GetRepeatedFieldRef<Message>(message, fd_map_int32_int32);
  const RepeatedFieldRef<Message> mf_int32_double =
      refl->GetRepeatedFieldRef<Message>(message, fd_map_int32_double);
  const RepeatedFieldRef<Message> mf_string_string =
      refl->GetRepeatedFieldRef<Message>(message, fd_map_string_string);
  const RepeatedFieldRef<Message> mf_int32_foreign_message =
      refl->GetRepeatedFieldRef<Message>(message, fd_map_int32_foreign_message);

  // Get mutable RepeatedFieldRef objects for all fields of interest.
  const MutableRepeatedFieldRef<Message> mmf_int32_int32 =
      refl->GetMutableRepeatedFieldRef<Message>(&message, fd_map_int32_int32);
  const MutableRepeatedFieldRef<Message> mmf_int32_double =
      refl->GetMutableRepeatedFieldRef<Message>(&message, fd_map_int32_double);
  const MutableRepeatedFieldRef<Message> mmf_string_string =
      refl->GetMutableRepeatedFieldRef<Message>(&message, fd_map_string_string);
  const MutableRepeatedFieldRef<Message> mmf_int32_foreign_message =
      refl->GetMutableRepeatedFieldRef<Message>(&message,
                                                fd_map_int32_foreign_message);

  // Get entry default instances
  std::unique_ptr<Message> entry_int32_int32(
      MessageFactory::generated_factory()
          ->GetPrototype(fd_map_int32_int32->message_type())
          ->New(message.GetArena()));
  std::unique_ptr<Message> entry_int32_double(
      MessageFactory::generated_factory()
          ->GetPrototype(fd_map_int32_double->message_type())
          ->New(message.GetArena()));
  std::unique_ptr<Message> entry_string_string(
      MessageFactory::generated_factory()
          ->GetPrototype(fd_map_string_string->message_type())
          ->New(message.GetArena()));
  std::unique_ptr<Message> entry_int32_foreign_message(
      MessageFactory::generated_factory()
          ->GetPrototype(fd_map_int32_foreign_message->message_type())
          ->New(message.GetArena()));

  EXPECT_EQ(10, mf_int32_int32.size());
  EXPECT_EQ(10, mmf_int32_int32.size());
  EXPECT_EQ(10, mf_int32_double.size());
  EXPECT_EQ(10, mmf_int32_double.size());
  EXPECT_EQ(10, mf_string_string.size());
  EXPECT_EQ(10, mmf_string_string.size());
  EXPECT_EQ(10, mf_int32_foreign_message.size());
  EXPECT_EQ(10, mmf_int32_foreign_message.size());

  EXPECT_FALSE(mf_int32_int32.empty());
  EXPECT_FALSE(mmf_int32_int32.empty());
  EXPECT_FALSE(mf_int32_double.empty());
  EXPECT_FALSE(mmf_int32_double.empty());
  EXPECT_FALSE(mf_string_string.empty());
  EXPECT_FALSE(mmf_string_string.empty());
  EXPECT_FALSE(mf_int32_foreign_message.empty());
  EXPECT_FALSE(mmf_int32_foreign_message.empty());

  // Make sure we can do gets through the RepeatedFieldRef objects.
  for (int i = 0; i < 10; ++i) {
    {
      // Check gets through const objects.
      const Message& message_int32_int32 =
          mf_int32_int32.Get(i, entry_int32_int32.get());
      int32_t key_int32_int32 = message_int32_int32.GetReflection()->GetInt32(
          message_int32_int32, fd_map_int32_in32_key);
      int32_t value_int32_int32 = message_int32_int32.GetReflection()->GetInt32(
          message_int32_int32, fd_map_int32_in32_value);
      EXPECT_EQ(value_int32_int32, Func(key_int32_int32, 1));

      const Message& message_int32_double =
          mf_int32_double.Get(i, entry_int32_double.get());
      int32_t key_int32_double = message_int32_double.GetReflection()->GetInt32(
          message_int32_double, fd_map_int32_double_key);
      double value_int32_double =
          message_int32_double.GetReflection()->GetDouble(
              message_int32_double, fd_map_int32_double_value);
      EXPECT_EQ(value_int32_double, Func(key_int32_double, 2));

      const Message& message_string_string =
          mf_string_string.Get(i, entry_string_string.get());
      std::string key_string_string =
          message_string_string.GetReflection()->GetString(
              message_string_string, fd_map_string_string_key);
      std::string value_string_string =
          message_string_string.GetReflection()->GetString(
              message_string_string, fd_map_string_string_value);
      EXPECT_EQ(value_string_string, StrFunc(Int(key_string_string), 5));

      const Message& message_int32_message =
          mf_int32_foreign_message.Get(i, entry_int32_foreign_message.get());
      int32_t key_int32_message =
          message_int32_message.GetReflection()->GetInt32(
              message_int32_message, fd_map_int32_foreign_message_key);
      const ForeignMessage& value_int32_message =
          DownCastMessage<ForeignMessage>(
              message_int32_message.GetReflection()->GetMessage(
                  message_int32_message, fd_map_int32_foreign_message_value));
      EXPECT_EQ(value_int32_message.c(), Func(key_int32_message, 6));
    }

    {
      // Check gets through mutable objects.
      const Message& message_int32_int32 =
          mmf_int32_int32.Get(i, entry_int32_int32.get());
      int32_t key_int32_int32 = message_int32_int32.GetReflection()->GetInt32(
          message_int32_int32, fd_map_int32_in32_key);
      int32_t value_int32_int32 = message_int32_int32.GetReflection()->GetInt32(
          message_int32_int32, fd_map_int32_in32_value);
      EXPECT_EQ(value_int32_int32, Func(key_int32_int32, 1));

      const Message& message_int32_double =
          mmf_int32_double.Get(i, entry_int32_double.get());
      int32_t key_int32_double = message_int32_double.GetReflection()->GetInt32(
          message_int32_double, fd_map_int32_double_key);
      double value_int32_double =
          message_int32_double.GetReflection()->GetDouble(
              message_int32_double, fd_map_int32_double_value);
      EXPECT_EQ(value_int32_double, Func(key_int32_double, 2));

      const Message& message_string_string =
          mmf_string_string.Get(i, entry_string_string.get());
      std::string key_string_string =
          message_string_string.GetReflection()->GetString(
              message_string_string, fd_map_string_string_key);
      std::string value_string_string =
          message_string_string.GetReflection()->GetString(
              message_string_string, fd_map_string_string_value);
      EXPECT_EQ(value_string_string, StrFunc(Int(key_string_string), 5));

      const Message& message_int32_message =
          mmf_int32_foreign_message.Get(i, entry_int32_foreign_message.get());
      int32_t key_int32_message =
          message_int32_message.GetReflection()->GetInt32(
              message_int32_message, fd_map_int32_foreign_message_key);
      const ForeignMessage& value_int32_message =
          DownCastMessage<ForeignMessage>(
              message_int32_message.GetReflection()->GetMessage(
                  message_int32_message, fd_map_int32_foreign_message_value));
      EXPECT_EQ(value_int32_message.c(), Func(key_int32_message, 6));
    }
  }

  // Make sure we can do sets through the RepeatedFieldRef objects.
  for (int i = 0; i < 10; i++) {
    const Message& message_int32_int32 =
        mmf_int32_int32.Get(i, entry_int32_int32.get());
    int key = message_int32_int32.GetReflection()->GetInt32(
        message_int32_int32, fd_map_int32_in32_key);

    entry_int32_int32->GetReflection()->SetInt32(
        entry_int32_int32.get(), fd_map_int32_int32->message_type()->field(0),
        key);
    entry_int32_int32->GetReflection()->SetInt32(
        entry_int32_int32.get(), fd_map_int32_int32->message_type()->field(1),
        Func(key, -1));
    entry_int32_double->GetReflection()->SetInt32(
        entry_int32_double.get(), fd_map_int32_double->message_type()->field(0),
        key);
    entry_int32_double->GetReflection()->SetDouble(
        entry_int32_double.get(), fd_map_int32_double->message_type()->field(1),
        Func(key, -2));
    entry_string_string->GetReflection()->SetString(
        entry_string_string.get(),
        fd_map_string_string->message_type()->field(0), StrFunc(key, 1));
    entry_string_string->GetReflection()->SetString(
        entry_string_string.get(),
        fd_map_string_string->message_type()->field(1), StrFunc(key, -5));
    entry_int32_foreign_message->GetReflection()->SetInt32(
        entry_int32_foreign_message.get(),
        fd_map_int32_foreign_message->message_type()->field(0), key);
    Message* value_message =
        entry_int32_foreign_message->GetReflection()->MutableMessage(
            entry_int32_foreign_message.get(),
            fd_map_int32_foreign_message->message_type()->field(1));
    value_message->GetReflection()->SetInt32(
        value_message, value_message->GetDescriptor()->FindFieldByName("c"),
        Func(key, -6));

    mmf_int32_int32.Set(i, *entry_int32_int32);
    mmf_int32_double.Set(i, *entry_int32_double);
    mmf_string_string.Set(i, *entry_string_string);
    mmf_int32_foreign_message.Set(i, *entry_int32_foreign_message);
  }

  for (int i = 0; i < 10; i++) {
    EXPECT_EQ(Func(i, -1), message.map_int32_int32().at(i));
    EXPECT_EQ(Func(i, -2), message.map_int32_double().at(i));
    EXPECT_EQ(StrFunc(i, -5), message.map_string_string().at(StrFunc(i, 1)));
    EXPECT_EQ(Func(i, -6), message.map_int32_foreign_message().at(i).c());
  }

  // Test iterators.
  {
    int index = 0;
    absl::flat_hash_map<int32_t, int32_t> result;
    for (const auto& message : mf_int32_int32) {
      int32_t key =
          message.GetReflection()->GetInt32(message, fd_map_int32_in32_key);
      int32_t value =
          message.GetReflection()->GetInt32(message, fd_map_int32_in32_value);
      result[key] = value;
      ++index;
    }
    EXPECT_EQ(10, index);
    for (const auto& kv : result) {
      EXPECT_EQ(message.map_int32_int32().at(kv.first), kv.second);
    }
  }

  {
    int index = 0;
    absl::flat_hash_map<int32_t, double> result;
    for (const auto& message : mf_int32_double) {
      int32_t key =
          message.GetReflection()->GetInt32(message, fd_map_int32_double_key);
      double value = message.GetReflection()->GetDouble(
          message, fd_map_int32_double_value);
      result[key] = value;
      ++index;
    }
    EXPECT_EQ(10, index);
    for (const auto& kv : result) {
      EXPECT_EQ(message.map_int32_double().at(kv.first), kv.second);
    }
  }

  {
    int index = 0;
    absl::flat_hash_map<std::string, std::string> result;
    for (const auto& message : mf_string_string) {
      std::string key =
          message.GetReflection()->GetString(message, fd_map_string_string_key);
      std::string value = message.GetReflection()->GetString(
          message, fd_map_string_string_value);
      result[key] = value;
      ++index;
    }
    EXPECT_EQ(10, index);
    for (const auto& kv : result) {
      EXPECT_EQ(message.map_string_string().at(kv.first), kv.second);
    }
  }

  {
    int index = 0;
    absl::flat_hash_map<int32_t, ForeignMessage> result;
    for (RepeatedFieldRef<Message>::iterator it =
             mf_int32_foreign_message.begin();
         it != mf_int32_foreign_message.end(); ++it) {
      const Message& message = *it;
      int32_t key = message.GetReflection()->GetInt32(
          message, fd_map_int32_foreign_message_key);
      const ForeignMessage& sub_message =
          DownCastMessage<ForeignMessage>(message.GetReflection()->GetMessage(
              message, fd_map_int32_foreign_message_value));
      result[key].MergeFrom(sub_message);
      ++index;
    }
    EXPECT_EQ(10, index);
    for (const auto& e : result) {
      EXPECT_EQ(message.map_int32_foreign_message().at(e.first).c(),
                e.second.c());
    }
  }

  // Test MutableRepeatedFieldRef::Add()
  entry_int32_int32->GetReflection()->SetInt32(
      entry_int32_int32.get(), fd_map_int32_int32->message_type()->field(0),
      4321);
  entry_int32_int32->GetReflection()->SetInt32(
      entry_int32_int32.get(), fd_map_int32_int32->message_type()->field(1),
      1234);
  mmf_int32_int32.Add(*entry_int32_int32);
  EXPECT_EQ(1234, message.map_int32_int32().at(4321));

  entry_int32_double->GetReflection()->SetInt32(
      entry_int32_double.get(), fd_map_int32_double->message_type()->field(0),
      4321);
  entry_int32_double->GetReflection()->SetDouble(
      entry_int32_double.get(), fd_map_int32_double->message_type()->field(1),
      1234.0);
  mmf_int32_double.Add(*entry_int32_double);
  EXPECT_EQ(1234.0, message.map_int32_double().at(4321));

  entry_string_string->GetReflection()->SetString(
      entry_string_string.get(), fd_map_string_string->message_type()->field(0),
      "4321");
  entry_string_string->GetReflection()->SetString(
      entry_string_string.get(), fd_map_string_string->message_type()->field(1),
      "1234");
  mmf_string_string.Add(*entry_string_string);
  EXPECT_EQ("1234", message.map_string_string().at("4321"));

  entry_int32_foreign_message->GetReflection()->SetInt32(
      entry_int32_foreign_message.get(),
      fd_map_int32_foreign_message->message_type()->field(0), 4321);
  Message* value_message =
      entry_int32_foreign_message->GetReflection()->MutableMessage(
          entry_int32_foreign_message.get(),
          fd_map_int32_foreign_message->message_type()->field(1));
  ForeignMessage foreign_message;
  foreign_message.set_c(1234);
  value_message->CopyFrom(foreign_message);

  mmf_int32_foreign_message.Add(*entry_int32_foreign_message);
  EXPECT_EQ(1234, message.map_int32_foreign_message().at(4321).c());

  // Test Reflection::AddAllocatedMessage
  Message* free_entry_string_string =
      MessageFactory::generated_factory()
          ->GetPrototype(fd_map_string_string->message_type())
          ->New();
  entry_string_string->GetReflection()->SetString(
      free_entry_string_string, fd_map_string_string->message_type()->field(0),
      "4321");
  entry_string_string->GetReflection()->SetString(
      free_entry_string_string, fd_map_string_string->message_type()->field(1),
      "1234");
  refl->AddAllocatedMessage(&message, fd_map_string_string,
                            free_entry_string_string);

  // Test MutableRepeatedFieldRef::RemoveLast()
  mmf_int32_int32.RemoveLast();
  mmf_int32_double.RemoveLast();
  mmf_string_string.RemoveLast();
  mmf_int32_foreign_message.RemoveLast();
  EXPECT_EQ(10, message.map_int32_int32().size());
  EXPECT_EQ(10, message.map_int32_double().size());
  EXPECT_EQ(11, message.map_string_string().size());
  EXPECT_EQ(10, message.map_int32_foreign_message().size());

  // Test MutableRepeatedFieldRef::SwapElements()
  {
    const Message& message0a = mmf_int32_int32.Get(0, entry_int32_int32.get());
    int32_t int32_value0a =
        message0a.GetReflection()->GetInt32(message0a, fd_map_int32_in32_value);
    const Message& message9a = mmf_int32_int32.Get(9, entry_int32_int32.get());
    int32_t int32_value9a =
        message9a.GetReflection()->GetInt32(message9a, fd_map_int32_in32_value);

    mmf_int32_int32.SwapElements(0, 9);

    const Message& message0b = mmf_int32_int32.Get(0, entry_int32_int32.get());
    int32_t int32_value0b =
        message0b.GetReflection()->GetInt32(message0b, fd_map_int32_in32_value);
    const Message& message9b = mmf_int32_int32.Get(9, entry_int32_int32.get());
    int32_t int32_value9b =
        message9b.GetReflection()->GetInt32(message9b, fd_map_int32_in32_value);

    EXPECT_EQ(int32_value9a, int32_value0b);
    EXPECT_EQ(int32_value0a, int32_value9b);
  }

  {
    const Message& message0a =
        mmf_int32_double.Get(0, entry_int32_double.get());
    double double_value0a = message0a.GetReflection()->GetDouble(
        message0a, fd_map_int32_double_value);
    const Message& message9a =
        mmf_int32_double.Get(9, entry_int32_double.get());
    double double_value9a = message9a.GetReflection()->GetDouble(
        message9a, fd_map_int32_double_value);

    mmf_int32_double.SwapElements(0, 9);

    const Message& message0b =
        mmf_int32_double.Get(0, entry_int32_double.get());
    double double_value0b = message0b.GetReflection()->GetDouble(
        message0b, fd_map_int32_double_value);
    const Message& message9b =
        mmf_int32_double.Get(9, entry_int32_double.get());
    double double_value9b = message9b.GetReflection()->GetDouble(
        message9b, fd_map_int32_double_value);

    EXPECT_EQ(double_value9a, double_value0b);
    EXPECT_EQ(double_value0a, double_value9b);
  }

  {
    const Message& message0a =
        mmf_string_string.Get(0, entry_string_string.get());
    std::string string_value0a = message0a.GetReflection()->GetString(
        message0a, fd_map_string_string_value);
    const Message& message9a =
        mmf_string_string.Get(9, entry_string_string.get());
    std::string string_value9a = message9a.GetReflection()->GetString(
        message9a, fd_map_string_string_value);

    mmf_string_string.SwapElements(0, 9);

    const Message& message0b =
        mmf_string_string.Get(0, entry_string_string.get());
    std::string string_value0b = message0b.GetReflection()->GetString(
        message0b, fd_map_string_string_value);
    const Message& message9b =
        mmf_string_string.Get(9, entry_string_string.get());
    std::string string_value9b = message9b.GetReflection()->GetString(
        message9b, fd_map_string_string_value);

    EXPECT_EQ(string_value9a, string_value0b);
    EXPECT_EQ(string_value0a, string_value9b);
  }

  {
    const Message& message0a =
        mmf_int32_foreign_message.Get(0, entry_int32_foreign_message.get());
    const ForeignMessage& sub_message0a =
        DownCastMessage<ForeignMessage>(message0a.GetReflection()->GetMessage(
            message0a, fd_map_int32_foreign_message_value));
    int32_t int32_value0a = sub_message0a.c();
    const Message& message9a =
        mmf_int32_foreign_message.Get(9, entry_int32_foreign_message.get());
    const ForeignMessage& sub_message9a =
        DownCastMessage<ForeignMessage>(message9a.GetReflection()->GetMessage(
            message9a, fd_map_int32_foreign_message_value));
    int32_t int32_value9a = sub_message9a.c();

    mmf_int32_foreign_message.SwapElements(0, 9);

    const Message& message0b =
        mmf_int32_foreign_message.Get(0, entry_int32_foreign_message.get());
    const ForeignMessage& sub_message0b =
        DownCastMessage<ForeignMessage>(message0b.GetReflection()->GetMessage(
            message0b, fd_map_int32_foreign_message_value));
    int32_t int32_value0b = sub_message0b.c();
    const Message& message9b =
        mmf_int32_foreign_message.Get(9, entry_int32_foreign_message.get());
    const ForeignMessage& sub_message9b =
        DownCastMessage<ForeignMessage>(message9b.GetReflection()->GetMessage(
            message9b, fd_map_int32_foreign_message_value));
    int32_t int32_value9b = sub_message9b.c();

    EXPECT_EQ(int32_value9a, int32_value0b);
    EXPECT_EQ(int32_value0a, int32_value9b);
  }

  // TODO: After supporting arena agnostic delete or let map entry
  // handle heap allocation, this could be removed.
  if (message.GetArena() != nullptr) {
    entry_int32_int32.release();
    entry_int32_double.release();
    entry_string_string.release();
    entry_int32_foreign_message.release();
  }
}

TEST_F(MapFieldReflectionTest, RepeatedFieldRefMergeFromAndSwap) {
  // Set-up message content.
  TestMap m0, m1, m2;
  for (int i = 0; i < 10; ++i) {
    (*m0.mutable_map_int32_int32())[i] = Func(i, 1);
    (*m0.mutable_map_int32_double())[i] = Func(i, 2);
    (*m0.mutable_map_string_string())[StrFunc(i, 1)] = StrFunc(i, 5);
    (*m0.mutable_map_int32_foreign_message())[i].set_c(Func(i, 6));
    (*m1.mutable_map_int32_int32())[i + 10] = Func(i, 11);
    (*m1.mutable_map_int32_double())[i + 10] = Func(i, 12);
    (*m1.mutable_map_string_string())[StrFunc(i + 10, 1)] = StrFunc(i, 15);
    (*m1.mutable_map_int32_foreign_message())[i + 10].set_c(Func(i, 16));
    (*m2.mutable_map_int32_int32())[i + 20] = Func(i, 21);
    (*m2.mutable_map_int32_double())[i + 20] = Func(i, 22);
    (*m2.mutable_map_string_string())[StrFunc(i + 20, 1)] = StrFunc(i, 25);
    (*m2.mutable_map_int32_foreign_message())[i + 20].set_c(Func(i, 26));
  }

  const Reflection* refl = m0.GetReflection();
  const Descriptor* desc = m0.GetDescriptor();

  // Get FieldDescriptors for all the fields of interest.
  const FieldDescriptor* fd_map_int32_int32 =
      desc->FindFieldByName("map_int32_int32");
  const FieldDescriptor* fd_map_int32_double =
      desc->FindFieldByName("map_int32_double");
  const FieldDescriptor* fd_map_string_string =
      desc->FindFieldByName("map_string_string");
  const FieldDescriptor* fd_map_int32_foreign_message =
      desc->FindFieldByName("map_int32_foreign_message");

  // Get MutableRepeatedFieldRef objects for all fields of interest.
  const MutableRepeatedFieldRef<Message> mmf_int32_int32 =
      refl->GetMutableRepeatedFieldRef<Message>(&m0, fd_map_int32_int32);
  const MutableRepeatedFieldRef<Message> mmf_int32_double =
      refl->GetMutableRepeatedFieldRef<Message>(&m0, fd_map_int32_double);
  const MutableRepeatedFieldRef<Message> mmf_string_string =
      refl->GetMutableRepeatedFieldRef<Message>(&m0, fd_map_string_string);
  const MutableRepeatedFieldRef<Message> mmf_int32_foreign_message =
      refl->GetMutableRepeatedFieldRef<Message>(&m0,
                                                fd_map_int32_foreign_message);

  // Test MutableRepeatedRef::CopyFrom
  mmf_int32_int32.CopyFrom(
      refl->GetRepeatedFieldRef<Message>(m1, fd_map_int32_int32));
  mmf_int32_double.CopyFrom(
      refl->GetRepeatedFieldRef<Message>(m1, fd_map_int32_double));
  mmf_string_string.CopyFrom(
      refl->GetRepeatedFieldRef<Message>(m1, fd_map_string_string));
  mmf_int32_foreign_message.CopyFrom(
      refl->GetRepeatedFieldRef<Message>(m1, fd_map_int32_foreign_message));

  for (int i = 0; i < 10; ++i) {
    EXPECT_EQ(Func(i, 11), m0.map_int32_int32().at(i + 10));
    EXPECT_EQ(Func(i, 12), m0.map_int32_double().at(i + 10));
    EXPECT_EQ(StrFunc(i, 15), m0.map_string_string().at(StrFunc(i + 10, 1)));
    EXPECT_EQ(Func(i, 16), m0.map_int32_foreign_message().at(i + 10).c());
  }

  // Test MutableRepeatedRef::MergeFrom
  mmf_int32_int32.MergeFrom(
      refl->GetRepeatedFieldRef<Message>(m2, fd_map_int32_int32));
  mmf_int32_double.MergeFrom(
      refl->GetRepeatedFieldRef<Message>(m2, fd_map_int32_double));
  mmf_string_string.MergeFrom(
      refl->GetRepeatedFieldRef<Message>(m2, fd_map_string_string));
  mmf_int32_foreign_message.MergeFrom(
      refl->GetRepeatedFieldRef<Message>(m2, fd_map_int32_foreign_message));
  for (int i = 0; i < 10; ++i) {
    EXPECT_EQ(Func(i, 21), m0.map_int32_int32().at(i + 20));
    EXPECT_EQ(Func(i, 22), m0.map_int32_double().at(i + 20));
    EXPECT_EQ(StrFunc(i, 25), m0.map_string_string().at(StrFunc(i + 20, 1)));
    EXPECT_EQ(Func(i, 26), m0.map_int32_foreign_message().at(i + 20).c());
  }

  // Test MutableRepeatedRef::Swap
  // Swap between m0 and m2.
  mmf_int32_int32.Swap(
      refl->GetMutableRepeatedFieldRef<Message>(&m2, fd_map_int32_int32));
  mmf_int32_double.Swap(
      refl->GetMutableRepeatedFieldRef<Message>(&m2, fd_map_int32_double));
  mmf_string_string.Swap(
      refl->GetMutableRepeatedFieldRef<Message>(&m2, fd_map_string_string));
  mmf_int32_foreign_message.Swap(refl->GetMutableRepeatedFieldRef<Message>(
      &m2, fd_map_int32_foreign_message));
  for (int i = 0; i < 10; ++i) {
    // Check the content of m0.
    EXPECT_EQ(Func(i, 21), m0.map_int32_int32().at(i + 20));
    EXPECT_EQ(Func(i, 22), m0.map_int32_double().at(i + 20));
    EXPECT_EQ(StrFunc(i, 25), m0.map_string_string().at(StrFunc(i + 20, 1)));
    EXPECT_EQ(Func(i, 26), m0.map_int32_foreign_message().at(i + 20).c());

    // Check the content of m2.
    EXPECT_EQ(Func(i, 11), m2.map_int32_int32().at(i + 10));
    EXPECT_EQ(Func(i, 12), m2.map_int32_double().at(i + 10));
    EXPECT_EQ(StrFunc(i, 15), m2.map_string_string().at(StrFunc(i + 10, 1)));
    EXPECT_EQ(Func(i, 16), m2.map_int32_foreign_message().at(i + 10).c());
    EXPECT_EQ(Func(i, 21), m2.map_int32_int32().at(i + 20));
    EXPECT_EQ(Func(i, 22), m2.map_int32_double().at(i + 20));
    EXPECT_EQ(StrFunc(i, 25), m2.map_string_string().at(StrFunc(i + 20, 1)));
    EXPECT_EQ(Func(i, 26), m2.map_int32_foreign_message().at(i + 20).c());
  }

  // TODO: add test for duplicated key
}

TEST_F(MapFieldReflectionTest, MapSizeWithDuplicatedKey) {
  // Dynamic Message
  {
    DynamicMessageFactory factory;
    std::unique_ptr<Message> message(
        factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
    const Reflection* reflection = message->GetReflection();
    const FieldDescriptor* field =
        UNITTEST::TestMap::descriptor()->FindFieldByName("map_int32_int32");

    Message* entry1 = reflection->AddMessage(message.get(), field);
    Message* entry2 = reflection->AddMessage(message.get(), field);

    const Reflection* entry_reflection = entry1->GetReflection();
    const FieldDescriptor* key_field = entry1->GetDescriptor()->map_key();
    entry_reflection->SetInt32(entry1, key_field, 1);
    entry_reflection->SetInt32(entry2, key_field, 1);

    EXPECT_EQ(2, reflection->FieldSize(*message, field));
    EXPECT_EQ(1, MapSize(reflection, field, *message));
    EXPECT_EQ(2, reflection->FieldSize(*message, field));
  }

  // Generated Message
  {
    UNITTEST::TestMap message;
    const Reflection* reflection = message.GetReflection();
    const FieldDescriptor* field =
        message.GetDescriptor()->FindFieldByName("map_int32_int32");

    Message* entry1 = reflection->AddMessage(&message, field);
    Message* entry2 = reflection->AddMessage(&message, field);

    const Reflection* entry_reflection = entry1->GetReflection();
    const FieldDescriptor* key_field = entry1->GetDescriptor()->map_key();
    entry_reflection->SetInt32(entry1, key_field, 1);
    entry_reflection->SetInt32(entry2, key_field, 1);

    EXPECT_EQ(2, reflection->FieldSize(message, field));
    EXPECT_EQ(1, MapSize(reflection, field, message));
  }
}

TEST_F(MapFieldReflectionTest, UninitializedEntry) {
  UNITTEST::TestRequiredMessageMap message;
  const Reflection* reflection = message.GetReflection();
  const FieldDescriptor* field =
      message.GetDescriptor()->FindFieldByName("map_field");
  auto entry = reflection->AddMessage(&message, field);
  EXPECT_FALSE(entry->IsInitialized());
  EXPECT_FALSE(message.IsInitialized());
}

class MyMapEntry
    : public internal::MapEntry<::int32_t, ::int32_t,
                                internal::WireFormatLite::TYPE_INT32,
                                internal::WireFormatLite::TYPE_INT32> {
 public:
  constexpr MyMapEntry()
      : MyMapEntry::MapEntry(static_cast<ClassData*>(nullptr)) {}
  MyMapEntry(Arena* arena) : MyMapEntry::MapEntry(arena, nullptr) {}
  const ClassData* GetClassData() const {
    ABSL_CHECK(false);
    return nullptr;
  }
  static bool ValidateKey(void*) { return true; }
  static bool ValidateValue(void*) { return true; }
};

TEST(MapEntryTest, ConstInit) {
  // This verifies that `MapEntry` can be constant initialized.
  PROTOBUF_CONSTINIT static MyMapEntry entry{};
  // Use the object in some way to make sure the vtable is there.
  EXPECT_NE("", absl::StrFormat("%p", &entry));
}

// Generated Message Test ===========================================

TEST(GeneratedMapFieldTest, Accessors) {
  UNITTEST::TestMap message;

  MapTestUtil::SetMapFields(&message);
  MapTestUtil::ExpectMapFieldsSet(message);

  MapTestUtil::ModifyMapFields(&message);
  MapTestUtil::ExpectMapFieldsModified(message);
}

TEST(GeneratedMapFieldTest, SetMapFieldsInitialized) {
  UNITTEST::TestMap message;

  MapTestUtil::SetMapFieldsInitialized(&message);
  MapTestUtil::ExpectMapFieldsSetInitialized(message);
}

TEST(GeneratedMapFieldTest, Proto2SetMapFieldsInitialized) {
  UNITTEST::TestEnumMap message;
  EXPECT_EQ(UNITTEST::PROTO2_MAP_ENUM_FOO,
            (*message.mutable_known_map_field())[0]);
}

TEST(GeneratedMapFieldTest, Clear) {
  UNITTEST::TestMap message;

  MapTestUtil::SetMapFields(&message);
  message.Clear();
  MapTestUtil::ExpectClear(message);
}

TEST(GeneratedMapFieldTest, ClearMessageMap) {
  UNITTEST::TestMessageMap message;

  // Creates a TestAllTypes with default value
  TestUtil::ExpectClear((*message.mutable_map_int32_message())[0]);
}

TEST(GeneratedMapFieldTest, CopyFrom) {
  UNITTEST::TestMap message1, message2;

  MapTestUtil::SetMapFields(&message1);
  message2.CopyFrom(message1);
  MapTestUtil::ExpectMapFieldsSet(message2);

  // Copying from self should be a no-op.
  message2.CopyFrom(message2);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(GeneratedMapFieldTest, CopyFromMessageMap) {
  UNITTEST::TestMessageMap message1, message2;

  (*message1.mutable_map_int32_message())[0].add_repeated_int32(100);
  (*message2.mutable_map_int32_message())[0].add_repeated_int32(101);

  message1.CopyFrom(message2);

  // Checks repeated field is overwritten.
  EXPECT_EQ(1, message1.map_int32_message().at(0).repeated_int32_size());
  EXPECT_EQ(101, message1.map_int32_message().at(0).repeated_int32(0));
}

TEST(GeneratedMapFieldTest, SwapWithEmpty) {
  UNITTEST::TestMap message1, message2;

  MapTestUtil::SetMapFields(&message1);
  MapTestUtil::ExpectMapFieldsSet(message1);
  MapTestUtil::ExpectClear(message2);

  message1.Swap(&message2);
  MapTestUtil::ExpectMapFieldsSet(message2);
  MapTestUtil::ExpectClear(message1);
}

TEST(GeneratedMapFieldTest, SwapWithSelf) {
  UNITTEST::TestMap message;

  MapTestUtil::SetMapFields(&message);
  MapTestUtil::ExpectMapFieldsSet(message);

  message.Swap(&message);
  MapTestUtil::ExpectMapFieldsSet(message);
}

TEST(GeneratedMapFieldTest, SwapWithOther) {
  UNITTEST::TestMap message1, message2;

  MapTestUtil::SetMapFields(&message1);
  MapTestUtil::SetMapFields(&message2);
  MapTestUtil::ModifyMapFields(&message2);

  message1.Swap(&message2);
  MapTestUtil::ExpectMapFieldsModified(message1);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(GeneratedMapFieldTest, CopyConstructor) {
  UNITTEST::TestMap message1;
  MapTestUtil::SetMapFields(&message1);

  UNITTEST::TestMap message2(message1);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(GeneratedMapFieldTest, CopyAssignmentOperator) {
  UNITTEST::TestMap message1;
  MapTestUtil::SetMapFields(&message1);

  UNITTEST::TestMap message2;
  message2 = message1;
  MapTestUtil::ExpectMapFieldsSet(message2);

  // Make sure that self-assignment does something sane.
  message2.operator=(message2);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

#if !defined(PROTOBUF_TEST_NO_DESCRIPTORS) || PROTOBUF_RTTI
TEST(GeneratedMapFieldTest, UpcastCopyFrom) {
  // Test the CopyFrom method that takes in the generic const Message&
  // parameter.
  UNITTEST::TestMap message1, message2;

  MapTestUtil::SetMapFields(&message1);

  const Message* source = absl::implicit_cast<const Message*>(&message1);
  message2.CopyFrom(*source);

  MapTestUtil::ExpectMapFieldsSet(message2);
}
#endif

#ifndef PROTOBUF_TEST_NO_DESCRIPTORS

TEST(GeneratedMapFieldTest, CopyFromDynamicMessage) {
  // Test copying from a DynamicMessage, which must fall back to using
  // reflection.
  UNITTEST::TestMap message2;

  // Construct a new version of the dynamic message via the factory.
  DynamicMessageFactory factory;
  std::unique_ptr<Message> message1;
  message1.reset(factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaReflection(message1.get());
  reflection_tester.ExpectMapFieldsSetViaReflection(*message1);
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(message1.get());
  message2.CopyFrom(*message1);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(GeneratedMapFieldTest, CopyFromDynamicMessageMapReflection) {
  UNITTEST::TestMap message2;

  // Construct a new version of the dynamic message via the factory.
  DynamicMessageFactory factory;
  std::unique_ptr<Message> message1;
  message1.reset(factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaMapReflection(message1.get());
  reflection_tester.ExpectMapFieldsSetViaReflection(*message1);
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(message1.get());
  message2.CopyFrom(*message1);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(GeneratedMapFieldTest, DynamicMessageMergeFromDynamicMessage) {
  // Construct two dynamic message and sets via map reflection.
  DynamicMessageFactory factory;
  std::unique_ptr<Message> message1;
  message1.reset(factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaMapReflection(message1.get());

  // message2 is created by same factory.
  std::unique_ptr<Message> message2;
  message2.reset(factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  reflection_tester.SetMapFieldsViaMapReflection(message2.get());

  // message3 is created by different factory.
  DynamicMessageFactory factory3;
  std::unique_ptr<Message> message3;
  message3.reset(factory3.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  reflection_tester.SetMapFieldsViaMapReflection(message3.get());

  message2->MergeFrom(*message1);
  message3->MergeFrom(*message1);

  // Test MergeFrom does not sync to repeated fields and
  // there is no duplicate keys in text format.
  std::string output1, output2, output3;
  TextFormat::PrintToString(*message1, &output1);
  TextFormat::PrintToString(*message2, &output2);
  TextFormat::PrintToString(*message3, &output3);
  EXPECT_EQ(output1, output2);
  EXPECT_EQ(output1, output3);
}

TEST(GeneratedMapFieldTest, DynamicMessageCopyFrom) {
  // Test copying to a DynamicMessage, which must fall back to using reflection.
  UNITTEST::TestMap message2;
  MapTestUtil::SetMapFields(&message2);

  // Construct a new version of the dynamic message via the factory.
  DynamicMessageFactory factory;
  std::unique_ptr<Message> message1;
  message1.reset(factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());

  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  message1->MergeFrom(message2);
  reflection_tester.ExpectMapFieldsSetViaReflection(*message1);
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(message1.get());
}

TEST(GeneratedMapFieldTest, DynamicMessageCopyFromMapReflection) {
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  UNITTEST::TestMap message2;
  reflection_tester.SetMapFieldsViaMapReflection(&message2);

  // Construct a dynamic message via the factory.
  DynamicMessageFactory factory;
  std::unique_ptr<Message> message1;
  message1.reset(factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());

  message1->MergeFrom(message2);
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(message1.get());
  reflection_tester.ExpectMapFieldsSetViaReflection(*message1);
}

TEST(GeneratedMapFieldTest, SyncDynamicMapWithRepeatedField) {
  // Construct a dynamic message via the factory.
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  DynamicMessageFactory factory;
  std::unique_ptr<Message> message;
  message.reset(factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  reflection_tester.SetMapFieldsViaReflection(message.get());
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(message.get());
  reflection_tester.ExpectMapFieldsSetViaReflection(*message);
}

#endif  // !PROTOBUF_TEST_NO_DESCRIPTORS

TEST(GeneratedMapFieldTest, NonEmptyMergeFrom) {
  UNITTEST::TestMap message1, message2;

  MapTestUtil::SetMapFields(&message1);

  // This field will test merging into an empty spot.
  (*message2.mutable_map_int32_int32())[1] = 1;
  message1.mutable_map_int32_int32()->erase(1);

  // This tests overwriting.
  (*message2.mutable_map_int32_double())[1] = 1;
  (*message1.mutable_map_int32_double())[1] = 2;

  message1.MergeFrom(message2);
  MapTestUtil::ExpectMapFieldsSet(message1);

  // Test reflection MergeFrom does not sync to repeated field
  // and there is no duplicated keys.
  MapTestUtil::SetMapFields(&message1);
  MapTestUtil::SetMapFields(&message2);

  message2.MergeFrom(message1);

  std::string output1, output2;
  TextFormat::PrintToString(message1, &output1);
  TextFormat::PrintToString(message2, &output2);
  EXPECT_EQ(output1, output2);
}

TEST(GeneratedMapFieldTest, MergeFromMessageMap) {
  UNITTEST::TestMessageMap message1, message2;

  (*message1.mutable_map_int32_message())[0].add_repeated_int32(100);
  (*message2.mutable_map_int32_message())[0].add_repeated_int32(101);

  message1.MergeFrom(message2);

  // Checks repeated field is overwritten.
  EXPECT_EQ(1, message1.map_int32_message().at(0).repeated_int32_size());
  EXPECT_EQ(101, message1.map_int32_message().at(0).repeated_int32(0));
}

// Test the generated SerializeWithCachedSizesToArray()
TEST(GeneratedMapFieldTest, SerializationToArray) {
  UNITTEST::TestMap message1, message2;
  std::string data;
  MapTestUtil::SetMapFields(&message1);
  size_t size = message1.ByteSizeLong();
  data.resize(size);
  uint8_t* start = reinterpret_cast<uint8_t*>(&data[0]);
  uint8_t* end = message1.SerializeWithCachedSizesToArray(start);
  EXPECT_EQ(size, end - start);
  EXPECT_TRUE(message2.ParseFromString(data));
  MapTestUtil::ExpectMapFieldsSet(message2);
}

// Test the generated SerializeWithCachedSizes()
TEST(GeneratedMapFieldTest, SerializationToStream) {
  UNITTEST::TestMap message1, message2;
  MapTestUtil::SetMapFields(&message1);
  size_t size = message1.ByteSizeLong();
  std::string data;
  data.resize(size);
  {
    // Allow the output stream to buffer only one byte at a time.
    io::ArrayOutputStream array_stream(&data[0], size, 1);
    io::CodedOutputStream output_stream(&array_stream);
    message1.SerializeWithCachedSizes(&output_stream);
    EXPECT_FALSE(output_stream.HadError());
    EXPECT_EQ(size, output_stream.ByteCount());
  }
  EXPECT_TRUE(message2.ParseFromString(data));
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(GeneratedMapFieldTest, ParseFailsIfMalformed) {
  UNITTEST::TestMapSubmessage o, p;
  auto m = o.mutable_test_map()->mutable_map_int32_foreign_message();
  (*m)[0].set_c(-1);
  std::string serialized;
  EXPECT_TRUE(o.SerializeToString(&serialized));

  // Should parse correctly.
  EXPECT_TRUE(p.ParseFromString(serialized));

  // Overwriting the last byte to 0xFF results in malformed wire.
  serialized[serialized.size() - 1] = 0xFF;
  EXPECT_FALSE(p.ParseFromString(serialized));
}


TEST(GeneratedMapFieldTest, SameTypeMaps) {
  const Descriptor* map1 = UNITTEST::TestSameTypeMap::descriptor()
                               ->FindFieldByName("map1")
                               ->message_type();
  const Descriptor* map2 = UNITTEST::TestSameTypeMap::descriptor()
                               ->FindFieldByName("map2")
                               ->message_type();

  const Message* map1_entry =
      MessageFactory::generated_factory()->GetPrototype(map1);
  const Message* map2_entry =
      MessageFactory::generated_factory()->GetPrototype(map2);

  EXPECT_EQ(map1, map1_entry->GetDescriptor());
  EXPECT_EQ(map2, map2_entry->GetDescriptor());
}

TEST(GeneratedMapFieldTest, Proto2UnknownEnum) {
  UNITTEST::TestEnumMapPlusExtra from;
  (*from.mutable_known_map_field())[0] = UNITTEST::E_PROTO2_MAP_ENUM_FOO;
  (*from.mutable_unknown_map_field())[0] = UNITTEST::E_PROTO2_MAP_ENUM_EXTRA;
  std::string data;
  from.SerializeToString(&data);

  UNITTEST::TestEnumMap to;
  EXPECT_TRUE(to.ParseFromString(data));
  EXPECT_EQ(0, to.unknown_map_field().size());
  EXPECT_EQ(1, to.GetReflection()->GetUnknownFields(to).field_count());
  EXPECT_EQ(1, to.known_map_field().size());
  EXPECT_EQ(UNITTEST::PROTO2_MAP_ENUM_FOO, to.known_map_field().at(0));

  data.clear();
  from.Clear();
  to.SerializeToString(&data);
  EXPECT_TRUE(from.ParseFromString(data));
  EXPECT_EQ(0, from.GetReflection()->GetUnknownFields(from).field_count());
  EXPECT_EQ(1, from.known_map_field().size());
  EXPECT_EQ(UNITTEST::E_PROTO2_MAP_ENUM_FOO, from.known_map_field().at(0));
  EXPECT_EQ(1, from.unknown_map_field().size());
  EXPECT_EQ(UNITTEST::E_PROTO2_MAP_ENUM_EXTRA, from.unknown_map_field().at(0));

  // Test the same behavior with the reflection based parser.
  to.Clear();
  const char* ptr;
  internal::ParseContext ctx(io::CodedInputStream::GetDefaultRecursionLimit(),
                             false, &ptr, data);
  ptr = WireFormat::_InternalParse(&to, ptr, &ctx);
  ASSERT_TRUE(ptr);
  ASSERT_TRUE(ctx.EndedAtLimit());

  EXPECT_EQ(0, to.unknown_map_field().size());
  EXPECT_EQ(1, to.GetReflection()->GetUnknownFields(to).field_count());
  EXPECT_EQ(1, to.known_map_field().size());
  EXPECT_EQ(UNITTEST::PROTO2_MAP_ENUM_FOO, to.known_map_field().at(0));
}

TEST(GeneratedMapFieldTest, Proto2UnknownEnumThrowsAwayUnknownData) {
  UNITTEST::TestEnumMap to;
  // 101: {
  //   1: 2
  //   2: 3     # this is an unknown enum. It should be kept.
  //   3: 120   # this is an extra field that will be discarded.
  // }
  constexpr absl::string_view data = "\252\006\006\010\002\020\003\030x";
  // Same as above, but without the extra field.
  constexpr absl::string_view expected = "\252\006\004\010\002\020\003";
  ASSERT_TRUE(to.ParseFromString(data));
  EXPECT_EQ(expected, to.SerializeAsString());

  // Test the same behavior with the reflection based parser.
  to.Clear();
  const char* ptr;
  internal::ParseContext ctx(io::CodedInputStream::GetDefaultRecursionLimit(),
                             false, &ptr, data);
  ptr = WireFormat::_InternalParse(&to, ptr, &ctx);
  ASSERT_TRUE(ptr);
  ASSERT_TRUE(ctx.EndedAtLimit());
  EXPECT_EQ(expected, to.SerializeAsString());
}

TEST(GeneratedMapFieldTest, Proto2UnknownEnumAllKeyTypesWork) {
#define PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(Type, ...)                        \
  ABSL_LOG(INFO) << "Testing " << #Type;                                      \
  for (auto value : {__VA_ARGS__}) {                                          \
    UNITTEST::TestEnumMapPlusExtra from;                                      \
    (*from.mutable_unknown_map_field_##Type())[value] =                       \
        UNITTEST::E_PROTO2_MAP_ENUM_EXTRA;                                    \
    UNITTEST::TestEnumMap to;                                                 \
    ASSERT_TRUE(to.ParseFromString(from.SerializeAsString()));                \
    EXPECT_EQ(0, to.unknown_map_field_##Type().size())                        \
        << testing::PrintToString(to.unknown_map_field_##Type());             \
    const UnknownFieldSet& unknown_field_set =                                \
        to.GetReflection()->GetUnknownFields(to);                             \
    EXPECT_EQ(1, unknown_field_set.field_count());                            \
    from.Clear();                                                             \
    EXPECT_EQ(from.unknown_map_field_##Type().size(), 0);                     \
    EXPECT_TRUE(from.ParseFromString(to.SerializeAsString()));                \
    EXPECT_THAT(from.unknown_map_field_##Type(),                              \
                ElementsAre(Pair(value, UNITTEST::E_PROTO2_MAP_ENUM_EXTRA))); \
    EXPECT_EQ(from.SerializeAsString(), to.SerializeAsString());              \
  }

  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(int64, int64_t{17},
                                      std::numeric_limits<int64_t>::min(),
                                      std::numeric_limits<int64_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(uint64, uint64_t{17},
                                      std::numeric_limits<uint64_t>::min(),
                                      std::numeric_limits<uint64_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(int32, int32_t{17},
                                      std::numeric_limits<int32_t>::min(),
                                      std::numeric_limits<int32_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(uint32, uint32_t{17},
                                      std::numeric_limits<uint32_t>::min(),
                                      std::numeric_limits<uint32_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(fixed32, uint32_t{17},
                                      std::numeric_limits<uint32_t>::min(),
                                      std::numeric_limits<uint32_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(fixed64, uint64_t{17},
                                      std::numeric_limits<uint64_t>::min(),
                                      std::numeric_limits<uint64_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(bool, true);
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(string, "17");
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(sint32, int32_t{17},
                                      std::numeric_limits<int32_t>::min(),
                                      std::numeric_limits<int32_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(sint64, int64_t{17},
                                      std::numeric_limits<int64_t>::min(),
                                      std::numeric_limits<int64_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(sfixed32, int32_t{17},
                                      std::numeric_limits<int32_t>::min(),
                                      std::numeric_limits<int32_t>::max());
  PROTOBUF_INTERNAL_TEST_MAP_KEY_TYPE(sfixed64, int64_t{17},
                                      std::numeric_limits<int64_t>::min(),
                                      std::numeric_limits<int64_t>::max());
}

TEST(GeneratedMapFieldTest, StandardWireFormat) {
  UNITTEST::TestMap message;
  std::string data = "\x0A\x04\x08\x01\x10\x01";

  EXPECT_TRUE(message.ParseFromString(data));
  EXPECT_EQ(1, message.map_int32_int32().size());
  EXPECT_EQ(1, message.map_int32_int32().at(1));
}

TEST(GeneratedMapFieldTest, UnorderedWireFormat) {
  UNITTEST::TestMap message;

  // put value before key in wire format
  std::string data = "\x0A\x04\x10\x01\x08\x02";

  EXPECT_TRUE(message.ParseFromString(data));
  EXPECT_EQ(1, message.map_int32_int32().size());
  ASSERT_NE(message.map_int32_int32().find(2), message.map_int32_int32().end());
  EXPECT_EQ(1, message.map_int32_int32().at(2));
}

TEST(GeneratedMapFieldTest, DuplicatedKeyWireFormat) {
  UNITTEST::TestMap message;

  // Two key fields in wire format
  std::string data = "\x0A\x06\x08\x01\x08\x02\x10\x01";

  EXPECT_TRUE(message.ParseFromString(data));
  EXPECT_EQ(1, message.map_int32_int32().size());
  EXPECT_EQ(1, message.map_int32_int32().at(2));

  // A similar test, but with a map from int to a message type.
  // Again, we want to be sure that the "second one wins" when
  // there are two separate entries with the same key.
  const int key = 99;
  UNITTEST::TestRequiredMessageMap map_message;
  UNITTEST::TestRequired with_dummy4;
  with_dummy4.set_a(0);
  with_dummy4.set_b(0);
  with_dummy4.set_c(0);
  with_dummy4.set_dummy4(11);
  (*map_message.mutable_map_field())[key] = with_dummy4;
  std::string s = map_message.SerializeAsString();
  UNITTEST::TestRequired with_dummy5;
  with_dummy5.set_a(0);
  with_dummy5.set_b(0);
  with_dummy5.set_c(0);
  with_dummy5.set_dummy5(12);
  (*map_message.mutable_map_field())[key] = with_dummy5;
  std::string both = s + map_message.SerializeAsString();
  // We don't expect a merge now.  The "second one wins."
  ASSERT_TRUE(map_message.ParseFromString(both));
  ASSERT_EQ(1, map_message.map_field().size());
  ASSERT_EQ(1, map_message.map_field().count(key));
  EXPECT_EQ(0, map_message.map_field().find(key)->second.a());
  EXPECT_EQ(0, map_message.map_field().find(key)->second.b());
  EXPECT_EQ(0, map_message.map_field().find(key)->second.c());
  EXPECT_FALSE(map_message.map_field().find(key)->second.has_dummy4());
  ASSERT_TRUE(map_message.map_field().find(key)->second.has_dummy5());
  EXPECT_EQ(12, map_message.map_field().find(key)->second.dummy5());
}

// Exhaustive combinations of keys, values, and junk in any order.
// This re-tests some of the things tested above, but if it fails
// it's more work to determine what went wrong, so it isn't necessarily
// bad that we have the simpler tests too.
TEST(GeneratedMapFieldTest, KeysValuesUnknownsWireFormat) {
  UNITTEST::TestMap message;
  const int kMaxNumKeysAndValuesAndJunk = 4;
  const char kKeyTag = 0x08;
  const char kValueTag = 0x10;
  const char kJunkTag = 0x20;
  for (int items = 0; items <= kMaxNumKeysAndValuesAndJunk; items++) {
    std::string data = "\x0A";
    // Encode length of what will follow.
    data.push_back(items * 2);
    static const int kBitsOfIPerItem = 4;
    static const int mask = (1 << kBitsOfIPerItem) - 1;
    // Each iteration of the following is a test.  It uses i as bit vector
    // encoding the keys and values to put in the wire format.
    for (int i = 0; i < (1 << (items * kBitsOfIPerItem)); i++) {
      std::string wire_format = data;
      int expected_key = 0;
      int expected_value = 0;
      for (int k = i, j = 0; j < items; j++, k >>= kBitsOfIPerItem) {
        bool is_key = k & 0x1;
        bool is_value = !is_key && (k & 0x2);
        wire_format.push_back(is_key     ? kKeyTag
                              : is_value ? kValueTag
                                         : kJunkTag);
        char c = static_cast<char>(k & mask) >> 2;  // One char after the tag.
        wire_format.push_back(c);
        if (is_key) expected_key = static_cast<int>(c);
        if (is_value) expected_value = static_cast<int>(c);
        bool res = message.ParseFromString(wire_format);
        bool expect_success = true;
        // Unfortunately the old map parser accepts malformed input, the new
        // parser accepts only correct input.
        if (j != items - 1) expect_success = false;
        if (expect_success) {
          ASSERT_TRUE(res);
          ASSERT_EQ(1, message.map_int32_int32().size());
          ASSERT_EQ(expected_key, message.map_int32_int32().begin()->first);
          ASSERT_EQ(expected_value, message.map_int32_int32().begin()->second);
        } else {
          ASSERT_FALSE(res);
        }
      }
    }
  }
}

TEST(GeneratedMapFieldTest, DuplicatedValueWireFormat) {
  UNITTEST::TestMap message;

  // Two value fields in wire format
  std::string data = "\x0A\x06\x08\x01\x10\x01\x10\x02";

  EXPECT_TRUE(message.ParseFromString(data));
  EXPECT_EQ(1, message.map_int32_int32().size());
  EXPECT_EQ(2, message.map_int32_int32().at(1));
}

TEST(GeneratedMapFieldTest, MissedKeyWireFormat) {
  UNITTEST::TestMap message;

  // No key field in wire format
  std::string data = "\x0A\x02\x10\x01";

  EXPECT_TRUE(message.ParseFromString(data));
  EXPECT_EQ(1, message.map_int32_int32().size());
  ASSERT_NE(message.map_int32_int32().find(0), message.map_int32_int32().end());
  EXPECT_EQ(1, message.map_int32_int32().at(0));
}

TEST(GeneratedMapFieldTest, MissedValueWireFormat) {
  UNITTEST::TestMap message;

  // No value field in wire format
  std::string data = "\x0A\x02\x08\x01";

  EXPECT_TRUE(message.ParseFromString(data));
  EXPECT_EQ(1, message.map_int32_int32().size());
  ASSERT_NE(message.map_int32_int32().find(1), message.map_int32_int32().end());
  EXPECT_EQ(0, message.map_int32_int32().at(1));
}

TEST(GeneratedMapFieldTest, MissedValueTextFormat) {
  UNITTEST::TestMap message;

  // No value field in text format
  std::string text =
      "map_int32_foreign_message {\n"
      "  key: 1234567890\n"
      "}";

  EXPECT_TRUE(TextFormat::ParseFromString(text, &message));
  EXPECT_EQ(1, message.map_int32_foreign_message().size());
  EXPECT_EQ(11, message.ByteSizeLong());
}

TEST(GeneratedMapFieldTest, UnknownFieldWireFormat) {
  UNITTEST::TestMap message;

  // Unknown field in wire format
  std::string data = "\x0A\x06\x08\x02\x10\x03\x18\x01";

  EXPECT_TRUE(message.ParseFromString(data));
  EXPECT_EQ(1, message.map_int32_int32().size());
  EXPECT_EQ(3, message.map_int32_int32().at(2));
}

TEST(GeneratedMapFieldTest, ToplevelTagNotLengthPrefixed) {
  UNITTEST::TestMap message;

  // The toplevel matches a field number, but does not match the wire type.
  std::string data = "\x08\x81\x04";
  EXPECT_TRUE(message.ParseFromString(data));
  EXPECT_EQ(0, message.map_int32_int32().size());

  const UnknownFieldSet& unknown_field_set =
      message.GetReflection()->GetUnknownFields(message);
  ASSERT_EQ(1, unknown_field_set.field_count());
  auto& field = unknown_field_set.field(0);
  ASSERT_EQ(field.TYPE_VARINT, field.type());
  EXPECT_EQ(((0x1 << 0) | (0x04 << 7)), field.varint());
}

TEST(GeneratedMapFieldTest, InnerTagsInLongForm) {
  UNITTEST::TestMap message;

  // First, control
  absl::string_view data("\012\004\010\007\020\005", 6);
  ASSERT_TRUE(message.ParseFromString(data));
  EXPECT_THAT(message.map_int32_int32(), ElementsAre(Pair(7, 5)));

  // Now we make the key tag long form
  data = absl::string_view("\012\005\210\000\007\020\005", 7);
  ASSERT_TRUE(message.ParseFromString(data));
  EXPECT_THAT(message.map_int32_int32(), ElementsAre(Pair(7, 5)));

  // Now we make the value tag long form
  data = absl::string_view("\012\005\010\007\220\000\005", 7);
  ASSERT_TRUE(message.ParseFromString(data));
  EXPECT_THAT(message.map_int32_int32(), ElementsAre(Pair(7, 5)));
}

TEST(GeneratedMapFieldTest, CorruptedWireFormat) {
  UNITTEST::TestMap message;

  // corrupted data in wire format
  std::string data = "\x0A\x06\x08\x02\x11\x03";

  EXPECT_FALSE(message.ParseFromString(data));
}

TEST(GeneratedMapFieldTest, IsInitialized) {
  UNITTEST::TestRequiredMessageMap map_message;

  // Add an uninitialized message.
  (*map_message.mutable_map_field())[0];
  EXPECT_FALSE(map_message.IsInitialized());

  // Initialize uninitialized message
  (*map_message.mutable_map_field())[0].set_a(0);
  (*map_message.mutable_map_field())[0].set_b(0);
  (*map_message.mutable_map_field())[0].set_c(0);
  EXPECT_TRUE(map_message.IsInitialized());
}

TEST(GeneratedMapFieldTest, SpaceUsed) {
  UNITTEST::TestRequiredMessageMap map_message;
  const size_t initial = map_message.SpaceUsed();
  const size_t space_used_message = UNITTEST::TestRequired().SpaceUsed();

  auto& m = *map_message.mutable_map_field();
  constexpr int kNumValues = 100;
  for (int i = 0; i < kNumValues; ++i) {
    m[i];
  }

  // The exact value will depend on internal state, like collisions,
  // so we can't predict it. But we can predict a lower bound.
  size_t lower_bound =
      initial + kNumValues * (space_used_message + sizeof(int32_t) +
                              /* Node::next */ sizeof(void*) +
                              /* table entry */ sizeof(void*));

  EXPECT_LE(lower_bound, map_message.SpaceUsed());
}

TEST(GeneratedMapFieldTest, MessagesMustMerge) {
  UNITTEST::TestRequiredMessageMap map_message;

  UNITTEST::TestRequired with_dummy4;
  with_dummy4.set_a(97);
  with_dummy4.set_b(91);
  with_dummy4.set_dummy4(98);
  EXPECT_FALSE(with_dummy4.IsInitialized());
  (*map_message.mutable_map_field())[0] = with_dummy4;
  EXPECT_FALSE(map_message.IsInitialized());

  UNITTEST::TestRequired with_dummy5;
  with_dummy5.set_b(0);
  with_dummy5.set_c(33);
  with_dummy5.set_dummy5(99);
  EXPECT_FALSE(with_dummy5.IsInitialized());
  (*map_message.mutable_map_field())[0] = with_dummy5;
  EXPECT_FALSE(map_message.IsInitialized());

  // The wire format of MapEntry is straightforward (*) and can be manually
  // constructed to force merging of two uninitialized messages that would
  // result in an initialized message.
  //
  // (*) http://google3/net/proto2/internal/map_test.cc?l=2433&rcl=310012028
  std::string dummy4_s = with_dummy4.SerializePartialAsString();
  std::string dummy5_s = with_dummy5.SerializePartialAsString();
  int payload_size = dummy4_s.size() + dummy5_s.size();
  // Makes sure the payload size fits into one byte.
  ASSERT_LT(payload_size, 128);

  std::string s(6, 0);
  char* p = &s[0];
  *p++ = WireFormatLite::MakeTag(1, WireFormatLite::WIRETYPE_LENGTH_DELIMITED);
  // Length: 2B for key tag & val and 2B for val tag and length of the following
  // payload.
  *p++ = 4 + payload_size;
  *p++ = WireFormatLite::MakeTag(1, WireFormatLite::WIRETYPE_VARINT);
  *p++ = 0;
  *p++ = WireFormatLite::MakeTag(2, WireFormatLite::WIRETYPE_LENGTH_DELIMITED);
  *p++ = payload_size;
  absl::StrAppend(&s, dummy4_s, dummy5_s);

  // Test key then value then value.
  int key = 0;
  ASSERT_TRUE(map_message.ParseFromString(s));
  ASSERT_EQ(1, map_message.map_field().size());
  ASSERT_EQ(1, map_message.map_field().count(key));
  EXPECT_EQ(97, map_message.map_field().find(key)->second.a());
  EXPECT_EQ(0, map_message.map_field().find(key)->second.b());
  EXPECT_EQ(33, map_message.map_field().find(key)->second.c());
  EXPECT_EQ(98, map_message.map_field().find(key)->second.dummy4());
  EXPECT_EQ(99, map_message.map_field().find(key)->second.dummy5());

  // Test key then value then value then key.
  s.push_back(s[2]);  // Copy the key's tag.
  key = 19;
  s.push_back(key);  // Second key is 19 instead of 0.
  s[1] += 2;         // Adjust encoded size.
  ASSERT_TRUE(map_message.ParseFromString(s));
  ASSERT_EQ(1, map_message.map_field().size());
  ASSERT_EQ(1, map_message.map_field().count(key));
  EXPECT_EQ(97, map_message.map_field().find(key)->second.a());
  EXPECT_EQ(0, map_message.map_field().find(key)->second.b());
  EXPECT_EQ(33, map_message.map_field().find(key)->second.c());
  EXPECT_EQ(98, map_message.map_field().find(key)->second.dummy4());
  EXPECT_EQ(99, map_message.map_field().find(key)->second.dummy5());
}

// Generated Message Reflection Test ================================

TEST(GeneratedMapFieldReflectionTest, SpaceUsed) {
  UNITTEST::TestMap message;
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaReflection(&message);

  EXPECT_LT(0, message.GetReflection()->SpaceUsedLong(message));
}

TEST(GeneratedMapFieldReflectionTest, Accessors) {
  // Set every field to a unique value then go back and check all those
  // values.
  UNITTEST::TestMap message;
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaReflection(&message);
  MapTestUtil::ExpectMapFieldsSet(message);
  reflection_tester.ExpectMapFieldsSetViaReflection(message);
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(&message);

  reflection_tester.ModifyMapFieldsViaReflection(&message);
  MapTestUtil::ExpectMapFieldsModified(message);
}

TEST(GeneratedMapFieldReflectionTest,
     AccessingTheDefaultInstanceWithReflectionDoesNotModifyIt) {
  const auto& msg = UNITTEST::TestMap::default_instance();
  const auto clone_bytes = [&] {
    return std::string(reinterpret_cast<const char*>(&msg), sizeof(msg));
  };
  const auto before = clone_bytes();
  const auto& rep = msg.GetReflection()->GetRepeatedFieldRef<google::protobuf::Message>(
      msg, msg.GetDescriptor()->FindFieldByName("map_int32_int32"));
  EXPECT_THAT(rep, IsEmpty());
  const auto after = clone_bytes();
  EXPECT_EQ(before, after);
}

TEST(GeneratedMapFieldReflectionTest, Swap) {
  UNITTEST::TestMap message1;
  UNITTEST::TestMap message2;

  MapTestUtil::SetMapFields(&message1);

  const Reflection* reflection = message1.GetReflection();
  reflection->Swap(&message1, &message2);

  MapTestUtil::ExpectClear(message1);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(GeneratedMapFieldReflectionTest, SwapWithBothSet) {
  UNITTEST::TestMap message1;
  UNITTEST::TestMap message2;

  MapTestUtil::SetMapFields(&message1);
  MapTestUtil::SetMapFields(&message2);
  MapTestUtil::ModifyMapFields(&message2);

  const Reflection* reflection = message1.GetReflection();
  reflection->Swap(&message1, &message2);

  MapTestUtil::ExpectMapFieldsModified(message1);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(GeneratedMapFieldReflectionTest, SwapFields) {
  UNITTEST::TestMap message1;
  UNITTEST::TestMap message2;

  MapTestUtil::SetMapFields(&message2);

  std::vector<const FieldDescriptor*> fields;
  const Reflection* reflection = message1.GetReflection();
  reflection->ListFields(message2, &fields);
  reflection->SwapFields(&message1, &message2, fields);

  MapTestUtil::ExpectMapFieldsSet(message1);
  MapTestUtil::ExpectClear(message2);
}

TEST(GeneratedMapFieldReflectionTest, ClearField) {
  UNITTEST::TestMap message;
  MapTestUtil::SetMapFields(&message);
  MapTestUtil::ExpectMapFieldsSet(message);

  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.ClearMapFieldsViaReflection(&message);
  reflection_tester.ExpectClearViaReflection(message);
  reflection_tester.ExpectClearViaReflectionIterator(&message);
}

TEST(GeneratedMapFieldReflectionTest, RemoveLast) {
  UNITTEST::TestMap message;
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());

  MapTestUtil::SetMapFields(&message);
  MapTestUtil::ExpectMapsSize(message, 2);
  std::vector<const Message*> expected_entries =
      MapTestUtil::GetMapEntries(message, 0);

  reflection_tester.RemoveLastMapsViaReflection(&message);

  MapTestUtil::ExpectMapsSize(message, 1);
  std::vector<const Message*> remained_entries =
      MapTestUtil::GetMapEntries(message, 0);
  EXPECT_TRUE(expected_entries == remained_entries);
}

TEST(GeneratedMapFieldReflectionTest, ReleaseLast) {
  UNITTEST::TestMap message;
  const Descriptor* descriptor = message.GetDescriptor();
  MapReflectionTester reflection_tester(descriptor);

  MapTestUtil::SetMapFields(&message);

  MapTestUtil::ExpectMapsSize(message, 2);

  reflection_tester.ReleaseLastMapsViaReflection(&message);

  MapTestUtil::ExpectMapsSize(message, 1);

  // Now test that we actually release the right message.
  message.Clear();
  MapTestUtil::SetMapFields(&message);

  MapTestUtil::ExpectMapsSize(message, 2);
  std::vector<const Message*> expect_last =
      MapTestUtil::GetMapEntries(message, 1);
  std::vector<const Message*> release_last =
      MapTestUtil::GetMapEntriesFromRelease(&message);
  MapTestUtil::ExpectMapsSize(message, 1);
  if (!internal::DebugHardenForceCopyInRelease()) {
    EXPECT_TRUE(expect_last == release_last);
  }
  for (std::vector<const Message*>::iterator it = release_last.begin();
       it != release_last.end(); ++it) {
    delete *it;
  }
}

TEST(GeneratedMapFieldReflectionTest, SwapElements) {
  UNITTEST::TestMap message;
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());

  MapTestUtil::SetMapFields(&message);

  // Get pointers of map entries at their original position
  std::vector<const Message*> entries0 = MapTestUtil::GetMapEntries(message, 0);
  std::vector<const Message*> entries1 = MapTestUtil::GetMapEntries(message, 1);

  // Swap the first time.
  reflection_tester.SwapMapsViaReflection(&message);

  // Get pointer of map entry after swap once.
  std::vector<const Message*> entries0_once =
      MapTestUtil::GetMapEntries(message, 0);
  std::vector<const Message*> entries1_once =
      MapTestUtil::GetMapEntries(message, 1);

  // Test map entries are swapped.
  MapTestUtil::ExpectMapsSize(message, 2);
  EXPECT_TRUE(entries0 == entries1_once);
  EXPECT_TRUE(entries1 == entries0_once);

  // Swap the second time.
  reflection_tester.SwapMapsViaReflection(&message);

  // Get pointer of map entry after swap once.
  std::vector<const Message*> entries0_twice =
      MapTestUtil::GetMapEntries(message, 0);
  std::vector<const Message*> entries1_twice =
      MapTestUtil::GetMapEntries(message, 1);

  // Test map entries are swapped back.
  MapTestUtil::ExpectMapsSize(message, 2);
  EXPECT_TRUE(entries0 == entries0_twice);
  EXPECT_TRUE(entries1 == entries1_twice);
}

TEST(GeneratedMapFieldReflectionTest, MutableUnknownFields) {
  UNITTEST::TestMap message;
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.MutableUnknownFieldsOfMapFieldsViaReflection(&message);
}

TEST(GeneratedMapFieldReflectionTest, EmbedProto2Message) {
  UNITTEST::TestMessageMap message;

  const FieldDescriptor* map_field =
      UNITTEST::TestMessageMap::descriptor()->FindFieldByName(
          "map_int32_message");
  const FieldDescriptor* value = map_field->message_type()->map_value();

  Message* entry_message =
      message.GetReflection()->AddMessage(&message, map_field);
  EXPECT_EQ(
      &entry_message->GetReflection()->GetMessage(*entry_message, value),
      reinterpret_cast<const Message*>(&TestAllTypes::default_instance()));

  Message* proto2_message =
      entry_message->GetReflection()->MutableMessage(entry_message, value);
  EXPECT_EQ(UNITTEST::TestAllTypes::descriptor(),
            proto2_message->GetDescriptor());
  ASSERT_EQ(1, message.map_int32_message().size());
}

TEST(GeneratedMapFieldReflectionTest, MergeFromClearMapEntry) {
  UNITTEST::TestMap message;
  const FieldDescriptor* map_field =
      UNITTEST::TestMap::descriptor()->FindFieldByName("map_int32_int32");
  const FieldDescriptor* key = map_field->message_type()->map_key();
  const FieldDescriptor* value = map_field->message_type()->map_value();

  Message* entry_message1 =
      message.GetReflection()->AddMessage(&message, map_field);
  EXPECT_FALSE(entry_message1->GetReflection()->HasField(*entry_message1, key));
  EXPECT_FALSE(
      entry_message1->GetReflection()->HasField(*entry_message1, value));

  Message* entry_message2 =
      message.GetReflection()->AddMessage(&message, map_field);
  EXPECT_FALSE(entry_message2->GetReflection()->HasField(*entry_message2, key));
  EXPECT_FALSE(
      entry_message2->GetReflection()->HasField(*entry_message2, value));

  entry_message1->MergeFrom(*entry_message2);
  EXPECT_FALSE(entry_message1->GetReflection()->HasField(*entry_message1, key));
  EXPECT_FALSE(
      entry_message1->GetReflection()->HasField(*entry_message1, value));
}

TEST(GeneratedMapFieldReflectionTest, MapEntryClear) {
  UNITTEST::TestMap message;
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.MutableUnknownFieldsOfMapFieldsViaReflection(&message);
}

TEST(GeneratedMapFieldReflectionTest, Proto2MapEntryClear) {
  UNITTEST::TestEnumMap message;
  const Descriptor* descriptor = message.GetDescriptor();
  const FieldDescriptor* field_descriptor =
      descriptor->FindFieldByName("known_map_field");
  const FieldDescriptor* value_descriptor =
      field_descriptor->message_type()->map_value();
  Message* sub_message =
      message.GetReflection()->AddMessage(&message, field_descriptor);
  EXPECT_EQ(0, sub_message->GetReflection()->GetEnumValue(*sub_message,
                                                          value_descriptor));
}

// Map Reflection API Test =========================================

TEST(GeneratedMapFieldReflectionTest, SetViaMapReflection) {
  UNITTEST::TestMap message;
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaMapReflection(&message);
  reflection_tester.ExpectMapFieldsSetViaReflection(message);
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(&message);
}

// Dynamic Message Test =============================================

class MapFieldInDynamicMessageTest : public testing::Test {
 protected:
  const DescriptorPool* pool_;
  DynamicMessageFactory factory_;
  const Descriptor* map_descriptor_;
  const Descriptor* recursive_map_descriptor_;
  const Message* map_prototype_;

  MapFieldInDynamicMessageTest()
      : pool_(DescriptorPool::generated_pool()), factory_(pool_) {}

  void SetUp() override {
    map_descriptor_ = pool_->FindMessageTypeByName(
        absl::StrCat(UNITTEST_PACKAGE_NAME, ".TestMap"));
    recursive_map_descriptor_ = pool_->FindMessageTypeByName(
        absl::StrCat(UNITTEST_PACKAGE_NAME, ".TestRecursiveMapMessage"));
    ASSERT_TRUE(map_descriptor_ != nullptr);
    ASSERT_TRUE(recursive_map_descriptor_ != nullptr);
    map_prototype_ = factory_.GetPrototype(map_descriptor_);
  }
};

TEST_F(MapFieldInDynamicMessageTest, MapIndependentOffsets) {
  // Check that all fields have independent offsets by setting each
  // one to a unique value then checking that they all still have those
  // unique values (i.e. they don't stomp each other).
  std::unique_ptr<Message> message(map_prototype_->New());
  MapReflectionTester reflection_tester(map_descriptor_);

  reflection_tester.SetMapFieldsViaReflection(message.get());
  reflection_tester.ExpectMapFieldsSetViaReflection(*message);
}

TEST_F(MapFieldInDynamicMessageTest, DynamicMapReflection) {
  // Check that map fields work properly.
  std::unique_ptr<Message> message(map_prototype_->New());

  // Check set functions.
  MapReflectionTester reflection_tester(map_descriptor_);
  reflection_tester.SetMapFieldsViaMapReflection(message.get());
  reflection_tester.ExpectMapFieldsSetViaReflection(*message);
}

TEST_F(MapFieldInDynamicMessageTest, MapSpaceUsed) {
  // Test that SpaceUsedLong() works properly

  // Since we share the implementation with generated messages, we don't need
  // to test very much here.  Just make sure it appears to be working.

  std::unique_ptr<Message> message(map_prototype_->New());
  MapReflectionTester reflection_tester(map_descriptor_);

  int initial_space_used = message->SpaceUsedLong();

  reflection_tester.SetMapFieldsViaReflection(message.get());
  EXPECT_LT(initial_space_used, message->SpaceUsedLong());
}

TEST_F(MapFieldInDynamicMessageTest, RecursiveMap) {
  TestRecursiveMapMessage from;
  (*from.mutable_a())[""];
  std::string data = from.SerializeAsString();
  std::unique_ptr<Message> to(
      factory_.GetPrototype(recursive_map_descriptor_)->New());
  ASSERT_TRUE(to->ParseFromString(data));
}

TEST_F(MapFieldInDynamicMessageTest, MapValueReferernceValidAfterSerialize) {
  std::unique_ptr<Message> message(map_prototype_->New());
  MapReflectionTester reflection_tester(map_descriptor_);
  reflection_tester.SetMapFieldsViaMapReflection(message.get());

  // Get value reference before serialization, so that we know the value is from
  // map.
  MapKey map_key;
  MapValueRef map_val;
  map_key.SetInt32Value(0);
  reflection_tester.GetMapValueViaMapReflection(
      message.get(), "map_int32_foreign_message", map_key, &map_val);
  Message* submsg = map_val.MutableMessageValue();

  // In previous implementation, calling SerializeToString will cause syncing
  // from map to repeated field, which will invalidate the submsg we previously
  // got.
  std::string data;
  message->SerializeToString(&data);

  const Reflection* submsg_reflection = submsg->GetReflection();
  const Descriptor* submsg_desc = submsg->GetDescriptor();
  const FieldDescriptor* submsg_field = submsg_desc->FindFieldByName("c");
  submsg_reflection->SetInt32(submsg, submsg_field, 128);

  message->SerializeToString(&data);
  TestMap to;
  ASSERT_TRUE(to.ParseFromString(data));
  EXPECT_EQ(128, to.map_int32_foreign_message().at(0).c());
}

TEST_F(MapFieldInDynamicMessageTest, MapEntryReferernceValidAfterSerialize) {
  std::unique_ptr<Message> message(map_prototype_->New());
  MapReflectionTester reflection_tester(map_descriptor_);
  reflection_tester.SetMapFieldsViaReflection(message.get());

  // Get map entry before serialization, so that we know the it is from
  // repeated field.
  Message* map_entry = reflection_tester.GetMapEntryViaReflection(
      message.get(), "map_int32_foreign_message", 0);
  const Reflection* map_entry_reflection = map_entry->GetReflection();
  const Descriptor* map_entry_desc = map_entry->GetDescriptor();
  const FieldDescriptor* value_field = map_entry_desc->map_value();
  Message* submsg =
      map_entry_reflection->MutableMessage(map_entry, value_field);

  // In previous implementation, calling SerializeToString will cause syncing
  // from repeated field to map, which will invalidate the map_entry we
  // previously got.
  std::string data;
  message->SerializeToString(&data);

  const Reflection* submsg_reflection = submsg->GetReflection();
  const Descriptor* submsg_desc = submsg->GetDescriptor();
  const FieldDescriptor* submsg_field = submsg_desc->FindFieldByName("c");
  submsg_reflection->SetInt32(submsg, submsg_field, 128);

  message->SerializeToString(&data);
  TestMap to;
  to.ParseFromString(data);
  EXPECT_EQ(128, to.map_int32_foreign_message().at(0).c());
}

// ReflectionOps Test ===============================================

TEST(ReflectionOpsForMapFieldTest, MapSanityCheck) {
  UNITTEST::TestMap message;

  MapTestUtil::SetMapFields(&message);
  MapTestUtil::ExpectMapFieldsSet(message);
}

TEST(ReflectionOpsForMapFieldTest, MapCopy) {
  UNITTEST::TestMap message, message2;

  MapTestUtil::SetMapFields(&message);

  ReflectionOps::Copy(message, &message2);

  MapTestUtil::ExpectMapFieldsSet(message2);

  // Copying from self should be a no-op.
  ReflectionOps::Copy(message2, &message2);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(ReflectionOpsForMapFieldTest, MergeMap) {
  // Note:  Copy is implemented in terms of Merge() so technically the Copy
  //   test already tested most of this.

  UNITTEST::TestMap message, message2;

  MapTestUtil::SetMapFields(&message);

  ReflectionOps::Merge(message2, &message);

  MapTestUtil::ExpectMapFieldsSet(message);
}

TEST(ReflectionOpsForMapFieldTest, ClearMap) {
  UNITTEST::TestMap message;

  MapTestUtil::SetMapFields(&message);

  ReflectionOps::Clear(&message);

  MapTestUtil::ExpectClear(message);
}

TEST(ReflectionOpsForMapFieldTest, MapDiscardUnknownFields) {
  UNITTEST::TestMap message;
  MapTestUtil::SetMapFields(&message);

  // Set some unknown fields in message.
  message.GetReflection()->MutableUnknownFields(&message)->AddVarint(123456,
                                                                     654321);

  // Discard them.
  ReflectionOps::DiscardUnknownFields(&message);
  MapTestUtil::ExpectMapFieldsSet(message);

  EXPECT_EQ(0,
            message.GetReflection()->GetUnknownFields(message).field_count());
}

TEST(ReflectionOpsForMapFieldTest, IsInitialized) {
  UNITTEST::TestRequiredMessageMap map_message;

  // Add an uninitialized message.
  (*map_message.mutable_map_field())[0];
  EXPECT_FALSE(ReflectionOps::IsInitialized(map_message));

  // Initialize uninitialized message
  (*map_message.mutable_map_field())[0].set_a(0);
  (*map_message.mutable_map_field())[0].set_b(0);
  (*map_message.mutable_map_field())[0].set_c(0);
  EXPECT_TRUE(ReflectionOps::IsInitialized(map_message));
}

// Wire Format Test =================================================

TEST(WireFormatForMapFieldTest, ParseMap) {
  UNITTEST::TestMap source, dest;
  std::string data;

  // Serialize using the generated code.
  MapTestUtil::SetMapFields(&source);
  source.SerializeToString(&data);

  // Parse using WireFormat.
  io::ArrayInputStream raw_input(data.data(), data.size());
  io::CodedInputStream input(&raw_input);
  WireFormat::ParseAndMergePartial(&input, &dest);

  // Check.
  MapTestUtil::ExpectMapFieldsSet(dest);
}

TEST(WireFormatForMapFieldTest, MapByteSize) {
  UNITTEST::TestMap message;
  MapTestUtil::SetMapFields(&message);

  EXPECT_EQ(message.ByteSizeLong(), WireFormat::ByteSize(message));
  message.Clear();
  EXPECT_EQ(0, message.ByteSizeLong());
  EXPECT_EQ(0, WireFormat::ByteSize(message));
}

TEST(WireFormatForMapFieldTest, SerializeMap) {
  UNITTEST::TestMap message;
  std::string generated_data;
  std::string dynamic_data;

  MapTestUtil::SetMapFields(&message);

  // Serialize using the generated code.
  {
    message.ByteSizeLong();
    io::StringOutputStream raw_output(&generated_data);
    io::CodedOutputStream output(&raw_output);
    message.SerializeWithCachedSizes(&output);
    ASSERT_FALSE(output.HadError());
  }

  // Serialize using WireFormat.
  {
    io::StringOutputStream raw_output(&dynamic_data);
    io::CodedOutputStream output(&raw_output);
    size_t size = WireFormat::ByteSize(message);
    WireFormat::SerializeWithCachedSizes(message, size, &output);
    ASSERT_FALSE(output.HadError());
  }

  // Should parse to the same message.
  EXPECT_TRUE(TestUtil::EqualsToSerialized(message, generated_data));
  EXPECT_TRUE(TestUtil::EqualsToSerialized(message, dynamic_data));
}

TEST(WireFormatForMapFieldTest, SerializeMapDynamicMessage) {
  DynamicMessageFactory factory;
  std::unique_ptr<Message> dynamic_message;
  dynamic_message.reset(
      factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaReflection(dynamic_message.get());
  reflection_tester.ExpectMapFieldsSetViaReflection(*dynamic_message);

  UNITTEST::TestMap generated_message;
  MapTestUtil::SetMapFields(&generated_message);
  MapTestUtil::ExpectMapFieldsSet(generated_message);

  std::string generated_data;
  std::string dynamic_data;

  // Serialize.
  generated_message.SerializeToString(&generated_data);
  dynamic_message->SerializeToString(&dynamic_data);

  // Because map serialization doesn't guarantee order, we just compare
  // serialized size here. This is enough to tell dynamic message doesn't miss
  // anything in serialization.
  EXPECT_TRUE(dynamic_data.size() == generated_data.size());
}

TEST(WireFormatForMapFieldTest, MapByteSizeDynamicMessage) {
  DynamicMessageFactory factory;
  std::unique_ptr<Message> dynamic_message;
  dynamic_message.reset(
      factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaReflection(dynamic_message.get());
  reflection_tester.ExpectMapFieldsSetViaReflection(*dynamic_message);
  std::string expected_serialized_data;
  dynamic_message->SerializeToString(&expected_serialized_data);
  int expected_size = expected_serialized_data.size();
  EXPECT_EQ(dynamic_message->ByteSizeLong(), expected_size);
  TestMap expected_message;
  expected_message.ParseFromString(expected_serialized_data);

  std::unique_ptr<Message> message2;
  message2.reset(factory.GetPrototype(UNITTEST::TestMap::descriptor())->New());
  reflection_tester.SetMapFieldsViaMapReflection(message2.get());

  const FieldDescriptor* field =
      UNITTEST::TestMap::descriptor()->FindFieldByName("map_int32_int32");
  const Reflection* reflection = dynamic_message->GetReflection();

  // Force the map field to mark with STATE_MODIFIED_REPEATED
  reflection->RemoveLast(dynamic_message.get(), field);
  dynamic_message->MergeFrom(*message2);
  dynamic_message->MergeFrom(*message2);
  // The map field is marked as STATE_MODIFIED_REPEATED, ByteSizeLong() will use
  // repeated field which have duplicate keys to calculate.
  size_t duplicate_size = dynamic_message->ByteSizeLong();
  EXPECT_TRUE(duplicate_size > expected_size);
  std::string duplicate_serialized_data;
  dynamic_message->SerializeToString(&duplicate_serialized_data);
  EXPECT_EQ(dynamic_message->ByteSizeLong(), duplicate_serialized_data.size());

  // Force the map field to mark with map CLEAN
  auto& msg = *dynamic_message;
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_int32_int32"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_int32_int32"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_int64_int64"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_uint32_uint32"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_uint64_uint64"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_sint32_sint32"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_sint64_sint64"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_fixed32_fixed32"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_fixed64_fixed64"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_sfixed32_sfixed32"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_sfixed64_sfixed64"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_int32_float"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_int32_double"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_bool_bool"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_string_string"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_int32_bytes"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_int32_enum"), 2);
  EXPECT_EQ(reflection_tester.MapSize(msg, "map_int32_foreign_message"), 2);

  // The map field is marked as CLEAN, ByteSizeLong() will use map which do not
  // have duplicate keys to calculate.
  int size = dynamic_message->ByteSizeLong();
  EXPECT_EQ(expected_size, size);

  // Protobuf used to have a bug for serialize when map it marked CLEAN. It used
  // repeated field to calculate ByteSizeLong but use map to serialize the real
  // data, thus the ByteSizeLong may bigger than real serialized size. A crash
  // might be happen at SerializeToString(). Or an "unexpected end group"
  // warning was raised at parse back if user use SerializeWithCachedSizes()
  // which avoids size check at serialize.
  std::string serialized_data;
  dynamic_message->SerializeToString(&serialized_data);
  EXPECT_TRUE(dynamic_message->ParseFromString(serialized_data));
}

TEST(WireFormatForMapFieldTest, MapParseHelpers) {
  std::string data;

  {
    // Set up.
    UNITTEST::TestMap message;
    MapTestUtil::SetMapFields(&message);
    message.SerializeToString(&data);
  }

  {
    // Test ParseFromString.
    UNITTEST::TestMap message;
    EXPECT_TRUE(message.ParseFromString(data));
    MapTestUtil::ExpectMapFieldsSet(message);
  }

  {
    // Test ParseFromIstream.
    UNITTEST::TestMap message;
    std::stringstream stream(data);
    EXPECT_TRUE(message.ParseFromIstream(&stream));
    EXPECT_TRUE(stream.eof());
    MapTestUtil::ExpectMapFieldsSet(message);
  }

  {
    // Test ParseFromBoundedZeroCopyStream.
    std::string data_with_junk(data);
    data_with_junk.append("some junk on the end");
    io::ArrayInputStream stream(data_with_junk.data(), data_with_junk.size());
    UNITTEST::TestMap message;
    EXPECT_TRUE(message.ParseFromBoundedZeroCopyStream(&stream, data.size()));
    MapTestUtil::ExpectMapFieldsSet(message);
  }

  {
    // Test that ParseFromBoundedZeroCopyStream fails (but doesn't crash) if
    // EOF is reached before the expected number of bytes.
    io::ArrayInputStream stream(data.data(), data.size());
    UNITTEST::TestAllTypes message;
    EXPECT_FALSE(
        message.ParseFromBoundedZeroCopyStream(&stream, data.size() + 1));
  }
}

std::string WriteVarint(int number, uint64_t v) {
  uint8_t buf[16];
  return std::string(buf, WireFormatLite::WriteUInt64ToArray(number, v, buf));
}

std::string WriteString(int number, const std::string& str) {
  uint8_t buf[100];
  return std::string(buf, WireFormatLite::WriteStringToArray(number, str, buf));
}

TEST(WireFormatForMapFieldTest, BoolWorksWithOverlongValues) {
  // map<bool, bool> map_bool_bool = 13;
  for (uint64_t v : {uint64_t{1}, uint64_t{1000}, uint64_t{100000},
                     uint64_t{1} << 32, uint64_t{1} << 63}) {
    SCOPED_TRACE(v);
    std::string payload =
        WriteString(13, WriteVarint(1, v) + WriteVarint(2, v));
    UNITTEST::TestMap obj;
    ASSERT_TRUE(obj.ParseFromString(payload));
    EXPECT_THAT(obj.map_bool_bool(), ElementsAre(Pair(true, true)));

    io::ArrayInputStream raw_input(payload.data(), payload.size());
    io::CodedInputStream input(&raw_input);
    obj.Clear();
    ASSERT_TRUE(WireFormat::ParseAndMergePartial(&input, &obj));
    EXPECT_THAT(obj.map_bool_bool(), ElementsAre(Pair(true, true)));
  }
}

// Deterministic Serialization Test ==========================================

template <typename T>
static std::string DeterministicSerializationWithSerializePartialToCodedStream(
    const T& t) {
  const size_t size = t.ByteSizeLong();
  std::string result(size, '\0');
  io::ArrayOutputStream array_stream(&result[0], size);
  io::CodedOutputStream output_stream(&array_stream);
  output_stream.SetSerializationDeterministic(true);
  t.SerializePartialToCodedStream(&output_stream);
  EXPECT_FALSE(output_stream.HadError());
  EXPECT_EQ(size, output_stream.ByteCount());
  return result;
}

template <typename T>
static std::string DeterministicSerializationWithSerializeToCodedStream(
    const T& t) {
  const size_t size = t.ByteSizeLong();
  std::string result(size, '\0');
  io::ArrayOutputStream array_stream(&result[0], size);
  io::CodedOutputStream output_stream(&array_stream);
  output_stream.SetSerializationDeterministic(true);
  t.SerializeToCodedStream(&output_stream);
  EXPECT_FALSE(output_stream.HadError());
  EXPECT_EQ(size, output_stream.ByteCount());
  return result;
}

template <typename T>
static std::string DeterministicSerialization(const T& t) {
  const size_t size = t.ByteSizeLong();
  std::string result(size, '\0');
  io::ArrayOutputStream array_stream(&result[0], size);
  {
    io::CodedOutputStream output_stream(&array_stream);
    output_stream.SetSerializationDeterministic(true);
    t.SerializeWithCachedSizes(&output_stream);
    EXPECT_FALSE(output_stream.HadError());
    EXPECT_EQ(size, output_stream.ByteCount());
  }
  EXPECT_EQ(result, DeterministicSerializationWithSerializeToCodedStream(t));
  EXPECT_EQ(result,
            DeterministicSerializationWithSerializePartialToCodedStream(t));
  return result;
}

// Helper for MapSerializationTest.  Return a 7-bit ASCII string.
static std::string ConstructKey(uint64_t n) {
  std::string s(n % static_cast<uint64_t>(9), '\0');
  if (s.empty()) {
    return absl::StrCat(n);
  } else {
    while (n != 0) {
      s[n % s.size()] = (n >> 10) & 0x7f;
      n /= 888;
    }
    return s;
  }
}

TEST(MapSerializationTest, Deterministic) {
  const int kIters = 25;
  UNITTEST::TestMaps t;
  UNITTEST::TestIntIntMap inner;
  (*inner.mutable_m())[0] = (*inner.mutable_m())[10] =
      (*inner.mutable_m())[-200] = 0;
  uint64_t frog = 9;
  const uint64_t multiplier = 0xa29cd16f;
  for (int i = 0; i < kIters; i++) {
    const int32_t i32 = static_cast<int32_t>(frog & 0xffffffff);
    const uint32_t u32 = static_cast<uint32_t>(i32) * 91919;
    const int64_t i64 = static_cast<int64_t>(frog);
    const uint64_t u64 = frog * static_cast<uint64_t>(187321);
    const bool b = i32 > 0;
    const std::string s = ConstructKey(frog);
    (*inner.mutable_m())[i] = i32;
    (*t.mutable_m_int32())[i32] = (*t.mutable_m_sint32())[i32] =
        (*t.mutable_m_sfixed32())[i32] = inner;
    (*t.mutable_m_uint32())[u32] = (*t.mutable_m_fixed32())[u32] = inner;
    (*t.mutable_m_int64())[i64] = (*t.mutable_m_sint64())[i64] =
        (*t.mutable_m_sfixed64())[i64] = inner;
    (*t.mutable_m_uint64())[u64] = (*t.mutable_m_fixed64())[u64] = inner;
    (*t.mutable_m_bool())[b] = inner;
    (*t.mutable_m_string())[s] = inner;
    (*t.mutable_m_string())[s + std::string(
                                    1 << (u32 % static_cast<uint32_t>(9)), b)] =
        inner;
    inner.mutable_m()->erase(i);
    frog = frog * multiplier + i;
    frog ^= (frog >> 41);
  }

  // Verifies if two consecutive calls to deterministic serialization produce
  // the same bytes. Deterministic serialization means the same serialization
  // bytes in the same binary.
  const std::string s1 = DeterministicSerialization(t);
  const std::string s2 = DeterministicSerialization(t);
  EXPECT_EQ(s1, s2);

  UNITTEST::TestMaps u;
  EXPECT_TRUE(u.ParseFromString(s1));
  EXPECT_TRUE(util::MessageDifferencer::Equals(u, t));
}

static std::string GetGoldenMessageTextProto() {
  static std::string* golden_message_textproto = [] {
    std::string* textproto = new std::string;
    ABSL_CHECK_OK(File::GetContents(
        TestUtil::GetTestDataPath("google/protobuf/"
                                  "testdata/map_test_data.txt"),
        textproto, true));
    return textproto;
  }();
  return *golden_message_textproto;
}

static std::string GetGoldenMessageBinary() {
  static std::string* golden_message_binary = [] {
    UNITTEST::TestMaps t;
    TextFormat::Parser parser;
    parser.ParseFromString(GetGoldenMessageTextProto(), &t);

    std::string* result = new std::string;
    t.SerializeToString(result);
    return result;
  }();
  return *golden_message_binary;
}

TEST(MapSerializationTest, DeterministicSubmessage) {
  UNITTEST::TestSubmessageMaps p;
  UNITTEST::TestMaps t;
  t.ParseFromString(GetGoldenMessageBinary());
  *(p.mutable_m()) = t;
  std::vector<std::string> v;
  // Use multiple attempts to increase the chance of a failure if something is
  // buggy.  For example, each separate copy of a map might use a different
  // randomly-chosen hash function.
  const int kAttempts = 10;
  for (int i = 0; i < kAttempts; i++) {
    // NOLINTNEXTLINE(performance-unnecessary-copy-initialization)
    UNITTEST::TestSubmessageMaps q(p);
    ASSERT_EQ(DeterministicSerialization(q), DeterministicSerialization(p));
  }
}

// Text Format Test =================================================

TEST(TextFormatMapTest, SerializeAndParse) {
  UNITTEST::TestMap source;
  UNITTEST::TestMap dest;
  MapTestUtil::SetMapFields(&source);
  std::string output;

  // Test compact ASCII
  TextFormat::Printer printer;
  printer.PrintToString(source, &output);
  TextFormat::Parser parser;
  EXPECT_TRUE(parser.ParseFromString(output, &dest));
  MapTestUtil::ExpectMapFieldsSet(dest);
}

TEST(TextFormatMapTest, DynamicMessage) {
  TestMap prototype;
  DynamicMessageFactory factory;
  std::unique_ptr<Message> message(
      factory.GetPrototype(prototype.GetDescriptor())->New());
  MapReflectionTester tester(message->GetDescriptor());
  tester.SetMapFieldsViaReflection(message.get());

  std::string actual_text;
  TextFormat::PrintToString(*message, &actual_text);
  EXPECT_EQ(actual_text, GetGoldenMessageTextProto());
}

TEST(TextFormatMapTest, Sorted) {
  UNITTEST::TestMap message;
  MapReflectionTester tester(message.GetDescriptor());
  tester.SetMapFieldsViaReflection(&message);

  TextFormat::Printer printer;
  std::string actual_text;
  printer.PrintToString(message, &actual_text);
  EXPECT_EQ(actual_text, GetGoldenMessageTextProto());

  // Test again on the reverse order.
  UNITTEST::TestMap message2;
  tester.SetMapFieldsViaReflection(&message2);
  tester.SwapMapsViaReflection(&message2);
  printer.PrintToString(message2, &actual_text);
  EXPECT_EQ(actual_text, GetGoldenMessageTextProto());
}

// Previously, serializing to text format will disable iterator from generated
// API. Now, the iterator can be still used even after serializing to text
// format.
TEST(TextFormatMapTest, NoDisableIterator) {
  UNITTEST::TestMap source;
  (*source.mutable_map_int32_int32())[1] = 1;

  // Get iterator.
  Map<int32_t, int32_t>::iterator iter =
      source.mutable_map_int32_int32()->find(1);

  // Serialize message to text format, which will invalidate the previous
  // iterator previously.
  std::string output;
  TextFormat::Printer printer;
  printer.PrintToString(source, &output);

  // Modify map via the iterator (invalidated in previous implementation.).
  iter->second = 2;

  // In previous implementation, the new change won't be reflected in text
  // format, because the previous iterator has been invalidated.
  output.clear();
  printer.PrintToString(source, &output);
  std::string expected =
      "map_int32_int32 {\n"
      "  key: 1\n"
      "  value: 2\n"
      "}\n";
  EXPECT_EQ(output, expected);
}

// Previously, serializing to text format will disable iterator from reflection
// API.
TEST(TextFormatMapTest, NoDisableReflectionIterator) {
  UNITTEST::TestMap source;
  (*source.mutable_map_int32_int32())[1] = 1;

  // Get iterator. This will also sync internal repeated field with map inside
  // of MapField.
  const Reflection* reflection = source.GetReflection();
  const FieldDescriptor* field_desc =
      source.GetDescriptor()->FindFieldByName("map_int32_int32");
  RepeatedPtrField<Message>* map_field =
      reflection->MutableRepeatedPtrField<Message>(&source, field_desc);
  RepeatedPtrField<Message>::iterator iter = map_field->begin();

  // Serialize message to text format, which will invalidate the previous
  // iterator previously.
  std::string output;
  TextFormat::Printer printer;
  printer.PrintToString(source, &output);

  // Modify map via the iterator (invalidated in previous implementation.).
  const Reflection* map_entry_reflection = iter->GetReflection();
  const FieldDescriptor* value_field_desc = iter->GetDescriptor()->map_value();
  map_entry_reflection->SetInt32(&(*iter), value_field_desc, 2);
  ABSL_LOG(INFO) << iter->DebugString();

  // In previous implementation, the new change won't be reflected in text
  // format, because the previous iterator has been invalidated.
  output.clear();
  printer.PrintToString(source, &output);
  std::string expected =
      "map_int32_int32 {\n"
      "  key: 1\n"
      "  value: 2\n"
      "}\n";
  EXPECT_EQ(output, expected);
}

// arena support =================================================
TEST(ArenaTest, ParsingAndSerializingNoHeapAllocation) {
  // Allocate a large initial block to avoid mallocs during hooked test.
  std::vector<char> arena_block(128 * 1024);
  ArenaOptions options;
  options.initial_block = &arena_block[0];
  options.initial_block_size = arena_block.size();
  Arena arena(options);
  std::string data;
  data.reserve(128 * 1024);

  {
    // TODO: Enable no heap check when ArenaStringPtr is used in map.
    // NoHeapChecker no_heap;

    UNITTEST::TestArenaMap* from =
        Arena::Create<UNITTEST::TestArenaMap>(&arena);
    MapTestUtil::SetArenaMapFields(from);
    from->SerializeToString(&data);

    UNITTEST::TestArenaMap* to = Arena::Create<UNITTEST::TestArenaMap>(&arena);
    to->ParseFromString(data);
    MapTestUtil::ExpectArenaMapFieldsSet(*to);
  }
}

TEST(ArenaTest, SubmessageOnSameArena) {
  Arena arena;
  for (Arena* arena_to_use : {&arena, static_cast<Arena*>(nullptr)}) {
    ArenaHolder<UNITTEST::TestArenaMap> m(arena_to_use);
    auto* subm = &(*m->mutable_map_int32_foreign_message())[0];
    EXPECT_EQ(subm->GetArena(), arena_to_use);
  }
}

// Use text format parsing and serializing to test reflection api.
TEST(ArenaTest, ReflectionInTextFormat) {
  Arena arena;
  std::string data;

  TextFormat::Printer printer;
  TextFormat::Parser parser;

  UNITTEST::TestArenaMap* from = Arena::Create<UNITTEST::TestArenaMap>(&arena);
  UNITTEST::TestArenaMap* to = Arena::Create<UNITTEST::TestArenaMap>(&arena);

  MapTestUtil::SetArenaMapFields(from);
  printer.PrintToString(*from, &data);

  EXPECT_TRUE(parser.ParseFromString(data, to));
  MapTestUtil::ExpectArenaMapFieldsSet(*to);
}

// Make sure the memory allocated for string in map is deallocated.
TEST(ArenaTest, StringMapNoLeak) {
  Arena arena;
  UNITTEST::TestArenaMap* message =
      Arena::Create<UNITTEST::TestArenaMap>(&arena);
  std::string data;
  // String with length less than 16 will not be allocated from heap.
  int original_capacity = data.capacity();
  while (data.capacity() <= original_capacity) {
    data.append("a");
  }
  (*message->mutable_map_string_string())[data] = data;
  // Now we recreate the map via parsing
  message->ParseFromString(message->SerializeAsString());
  // And we cause value strings to grow to make sure they are handled correctly.
  std::string& value = (*message->mutable_map_string_string())[data];
  ASSERT_EQ(value, data);
  // Consume all the capacity.
  while (value.size() < value.capacity()) value.append("a");
  // And then grow the value.
  value.append("a");
}

TEST(ArenaTest, IsInitialized) {
  // Allocate a large initial polluted block.
  std::vector<char> arena_block(128 * 1024);
  std::fill(arena_block.begin(), arena_block.end(), '\xff');

  ArenaOptions options;
  options.initial_block = &arena_block[0];
  options.initial_block_size = arena_block.size();
  Arena arena(options);

  UNITTEST::TestArenaMap* message =
      Arena::Create<UNITTEST::TestArenaMap>(&arena);
  EXPECT_EQ(0, (*message->mutable_map_int32_int32())[0]);
}

TEST(ArenaTest, DynamicMapFieldOnArena) {
  Arena arena;
  UNITTEST::TestMap message2;

  DynamicMessageFactory factory;
  Message* message1 =
      factory.GetPrototype(UNITTEST::TestMap::descriptor())->New(&arena);
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  reflection_tester.SetMapFieldsViaReflection(message1);
  reflection_tester.ExpectMapFieldsSetViaReflection(*message1);
  reflection_tester.ExpectMapFieldsSetViaReflectionIterator(message1);
  message2.CopyFrom(*message1);
  MapTestUtil::ExpectMapFieldsSet(message2);
}

TEST(ArenaTest, DynamicMapFieldOnArenaMemoryLeak) {
  auto* desc = UNITTEST::TestMap::descriptor();
  auto* field = desc->FindFieldByName("map_int32_int32");

  Arena arena;
  DynamicMessageFactory factory;
  auto* message = factory.GetPrototype(desc)->New(&arena);
  auto* reflection = message->GetReflection();
  reflection->AddMessage(message, field);

  // Force internal syncing, which initializes the mutex.
  MapReflectionTester reflection_tester(UNITTEST::TestMap::descriptor());
  int size = reflection_tester.MapSize(*message, "map_int32_int32");
  EXPECT_EQ(size, 1);
}

TEST(MoveTest, MoveConstructorWorks) {
  Map<int32_t, TestAllTypes> original_map;
  original_map[42].mutable_optional_nested_message()->set_bb(42);
  original_map[43].mutable_optional_nested_message()->set_bb(43);
  const auto* nested_msg42_ptr = &original_map[42].optional_nested_message();
  const auto* nested_msg43_ptr = &original_map[43].optional_nested_message();

  Map<int32_t, TestAllTypes> moved_to_map(std::move(original_map));
  EXPECT_TRUE(original_map.empty());
  EXPECT_EQ(2, moved_to_map.size());
  EXPECT_EQ(42, moved_to_map[42].optional_nested_message().bb());
  EXPECT_EQ(43, moved_to_map[43].optional_nested_message().bb());
  // This test takes advantage of the fact that pointers are swapped, so there
  // should be pointer stability.
  EXPECT_EQ(nested_msg42_ptr, &moved_to_map[42].optional_nested_message());
  EXPECT_EQ(nested_msg43_ptr, &moved_to_map[43].optional_nested_message());
}

TEST(MoveTest, MoveAssignmentWorks) {
  Map<int32_t, TestAllTypes> original_map;
  original_map[42].mutable_optional_nested_message()->set_bb(42);
  original_map[43].mutable_optional_nested_message()->set_bb(43);
  const auto* nested_msg42_ptr = &original_map[42].optional_nested_message();
  const auto* nested_msg43_ptr = &original_map[43].optional_nested_message();

  Map<int32_t, TestAllTypes> moved_to_map = std::move(original_map);
  EXPECT_TRUE(original_map.empty());
  EXPECT_EQ(2, moved_to_map.size());
  EXPECT_EQ(42, moved_to_map[42].optional_nested_message().bb());
  EXPECT_EQ(43, moved_to_map[43].optional_nested_message().bb());
  // This test takes advantage of the fact that pointers are swapped, so there
  // should be pointer stability.
  EXPECT_EQ(nested_msg42_ptr, &moved_to_map[42].optional_nested_message());
  EXPECT_EQ(nested_msg43_ptr, &moved_to_map[43].optional_nested_message());
}

TEST(KeyMapBaseTest, InsertOrReplaceNodeWorks) {
  using M = Map<int32_t, std::string>;
  M map;
  EXPECT_TRUE(MapTestPeer::InsertOrReplaceNode(map, 10, "Foo"));
  EXPECT_EQ(map.size(), 1);
  EXPECT_EQ(map[10], "Foo");
  EXPECT_FALSE(MapTestPeer::InsertOrReplaceNode(map, 10, "Bar"));
  EXPECT_EQ(map.size(), 1);
  EXPECT_EQ(map[10], "Bar");
  map[100] = "BAD";
  EXPECT_EQ(map.size(), 2);
  EXPECT_EQ(map[10], "Bar");
  EXPECT_EQ(map[100], "BAD");
  EXPECT_FALSE(MapTestPeer::InsertOrReplaceNode(map, 100, "GOOD"));
  EXPECT_EQ(map.size(), 2);
  EXPECT_EQ(map[10], "Bar");
  EXPECT_EQ(map[100], "GOOD");
}

TEST(NonUtf8Test, StringValuePassesInProto2) {
  proto2_unittest::TestProto2BytesMap message;
  (*message.mutable_map_string())[1] = "\xFF";

  auto serialized = message.SerializeAsString();
  // Parsing passes, but a failure is logged in debug mode.
  ASSERT_TRUE(message.ParseFromString(serialized));
  EXPECT_EQ((*message.mutable_map_string())[1], "\xFF");
}

TEST(NonUtf8Test, BytesValuePassesInProto2) {
  proto2_unittest::TestProto2BytesMap message;
  (*message.mutable_map_bytes())[1] = "\xFF";

  auto serialized = message.SerializeAsString();
  ASSERT_TRUE(message.ParseFromString(serialized));
  EXPECT_EQ((*message.mutable_map_bytes())[1], "\xFF");
}

TEST(NonUtf8Test, StringValueFailsInProto3) {
  proto3_unittest::TestProto3BytesMap message;
  (*message.mutable_map_string())[1] = "\xFF";

  auto serialized = message.SerializeAsString();
  // It will fail, and log an error.
  ASSERT_FALSE(message.ParseFromString(serialized));
}


TEST(NonUtf8Test, BytesValuePassesInProto3) {
  proto3_unittest::TestProto3BytesMap message;
  (*message.mutable_map_bytes())[1] = "\xFF";

  auto serialized = message.SerializeAsString();
  ASSERT_TRUE(message.ParseFromString(serialized));
  EXPECT_EQ((*message.mutable_map_bytes())[1], "\xFF");
}

}  // namespace
}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
