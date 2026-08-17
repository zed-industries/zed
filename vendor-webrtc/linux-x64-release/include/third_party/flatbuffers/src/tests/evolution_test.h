#ifndef TESTS_EVOLUTION_TEST_H
#define TESTS_EVOLUTION_TEST_H

#include <string>

namespace flatbuffers {
namespace tests {

void EvolutionTest(const std::string &tests_data_path);
void ConformTest();
void UnionDeprecationTest(const std::string &tests_data_path);

}  // namespace tests
}  // namespace flatbuffers

#endif
