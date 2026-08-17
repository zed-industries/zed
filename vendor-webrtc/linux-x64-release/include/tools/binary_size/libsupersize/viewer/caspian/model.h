// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_MODEL_H_
#define TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_MODEL_H_

#include <stdint.h>
#include <stdlib.h>

#include <array>
#include <deque>
#include <functional>
#include <map>
#include <ostream>
#include <string>
#include <string_view>
#include <vector>

#include "third_party/jsoncpp/source/include/json/json.h"
#include "tools/binary_size/libsupersize/viewer/caspian/grouped_path.h"

// Copied from representation in tools/binary_size/libsupersize/models.py

namespace caspian {

constexpr char kStringLiteralName[] = "string literal";

enum class ArtifactType : char {
  kSymbol = '\0',
  kDirectory = 'D',
  kGroup = 'G',
  kFile = 'F',
  kJavaClass = 'J',
};

enum class SectionId : char {
  // kNone is unused except for default-initializing in containers
  kNone = '\0',
  kArsc = 'a',
  kBss = 'b',
  kData = 'd',
  kDataRelRo = 'R',
  kDex = 'x',
  kDexMethod = 'm',
  kOther = 'o',
  kRoData = 'r',
  kText = 't',
  kPakNontranslated = 'P',
  kPakTranslations = 'p',
};

enum class DiffStatus : uint8_t {
  kUnchanged = 0,
  kChanged = 1,
  kAdded = 2,
  kRemoved = 3,
};

class SymbolFlag {
 public:
  static const int32_t kAnonymous = 1;
  static const int32_t kStartup = 2;
  static const int32_t kUnlikely = 4;
  static const int32_t kRel = 8;
  static const int32_t kRelLocal = 16;
  static const int32_t kGeneratedSource = 32;
  static const int32_t kClone = 64;
  static const int32_t kHot = 128;
  static const int32_t kCovered = 256;
  static const int32_t kUncompressed = 512;
};

struct Container {
  explicit Container(const std::string& name_in);
  ~Container();
  // Keep copy constructor but remove assignment operator.
  Container(const Container& other);
  Container& operator=(const Container& other) = delete;

  static void AssignShortNames(std::vector<Container>* containers);

  std::string name;
  std::string short_name;
  std::vector<const char*> section_names;
};

class Symbol;

class BaseSymbol {
 public:
  virtual ~BaseSymbol();

  virtual int32_t Size() const = 0;
  virtual int32_t Padding() const = 0;
  virtual int32_t Address() const = 0;
  virtual int32_t Flags() const = 0;

  virtual std::string_view FullName() const = 0;
  // Derived from |full_name|. Generated lazily and cached.
  virtual std::string_view TemplateName() const = 0;
  virtual std::string_view Name() const = 0;
  virtual const std::vector<Symbol*>* Aliases() const = 0;
  virtual SectionId Section() const = 0;

  virtual std::string_view ContainerName() const = 0;
  virtual const char* ObjectPath() const = 0;
  virtual const char* SourcePath() const = 0;
  virtual const char* GroupingPath() const = 0;
  virtual const char* SectionName() const = 0;
  virtual const char* Component() const = 0;
  virtual std::string* Disassembly() const = 0;

  virtual float Pss() const = 0;
  virtual float PssWithoutPadding() const = 0;
  virtual float PaddingPss() const = 0;
  virtual float BeforePss() const = 0;

  virtual DiffStatus GetDiffStatus() const = 0;

  int32_t SizeWithoutPadding() const { return Size() - Padding(); }

  int32_t EndAddress() const { return Address() + SizeWithoutPadding(); }

  int32_t NumAliases() const {
    const std::vector<Symbol*>* aliases = Aliases();
    return aliases ? aliases->size() : 1;
  }

  bool IsTemplate() const {
    // Because of the way these are derived from |FullName|, they have the
    // same contents if and only if they have the same length.
    return Name().size() != TemplateName().size();
  }

  bool IsOverhead() const { return FullName().substr(0, 10) == "Overhead: "; }

  bool IsBss() const { return Section() == SectionId::kBss; }

  bool IsDex() const {
    SectionId section_id = Section();
    return section_id == SectionId::kDex || section_id == SectionId::kDexMethod;
  }

  bool IsOther() const { return Section() == SectionId::kOther; }

