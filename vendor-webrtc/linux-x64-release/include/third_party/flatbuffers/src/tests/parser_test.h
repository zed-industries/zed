#ifndef TESTS_PARSER_TEST_H
#define TESTS_PARSER_TEST_H

namespace flatbuffers {
namespace tests {

void ErrorTest();
void EnumOutOfRangeTest();
void IntegerOutOfRangeTest();
void InvalidFloatTest();
void UnicodeInvalidSurrogatesTest();
void InvalidUTF8Test();
void ValueTest();
void NestedListTest();
void EnumStringsTest();
void EnumValueTest();
void IntegerBoundaryTest();
void ValidFloatTest();
void UnicodeTest();
void UnicodeTestAllowNonUTF8();
void UnicodeTestGenerateTextFailsOnNonUTF8();
void UnicodeSurrogatesTest();
void UnknownFieldsTest();
void ParseUnionTest();
void ValidSameNameDifferentNamespaceTest();
void WarningsAsErrorsTest();
void StringVectorDefaultsTest();
void FieldIdentifierTest();

}  // namespace tests
}  // namespace flatbuffers

#endif  // TESTS_PARSER_TEST_H
