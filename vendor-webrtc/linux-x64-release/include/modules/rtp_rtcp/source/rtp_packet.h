/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTP_PACKET_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_PACKET_H_

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "api/array_view.h"
#include "modules/rtp_rtcp/include/rtp_header_extension_map.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "rtc_base/copy_on_write_buffer.h"

namespace webrtc {

class RtpPacket {
 public:
  using ExtensionType = RTPExtensionType;
  using ExtensionManager = RtpHeaderExtensionMap;

  // `extensions` required for SetExtension/ReserveExtension functions during
  // packet creating and used if available in Parse function.
  // Adding and getting extensions will fail until `extensions` is
  // provided via constructor or IdentifyExtensions function.
  // |*extensions| is only accessed during construction; the pointer is not
  // stored.
  RtpPacket();
  explicit RtpPacket(const ExtensionManager* extensions);
  RtpPacket(const ExtensionManager* extensions, size_t capacity);

  RtpPacket(const RtpPacket&);
  RtpPacket(RtpPacket&&);
  RtpPacket& operator=(const RtpPacket&);
  RtpPacket& operator=(RtpPacket&&);

  ~RtpPacket();

  // Parse and copy given buffer into Packet.
  // Does not require extension map to be registered (map is only required to
  // read or allocate extensions in methods GetExtension, AllocateExtension,
  // etc.)
  bool Parse(const uint8_t* buffer, size_t size);
  bool Parse(ArrayView<const uint8_t> packet);

  // Parse and move given buffer into Packet.
  bool Parse(CopyOnWriteBuffer packet);

  // Maps extensions id to their types.
  void IdentifyExtensions(ExtensionManager extensions);

  // Returns the extension map used for identifying extensions in this packet.
  const ExtensionManager& extension_manager() const { return extensions_; }

  // Header.
  bool Marker() const { return marker_; }
  uint8_t PayloadType() const { return payload_type_; }
  uint16_t SequenceNumber() const { return sequence_number_; }
  uint32_t Timestamp() const { return timestamp_; }
  uint32_t Ssrc() const { return ssrc_; }
  std::vector<uint32_t> Csrcs() const;

  size_t headers_size() const { return payload_offset_; }

  // Payload.
  size_t payload_size() const { return payload_size_; }
  bool has_padding() const { return buffer_[0] & 0x20; }
  size_t padding_size() const { return padding_size_; }
  ArrayView<const uint8_t> payload() const {
    return MakeArrayView(data() + payload_offset_, payload_size_);
  }
  CopyOnWriteBuffer PayloadBuffer() const {
    return buffer_.Slice(payload_offset_, payload_size_);
  }

  // Buffer.
  CopyOnWriteBuffer Buffer() const { return buffer_; }
  size_t capacity() const { return buffer_.capacity(); }
  size_t size() const {
    return payload_offset_ + payload_size_ + padding_size_;
  }
  const uint8_t* data() const { return buffer_.cdata(); }
  size_t FreeCapacity() const { return capacity() - size(); }
  size_t MaxPayloadSize() const { return capacity() - headers_size(); }

  // Reset fields and buffer.
  void Clear();

  // Header setters.
  void CopyHeaderFrom(const RtpPacket& packet);
  void SetMarker(bool marker_bit);
  void SetPayloadType(uint8_t payload_type);
  void SetSequenceNumber(uint16_t seq_no);
  void SetTimestamp(uint32_t timestamp);
  void SetSsrc(uint32_t ssrc);

  // Fills with zeroes mutable extensions,
  // which are modified after FEC protection is generated.
  void ZeroMutableExtensions();

  // Removes extension of given `type`, returns false is extension was not
  // registered in packet's extension map or not present in the packet. Only
  // extension that should be removed must be registered, other extensions may
  // not be registered and will be preserved as is.
  bool RemoveExtension(ExtensionType type);

  // Writes csrc list. Assumes:
  // a) There is enough room left in buffer.
  // b) Extension headers, payload or padding data has not already been added.
  void SetCsrcs(ArrayView<const uint32_t> csrcs);

  // Header extensions.
  template <typename Extension>
  bool HasExtension() const;
  bool HasExtension(ExtensionType type) const;

  // Returns whether there is an associated id for the extension and thus it is
  // possible to set the extension.
  template <typename Extension>
  bool IsRegistered() const;

  template <typename Extension, typename FirstValue, typename... Values>
  bool GetExtension(FirstValue&&, Values&&...) const;

  template <typename Extension>
  std::optional<typename Extension::value_type> GetExtension() const;

  // Returns view of the raw extension or empty view on failure.
  template <typename Extension>
  ArrayView<const uint8_t> GetRawExtension() const;

  template <typename Extension, typename... Values>
  bool SetExtension(const Values&...);

  template <typename Extension>
  bool SetRawExtension(ArrayView<const uint8_t> data);

  template <typename Extension>
  bool ReserveExtension();