  bool IsPak() const {
    SectionId section_id = Section();
    return section_id == SectionId::kPakNontranslated ||
           section_id == SectionId::kPakTranslations;
  }

  bool IsNative() const {
    SectionId section_id = Section();
    return (section_id == SectionId::kBss || section_id == SectionId::kData ||
            section_id == SectionId::kDataRelRo ||
            section_id == SectionId::kText || section_id == SectionId::kRoData);
  }

  bool IsStringLiteral() const {
    std::string_view full_name = FullName();
    return !full_name.empty() &&
           (full_name[0] == '"' || full_name == kStringLiteralName);
  }

  bool IsGeneratedSource() const {
    return Flags() & SymbolFlag::kGeneratedSource;
  }

  bool IsNameUnique() const {
    return !(IsStringLiteral() || IsOverhead() ||
             (!FullName().empty() && FullName()[0] == '*') ||
             (IsNative() && FullName().find('.') != std::string_view::npos));
  }
};

struct BaseSizeInfo;
class Symbol;

class Symbol : public BaseSymbol {
 public:
  Symbol();
  ~Symbol() override;
  Symbol(const Symbol& other);

  int32_t Size() const override;
  int32_t Padding() const override;
  int32_t Address() const override;
  int32_t Flags() const override;

  std::string_view FullName() const override;
  // Derived from |full_name|. Generated lazily and cached.
  std::string_view TemplateName() const override;
  std::string_view Name() const override;
  const std::vector<Symbol*>* Aliases() const override;
  SectionId Section() const override;

  std::string_view ContainerName() const override;
  const char* ObjectPath() const override;
  const char* SourcePath() const override;
  const char* GroupingPath() const override;
  const char* SectionName() const override;
  const char* Component() const override;
  std::string* Disassembly() const override;

  float Pss() const override;
  float PssWithoutPadding() const override;
  float PaddingPss() const override;
  float BeforePss() const override;

  DiffStatus GetDiffStatus() const override;

  int32_t address_ = 0;
  int32_t size_ = 0;
  int32_t flags_ = 0;
  int32_t padding_ = 0;
  SectionId section_id_ = SectionId::kNone;
  std::string_view full_name_;
  // Derived lazily
  mutable std::string_view template_name_;
  mutable std::string_view name_;
  // Pointers into SizeInfo->raw_decompressed;
  const char* section_name_ = nullptr;
  const char* object_path_ = nullptr;
  const char* source_path_ = nullptr;
  const char* component_ = nullptr;
  std::string* disassembly_ = nullptr;

  std::vector<Symbol*>* aliases_ = nullptr;
  const Container* container_ = nullptr;

  // The SizeInfo the symbol was constructed from. Primarily used for
  // allocating commonly-reused strings in a context where they won't outlive
  // the symbol.
  BaseSizeInfo* size_info_ = nullptr;

 private:
  void DeriveNames() const;
};

class DeltaSymbol : public BaseSymbol {
 public:
  DeltaSymbol(const Symbol* before, const Symbol* after);
  ~DeltaSymbol() override;

  int32_t Size() const override;
  int32_t Padding() const override;
  int32_t Address() const override;
  int32_t Flags() const override;

  std::string_view FullName() const override;
  // Derived from |full_name|. Generated lazily and cached.
  std::string_view TemplateName() const override;
  std::string_view Name() const override;
  const std::vector<Symbol*>* Aliases() const override;
  SectionId Section() const override;

  std::string_view ContainerName() const override;
  const char* ObjectPath() const override;
  const char* SourcePath() const override;
  const char* GroupingPath() const override;
  const char* SectionName() const override;
  const char* Component() const override;
  std::string* Disassembly() const override;

  float Pss() const override;
  float PssWithoutPadding() const override;
  float PaddingPss() const override;
  float BeforePss() const override;

  DiffStatus GetDiffStatus() const override;

 private:
  const Symbol* before_ = nullptr;
  const Symbol* after_ = nullptr;
};

std::ostream& operator<<(std::ostream& os, const Symbol& sym);

struct BaseSizeInfo {
  BaseSizeInfo();
  BaseSizeInfo(const BaseSizeInfo&);
  virtual ~BaseSizeInfo();
  virtual bool IsSparse() const = 0;

