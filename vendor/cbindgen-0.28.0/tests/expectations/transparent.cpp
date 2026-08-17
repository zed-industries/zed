#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct DummyStruct;

struct EnumWithAssociatedConstantInImpl;

using TransparentComplexWrappingStructTuple = DummyStruct;

using TransparentPrimitiveWrappingStructTuple = uint32_t;

using TransparentComplexWrappingStructure = DummyStruct;

using TransparentPrimitiveWrappingStructure = uint32_t;

template<typename T>
using TransparentComplexWrapper = DummyStruct;

template<typename T>
using TransparentPrimitiveWrapper = uint32_t;

using TransparentPrimitiveWithAssociatedConstants = uint32_t;
constexpr static const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ZERO = 0;
constexpr static const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ONE = 1;

struct TransparentEmptyStructure {

};

constexpr static const TransparentPrimitiveWrappingStructure EnumWithAssociatedConstantInImpl_TEN = 10;

extern "C" {

void root(TransparentComplexWrappingStructTuple a,
          TransparentPrimitiveWrappingStructTuple b,
          TransparentComplexWrappingStructure c,
          TransparentPrimitiveWrappingStructure d,
          TransparentComplexWrapper<int32_t> e,
          TransparentPrimitiveWrapper<int32_t> f,
          TransparentPrimitiveWithAssociatedConstants g,
          TransparentEmptyStructure h,
          EnumWithAssociatedConstantInImpl i);

}  // extern "C"
