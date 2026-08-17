#ifndef MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_H_
#define MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <sstream>
#include <type_traits>

#include "absl/container/flat_hash_map.h"
#include "mediapipe/framework/deps/no_destructor.h"
#include "mediapipe/framework/tool/type_util.h"
#include "mediapipe/gpu/gpu_buffer_format.h"

namespace mediapipe {
namespace internal {

template <class... T>
struct types {};

// This template must be specialized for each view type V. Each specialization
// should define a pair of virtual methods called GetReadView and GetWriteView,
// whose first argument is a types<V> tag object. The result type and optional
// further arguments will depend on the view type.
//
// Example:
//   template <>
//   class ViewProvider<MyView> {
//    public:
//     virtual ~ViewProvider() = default;
//     virtual MyView GetReadView(types<MyView>) const = 0;
//     virtual MyView GetWriteView(types<MyView>) = 0;
//   };
//
// The additional arguments and result type are reflected in GpuBuffer's
// GetReadView and GetWriteView methods.
//
// Using a type tag for the first argument allows the methods to be overloaded,
// so that a single storage can implement provider methods for multiple views.
// Since these methods are not template methods, they can (and should) be
// virtual, which allows storage classes to override them, enforcing that all
// storages providing a given view type implement the same interface.
template <class V>
class ViewProvider;

// Generic interface for a backing storage for GpuBuffer.
//
// GpuBuffer is an opaque handle to an image. Its contents are handled by
// Storage classes. Application code does not interact with the storages
// directly; to access the data, it asks the GpuBuffer for a View, and in turn
// GpuBuffer looks for a storage that can provide that view.
// This architecture decouples application code from the underlying storage,
// making it possible to use platform-specific optimized storage systems, e.g.
// for zero-copy data sharing between CPU and GPU.
//
// Storage implementations should inherit from GpuBufferStorageImpl. See that
// class for details.
class GpuBufferStorage {
 public:
  virtual ~GpuBufferStorage() = default;

  // Concrete storage types should override the following three accessors.
  virtual int width() const = 0;
  virtual int height() const = 0;
  virtual GpuBufferFormat format() const = 0;

  // We can't use dynamic_cast since we want to support building without RTTI.
  // The public methods delegate to the type-erased private virtual method.
  template <class T>
  T* down_cast() {
    return static_cast<T*>(const_cast<void*>(down_cast(kTypeId<T>)));
  }
  template <class T>
  const T* down_cast() const {
    return static_cast<const T*>(down_cast(kTypeId<T>));
  }

  bool can_down_cast_to(TypeId to) const { return down_cast(to) != nullptr; }
  virtual TypeId storage_type() const = 0;

 private:
  virtual const void* down_cast(TypeId to) const = 0;
};

// Used to disambiguate between overloads by manually specifying their priority.
// Higher Ns will be picked first. The caller should pass overload_priority<M>
// where M is >= the largest N used in overloads (e.g. 10).
template <int N>
struct overload_priority : public overload_priority<N - 1> {};
template <>
struct overload_priority<0> {};

// Manages the available GpuBufferStorage implementations. The list of available
// implementations is built at runtime using a registration mechanism, so that
// it can be determined by the program's dependencies.
class GpuBufferStorageRegistry {
 public:
  struct RegistryToken {};

  using StorageFactory = std::function<std::shared_ptr<GpuBufferStorage>(
      int, int, GpuBufferFormat)>;
  using StorageConverter = std::function<std::shared_ptr<GpuBufferStorage>(
      std::shared_ptr<GpuBufferStorage>)>;

  static GpuBufferStorageRegistry& Get() {
    static NoDestructor<GpuBufferStorageRegistry> registry;
    return *registry;
  }

  // Registers a storage type by automatically creating a factory for it.
  // This is normally called by GpuBufferImpl.
  template <class Storage>
  RegistryToken Register() {
    return RegisterFactory<Storage>(
        [](int width, int height,
           GpuBufferFormat format) -> std::shared_ptr<Storage> {
          return CreateStorage<Storage>(overload_priority<10>{}, width, height,
                                        format);
        });
  }

  // Registers a new factory for a storage type.
  template <class Storage, class F>
  RegistryToken RegisterFactory(F&& factory) {
    if constexpr (kDisableRegistration<Storage>) {
      return {};
    }
    return Register(factory, Storage::GetProviderTypes());
  }

  // Registers a new converter from storage type StorageFrom to StorageTo.
  template <class StorageFrom, class StorageTo, class F>
  RegistryToken RegisterConverter(F&& converter) {
    if constexpr (kDisableRegistration<StorageTo>) {
      return {};
    }
    return Register(
        [converter](std::shared_ptr<GpuBufferStorage> source)
            -> std::shared_ptr<GpuBufferStorage> {
          return converter(std::static_pointer_cast<StorageFrom>(source));
        },
        StorageTo::GetProviderTypes(), kTypeId<StorageFrom>);
  }

