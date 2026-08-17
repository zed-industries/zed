// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_HUNSPELL_GOOGLE_BDICT_READER_H_
#define THIRD_PARTY_HUNSPELL_GOOGLE_BDICT_READER_H_

#include <stddef.h>

#include <string>
#include <vector>

#include "base/containers/span.h"
#include "third_party/hunspell/google/bdict.h"

namespace hunspell {

class BDictReader;
class NodeReader;

// Iterators -------------------------------------------------------------------

// Iterates through all words in the dictionary. It will fill the word into
// a caller-specified buffer.
class WordIterator {
 public:
  WordIterator(const WordIterator& other);
  ~WordIterator();

  // This must be explicitly declared and implemneted in the .cc file so it will
  // compile without knowing the size of NodeInfo.
  WordIterator& operator=(const WordIterator&);

  // Fills the buffer with the next word and the affixes for it into the given
  // array. Returns the number of affixes. A return value of 0 means there are
  // no more words.
  int Advance(char* output_buffer, size_t output_len,
              int affix_ids[BDict::MAX_AFFIXES_PER_WORD]);

 private:
  friend class BDictReader;
  struct NodeInfo;

  explicit WordIterator(const NodeReader& reader);

  // Called by Advance when a leaf is found to generate the word, affix list,
  // and return value.
  int FoundLeaf(const NodeReader& node, char cur_char,
                char* output_buffer, size_t output_len,
                int affix_ids[BDict::MAX_AFFIXES_PER_WORD]);

  std::vector<NodeInfo> stack_;
};

// Will iterate over a list of lines separated by NULLs.
class LineIterator {
 public:
  // Returns the next word in the sequence or NULL if there are no more.
  const char* Advance();

  // Returns true when all data has been read. We're done when we reach a
  // double-NULL or a the end of the input (shouldn't happen).
  bool IsDone() const;

 protected:
  friend class BDictReader;

  LineIterator();
  LineIterator(base::span<const unsigned char> bdict_data, size_t first_offset);

  base::span<const unsigned char> bdict_data_;

  // Current offset within bdict_data of the next string to read.
  size_t cur_offset_;
};

// Created by GetReplacementIterator to iterate over all replacement pairs.
class ReplacementIterator : public LineIterator {
 public:
  // Fills pointers to NULL terminated strings into the given output params.
  // Returns false if there are no more pairs and nothing was filled in.
  bool GetNext(const char** first, const char** second);

 private:
  friend class BDictReader;

  ReplacementIterator() = default;
  ReplacementIterator(base::span<const unsigned char> bdict_data,
                      size_t first_offset)
      : LineIterator(bdict_data, first_offset) {}
};

// Reads a BDict file mapped into memory.
class BDictReader {
 public:
  // You must call Init and it must succeed before calling any other functions.
  BDictReader();

  BDictReader(const BDictReader&) = delete;
  BDictReader& operator=(const BDictReader&) = delete;

  // Initializes the reader with the given data. The data does not transfer
  // ownership, and the caller must keep it valid until the reader is destroyed.
  // Returns true on success.
  bool Init(base::span<const unsigned char> bdict_data);

  // Returns true if Init() succeeded and other functions can be called.
  bool IsValid() const { return !bdict_data_.empty(); }

  // Locates the given word in the dictionary. There may be multiple matches if
  // the word is listed multiple times in the dictionary with different affix
  // rules.
  //
  // The number of matches is returned, and that number of corresponding affix
  // group IDs are filled into |*affix_indices|. These IDs may be 0 to indicate
  // there is no affix for that particular match. A return valuf of 0 means that
  // there are no matches.
  int FindWord(const char* word,
               int affix_indices[BDict::MAX_AFFIXES_PER_WORD]) const;

  // Returns an iterator that will go over all AF lines ("affix groups").
  LineIterator GetAfLineIterator() const;

  // Returns an iterator that will go over all SFX/PFX lines ("affix rules").
  LineIterator GetAffixLineIterator() const;

  // Returns an iterator that will go over all "other" lines.
  LineIterator GetOtherLineIterator() const;

  // Returns an iterator that can be used to iterate all replacements.
  ReplacementIterator GetReplacementIterator() const;

  // Used for testing, returns an iterator for all words in the dictionary.
  WordIterator GetAllWordIterator() const;

 private:
  // Non-empty indicates Init succeeded.
  base::span<const unsigned char> bdict_data_;

  BDict::Header header_ = {0};
  BDict::AffHeader aff_header_ = {0};
};

}  // namespace hunspell

#endif  // THIRD_PARTY_HUNSPELL_GOOGLE_BDICT_READER_H_
