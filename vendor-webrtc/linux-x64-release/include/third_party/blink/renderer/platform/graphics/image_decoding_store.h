/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_DECODING_STORE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_DECODING_STORE_H_

#include <memory>
#include <utility>

#include "base/check_op.h"
#include "base/memory/memory_pressure_listener.h"
#include "base/memory/ptr_util.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/synchronization/lock.h"
#include "cc/paint/paint_image_generator.h"
#include "third_party/blink/renderer/platform/graphics/image_frame_generator.h"
#include "third_party/blink/renderer/platform/graphics/skia/sk_size_hash.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/doubly_linked_list.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkSize.h"
#include "third_party/skia/include/core/SkTypes.h"

namespace blink {

// Decoder cache entry is identified by:
// 1. Pointer to ImageFrameGenerator.
// 2. Size of the image.
// 3. ImageDecoder::AlphaOption
struct DecoderCacheKey {
  raw_ptr<const blink::ImageFrameGenerator> gen_;
  SkISize size_;
  blink::ImageDecoder::AlphaOption alpha_option_;
  cc::PaintImage::GeneratorClientId client_id_;

  DecoderCacheKey()
      : gen_(nullptr),
        size_(SkISize::Make(0, 0)),
        alpha_option_(static_cast<blink::ImageDecoder::AlphaOption>(0)),
        client_id_(cc::PaintImage::kDefaultGeneratorClientId) {}
};

static inline bool operator==(const DecoderCacheKey& a,
                              const DecoderCacheKey& b) {
  return a.gen_ == b.gen_ && a.size_ == b.size_ &&
         a.alpha_option_ == b.alpha_option_ && a.client_id_ == b.client_id_;
}

static inline bool operator!=(const DecoderCacheKey& a,
                              const DecoderCacheKey& b) {
  return !(a == b);
}

// Base class for all cache entries.
class CacheEntry : public DoublyLinkedListNode<CacheEntry> {
  USING_FAST_MALLOC(CacheEntry);
  friend class WTF::DoublyLinkedListNode<CacheEntry>;

 public:
  enum CacheType {
    kTypeDecoder,
  };

  CacheEntry(const ImageFrameGenerator* generator, int use_count)
      : generator_(generator),
        use_count_(use_count),
        prev_(nullptr),
        next_(nullptr) {}
  CacheEntry(const CacheEntry&) = delete;
  CacheEntry& operator=(const CacheEntry&) = delete;

  virtual ~CacheEntry() { DCHECK(!use_count_); }

  const ImageFrameGenerator* Generator() const { return generator_; }
  int UseCount() const { return use_count_; }
  void IncrementUseCount() { ++use_count_; }
  void DecrementUseCount() {
    --use_count_;
    DCHECK_GE(use_count_, 0);
  }

  // FIXME: getSafeSize() returns the size in bytes truncated to a 32-bit
  // integer. Find a way to get the size in 64-bits.
  virtual size_t MemoryUsageInBytes() const = 0;
  virtual CacheType GetType() const = 0;

 protected:
  raw_ptr<const ImageFrameGenerator> generator_;
  int use_count_;

 private:
  // RAW_PTR_EXCLUSION: Rewriting causes a crash, because a base class ctor
  // accesses child class ptr fields before they're initialized (see
  // crbug.com/349213429).
  RAW_PTR_EXCLUSION CacheEntry* prev_;
  RAW_PTR_EXCLUSION CacheEntry* next_;
};

class DecoderCacheEntry final : public CacheEntry {
 public:
  DecoderCacheEntry(const ImageFrameGenerator* generator,
                    int count,
                    std::unique_ptr<ImageDecoder> decoder,
                    cc::PaintImage::GeneratorClientId client_id)
      : CacheEntry(generator, count),
        cached_decoder_(std::move(decoder)),
        size_(SkISize::Make(cached_decoder_->DecodedSize().width(),
                            cached_decoder_->DecodedSize().height())),
        alpha_option_(cached_decoder_->GetAlphaOption()),
        client_id_(client_id) {}

