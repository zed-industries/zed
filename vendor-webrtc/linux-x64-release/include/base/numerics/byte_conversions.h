// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_NUMERICS_BYTE_CONVERSIONS_H_
#define BASE_NUMERICS_BYTE_CONVERSIONS_H_

#include <array>
#include <bit>
#include <cstdint>
#include <cstring>
#include <span>
#include <type_traits>

#include "base/numerics/basic_ops_impl.h"
#include "build/build_config.h"

// Chromium only builds and runs on Little Endian machines.
static_assert(ARCH_CPU_LITTLE_ENDIAN);

namespace base {

// Returns a value with all bytes in |x| swapped, i.e. reverses the endianness.
// TODO(pkasting): Once C++23 is available, replace with std::byteswap.
template <class T>
  requires(std::is_integral_v<T>)
inline constexpr T ByteSwap(T value) {
  return internal::SwapBytes(value);
}

// Returns a uint8_t with the value in `bytes` interpreted as the native endian
// encoding of the integer for the machine.
//
// This is suitable for decoding integers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
//
// Note that since a single byte can have only one ordering, this just copies
// the byte out of the span. This provides a consistent function for the
// operation nonetheless.
inline constexpr uint8_t U8FromNativeEndian(
    std::span<const uint8_t, 1u> bytes) {
  return bytes[0];
}
// Returns a uint16_t with the value in `bytes` interpreted as the native endian
// encoding of the integer for the machine.
//
// This is suitable for decoding integers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
inline constexpr uint16_t U16FromNativeEndian(
    std::span<const uint8_t, 2u> bytes) {
  return internal::FromLittleEndian<uint16_t>(bytes);
}
// Returns a uint32_t with the value in `bytes` interpreted as the native endian
// encoding of the integer for the machine.
//
// This is suitable for decoding integers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
inline constexpr uint32_t U32FromNativeEndian(
    std::span<const uint8_t, 4u> bytes) {
  return internal::FromLittleEndian<uint32_t>(bytes);
}
// Returns a uint64_t with the value in `bytes` interpreted as the native endian
// encoding of the integer for the machine.
//
// This is suitable for decoding integers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
inline constexpr uint64_t U64FromNativeEndian(
    std::span<const uint8_t, 8u> bytes) {
  return internal::FromLittleEndian<uint64_t>(bytes);
}
// Returns a int8_t with the value in `bytes` interpreted as the native endian
// encoding of the integer for the machine.
//
// This is suitable for decoding integers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
//
// Note that since a single byte can have only one ordering, this just copies
// the byte out of the span. This provides a consistent function for the
// operation nonetheless.
inline constexpr int8_t I8FromNativeEndian(std::span<const uint8_t, 1u> bytes) {
  return static_cast<int8_t>(bytes[0]);
}
// Returns a int16_t with the value in `bytes` interpreted as the native endian
// encoding of the integer for the machine.
//
// This is suitable for decoding integers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
inline constexpr int16_t I16FromNativeEndian(
    std::span<const uint8_t, 2u> bytes) {
  return internal::FromLittleEndian<int16_t>(bytes);
}
// Returns a int32_t with the value in `bytes` interpreted as the native endian
// encoding of the integer for the machine.
//
// This is suitable for decoding integers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
inline constexpr int32_t I32FromNativeEndian(
    std::span<const uint8_t, 4u> bytes) {
  return internal::FromLittleEndian<int32_t>(bytes);
}
// Returns a int64_t with the value in `bytes` interpreted as the native endian
// encoding of the integer for the machine.
//
// This is suitable for decoding integers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
inline constexpr int64_t I64FromNativeEndian(
    std::span<const uint8_t, 8u> bytes) {
  return internal::FromLittleEndian<int64_t>(bytes);
}

// Returns a float with the value in `bytes` interpreted as the native endian
// encoding of the number for the machine.
//
// This is suitable for decoding numbers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
inline constexpr float FloatFromNativeEndian(
    std::span<const uint8_t, 4u> bytes) {
  return std::bit_cast<float>(U32FromNativeEndian(bytes));
}
// Returns a double with the value in `bytes` interpreted as the native endian
// encoding of the number for the machine.
//
// This is suitable for decoding numbers that were always kept in native
// encoding, such as when stored in shared-memory (or through IPC) as a byte
// buffer. Prefer an explicit little endian when storing and reading data from
// storage, and explicit big endian for network order.
inline constexpr double DoubleFromNativeEndian(
    std::span<const uint8_t, 8u> bytes) {
  return std::bit_cast<double>(U64FromNativeEndian(bytes));
}

// Returns a uint8_t with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
//
// Note that since a single byte can have only one ordering, this just copies
// the byte out of the span. This provides a consistent function for the
// operation nonetheless.
inline constexpr uint8_t U8FromLittleEndian(
    std::span<const uint8_t, 1u> bytes) {
  return bytes[0];
}
// Returns a uint16_t with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
inline constexpr uint16_t U16FromLittleEndian(
    std::span<const uint8_t, 2u> bytes) {
  return internal::FromLittleEndian<uint16_t>(bytes);
}
// Returns a uint32_t with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
inline constexpr uint32_t U32FromLittleEndian(
    std::span<const uint8_t, 4u> bytes) {
  return internal::FromLittleEndian<uint32_t>(bytes);
}
// Returns a uint64_t with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
inline constexpr uint64_t U64FromLittleEndian(
    std::span<const uint8_t, 8u> bytes) {
  return internal::FromLittleEndian<uint64_t>(bytes);
}
// Returns a int8_t with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
//
// Note that since a single byte can have only one ordering, this just copies
// the byte out of the span. This provides a consistent function for the
// operation nonetheless.
inline constexpr int8_t I8FromLittleEndian(std::span<const uint8_t, 1u> bytes) {
  return static_cast<int8_t>(bytes[0]);
}
// Returns a int16_t with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
inline constexpr int16_t I16FromLittleEndian(
    std::span<const uint8_t, 2u> bytes) {
  return internal::FromLittleEndian<int16_t>(bytes);
}
// Returns a int32_t with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
inline constexpr int32_t I32FromLittleEndian(
    std::span<const uint8_t, 4u> bytes) {
  return internal::FromLittleEndian<int32_t>(bytes);
}
// Returns a int64_t with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
inline constexpr int64_t I64FromLittleEndian(
    std::span<const uint8_t, 8u> bytes) {
  return internal::FromLittleEndian<int64_t>(bytes);
}
// Returns a float with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding numbers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
inline constexpr float FloatFromLittleEndian(
    std::span<const uint8_t, 4u> bytes) {
  return std::bit_cast<float>(U32FromLittleEndian(bytes));
}
// Returns a double with the value in `bytes` interpreted as a little-endian
// encoding of the integer.
//
// This is suitable for decoding numbers encoded explicitly in little endian,
// which is a good practice with storing and reading data from storage. Use
// the native-endian versions when working with values that were always in
// memory, such as when stored in shared-memory (or through IPC) as a byte
// buffer.
inline constexpr double DoubleFromLittleEndian(
    std::span<const uint8_t, 8u> bytes) {
  return std::bit_cast<double>(U64FromLittleEndian(bytes));
}

// Returns a uint8_t with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
//
// Note that since a single byte can have only one ordering, this just copies
// the byte out of the span. This provides a consistent function for the
// operation nonetheless.
inline constexpr uint8_t U8FromBigEndian(std::span<const uint8_t, 1u> bytes) {
  return bytes[0];
}
// Returns a uint16_t with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
inline constexpr uint16_t U16FromBigEndian(std::span<const uint8_t, 2u> bytes) {
  return ByteSwap(internal::FromLittleEndian<uint16_t>(bytes));
}
// Returns a uint32_t with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
inline constexpr uint32_t U32FromBigEndian(std::span<const uint8_t, 4u> bytes) {
  return ByteSwap(internal::FromLittleEndian<uint32_t>(bytes));
}
// Returns a uint64_t with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
inline constexpr uint64_t U64FromBigEndian(std::span<const uint8_t, 8u> bytes) {
  return ByteSwap(internal::FromLittleEndian<uint64_t>(bytes));
}
// Returns a int8_t with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
//
// Note that since a single byte can have only one ordering, this just copies
// the byte out of the span. This provides a consistent function for the
// operation nonetheless.
inline constexpr int8_t I8FromBigEndian(std::span<const uint8_t, 1u> bytes) {
  return static_cast<int8_t>(bytes[0]);
}
// Returns a int16_t with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
inline constexpr int16_t I16FromBigEndian(std::span<const uint8_t, 2u> bytes) {
  return ByteSwap(internal::FromLittleEndian<int16_t>(bytes));
}
// Returns a int32_t with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
inline constexpr int32_t I32FromBigEndian(std::span<const uint8_t, 4u> bytes) {
  return ByteSwap(internal::FromLittleEndian<int32_t>(bytes));
}
// Returns a int64_t with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding integers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
inline constexpr int64_t I64FromBigEndian(std::span<const uint8_t, 8u> bytes) {
  return ByteSwap(internal::FromLittleEndian<int64_t>(bytes));
}
// Returns a float with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding numbers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
inline constexpr float FloatFromBigEndian(std::span<const uint8_t, 4u> bytes) {
  return std::bit_cast<float>(U32FromBigEndian(bytes));
}
// Returns a double with the value in `bytes` interpreted as a big-endian
// encoding of the integer.
//
// This is suitable for decoding numbers encoded explicitly in big endian, such
// as for network order. Use the native-endian versions when working with values
// that were always in memory, such as when stored in shared-memory (or through
// IPC) as a byte buffer.
inline constexpr double DoubleFromBigEndian(
    std::span<const uint8_t, 8u> bytes) {
  return std::bit_cast<double>(U64FromBigEndian(bytes));
}

// Returns a byte array holding the value of a uint8_t encoded as the native
// endian encoding of the integer for the machine.
//
// This is suitable for encoding integers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 1u> U8ToNativeEndian(uint8_t val) {
  return {val};
}
// Returns a byte array holding the value of a uint16_t encoded as the native
// endian encoding of the integer for the machine.
//
// This is suitable for encoding integers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 2u> U16ToNativeEndian(uint16_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a uint32_t encoded as the native
// endian encoding of the integer for the machine.
//
// This is suitable for encoding integers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 4u> U32ToNativeEndian(uint32_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a uint64_t encoded as the native
// endian encoding of the integer for the machine.
//
// This is suitable for encoding integers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 8u> U64ToNativeEndian(uint64_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a int8_t encoded as the native
// endian encoding of the integer for the machine.
//
// This is suitable for encoding integers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 1u> I8ToNativeEndian(int8_t val) {
  return {static_cast<uint8_t>(val)};
}
// Returns a byte array holding the value of a int16_t encoded as the native
// endian encoding of the integer for the machine.
//
// This is suitable for encoding integers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 2u> I16ToNativeEndian(int16_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a int32_t encoded as the native
// endian encoding of the integer for the machine.
//
// This is suitable for encoding integers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 4u> I32ToNativeEndian(int32_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a int64_t encoded as the native
// endian encoding of the integer for the machine.
//
// This is suitable for encoding integers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 8u> I64ToNativeEndian(int64_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a float encoded as the native
// endian encoding of the number for the machine.
//
// This is suitable for encoding numbers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 4u> FloatToNativeEndian(float val) {
  return U32ToNativeEndian(std::bit_cast<uint32_t>(val));
}
// Returns a byte array holding the value of a double encoded as the native
// endian encoding of the number for the machine.
//
// This is suitable for encoding numbers that will always be kept in native
// encoding, such as for storing in shared-memory (or sending through IPC) as a
// byte buffer. Prefer an explicit little endian when storing data into external
// storage, and explicit big endian for network order.
inline constexpr std::array<uint8_t, 8u> DoubleToNativeEndian(double val) {
  return U64ToNativeEndian(std::bit_cast<uint64_t>(val));
}

