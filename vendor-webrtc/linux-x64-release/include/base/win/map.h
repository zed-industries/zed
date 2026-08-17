// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_MAP_H_
#define BASE_WIN_MAP_H_

#include <windows.foundation.collections.h>
#include <wrl/implements.h>

#include <map>
#include <utility>

#include "base/check_op.h"
#include "base/containers/contains.h"
#include "base/notimplemented.h"
#include "base/win/vector.h"
#include "base/win/winrt_foundation_helpers.h"

namespace base {
namespace win {

template <typename K, typename V>
class Map;

namespace internal {

// Template tricks needed to dispatch to the correct implementation.
// See base/win/winrt_foundation_helpers.h for explanation.

template <typename K, typename V>
using ComplexK =
    typename ABI::Windows::Foundation::Collections::IMap<K, V>::K_complex;

template <typename K, typename V>
using ComplexV =
    typename ABI::Windows::Foundation::Collections::IMap<K, V>::V_complex;

template <typename K, typename V>
using LogicalK = LogicalType<ComplexK<K, V>>;

template <typename K, typename V>
using LogicalV = LogicalType<ComplexV<K, V>>;

template <typename K, typename V>
using AbiK = AbiType<ComplexK<K, V>>;

template <typename K, typename V>
using AbiV = AbiType<ComplexV<K, V>>;

template <typename K, typename V>
using StorageK = StorageType<ComplexK<K, V>>;

template <typename K, typename V>
using StorageV = StorageType<ComplexV<K, V>>;

template <typename K, typename V>
class KeyValuePair : public Microsoft::WRL::RuntimeClass<
                         Microsoft::WRL::RuntimeClassFlags<
                             Microsoft::WRL::WinRtClassicComMix |
                             Microsoft::WRL::InhibitRoOriginateError>,
                         ABI::Windows::Foundation::Collections::
                             IKeyValuePair<LogicalK<K, V>, LogicalV<K, V>>> {
 public:
  using AbiK = AbiK<K, V>;
  using AbiV = AbiV<K, V>;
  using StorageK = StorageK<K, V>;
  using StorageV = StorageV<K, V>;

  KeyValuePair(StorageK key, StorageV value)
      : key_(std::move(key)), value_(std::move(value)) {}

  // ABI::Windows::Foundation::Collections::IKeyValuePair:
  IFACEMETHODIMP get_Key(AbiK* key) { return CopyTo(key_, key); }

  IFACEMETHODIMP get_Value(AbiV* value) { return CopyTo(value_, value); }

 private:
  StorageK key_;
  StorageV value_;
};

template <typename K>
class MapChangedEventArgs
    : public Microsoft::WRL::RuntimeClass<
          Microsoft::WRL::RuntimeClassFlags<
              Microsoft::WRL::WinRtClassicComMix |
              Microsoft::WRL::InhibitRoOriginateError>,
          ABI::Windows::Foundation::Collections::IMapChangedEventArgs<K>> {
 public:
  MapChangedEventArgs(
      ABI::Windows::Foundation::Collections::CollectionChange change,
      K key)
      : change_(change), key_(std::move(key)) {}

  ~MapChangedEventArgs() override = default;

  // ABI::Windows::Foundation::Collections::IMapChangedEventArgs:
  IFACEMETHODIMP get_CollectionChange(
      ABI::Windows::Foundation::Collections::CollectionChange* value) override {
    *value = change_;
    return S_OK;
  }

  IFACEMETHODIMP get_Key(K* value) override {
    *value = key_;
    return S_OK;
  }

 private:
  const ABI::Windows::Foundation::Collections::CollectionChange change_;
  K key_;
};

}  // namespace internal

// This file provides an implementation of Windows::Foundation::IMap. It
// functions as a thin wrapper around an std::map, and dispatches
// method calls to either the corresponding std::map API or
// appropriate std algorithms. Furthermore, it notifies its observers whenever
// its observable state changes, and is iterable. Please notice also that if the
// map is modified while iterating over it, iterator methods will return
// E_CHANGED_STATE. A base::win::Map can be constructed for any types <K,V>, and
// is implicitly constructible from a std::map. In the case where K or V is a
// pointer derived from IUnknown, the std::map needs to be of type
// Microsoft::WRL::ComPtr<K> or Microsoft::WRL::ComPtr<V>. This enforces proper
// reference counting and improves safety.
template <typename K, typename V>
class Map
    : public Microsoft::WRL::RuntimeClass<
          Microsoft::WRL::RuntimeClassFlags<
              Microsoft::WRL::WinRt | Microsoft::WRL::InhibitRoOriginateError>,
          ABI::Windows::Foundation::Collections::IMap<internal::LogicalK<K, V>,
                                                      internal::LogicalV<K, V>>,
          ABI::Windows::Foundation::Collections::IObservableMap<
              internal::LogicalK<K, V>,
              internal::LogicalV<K, V>>,
          ABI::Windows::Foundation::Collections::IIterable<
              ABI::Windows::Foundation::Collections::IKeyValuePair<
                  internal::LogicalK<K, V>,
                  internal::LogicalV<K, V>>*>> {
 public:
  using LogicalK = internal::LogicalK<K, V>;
  using LogicalV = internal::LogicalV<K, V>;
  using AbiK = internal::AbiK<K, V>;
  using AbiV = internal::AbiV<K, V>;
  using StorageK = internal::StorageK<K, V>;
  using StorageV = internal::StorageV<K, V>;

 private:
  class MapView;

  // Iterates over base::win::Map.
  // Its methods return E_CHANGED_STATE is the map is modified.
  // TODO(crbug.com/40637532): Refactor MapIterator to leverage
  // std::map::iterator.
  class MapIterator
      : public Microsoft::WRL::RuntimeClass<
            Microsoft::WRL::RuntimeClassFlags<
                Microsoft::WRL::WinRtClassicComMix |
                Microsoft::WRL::InhibitRoOriginateError>,
            ABI::Windows::Foundation::Collections::IIterator<
                ABI::Windows::Foundation::Collections::IKeyValuePair<
                    internal::LogicalK<K, V>,
                    internal::LogicalV<K, V>>*>> {
   public:
    explicit MapIterator(Microsoft::WRL::ComPtr<MapView> view)
        : view_(std::move(view)) {
      DCHECK(view_->ValidState());
      ConvertMapToVectorIterator();
    }

    // ABI::Windows::Foundation::Collections::IIterator:
    IFACEMETHODIMP get_Current(
        ABI::Windows::Foundation::Collections::IKeyValuePair<LogicalK,
                                                             LogicalV>**
            current) override {
      return view_->ValidState() ? iterator_->get_Current(current)
                                 : E_CHANGED_STATE;
    }

    IFACEMETHODIMP get_HasCurrent(boolean* has_current) override {
      return view_->ValidState() ? iterator_->get_HasCurrent(has_current)
                                 : E_CHANGED_STATE;
    }

    IFACEMETHODIMP MoveNext(boolean* has_current) override {
      return view_->ValidState() ? iterator_->MoveNext(has_current)
                                 : E_CHANGED_STATE;
    }

    IFACEMETHODIMP GetMany(
        unsigned capacity,
        ABI::Windows::Foundation::Collections::IKeyValuePair<LogicalK,
                                                             LogicalV>** value,
        unsigned* actual) override {
      return view_->ValidState() ? iterator_->GetMany(capacity, value, actual)
                                 : E_CHANGED_STATE;
    }

   private:
    // Helper for iteration:
    void ConvertMapToVectorIterator() {
      // Create a vector that will hold Map's key-value pairs.
      auto vector = Microsoft::WRL::Make<
          Vector<ABI::Windows::Foundation::Collections::IKeyValuePair<
              LogicalK, LogicalV>*>>();

      // Fill the vector with container data.
      for (const auto& pair : view_->get_map()) {
        auto key_value_pair =
            Microsoft::WRL::Make<internal::KeyValuePair<AbiK, AbiV>>(
                pair.first, pair.second);
        vector->Append(key_value_pair.Get());
      }

      // Return an iterator to that vector.
      // Iterator is immutable (wraps an IVectorView) and Vector's lifecycle is
      // ensured cause the view holds a reference to the vector, and iterator
      // holds a reference to the view.
      HRESULT hr = vector->First(&iterator_);
      DCHECK(SUCCEEDED(hr));
    }

    Microsoft::WRL::ComPtr<MapView> view_;
    Microsoft::WRL::ComPtr<ABI::Windows::Foundation::Collections::IIterator<
        ABI::Windows::Foundation::Collections::IKeyValuePair<LogicalK,
                                                             LogicalV>*>>
        iterator_;
  };

  class MapView
      : public Microsoft::WRL::RuntimeClass<
            Microsoft::WRL::RuntimeClassFlags<
                Microsoft::WRL::WinRtClassicComMix |
                Microsoft::WRL::InhibitRoOriginateError>,
            ABI::Windows::Foundation::Collections::
                IMapView<internal::LogicalK<K, V>, internal::LogicalV<K, V>>,
            ABI::Windows::Foundation::Collections::IIterable<
                ABI::Windows::Foundation::Collections::IKeyValuePair<
                    internal::LogicalK<K, V>,
                    internal::LogicalV<K, V>>*>,
            ABI::Windows::Foundation::Collections::MapChangedEventHandler<
                internal::LogicalK<K, V>,
                internal::LogicalV<K, V>>> {
   public:
    explicit MapView(Microsoft::WRL::ComPtr<Map<LogicalK, LogicalV>> map)
        : map_(std::move(map)) {
      map_->add_MapChanged(this, &map_changed_token_);
    }

    ~MapView() override {
      if (map_) {
        map_->remove_MapChanged(map_changed_token_);
      }
    }

    // ABI::Windows::Foundation::Collections::IMapView:
    IFACEMETHODIMP Lookup(AbiK key, AbiV* value) override {
      return map_ ? map_->Lookup(key, value) : E_CHANGED_STATE;
    }

    IFACEMETHODIMP get_Size(unsigned int* size) override {
      return map_ ? map_->get_Size(size) : E_CHANGED_STATE;
    }

    IFACEMETHODIMP HasKey(AbiK key, boolean* found) override {
      return map_ ? map_->HasKey(key, found) : E_CHANGED_STATE;
    }

    IFACEMETHODIMP Split(
        ABI::Windows::Foundation::Collections::IMapView<LogicalK, LogicalV>**
            first_partition,
        ABI::Windows::Foundation::Collections::IMapView<LogicalK, LogicalV>**
            second_partition) override {
      NOTIMPLEMENTED();
      return E_NOTIMPL;
    }

    // ABI::Windows::Foundation::Collections::IIterable:
    IFACEMETHODIMP First(
        ABI::Windows::Foundation::Collections::IIterator<
            ABI::Windows::Foundation::Collections::IKeyValuePair<LogicalK,
                                                                 LogicalV>*>**
            first) override {
      return map_ ? map_->First(first) : E_CHANGED_STATE;
    }

    // ABI::Windows::Foundation::Collections::MapChangedEventHandler:
    IFACEMETHODIMP Invoke(
        ABI::Windows::Foundation::Collections::IObservableMap<LogicalK,
                                                              LogicalV>* sender,
        ABI::Windows::Foundation::Collections::IMapChangedEventArgs<LogicalK>*
            e) override {
      DCHECK_EQ(map_.Get(), sender);
      map_.Reset();
      sender->remove_MapChanged(map_changed_token_);
      return S_OK;
    }

    // Accessor used in MapIterator for iterating over Map's container.
    // Will remain valid during the entire iteration.
    const std::map<StorageK, StorageV, internal::Less>& get_map() {
      DCHECK(map_);
      return map_->map_;
    }

    bool ValidState() const { return map_; }

   private:
    Microsoft::WRL::ComPtr<Map<LogicalK, LogicalV>> map_;
    EventRegistrationToken map_changed_token_;
  };

 public:
  Map() = default;
  explicit Map(const std::map<StorageK, StorageV, internal::Less>& map)
      : map_(map) {}
  explicit Map(std::map<StorageK, StorageV, internal::Less>&& map)
      : map_(std::move(map)) {}

  // ABI::Windows::Foundation::Collections::IMap:
  IFACEMETHODIMP Lookup(AbiK key, AbiV* value) override {
    auto it = map_.find(key);
    if (it == map_.cend()) {
      return E_BOUNDS;
    }

    return internal::CopyTo(it->second, value);
  }

  IFACEMETHODIMP get_Size(unsigned int* size) override {
    *size = map_.size();
    return S_OK;
  }

  IFACEMETHODIMP HasKey(AbiK key, boolean* found) override {
    *found = Contains(map_, key);
    return S_OK;
  }

  IFACEMETHODIMP GetView(
      ABI::Windows::Foundation::Collections::IMapView<LogicalK, LogicalV>**
          view) override {
    return Microsoft::WRL::Make<MapView>(this).CopyTo(view);
  }

  IFACEMETHODIMP Insert(AbiK key, AbiV value, boolean* replaced) override {
    auto [it, inserted] = map_.insert_or_assign(key, std::move(value));
    *replaced = !inserted;
    NotifyMapChanged(*replaced ? ABI::Windows::Foundation::Collections::
                                     CollectionChange_ItemChanged
                               : ABI::Windows::Foundation::Collections::
                                     CollectionChange_ItemInserted,
                     key);
    return S_OK;
  }

  IFACEMETHODIMP Remove(AbiK key) override {
    if (!map_.erase(key)) {
      return E_BOUNDS;
    }

    NotifyMapChanged(
        ABI::Windows::Foundation::Collections::CollectionChange_ItemRemoved,
        key);
    return S_OK;
  }

  IFACEMETHODIMP Clear() override {
    map_.clear();
    NotifyMapChanged(
        ABI::Windows::Foundation::Collections::CollectionChange_Reset, 0);
    return S_OK;
  }

  // ABI::Windows::Foundation::Collections::IObservableMap:
  IFACEMETHODIMP add_MapChanged(
      ABI::Windows::Foundation::Collections::MapChangedEventHandler<LogicalK,
                                                                    LogicalV>*
          handler,
      EventRegistrationToken* token) override {
    token->value = handler_id_++;
    handlers_.emplace_hint(handlers_.end(), token->value, handler);
    return S_OK;
  }

  IFACEMETHODIMP remove_MapChanged(EventRegistrationToken token) override {
    return handlers_.erase(token.value) ? S_OK : E_BOUNDS;
  }

  // ABI::Windows::Foundation::Collections::IIterable:
  IFACEMETHODIMP First(
      ABI::Windows::Foundation::Collections::IIterator<
          ABI::Windows::Foundation::Collections::IKeyValuePair<LogicalK,
                                                               LogicalV>*>**
          first) override {
    return Microsoft::WRL::Make<MapIterator>(
               Microsoft::WRL::Make<MapView>(this))
        .CopyTo(first);
  }

 private:
  ~Map() override {
    // Handlers should not outlive the Map. Furthermore, they must ensure
    // they are unregistered before the handler is destroyed. This implies
    // there should be no handlers left when the Map is destructed.
    DCHECK(handlers_.empty());
  }

  void NotifyMapChanged(
      ABI::Windows::Foundation::Collections::CollectionChange change,
      AbiK key) {
    auto args =
        Microsoft::WRL::Make<internal::MapChangedEventArgs<AbiK>>(change, key);

    // Invoking the handlers could result in mutations to the map, thus we make
    // a copy beforehand.
    auto handlers = handlers_;
    for (auto& handler : handlers) {
      handler.second->Invoke(this, args.Get());
    }
  }

  std::map<StorageK, StorageV, internal::Less> map_;
  base::flat_map<int64_t,
                 ABI::Windows::Foundation::Collections::
                     MapChangedEventHandler<LogicalK, LogicalV>*>
      handlers_;
  int64_t handler_id_ = 0;
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_MAP_H_
