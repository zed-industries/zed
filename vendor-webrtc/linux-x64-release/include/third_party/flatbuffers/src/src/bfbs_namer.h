#ifndef FLATBUFFERS_BFBS_NAMER
#define FLATBUFFERS_BFBS_NAMER

#include "flatbuffers/reflection.h"
#include "namer.h"

namespace flatbuffers {

// Provides Namer capabilities to types defined in the flatbuffers reflection.
class BfbsNamer : public Namer {
 public:
  explicit BfbsNamer(Config config, std::set<std::string> keywords)
      : Namer(config, std::move(keywords)) {}

  using Namer::Constant;
  using Namer::Denamespace;
  using Namer::Directories;
  using Namer::Field;
  using Namer::File;
  using Namer::Function;
  using Namer::Method;
  using Namer::Namespace;
  using Namer::NamespacedType;
  using Namer::ObjectType;
  using Namer::Type;
  using Namer::Variable;
  using Namer::Variant;

  template<typename T>
  std::string Denamespace(T t, std::string &namespace_prefix,
                          const char delimiter = '.') const {
    return Namer::Denamespace(t->name()->c_str(), namespace_prefix, delimiter);
  }

  template<typename T>
  std::string Denamespace(T t, const char delimiter = '.') const {
    return Namer::Denamespace(t->name()->c_str(), delimiter);
  }

  virtual std::string Field(const ::reflection::Field &f) const {
    return Field(f.name()->str());
  }

  virtual std::string Variable(const ::reflection::Field &f) const {
    return Variable(f.name()->str());
  }
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_BFBS_NAMER