// Returns a byte array holding the value of a uint8_t encoded as the
// little-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 1u> U8ToLittleEndian(uint8_t val) {
  return {val};
}
// Returns a byte array holding the value of a uint16_t encoded as the
// little-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 2u> U16ToLittleEndian(uint16_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a uint32_t encoded as the
// little-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 4u> U32ToLittleEndian(uint32_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a uint64_t encoded as the
// little-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 8u> U64ToLittleEndian(uint64_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a int8_t encoded as the
// little-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 1u> I8ToLittleEndian(int8_t val) {
  return {static_cast<uint8_t>(val)};
}
// Returns a byte array holding the value of a int16_t encoded as the
// little-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 2u> I16ToLittleEndian(int16_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a int32_t encoded as the
// little-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 4u> I32ToLittleEndian(int32_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a int64_t encoded as the
// little-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 8u> I64ToLittleEndian(int64_t val) {
  return internal::ToLittleEndian(val);
}
// Returns a byte array holding the value of a float encoded as the
// little-endian encoding of the number.
//
// This is suitable for encoding numbers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 4u> FloatToLittleEndian(float val) {
  return internal::ToLittleEndian(std::bit_cast<uint32_t>(val));
}
// Returns a byte array holding the value of a double encoded as the
// little-endian encoding of the number.
//
// This is suitable for encoding numbers explicitly in little endian, which is
// a good practice with storing and reading data from storage. Use the
// native-endian versions when working with values that will always be in
// memory, such as when stored in shared-memory (or passed through IPC) as a
// byte buffer.
inline constexpr std::array<uint8_t, 8u> DoubleToLittleEndian(double val) {
  return internal::ToLittleEndian(std::bit_cast<uint64_t>(val));
}

