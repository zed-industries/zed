// Copyright 2009 The RE2 Authors.  All Rights Reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#ifndef RE2_PREFILTER_H_
#define RE2_PREFILTER_H_

// Prefilter is the class used to extract string guards from regexps.
// Rather than using Prefilter class directly, use FilteredRE2.
// See filtered_re2.h

#include <set>
#include <string>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"

namespace re2 {

class RE2;

class Regexp;

class Prefilter {
  // Instead of using Prefilter directly, use FilteredRE2; see filtered_re2.h
 public:
  enum Op {
    ALL = 0,  // Everything matches
    NONE,  // Nothing matches
    ATOM,  // The string atom() must match
    AND,   // All in subs() must match
    OR,   // One of subs() must match
  };

  explicit Prefilter(Op op);
  ~Prefilter();

  Op op() { return op_; }
  const std::string& atom() const { return atom_; }
  void set_unique_id(int id) { unique_id_ = id; }
  int unique_id() const { return unique_id_; }

  // The children of the Prefilter node.
  std::vector<Prefilter*>* subs() {
    ABSL_DCHECK(op_ == AND || op_ == OR);
    return subs_;
  }

  // Set the children vector. Prefilter takes ownership of subs and
  // subs_ will be deleted when Prefilter is deleted.
  void set_subs(std::vector<Prefilter*>* subs) { subs_ = subs; }

  // Given a RE2, return a Prefilter. The caller takes ownership of
  // the Prefilter and should deallocate it. Returns NULL if Prefilter
  // cannot be formed.
  static Prefilter* FromRE2(const RE2* re2);

  // Returns a readable debug string of the prefilter.
  std::string DebugString() const;

 private:
  template <typename H>
  friend H AbslHashValue(H h, const Prefilter& a) {
    h = H::combine(std::move(h), a.op_);
    if (a.op_ == ATOM) {
      h = H::combine(std::move(h), a.atom_);
    } else if (a.op_ == AND || a.op_ == OR) {
      h = H::combine(std::move(h), a.subs_->size());
      for (size_t i = 0; i < a.subs_->size(); ++i) {
        h = H::combine(std::move(h), (*a.subs_)[i]->unique_id_);
      }
    }
    return h;
  }

  friend bool operator==(const Prefilter& a, const Prefilter& b) {
    if (&a == &b) {
      return true;
    }
    if (a.op_ != b.op_) {
      return false;
    }
    if (a.op_ == ATOM) {
      if (a.atom_ != b.atom_) {
        return false;
      }
    } else if (a.op_ == AND || a.op_ == OR) {
      if (a.subs_->size() != b.subs_->size()) {
        return false;
      }
      for (size_t i = 0; i < a.subs_->size(); ++i) {
        if ((*a.subs_)[i]->unique_id_ != (*b.subs_)[i]->unique_id_) {
          return false;
        }
      }
    }
    return true;
  }

  // A comparator used to store exact strings. We compare by length,
  // then lexicographically. This ordering makes it easier to reduce the
  // set of strings in SimplifyStringSet.
  struct LengthThenLex {
    bool operator()(const std::string& a, const std::string& b) const {
       return (a.size() < b.size()) || (a.size() == b.size() && a < b);
    }
  };

  class Info;

  using SSet = std::set<std::string, LengthThenLex>;
  using SSIter = SSet::iterator;
  using ConstSSIter = SSet::const_iterator;

  // Combines two prefilters together to create an AND. The passed
  // Prefilters will be part of the returned Prefilter or deleted.
  static Prefilter* And(Prefilter* a, Prefilter* b);

  // Combines two prefilters together to create an OR. The passed
  // Prefilters will be part of the returned Prefilter or deleted.
  static Prefilter* Or(Prefilter* a, Prefilter* b);

  // Generalized And/Or
  static Prefilter* AndOr(Op op, Prefilter* a, Prefilter* b);

  static Prefilter* FromRegexp(Regexp* a);

  static Prefilter* FromString(const std::string& str);

  static Prefilter* OrStrings(SSet* ss);

  static Info* BuildInfo(Regexp* re);

  Prefilter* Simplify();

  // Removes redundant strings from the set. A string is redundant if
  // any of the other strings appear as a substring. The empty string
  // is a special case, which is ignored.
  static void SimplifyStringSet(SSet* ss);

  // Adds the cross-product of a and b to dst.
  // (For each string i in a and j in b, add i+j.)
  static void CrossProduct(const SSet& a, const SSet& b, SSet* dst);

  // Kind of Prefilter.
  Op op_;

  // Sub-matches for AND or OR Prefilter.
  std::vector<Prefilter*>* subs_;

  // Actual string to match in leaf node.
  std::string atom_;

  // If different prefilters have the same string atom, or if they are
  // structurally the same (e.g., OR of same atom strings) they are
  // considered the same unique nodes. This is the id for each unique
  // node. This field is populated with a unique id for every node,
  // and -1 for duplicate nodes.
  int unique_id_;

  Prefilter(const Prefilter&) = delete;
  Prefilter& operator=(const Prefilter&) = delete;
};

}  // namespace re2

#endif  // RE2_PREFILTER_H_