  Json::Value fields;
  std::deque<std::string> owned_strings;
  SectionId ShortSectionName(const char* section_name);
};

struct SizeInfo : BaseSizeInfo {
  SizeInfo();
  ~SizeInfo() override;
  SizeInfo(const SizeInfo& other) = delete;
  SizeInfo& operator=(const SizeInfo& other) = delete;
  bool IsSparse() const override;

  std::vector<Container> containers;

  // Entries in |raw_symbols| hold pointers to this data.
  std::vector<const char*> object_paths;
  std::vector<const char*> source_paths;
  std::vector<const char*> components;
  std::vector<char> raw_decompressed;

  std::vector<Symbol> raw_symbols;

  // A container for each symbol group.
  std::deque<std::vector<Symbol*>> alias_groups;

  bool is_sparse = false;
};

struct DeltaSizeInfo : BaseSizeInfo {
  DeltaSizeInfo(const SizeInfo* before_in,
                const SizeInfo* after_in,
                const std::vector<std::string>* removed_sources_in,
                const std::vector<std::string>* added_sources_in);
  ~DeltaSizeInfo() override;
  DeltaSizeInfo(const DeltaSizeInfo&);
  DeltaSizeInfo& operator=(const DeltaSizeInfo&);
  bool IsSparse() const override;

  using Results = std::array<int32_t, 4>;
  Results CountsByDiffStatus() const {
    Results ret{0};
    for (const DeltaSymbol& sym : delta_symbols) {
      ret[static_cast<uint8_t>(sym.GetDiffStatus())]++;
    }
    return ret;
  }

  const SizeInfo* before = nullptr;
  const SizeInfo* after = nullptr;
  const std::vector<std::string>* removed_sources;
  const std::vector<std::string>* added_sources;
  std::vector<DeltaSymbol> delta_symbols;
  // Symbols created during diffing, e.g. aggregated padding symbols.
  std::deque<Symbol> owned_symbols;
};

struct JsonWriteOptions {
  bool is_sparse;
  bool diff_mode;
  bool method_count_mode;
};

struct Stat {
  int32_t count = 0;
  int32_t added = 0;
  int32_t removed = 0;
  int32_t changed = 0;
  float size = 0.0f;

  void operator+=(const Stat& other) {
    count += other.count;
    size += other.size;
    added += other.added;
    removed += other.removed;
    changed += other.changed;
  }
};

struct NodeStats {
  NodeStats();
  ~NodeStats();
  explicit NodeStats(const BaseSymbol& symbol);
  void WriteIntoJson(const JsonWriteOptions& opts, Json::Value* out) const;
  NodeStats& operator+=(const NodeStats& other);
  SectionId ComputeBiggestSection() const;
  int32_t SumCount() const;
  int32_t SumAdded() const;
  int32_t SumRemoved() const;
  DiffStatus GetGlobalDiffStatus() const;

  std::map<SectionId, Stat> child_stats;
  DiffStatus imposed_diff_status = DiffStatus::kUnchanged;
};

class TreeNodeFactory;

struct TreeNode {
 private:
  TreeNode(ArtifactType artifact_type_in, int32_t id_in);
  friend TreeNodeFactory;

 public:
  ~TreeNode();

  using CompareFunc =
      std::function<bool(const TreeNode* const& l, const TreeNode* const& r)>;

  void WriteIntoJson(const JsonWriteOptions& opts,
                     CompareFunc compare_func,
                     int depth,
                     Json::Value* out);

  const ArtifactType artifact_type;
  const int32_t id;
  GroupedPath id_path;
  const char* src_path = nullptr;
  const char* component = nullptr;
  float size = 0.0f;
  float before_size = 0.0f;
  float padding = 0.0f;
  int32_t address = 0;
  int32_t flags = 0;
  NodeStats node_stats;
  int32_t short_name_index = 0;

  std::vector<TreeNode*> children;
  TreeNode* parent = nullptr;
  const BaseSymbol* symbol = nullptr;
};

class TreeNodeFactory {
 public:
  TreeNodeFactory();
  ~TreeNodeFactory();
  TreeNodeFactory(const TreeNodeFactory&) = delete;
  TreeNodeFactory& operator=(const TreeNodeFactory&) = delete;

  TreeNode* Make(ArtifactType artifact_type);

 private:
  int32_t next_id = 0;
};

}  // namespace caspian
#endif  // TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_MODEL_H_
