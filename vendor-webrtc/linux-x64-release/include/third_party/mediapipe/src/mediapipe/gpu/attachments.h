#ifndef MEDIAPIPE_GPU_ATTACHMENTS_H_
#define MEDIAPIPE_GPU_ATTACHMENTS_H_

#include <functional>
#include <memory>

namespace mediapipe {
namespace internal {

// Unique pointer with a type-erased destructor.
template <class T>
using AttachmentPtr = std::unique_ptr<T, std::function<void(void*)>>;

// Like make_unique.
template <class T, class... Args>
static std::enable_if_t<!std::is_array<T>::value, AttachmentPtr<T>>
MakeAttachmentPtr(Args&&... args) {
  return {new T(std::forward<Args>(args)...),
          [](void* ptr) { delete static_cast<T*>(ptr); }};
}

template <class Context>
class AttachmentBase {};

// An cacheable resource that can be associated with a context.
// Attachments are defined as constants.
// When access to an attachment is requested, it will be retrieved from the
// context if already created, or the factory function will be invoked to create
// it. The factory function for a given attachment is invoked at most once per
// context. The lifetime of the object it returns is managed by the context.
template <class Context, class T>
class Attachment : public AttachmentBase<Context> {
 public:
  using FactoryT = AttachmentPtr<T> (*)(Context&);
  explicit constexpr Attachment(FactoryT factory) : factory_(factory) {}

  Attachment(const Attachment&) = delete;
  Attachment(Attachment&&) = delete;
  Attachment& operator=(const Attachment&) = delete;
  Attachment& operator=(Attachment&&) = delete;

  T& Get(Context& ctx) const { return ctx.GetCachedAttachment(*this); }

  const FactoryT& factory() const { return factory_; }

  // Ptr and MakePtr here make it more convenient to define new types of
  // attachment contexts, since you only need a using declaration for Attachment
  // and can refer to Ptr from it.
  using Ptr = AttachmentPtr<T>;

  template <class... Args>
  inline static std::enable_if_t<!std::is_array<T>::value, AttachmentPtr<T>>
  MakePtr(Args&&... args) {
    return MakeAttachmentPtr<T>(std::forward<Args>(args)...);
  }

 private:
  FactoryT factory_;
};

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_ATTACHMENTS_H_
