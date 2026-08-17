#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_TESTING_CONSTANTS_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_TESTING_CONSTANTS_H_

#include <openssl/obj_mac.h>

namespace private_membership {
namespace rlwe {

// Identifier of the elliptic curve used in tests.
constexpr int kTestCurveId = NID_X9_62_prime256v1;

}  // namespace rlwe
}  // namespace private_membership

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_TESTING_CONSTANTS_H_
