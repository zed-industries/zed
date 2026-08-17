#ifndef TESTS_MONSTER_TEST_H
#define TESTS_MONSTER_TEST_H

#include <string>

#include "flatbuffers/detached_buffer.h"
#include "monster_test_generated.h"

namespace flatbuffers {
namespace tests {

flatbuffers::DetachedBuffer CreateFlatBufferTest(std::string &buffer);

void AccessFlatBufferTest(const uint8_t *flatbuf, size_t length,
                          bool pooled = true); 

void MutateFlatBuffersTest(uint8_t *flatbuf, std::size_t length);

void ObjectFlatBuffersTest(uint8_t *flatbuf);

void CheckMonsterObject(MyGame::Example::MonsterT *monster2);

void SizePrefixedTest();

void TestMonsterExtraFloats(const std::string& tests_data_path);

void EnumNamesTest();

void TypeAliasesTest();

void ParseAndGenerateTextTest(const std::string& tests_data_path, bool binary);

void UnPackTo(const uint8_t *flatbuf);

}  // namespace tests
}  // namespace flatbuffers

#endif