// Returns a byte array holding the value of a uint8_t encoded as the big-endian
// encoding of the integer.
//
// This is suitable for encoding integers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 1u> U8ToBigEndian(uint8_t val) {
  return {val};
}
// Returns a byte array holding the value of a uint16_t encoded as the
// big-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 2u> U16ToBigEndian(uint16_t val) {
  return internal::ToLittleEndian(ByteSwap(val));
}
// Returns a byte array holding the value of a uint32_t encoded as the
// big-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 4u> U32ToBigEndian(uint32_t val) {
  return internal::ToLittleEndian(ByteSwap(val));
}
// Returns a byte array holding the value of a uint64_t encoded as the
// big-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 8u> U64ToBigEndian(uint64_t val) {
  return internal::ToLittleEndian(ByteSwap(val));
}
// Returns a byte array holding the value of a int8_t encoded as the big-endian
// encoding of the integer.
//
// This is suitable for encoding integers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 1u> I8ToBigEndian(int8_t val) {
  return {static_cast<uint8_t>(val)};
}
// Returns a byte array holding the value of a int16_t encoded as the
// big-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 2u> I16ToBigEndian(int16_t val) {
  return internal::ToLittleEndian(ByteSwap(val));
}
// Returns a byte array holding the value of a int32_t encoded as the
// big-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 4u> I32ToBigEndian(int32_t val) {
  return internal::ToLittleEndian(ByteSwap(val));
}
// Returns a byte array holding the value of a int64_t encoded as the
// big-endian encoding of the integer.
//
// This is suitable for encoding integers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 8u> I64ToBigEndian(int64_t val) {
  return internal::ToLittleEndian(ByteSwap(val));
}
// Returns a byte array holding the value of a float encoded as the big-endian
// encoding of the number.
//
// This is suitable for encoding numbers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 4u> FloatToBigEndian(float val) {
  return internal::ToLittleEndian(ByteSwap(std::bit_cast<uint32_t>(val)));
}
// Returns a byte array holding the value of a double encoded as the big-endian
// encoding of the number.
//
// This is suitable for encoding numbers explicitly in big endian, such as for
// network order. Use the native-endian versions when working with values that
// are always in memory, such as when stored in shared-memory (or passed through
// IPC) as a byte buffer. Use the little-endian encoding for storing and reading
// from storage.
inline constexpr std::array<uint8_t, 8u> DoubleToBigEndian(double val) {
  return internal::ToLittleEndian(ByteSwap(std::bit_cast<uint64_t>(val)));
}