  size_t MemoryUsageInBytes() const override {
    return size_.width() * size_.height() * 4;
  }
  CacheType GetType() const override { return kTypeDecoder; }

  static DecoderCacheKey MakeCacheKey(
      const ImageFrameGenerator* generator,
      const SkISize& size,
      ImageDecoder::AlphaOption alpha_option,
      cc::PaintImage::GeneratorClientId client_id) {
    DecoderCacheKey key;
    key.gen_ = generator;
    key.size_ = size;
    key.alpha_option_ = alpha_option;
    key.client_id_ = client_id;
    return key;
  }
  static DecoderCacheKey MakeCacheKey(
      const ImageFrameGenerator* generator,
      const ImageDecoder* decoder,
      cc::PaintImage::GeneratorClientId client_id) {
    return MakeCacheKey(generator,
                        SkISize::Make(decoder->DecodedSize().width(),
                                      decoder->DecodedSize().height()),
                        decoder->GetAlphaOption(), client_id);
  }
  DecoderCacheKey CacheKey() const {
    return MakeCacheKey(generator_, size_, alpha_option_, client_id_);
  }
  ImageDecoder* CachedDecoder() const { return cached_decoder_.get(); }

 private:
  std::unique_ptr<ImageDecoder> cached_decoder_;
  SkISize size_;
  ImageDecoder::AlphaOption alpha_option_;
  cc::PaintImage::GeneratorClientId client_id_;
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::DecoderCacheKey>
    : GenericHashTraits<blink::DecoderCacheKey> {
  STATIC_ONLY(HashTraits);
  static unsigned GetHash(const blink::DecoderCacheKey& p) {
    auto first = HashInts(
        WTF::GetHash(const_cast<blink::ImageFrameGenerator*>(p.gen_.get())),
        WTF::GetHash(p.size_));
    auto second = HashInts(WTF::GetHash(static_cast<uint8_t>(p.alpha_option_)),
                           p.client_id_);
    return HashInts(first, second);
  }

  static const bool kEmptyValueIsZero = true;
  static blink::DecoderCacheKey EmptyValue() {
    return blink::DecoderCacheEntry::MakeCacheKey(
        nullptr, SkISize::Make(0, 0),
        static_cast<blink::ImageDecoder::AlphaOption>(0),
        cc::PaintImage::kDefaultGeneratorClientId);
  }
  static blink::DecoderCacheKey DeletedValue() {
    return blink::DecoderCacheEntry::MakeCacheKey(
        nullptr, SkISize::Make(-1, -1),
        static_cast<blink::ImageDecoder::AlphaOption>(0),
        cc::PaintImage::kDefaultGeneratorClientId);
  }
};

}  // namespace WTF

namespace blink {

// FUNCTION
//
// ImageDecodingStore is a class used to manage cached decoder objects.
//
// EXTERNAL OBJECTS
//
// ImageDecoder
//   A decoder object. It is used to decode raw data into bitmap images.
//
// ImageFrameGenerator
//   This is a direct user of this cache. Responsible for generating bitmap
//   images using an ImageDecoder. It contains encoded image data and is used
//   to represent one image file. It is used to index image and decoder
//   objects in the cache.
//
// THREAD SAFETY
//
// All public methods can be used on any thread.

class PLATFORM_EXPORT ImageDecodingStore final {
  USING_FAST_MALLOC(ImageDecodingStore);

 public:
  ImageDecodingStore();
  ImageDecodingStore(const ImageDecodingStore&) = delete;
  ImageDecodingStore& operator=(const ImageDecodingStore&) = delete;
  ~ImageDecodingStore();

  static ImageDecodingStore& Instance();

