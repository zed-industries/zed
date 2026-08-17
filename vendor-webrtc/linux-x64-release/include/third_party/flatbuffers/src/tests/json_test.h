#ifndef TESTS_JSON_TEST_H
#define TESTS_JSON_TEST_H

#include <string>

namespace flatbuffers {
namespace tests {

void JsonDefaultTest(const std::string& tests_data_path);
void JsonEnumsTest(const std::string& tests_data_path);
void JsonOptionalTest(const std::string& tests_data_path, bool default_scalars);
void ParseIncorrectMonsterJsonTest(const std::string& tests_data_path);
void JsonUnsortedArrayTest();
void JsonUnionStructTest();

}  // namespace tests
}  // namespace flatbuffers

#endif