// Deprecated: Prefer the shorter, less-namespaced names.
// TODO(crbug.com/409350835): Remove these when callers have been migrated to
// the shorter name.
namespace numerics {
using ::base::ByteSwap;
using ::base::DoubleFromBigEndian;
using ::base::DoubleFromLittleEndian;
using ::base::DoubleFromNativeEndian;
using ::base::DoubleToBigEndian;
using ::base::DoubleToLittleEndian;
using ::base::DoubleToNativeEndian;
using ::base::FloatFromBigEndian;
using ::base::FloatFromLittleEndian;
using ::base::FloatFromNativeEndian;
using ::base::FloatToBigEndian;
using ::base::FloatToLittleEndian;
using ::base::FloatToNativeEndian;
using ::base::I16FromBigEndian;
using ::base::I16FromLittleEndian;
using ::base::I16FromNativeEndian;
using ::base::I16ToBigEndian;
using ::base::I16ToLittleEndian;
using ::base::I16ToNativeEndian;
using ::base::I32FromBigEndian;
using ::base::I32FromLittleEndian;
using ::base::I32FromNativeEndian;
using ::base::I32ToBigEndian;
using ::base::I32ToLittleEndian;
using ::base::I32ToNativeEndian;
using ::base::I64FromBigEndian;
using ::base::I64FromLittleEndian;
using ::base::I64FromNativeEndian;
using ::base::I64ToBigEndian;
using ::base::I64ToLittleEndian;
using ::base::I64ToNativeEndian;
using ::base::I8FromBigEndian;
using ::base::I8FromLittleEndian;
using ::base::I8FromNativeEndian;
using ::base::I8ToBigEndian;
using ::base::I8ToLittleEndian;
using ::base::I8ToNativeEndian;
using ::base::U16FromBigEndian;
using ::base::U16FromLittleEndian;
using ::base::U16FromNativeEndian;
using ::base::U16ToBigEndian;
using ::base::U16ToLittleEndian;
using ::base::U16ToNativeEndian;
using ::base::U32FromBigEndian;
using ::base::U32FromLittleEndian;
using ::base::U32FromNativeEndian;
using ::base::U32ToBigEndian;
using ::base::U32ToLittleEndian;
using ::base::U32ToNativeEndian;
using ::base::U64FromBigEndian;
using ::base::U64FromLittleEndian;
using ::base::U64FromNativeEndian;
using ::base::U64ToBigEndian;
using ::base::U64ToLittleEndian;
using ::base::U64ToNativeEndian;
using ::base::U8FromBigEndian;
using ::base::U8FromLittleEndian;
using ::base::U8FromNativeEndian;
using ::base::U8ToBigEndian;
using ::base::U8ToLittleEndian;
using ::base::U8ToNativeEndian;
}  // namespace numerics

}  // namespace base

#endif  // BASE_NUMERICS_BYTE_CONVERSIONS_H_
