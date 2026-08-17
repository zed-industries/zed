// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZATION_TAG_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZATION_TAG_H_

#include <cstdint>

namespace blink {

// Serialization format is a sequence of tags followed by zero or more data
// arguments.  Tags always take exactly one byte. A serialized stream first
// begins with a complete VersionTag. If the stream does not begin with a
// VersionTag, we assume that the stream is in format 0.

// Tags which are not interpreted by Blink (but instead by V8) are omitted here.

// This format is private to the implementation of SerializedScriptValue. Do not
// rely on it externally. It is safe to persist a SerializedScriptValue as a
// binary blob, but this code should always be used to interpret it.

// WebCoreStrings are read as (length:uint32_t, string:UTF8[length]).
// RawStrings are read as (length:uint32_t, string:UTF8[length]).
// RawUCharStrings are read as
//     (length:uint32_t, string:UChar[length/sizeof(UChar)]).
// RawFiles are read as
//     (path:WebCoreString, url:WebCoreStrng, type:WebCoreString).
// There is a reference table that maps object references (uint32_t) to
// v8::Values.
// Tokens marked with (ref) are inserted into the reference table and given the
// next object reference ID after decoding.
// All tags except InvalidTag, PaddingTag, ReferenceCountTag, VersionTag,
// GenerateFreshObjectTag and GenerateFreshArrayTag push their results to the
// deserialization stack.
// There is also an 'open' stack that is used to resolve circular references.
// Objects or arrays may contain self-references. Before we begin to deserialize
// the contents of these values, they are first given object reference IDs (by
// GenerateFreshObjectTag/GenerateFreshArrayTag); these reference IDs are then
// used with ObjectReferenceTag to tie the recursive knot.
enum SerializationTag : uint8_t {
  kMessagePortTag = 'M',  // index:int -> MessagePort. Fills the result with
                          // transferred MessagePort.
  kMojoHandleTag = 'h',   // index:int -> MojoHandle. Fills the result with
                          // transferred MojoHandle.
  kBlobTag = 'b',  // uuid:WebCoreString, type:WebCoreString, size:uint64_t ->
                   // Blob (ref)
  kBlobIndexTag = 'i',             // index:int32_t -> Blob (ref)
  kFileTag = 'f',                  // file:RawFile -> File (ref)
  kFileIndexTag = 'e',             // index:int32_t -> File (ref)
  kDOMFileSystemTag = 'd',         // type:int32_t, name:WebCoreString,
                                   // uuid:WebCoreString -> FileSystem (ref)
  kFileSystemFileHandleTag = 'n',  // name:WebCoreString, index:uint32_t
                                   // -> FileSystemFileHandle (ref)
  kFileSystemDirectoryHandleTag = 'N',  // name:WebCoreString, index:uint32_t ->
                                        // FileSystemDirectoryHandle (ref)
  kFileListTag =
      'l',  // length:uint32_t, files:RawFile[length] -> FileList (ref)
  kFileListIndexTag =
      'L',  // length:uint32_t, files:int32_t[length] -> FileList (ref)
  kImageDataTag = '#',    // tags terminated by ImageSerializationTag::kEnd (see
                          // SerializedColorParams.h), width:uint32_t,
                          // height:uint32_t, pixelDataLength:uint64_t,
                          // data:byte[pixelDataLength]
                          // -> ImageData (ref)
  kImageBitmapTag = 'g',  // tags terminated by ImageSerializationTag::kEnd (see
                          // SerializedColorParams.h), width:uint32_t,
                          // height:uint32_t, pixelDataLength:uint32_t,
                          // data:byte[pixelDataLength]
                          // -> ImageBitmap (ref)
  kImageBitmapTransferTag =
      'G',  // index:uint32_t -> ImageBitmap. For ImageBitmap transfer
  kOffscreenCanvasTransferTag = 'H',  // index, width, height, id,
                                      // filter_quality::uint32_t ->
                                      // OffscreenCanvas. For OffscreenCanvas
                                      // transfer
  kReadableStreamTransferTag = 'r',   // index:uint32_t
  kTransformStreamTransferTag = 'm',  // index:uint32_t
  kWritableStreamTransferTag = 'w',   // index:uint32_t
  kMediaStreamTrack =
      's',  // trackImplSubtype:Uint32Enum, session_id.high:uint64_t,
            // session_id.low:uint64_t, transfer_id.high:uint64_t,
            // transfer_id.low:uint64_t, kind:WebCoreString, id:WebCoreString,
            // label:WebCoreString, enabled:byte, muted:byte,
            // contentHint:Uint32Enum, readyState:Uint32Enum
            // If trackImplSubtype=BrowserCapture: cropVersion:uint32_t
  kDOMPointTag = 'Q',          // x:Double, y:Double, z:Double, w:Double
  kDOMPointReadOnlyTag = 'W',  // x:Double, y:Double, z:Double, w:Double
  kDOMRectTag = 'E',          // x:Double, y:Double, width:Double, height:Double
  kDOMRectReadOnlyTag = 'R',  // x:Double, y:Double, width:Double, height:Double
  kDOMQuadTag = 'T',          // p1:Double, p2:Double, p3:Double, p4:Double
  kDOMMatrixTag = 'Y',        // m11..m44: 16 Double
  kDOMMatrixReadOnlyTag = 'U',    // m11..m44: 16 Double
  kDOMMatrix2DTag = 'I',          // a..f: 6 Double
  kDOMMatrix2DReadOnlyTag = 'O',  // a..f: 6 Double
  kCryptoKeyTag = 'K',            // subtag:byte, props, usages:uint32_t,
                        // keyDataLength:uint32_t, keyData:byte[keyDataLength]
  //                 If subtag=AesKeyTag:
  //                     props = keyLengthBytes:uint32_t, algorithmId:uint32_t
  //                 If subtag=HmacKeyTag:
  //                     props = keyLengthBytes:uint32_t, hashId:uint32_t
  //                 If subtag=RsaHashedKeyTag:
  //                     props = algorithmId:uint32_t, type:uint32_t,
  //                     modulusLengthBits:uint32_t,
  //                     publicExponentLength:uint32_t,
  //                     publicExponent:byte[publicExponentLength],
  //                     hashId:uint32_t
  //                 If subtag=EcKeyTag:
  //                     props = algorithmId:uint32_t, type:uint32_t,
  //                     namedCurve:uint32_t
  kRTCCertificateTag = 'k',  // length:uint32_t, pemPrivateKey:WebCoreString,
                             // pemCertificate:WebCoreString
  kRTCEncodedAudioFrameTag = 'A',  // uint32_t -> transferred audio frame ID
  kRTCEncodedVideoFrameTag = 'V',  // uint32_t -> transferred video frame ID
  kRTCDataChannel = 'p',  // uint32_t -> transferred webrtc datachannel ID