  // Find or allocate an extension `type`. Returns view of size `length`
  // to write raw extension to or an empty view on failure.
  ArrayView<uint8_t> AllocateExtension(ExtensionType type, size_t length);

  // Find an extension `type`.
  // Returns view of the raw extension or empty view on failure.
  ArrayView<const uint8_t> FindExtension(ExtensionType type) const;

  // Returns pointer to the payload of size at least `size_bytes`.
  // Keeps original payload, if any. If `size_bytes` is larger than current
  // `payload_size()`, remaining bytes are uninitialized.
  uint8_t* SetPayloadSize(size_t size_bytes);

  // Same as SetPayloadSize but doesn't guarantee to keep current payload.
  uint8_t* AllocatePayload(size_t size_bytes);

  bool SetPadding(size_t padding_size);

  // Returns debug string of RTP packet (without detailed extension info).
  std::string ToString() const;

 private:
  struct ExtensionInfo {
    explicit ExtensionInfo(uint8_t id) : ExtensionInfo(id, 0, 0) {}
    ExtensionInfo(uint8_t id, uint8_t length, uint16_t offset)
        : id(id), length(length), offset(offset) {}
    uint8_t id;
    uint8_t length;
    uint16_t offset;
  };

  // Helper function for Parse. Fill header fields using data in given buffer,
  // but does not touch packet own buffer, leaving packet in invalid state.
  bool ParseBuffer(const uint8_t* buffer, size_t size);

  // Returns pointer to extension info for a given id. Returns nullptr if not
  // found.
  const ExtensionInfo* FindExtensionInfo(int id) const;

  // Returns reference to extension info for a given id. Creates a new entry
  // with the specified id if not found.
  ExtensionInfo& FindOrCreateExtensionInfo(int id);

  // Allocates and returns place to store rtp header extension.
  // Returns empty arrayview on failure.
  ArrayView<uint8_t> AllocateRawExtension(int id, size_t length);

  // Promotes existing one-byte header extensions to two-byte header extensions
  // by rewriting the data and updates the corresponding extension offsets.
  void PromoteToTwoByteHeaderExtension();

  uint16_t SetExtensionLengthMaybeAddZeroPadding(size_t extensions_offset);

  uint8_t* WriteAt(size_t offset) { return buffer_.MutableData() + offset; }
  void WriteAt(size_t offset, uint8_t byte) {
    buffer_.MutableData()[offset] = byte;
  }
  const uint8_t* ReadAt(size_t offset) const { return buffer_.data() + offset; }

  // Header.
  bool marker_;
  uint8_t payload_type_;
  uint8_t padding_size_;
  uint16_t sequence_number_;
  uint32_t timestamp_;
  uint32_t ssrc_;
  size_t payload_offset_;  // Match header size with csrcs and extensions.
  size_t payload_size_;

  ExtensionManager extensions_;
  std::vector<ExtensionInfo> extension_entries_;
  size_t extensions_size_ = 0;  // Unaligned.
  CopyOnWriteBuffer buffer_;
};

template <typename Extension>
bool RtpPacket::HasExtension() const {
  return HasExtension(Extension::kId);
}

template <typename Extension>
bool RtpPacket::IsRegistered() const {
  return extensions_.IsRegistered(Extension::kId);
}

template <typename Extension, typename FirstValue, typename... Values>
bool RtpPacket::GetExtension(FirstValue&& first, Values&&... values) const {
  auto raw = FindExtension(Extension::kId);
  if (raw.empty())
    return false;
  return Extension::Parse(raw, std::forward<FirstValue>(first),
                          std::forward<Values>(values)...);
}

template <typename Extension>
std::optional<typename Extension::value_type> RtpPacket::GetExtension() const {
  std::optional<typename Extension::value_type> result;
  auto raw = FindExtension(Extension::kId);
  if (raw.empty() || !Extension::Parse(raw, &result.emplace()))
    result = std::nullopt;
  return result;
}

template <typename Extension>
ArrayView<const uint8_t> RtpPacket::GetRawExtension() const {
  return FindExtension(Extension::kId);
}

template <typename Extension, typename... Values>
bool RtpPacket::SetExtension(const Values&... values) {
  const size_t value_size = Extension::ValueSize(values...);
  auto buffer = AllocateExtension(Extension::kId, value_size);
  if (buffer.empty())
    return false;
  return Extension::Write(buffer, values...);
}

template <typename Extension>
bool RtpPacket::SetRawExtension(ArrayView<const uint8_t> data) {
  ArrayView<uint8_t> buffer = AllocateExtension(Extension::kId, data.size());
  if (buffer.empty()) {
    return false;
  }
  std::memcpy(buffer.data(), data.data(), data.size());
  return true;
}

template <typename Extension>
bool RtpPacket::ReserveExtension() {
  auto buffer = AllocateExtension(Extension::kId, Extension::kValueSizeBytes);
  if (buffer.empty())
    return false;
  memset(buffer.data(), 0, Extension::kValueSizeBytes);
  return true;
}

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_PACKET_H_