  // Accesses a cached decoder object. A decoder is indexed by origin
  // (ImageFrameGenerator) and scaled size. Returns true if the cached object
  // is found.
  bool LockDecoder(const ImageFrameGenerator*,
                   const SkISize& scaled_size,
                   ImageDecoder::AlphaOption,
                   cc::PaintImage::GeneratorClientId client_id,
                   ImageDecoder**);
  void UnlockDecoder(const ImageFrameGenerator*,
                     cc::PaintImage::GeneratorClientId client_id,
                     const ImageDecoder*);
  void InsertDecoder(const ImageFrameGenerator*,
                     cc::PaintImage::GeneratorClientId client_id,
                     std::unique_ptr<ImageDecoder>);
  void RemoveDecoder(const ImageFrameGenerator*,
                     cc::PaintImage::GeneratorClientId client_id,
                     const ImageDecoder*);

  // Remove all cache entries indexed by ImageFrameGenerator.
  void RemoveCacheIndexedByGenerator(const ImageFrameGenerator*);

  void Clear();
  void SetCacheLimitInBytes(size_t);
  size_t MemoryUsageInBytes();
  int CacheEntries();
  int DecoderCacheEntries();

 private:
  void Prune();

  // Called by the memory pressure listener when the memory pressure rises.
  void OnMemoryPressure(
      base::MemoryPressureListener::MemoryPressureLevel level);

  // These helper methods are called while |lock_| is held.
  template <class T, class U, class V>
  void InsertCacheInternal(std::unique_ptr<T> cache_entry,
                           U* cache_map,
                           V* identifier_map) EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Helper method to remove a cache entry. Ownership is transferred to
  // deletionList. Use of Vector<> is handy when removing multiple entries.
  template <class T, class U, class V>
  void RemoveFromCacheInternal(
      const T* cache_entry,
      U* cache_map,
      V* identifier_map,
      Vector<std::unique_ptr<CacheEntry>>* deletion_list)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Helper method to remove a cache entry. Uses the templated version base on
  // the type of cache entry.
  void RemoveFromCacheInternal(
      const CacheEntry*,
      Vector<std::unique_ptr<CacheEntry>>* deletion_list)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Helper method to remove all cache entries associated with an
  // ImageFrameGenerator. Ownership of the cache entries is transferred to
  // |deletionList|.
  template <class U, class V>
  void RemoveCacheIndexedByGeneratorInternal(
      U* cache_map,
      V* identifier_map,
      const ImageFrameGenerator*,
      Vector<std::unique_ptr<CacheEntry>>* deletion_list)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Helper method to remove cache entry pointers from the LRU list.
  void RemoveFromCacheListInternal(
      const Vector<std::unique_ptr<CacheEntry>>& deletion_list)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // A doubly linked list that maintains usage history of cache entries.
  // This is used for eviction of old entries.
  // Head of this list is the least recently used cache entry.
  // Tail of this list is the most recently used cache entry.
  DoublyLinkedList<CacheEntry> ordered_cache_list_ GUARDED_BY(lock_);

  // A lookup table for all decoder cache objects. Owns all decoder cache
  // objects.
  typedef HashMap<DecoderCacheKey, std::unique_ptr<DecoderCacheEntry>>
      DecoderCacheMap;
  DecoderCacheMap decoder_cache_map_ GUARDED_BY(lock_);

  // A lookup table to map ImageFrameGenerator to all associated
  // decoder cache keys.
  typedef HashSet<DecoderCacheKey> DecoderCacheKeySet;
  typedef HashMap<const ImageFrameGenerator*, DecoderCacheKeySet>
      DecoderCacheKeyMap;
  DecoderCacheKeyMap decoder_cache_key_map_ GUARDED_BY(lock_);

  size_t heap_limit_in_bytes_ GUARDED_BY(lock_);
  size_t heap_memory_usage_in_bytes_ GUARDED_BY(lock_);

  // A listener to global memory pressure events.
  base::MemoryPressureListener memory_pressure_listener_;

  // Also protects:
  // - the CacheEntry in |decoder_cache_map_|.
  // - calls to underlying skBitmap's LockPixels()/UnlockPixels() as they are
  //   not threadsafe.
  base::Lock lock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_DECODING_STORE_H_
