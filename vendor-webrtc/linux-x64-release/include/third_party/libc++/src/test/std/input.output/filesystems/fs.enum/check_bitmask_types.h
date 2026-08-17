#ifndef TEST_BITMASK_TYPE_H
#define TEST_BITMASK_TYPE_H

#include <type_traits>
#include <cassert>

#include "test_macros.h"


template <class EnumType, EnumType Val1, EnumType Val2,
          class UT = typename std::underlying_type<EnumType>::type,
          UT UVal1 = static_cast<UT>(Val1),
          UT UVal2 = static_cast<UT>(Val2),
          UT UZero = static_cast<UT>(0),
          EnumType Zero = static_cast<EnumType>(0)
        >
struct check_bitmask_type {

  static constexpr UT dcast(EnumType e) { return static_cast<UT>(e); }
  static constexpr UT unpromote(decltype((~UZero)) promoted) { return static_cast<UT>(promoted); }
  // We need two values that are non-zero and share at least one bit.
  static_assert(Val1 != Zero && Val2 != Zero, "");
  static_assert(Val1 != Val2, "");
  static_assert((UVal1 & UVal2) == 0, "");


  static bool check()
  {
    {
      EnumType ValRef = Val1;
      ASSERT_SAME_TYPE(EnumType, decltype(Val1 & Val2));
      ASSERT_SAME_TYPE(EnumType, decltype(Val1 | Val2));
      ASSERT_SAME_TYPE(EnumType, decltype(Val1 ^ Val2));
      ASSERT_SAME_TYPE(EnumType, decltype((~Val1)));
      ASSERT_SAME_TYPE(EnumType&, decltype(ValRef &= Val2));
      ASSERT_SAME_TYPE(EnumType&, decltype(ValRef |= Val2));
      ASSERT_SAME_TYPE(EnumType&, decltype(ValRef ^= Val2));
    }

    static_assert((Val1 & Zero) == Zero, "");
    static_assert((Val1 & Val1) == Val1, "");
    static_assert(dcast(Val1 & Val2) == (UVal1 & UVal2), "");

    static_assert((Val1 | Zero) == Val1, "");
    static_assert(dcast(Val1 | Val2) == (UVal1 | UVal2), "");

    static_assert((Val1 ^ Zero) == Val1, "");
    static_assert(dcast(Val1 ^ Val2) == (UVal1 ^ UVal2), "");

    static_assert(dcast(~Zero) == unpromote(~UZero), "");
    static_assert(dcast(~Val1) == unpromote(~UVal1), "");

    {
      EnumType e = Val1;
      EnumType& eref = (e &= Val2);
      assert(&eref == &e);
      assert(dcast(eref) == (UVal1 & UVal2));
    }
    {
      EnumType e = Val1;
      EnumType& eref = (e |= Val2);
      assert(&eref == &e);
      assert(dcast(eref) == (UVal1 | UVal2));
    }
    {
      EnumType e = Val1;
      EnumType& eref = (e ^= Val2);
      assert(&eref == &e);
      assert(dcast(eref) == (UVal1 ^ UVal2));
    }
    return true;
  }
};

#endif // TEST_BITMASK_TYPE
