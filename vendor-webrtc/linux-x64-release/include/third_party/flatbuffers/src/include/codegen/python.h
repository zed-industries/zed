#ifndef FLATBUFFERS_INCLUDE_CODEGEN_PYTHON_H_
#define FLATBUFFERS_INCLUDE_CODEGEN_PYTHON_H_

#include <cstdint>
#include <set>
#include <string>
#include <vector>

#include "codegen/namer.h"

namespace flatbuffers {
namespace python {
static const Namer::Config kConfig = {
    /*types=*/Case::kKeep,
    /*constants=*/Case::kScreamingSnake,
    /*methods=*/Case::kUpperCamel,
    /*functions=*/Case::kUpperCamel,
    /*fields=*/Case::kLowerCamel,
    /*variable=*/Case::kLowerCamel,
    /*variants=*/Case::kKeep,
    /*enum_variant_seperator=*/".",
    /*escape_keywords=*/Namer::Config::Escape::AfterConvertingCase,
    /*namespaces=*/Case::kKeep,  // Packages in python.
    /*namespace_seperator=*/".",
    /*object_prefix=*/"",
    /*object_suffix=*/"T",
    /*keyword_prefix=*/"",
    /*keyword_suffix=*/"_",
    /*filenames=*/Case::kKeep,
    /*directories=*/Case::kKeep,
    /*output_path=*/"",
    /*filename_suffix=*/"",
    /*filename_extension=*/".py",
};

static const Namer::Config kStubConfig = {
    /*types=*/Case::kKeep,
    /*constants=*/Case::kScreamingSnake,
    /*methods=*/Case::kUpperCamel,
    /*functions=*/Case::kUpperCamel,
    /*fields=*/Case::kLowerCamel,
    /*variables=*/Case::kLowerCamel,
    /*variants=*/Case::kKeep,
    /*enum_variant_seperator=*/".",
    /*escape_keywords=*/Namer::Config::Escape::AfterConvertingCase,
    /*namespaces=*/Case::kKeep,  // Packages in python.
    /*namespace_seperator=*/".",
    /*object_prefix=*/"",
    /*object_suffix=*/"T",
    /*keyword_prefix=*/"",
    /*keyword_suffix=*/"_",
    /*filenames=*/Case::kKeep,
    /*directories=*/Case::kKeep,
    /*output_path=*/"",
    /*filename_suffix=*/"",
    /*filename_extension=*/".pyi",
};

// `Version` represent a Python version.
//
// The zero value (i.e. `Version{}`) represents both Python2 and Python3.
//
// https://docs.python.org/3/faq/general.html#how-does-the-python-version-numbering-scheme-work
struct Version {
  explicit Version(const std::string &version);

  bool IsValid() const;

  int16_t major = 0;
  int16_t minor = 0;
  int16_t micro = 0;
};

std::set<std::string> Keywords(const Version &version);

struct Import {
  bool IsLocal() const { return module == "."; }

  std::string module;
  std::string name;
};

struct Imports {
  const python::Import &Import(const std::string &module);
  const python::Import &Import(const std::string &module,
                               const std::string &name);

  std::vector<python::Import> imports;
};
}  // namespace python
}  // namespace flatbuffers

#endif  // FLATBUFFERS_INCLUDE_CODEGEN_PYTHON_H_
