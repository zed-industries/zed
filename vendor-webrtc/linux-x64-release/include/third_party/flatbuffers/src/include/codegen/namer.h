#ifndef FLATBUFFERS_INCLUDE_CODEGEN_NAMER_H_
#define FLATBUFFERS_INCLUDE_CODEGEN_NAMER_H_

#include "flatbuffers/util.h"

namespace flatbuffers {

// Options for Namer::File.
enum class SkipFile {
  None = 0,
  Suffix = 1,
  Extension = 2,
  SuffixAndExtension = 3,
};
inline SkipFile operator&(SkipFile a, SkipFile b) {
  return static_cast<SkipFile>(static_cast<int>(a) & static_cast<int>(b));
}
// Options for Namer::Directories
enum class SkipDir {
  None = 0,
  // Skip prefixing the -o $output_path.
  OutputPath = 1,
  // Skip trailing path seperator.
  TrailingPathSeperator = 2,
  OutputPathAndTrailingPathSeparator = 3,
};
inline SkipDir operator&(SkipDir a, SkipDir b) {
  return static_cast<SkipDir>(static_cast<int>(a) & static_cast<int>(b));
}

// `Namer` applies style configuration to symbols in generated code. It manages
// casing, escapes keywords, and object API naming.
// TODO: Refactor all code generators to use this.
class Namer {
 public:
  struct Config {
    // Symbols in code.

    // Case style for flatbuffers-defined types.
    // e.g. `class TableA {}`
    Case types;
    // Case style for flatbuffers-defined constants.
    // e.g. `uint64_t ENUM_A_MAX`;
    Case constants;
    // Case style for flatbuffers-defined methods.
    // e.g. `class TableA { int field_a(); }`
    Case methods;
    // Case style for flatbuffers-defined functions.
    // e.g. `TableA* get_table_a_root()`;
    Case functions;
    // Case style for flatbuffers-defined fields.
    // e.g. `struct Struct { int my_field; }`
    Case fields;
    // Case style for flatbuffers-defined variables.
    // e.g. `int my_variable = 2`
    Case variables;
    // Case style for flatbuffers-defined variants.
    // e.g. `enum class Enum { MyVariant, }`
    Case variants;
    // Seperator for qualified enum names.
    // e.g. `Enum::MyVariant` uses `::`.
    std::string enum_variant_seperator;

    // Configures, when formatting code, whether symbols are checked against
    // keywords and escaped before or after case conversion. It does not make
    // sense to do so before, but its legacy behavior. :shrug:
    // TODO(caspern): Deprecate.
    enum class Escape {
      BeforeConvertingCase,
      AfterConvertingCase,
    };
    Escape escape_keywords;

    // Namespaces

    // e.g. `namespace my_namespace {}`
    Case namespaces;
    // The seperator between namespaces in a namespace path.
    std::string namespace_seperator;

    // Object API.
    // Native versions flatbuffers types have this prefix.
    // e.g. "" (it's usually empty string)
    std::string object_prefix;
    // Native versions flatbuffers types have this suffix.
    // e.g. "T"
    std::string object_suffix;

    // Keywords.
    // Prefix used to escape keywords. It is usually empty string.
    std::string keyword_prefix;
    // Suffix used to escape keywords. It is usually "_".
    std::string keyword_suffix;

    // Files.

    // Case style for filenames. e.g. `foo_bar_generated.rs`
    Case filenames;
    // Case style for directories, e.g. `output_files/foo_bar/baz/`
    Case directories;
    // The directory within which we will generate files.
    std::string output_path;
    // Suffix for generated file names, e.g. "_generated".
    std::string filename_suffix;
    // Extension for generated files, e.g. ".cpp" or ".rs".
    std::string filename_extension;
  };
  Namer(Config config, std::set<std::string> keywords)
      : config_(config), keywords_(std::move(keywords)) {}

  virtual ~Namer() {}

  template<typename T> std::string Method(const T &s) const {
    return Method(s.name);
  }

  virtual std::string Method(const std::string &pre,
                             const std::string &mid,
                             const std::string &suf) const {
    return Format(pre + "_" +  mid + "_" + suf, config_.methods);
  }
  virtual std::string Method(const std::string &pre,
                             const std::string &suf) const {
    return Format(pre + "_" + suf, config_.methods);
  }
  virtual std::string Method(const std::string &s) const {
    return Format(s, config_.methods);
  }

  virtual std::string Constant(const std::string &s) const {
    return Format(s, config_.constants);
  }

  virtual std::string Function(const std::string &s) const {
    return Format(s, config_.functions);
  }

  virtual std::string Variable(const std::string &s) const {
    return Format(s, config_.variables);
  }