  // Returns a factory function for a storage that implements
  // view_provider_type.
  StorageFactory StorageFactoryForViewProvider(TypeId view_provider_type);

  // Returns a conversion function that, given a storage of
  // existing_storage_type, converts its contents to a new storage that
  // implements view_provider_type.
  StorageConverter StorageConverterForViewProvider(
      TypeId view_provider_type, TypeId existing_storage_type);

 private:
  template <class Storage, class... Args>
  static auto CreateStorage(overload_priority<1>, Args... args)
      -> decltype(Storage::Create(args...)) {
    return Storage::Create(args...);
  }

  template <class Storage, class... Args>
  static auto CreateStorage(overload_priority<0>, Args... args) {
    return std::make_shared<Storage>(args...);
  }

  // Temporary workaround: a Storage class can define a static constexpr
  // kDisableGpuBufferRegistration member to true to prevent registering any
  // factory of converter that would produce it.
  // TODO: better solution for storage priorities.
  template <class Storage, typename = void>
  static constexpr bool kDisableRegistration = false;

  RegistryToken Register(StorageFactory factory,
                         std::vector<TypeId> provider_hashes);
  RegistryToken Register(StorageConverter converter,
                         std::vector<TypeId> provider_hashes,
                         TypeId source_storage);

  absl::flat_hash_map<TypeId, StorageFactory> factory_for_view_provider_;
  absl::flat_hash_map<std::pair<TypeId, TypeId>, StorageConverter>
      converter_for_view_provider_and_existing_storage_;
};

// Putting this outside the class body to work around a GCC bug.
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=71954
template <class Storage>
constexpr bool GpuBufferStorageRegistry::kDisableRegistration<
    Storage, std::void_t<decltype(&Storage::kDisableGpuBufferRegistration)>> =
    Storage::kDisableGpuBufferRegistration;

// Defining a member of this type causes P to be ODR-used, which forces its
// instantiation if it's a static member of a template.
template <auto* P>
struct ForceStaticInstantiation {
#ifdef _MSC_VER
  // Just having it as the template argument does not count as a use for
  // MSVC.
  static constexpr bool Use() { return P != nullptr; }
  char force_static[Use()];
#endif  // _MSC_VER
};

// Inherit from this class to define a new storage type. The storage type itself
// should be passed as the first template argument (CRTP), followed by one or
// more specializations of ViewProvider.
//
// Concrete storage types should implement the basic accessors from
// GpuBufferStorage, plus the view read/write getters for each ViewProvider they
// implement. This class handles the rest.
//
// Arguments:
//   T: storage type
//   U...: ViewProvider<SomeView>
// Example:
//   class MyStorage : public GpuBufferStorageImpl<
//                                MyStorage, ViewProvider<GlTextureView>>
template <class T, class... U>
class GpuBufferStorageImpl : public GpuBufferStorage, public U... {
 public:
  static const std::vector<TypeId>& GetProviderTypes() {
    static std::vector<TypeId> kProviderIds{kTypeId<U>...};
    return kProviderIds;
  }

  // Exposing this as a function allows dependent initializers to call this to
  // ensure proper ordering.
  static GpuBufferStorageRegistry::RegistryToken RegisterOnce() {
    static auto ordered_registration =
        GpuBufferStorageRegistry::Get().Register<T>();
    return ordered_registration;
  }

 private:
  // Allows a down_cast to any of the view provider types in U.
  const void* down_cast(TypeId to) const final {
    return down_cast_impl(to, types<T, U...>{});
  }
  TypeId storage_type() const final { return kTypeId<T>; }

  const void* down_cast_impl(TypeId to, types<>) const { return nullptr; }
  template <class V, class... W>
  const void* down_cast_impl(TypeId to, types<V, W...>) const {
    if (to == kTypeId<V>) return static_cast<const V*>(this);
    return down_cast_impl(to, types<W...>{});
  }

  inline static auto registration = RegisterOnce();
  using RequireStatics = ForceStaticInstantiation<&registration>;
};

#if !MEDIAPIPE_DISABLE_GPU && MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
// This function can be overridden to enable construction of a GpuBuffer from
// platform-specific types without having to expose that type in the GpuBuffer
// definition. It is only needed for backward compatibility reasons; do not add
// overrides for new types.
std::shared_ptr<internal::GpuBufferStorage> AsGpuBufferStorage();
#endif  // !MEDIAPIPE_DISABLE_GPU && MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_H_