  kAudioDataTag = 'a',          // uint32_t -> transferred audio data
  kVideoFrameTag = 'v',         // uint32_t -> transferred video frame ID
  kEncodedAudioChunkTag = 'y',  // uint32_t -> transferred chunk
  kEncodedVideoChunkTag = 'z',  // uint32_t -> transferred chunk

  kCropTargetTag = 'c',         // crop_id:WebCoreString
  kRestrictionTargetTag = 'D',  // restriction_id:WebCoreString

  kMediaSourceHandleTag = 'S',  // uint32_t -> transferred MediaSourceHandle

  // The following tags were used by the Shape Detection API implementation
  // between M71 and M81. During these milestones, the API was always behind
  // a flag. Usage was removed in https://crrev.com/c/2040378.
  kDeprecatedDetectedBarcodeTag = 'B',
  kDeprecatedDetectedFaceTag = 'F',
  kDeprecatedDetectedTextTag = 't',

  kFencedFrameConfigTag = 'C',

  kDOMExceptionTag = 'x',  // name:String,message:String,stack:String
  kTrailerOffsetTag =
      0xFE,  // offset:uint64_t (fixed width, network order) from buffer start
             // size:uint32_t (fixed width, network order)
  kVersionTag = 0xFF,  // version:uint32_t -> Uses this as the file version.

  // Tags used in trailers.
  kTrailerRequiresInterfacesTag = 0xA0,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZATION_TAG_H_