  template<typename T>
  std::string Variable(const std::string &p, const T &s) const {
    return Format(p + "_" + s.name, config_.variables);
  }
  virtual std::string Variable(const std::string &p,
                               const std::string &s) const {
    return Format(p + "_" + s, config_.variables);
  }

  virtual std::string Namespace(const std::string &s) const {
    return Format(s, config_.namespaces);
  }

  virtual std::string Namespace(const std::vector<std::string> &ns) const {
    std::string result;
    for (auto it = ns.begin(); it != ns.end(); it++) {
      if (it != ns.begin()) result += config_.namespace_seperator;
      result += Namespace(*it);
    }
    return result;
  }

  virtual std::string NamespacedType(const std::vector<std::string> &ns,
                                     const std::string &s) const {
    return (ns.empty() ? "" : (Namespace(ns) + config_.namespace_seperator)) +
           Type(s);
  }

  // Returns `filename` with the right casing, suffix, and extension.
  virtual std::string File(const std::string &filename,
                           SkipFile skips = SkipFile::None) const {
    const bool skip_suffix = (skips & SkipFile::Suffix) != SkipFile::None;
    const bool skip_ext = (skips & SkipFile::Extension) != SkipFile::None;
    return ConvertCase(filename, config_.filenames, Case::kUpperCamel) +
           (skip_suffix ? "" : config_.filename_suffix) +
           (skip_ext ? "" : config_.filename_extension);
  }
  template<typename T>
  std::string File(const T &f, SkipFile skips = SkipFile::None) const {
    return File(f.name, skips);
  }

  // Formats `directories` prefixed with the output_path and joined with the
  // right seperator. Output path prefixing and the trailing separator may be
  // skiped using `skips`.
  // Callers may want to use `EnsureDirExists` with the result.
  // input_case is used to tell how to modify namespace. e.g. kUpperCamel will
  // add a underscode between case changes, so MyGame turns into My_Game
  // (depending also on the output_case).
  virtual std::string Directories(const std::vector<std::string> &directories,
                                  SkipDir skips = SkipDir::None,
                                  Case input_case = Case::kUpperCamel) const {
    const bool skip_output_path =
        (skips & SkipDir::OutputPath) != SkipDir::None;
    const bool skip_trailing_seperator =
        (skips & SkipDir::TrailingPathSeperator) != SkipDir::None;
    std::string result = skip_output_path ? "" : config_.output_path;
    for (auto d = directories.begin(); d != directories.end(); d++) {
      result += ConvertCase(*d, config_.directories, input_case);
      result.push_back(kPathSeparator);
    }
    if (skip_trailing_seperator && !result.empty()) result.pop_back();
    return result;
  }

  virtual std::string EscapeKeyword(const std::string &name) const {
    if (keywords_.find(name) == keywords_.end()) {
      return name;
    } else {
      return config_.keyword_prefix + name + config_.keyword_suffix;
    }
  }

  virtual std::string Type(const std::string &s) const {
    return Format(s, config_.types);
  }
  virtual std::string Type(const std::string &t, const std::string &s) const {
    return Format(t + "_" + s, config_.types);
  }

  virtual std::string ObjectType(const std::string &s) const {
    return config_.object_prefix + Type(s) + config_.object_suffix;
  }

  virtual std::string Field(const std::string &s) const {
    return Format(s, config_.fields);
  }

  virtual std::string Variant(const std::string &s) const {
    return Format(s, config_.variants);
  }

  virtual std::string Format(const std::string &s, Case casing) const {
    if (config_.escape_keywords == Config::Escape::BeforeConvertingCase) {
      return ConvertCase(EscapeKeyword(s), casing, Case::kLowerCamel);
    } else {
      return EscapeKeyword(ConvertCase(s, casing, Case::kLowerCamel));
    }
  }

  // Denamespaces a string (e.g. The.Quick.Brown.Fox) by returning the last part
  // after the `delimiter` (Fox) and placing the rest in `namespace_prefix`
  // (The.Quick.Brown).
  virtual std::string Denamespace(const std::string &s,
                                  std::string &namespace_prefix,
                                  const char delimiter = '.') const {
    const size_t pos = s.find_last_of(delimiter);
    if (pos == std::string::npos) {
      namespace_prefix = "";
      return s;
    }
    namespace_prefix = s.substr(0, pos);
    return s.substr(pos + 1);
  }

  // Same as above, but disregards the prefix.
  virtual std::string Denamespace(const std::string &s,
                                  const char delimiter = '.') const {
    std::string prefix;
    return Denamespace(s, prefix, delimiter);
  }

  const Config config_;
  const std::set<std::string> keywords_;
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_INCLUDE_CODEGEN_NAMER_H_
