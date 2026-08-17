// Copyright 2009 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Copyright (c) 1994-2006 Sun Microsystems Inc.
// All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistribution in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
// - Neither the name of Sun Microsystems or the names of contributors may
// be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// The original source code covered by the above license above has been
// modified significantly by Google Inc.
// Copyright 2006-2008 the V8 project authors. All rights reserved.

// This implements a C++ assembler for dynamically generating machine code.
// It is heavily based on the v8 assembler, which has a long history of its
// own.  Relocation information has been removed, and in general things were
// made a bit simpler (and slower).  Everything is implemented inline.

#ifndef TRACELINE_ASSEMBLER_H_
#define TRACELINE_ASSEMBLER_H_

#include <windows.h>

#include <stddef.h>
#include <stdio.h>

#include <string>

#include "logging.h"

#define ASSERT(x) CHECK(x)

enum Register {
  EAX = 0,
  ECX = 1,
  EDX = 2,
  EBX = 3,
  ESP = 4,
  EBP = 5,
  ESI = 6,
  EDI = 7
};

enum Condition {
  overflow      =  0,
  no_overflow   =  1,
  below         =  2,
  above_equal   =  3,
  equal         =  4,
  not_equal     =  5,
  below_equal   =  6,
  above         =  7,
  sign          =  8,
  not_sign      =  9,
  parity_even   = 10,
  parity_odd    = 11,
  less          = 12,
  greater_equal = 13,
  less_equal    = 14,
  greater       = 15,

  // aliases
  zero          = equal,
  not_zero      = not_equal,
  negative      = sign,
  positive      = not_sign
};

// Labels are used for branching, and marks an offset in the CodeBuffer.
// A label can be in 3 states:
//  - Unused, the label has never be used in an instruction.
//  - Linked, the label has been referenced (by a jump, for example), but the
//    target is not yet known, because the label is unbound.
//  - Bound, the label has been bound so the offset is known.
class Label {
 public:
  Label() { Unuse(); }
  ~Label() { ASSERT(!is_linked()); }

  void Unuse() {
    num_ = 0;
  }

  bool is_unused() const { return num_ == 0; }
  bool is_bound() const { return num_ == -1; }
  bool is_linked() const { return num_ > 0; }

  int binding_pos() const {
    ASSERT(is_bound());
    return table_[0];
  }

  int num_links() const {
    ASSERT(!is_bound());
    return num_;  // Will return 0 if unused.
  }

  int link_pos(int i) const {
    ASSERT(is_linked());
    ASSERT(i < num_);
    return table_[i];
  }

 private:
  void bind_to(int pos)  {
    ASSERT(!is_bound());
    table_[0] = pos;
    num_ = -1;
  }
  void link_to(int pos)  {
    ASSERT(!is_bound());
    ASSERT(num_ < kTableSize);

    table_[num_] = pos;
    ++num_;
  }

  static const int kTableSize = 3;

  // We store all links in a fixed size table.  When we're bound, we store the
  // binding position in the first entry of the table.
  int table_[kTableSize];
  // The number of entries in our table, if we're linked.  If 0, then we're
  // unusued.  If -1, then we are bound (and the pos is at table_[0]).
  int num_;

  friend class CodeBuffer;  // For binding, linking, etc
};


enum ScaleFactor {
  SCALE_TIMES_1 = 0,
  SCALE_TIMES_2 = 1,
  SCALE_TIMES_4 = 2,
  SCALE_TIMES_8 = 3
};


class Operand {
 public:
  explicit Operand(const Operand& x) : len_(x.len_) {
    memcpy(buf_, x.buf_, sizeof(buf_));
  }

  // reg
  explicit Operand(Register reg) {
    Init(reg);
  }

  // [disp/r]
  explicit Operand(int disp) {
    Init(disp);
  }

  // [base + disp/r]
  Operand(Register base, int disp) {
    Init(base, disp);
  }

  // [base + index*scale + disp/r]
  Operand(Register base, Register index, ScaleFactor scale, int disp) {
    Init(base, index, scale, disp);
  }

  // [index*scale + disp/r]
  Operand(Register index, ScaleFactor scale, int disp) {
    Init(index, scale, disp);
  }

  void set_reg(Register reg) {
    ASSERT(len_ > 0);
    buf_[0] = (buf_[0] & ~0x38) | static_cast<char>(reg << 3);
  }

  char* data() { return buf_; }
  int length() { return len_; }

 private:
  // reg
  void Init(Register reg) {
    set_modrm(3, reg);
  }

  // [disp/r]
  void Init(int disp) {
    set_modrm(0, EBP);
    set_dispr(disp);
  }

  // [base + disp/r]
  void Init(Register base, int disp) {
    if (disp == 0) {
      // [base]
      set_modrm(0, base);
      if (base == ESP) set_sib(SCALE_TIMES_1, ESP, base);
    } else if (is_int8(disp)) {
      // [base + disp8]
      set_modrm(1, base);
      if (base == ESP) set_sib(SCALE_TIMES_1, ESP, base);
      set_disp8(disp);
    } else {
      // [base + disp/r]
      set_modrm(2, base);
      if (base == ESP) set_sib(SCALE_TIMES_1, ESP, base);
      set_dispr(disp);
    }
  }

  // [base + index*scale + disp/r]
  void Init(Register base,
            Register index,
            ScaleFactor scale,
            int disp) {
    ASSERT(index != ESP);  // illegal addressing mode
    if (disp == 0 && base != EBP) {
      // [base + index*scale]
      set_modrm(0, ESP);
      set_sib(scale, index, base);
    } else if (is_int8(disp)) {
      // [base + index*scale + disp8]
      set_modrm(1, ESP);
      set_sib(scale, index, base);
      set_disp8(disp);
    } else {
      // [base + index*scale + disp/r]
      set_modrm(2, ESP);
      set_sib(scale, index, base);
      set_dispr(disp);
    }
  }

  // [index*scale + disp/r]
  void Init(Register index,
            ScaleFactor scale,
            int disp) {
    ASSERT(index != ESP);  // illegal addressing mode
    // We can reduce instruction size by translating instructions of the form:
    //   8D044510000000    lea eax,[eax*2+0x10]
    // To the more concise scale=1 version:
    //   8D440010          lea eax,[eax+eax+0x10]
    if (scale == SCALE_TIMES_2) {
      Init(index, index, SCALE_TIMES_1, disp);
    } else {
      set_modrm(0, ESP);
      set_sib(scale, index, EBP);
      set_dispr(disp);
    }
  }

  // Returns true if this Operand is a wrapper for the specified register.
  bool is_reg(Register reg) const {
    return ((buf_[0] & 0xF8) == 0xC0)  // addressing mode is register only.
        && ((buf_[0] & 0x07) == reg);  // register codes match.
  }

  void set_modrm(int mod, Register rm) {  // reg == 0
    ASSERT((mod & -4) == 0);
    buf_[0] = mod << 6 | rm;
    len_ = 1;
  }

  void set_sib(ScaleFactor scale, Register index, Register base) {
    ASSERT(len_ == 1);
    ASSERT((scale & -4) == 0);
    buf_[1] = scale << 6 | index << 3 | base;
    len_ = 2;
  }

  void set_disp8(char disp) {
    ASSERT(len_ == 1 || len_ == 2);
    *reinterpret_cast<char*>(&buf_[len_++]) = disp;
  }

  void set_dispr(int disp) {
    ASSERT(len_ == 1 || len_ == 2);
    *reinterpret_cast<int*>(&buf_[len_]) = disp;
    len_ += sizeof(int);
  }

  bool is_int8(int x) { return x >= -128 && x <= 127; }

  // Mutable because reg in ModR/M byte is set by Assembler via set_reg().
  char buf_[6];
  // The number of bytes in buf_.
  unsigned int len_;
};

// A convenient wrapper around a buffer for emitting code or data, etc.
class CodeBuffer {
 public:
  // Use an externally managed buffer
  explicit CodeBuffer(char* buf) : pos_(0), buf_(buf) { }

  void* data() { return buf_; }
  int size() { return pos_; }

  void emit(unsigned char b) {
    buf_[pos_++] = b;
  }
  void emit_word(unsigned short w) {
    *reinterpret_cast<unsigned short*>(&buf_[pos_]) = w;
    pos_ += 2;
  }
  void emit_dword(unsigned int d) {
    *reinterpret_cast<unsigned int*>(&buf_[pos_]) = d;
    pos_ += 4;
  }

  void emit_bytes(const char* bytes, size_t size) {
    for (size_t i = 0; i < size; ++i)
      emit(bytes[i]);
  }

  void emit_bytes(const std::string& bytes) {
    emit_bytes(bytes.data(), bytes.size());
  }

  void put_dword_at(int pos, unsigned int d) {
    *reinterpret_cast<unsigned int*>(&buf_[pos]) = d;
  }

  // We pass by value so that we get a copy that we can modify.
  void emit_operand(Register reg, Operand operand) {
    operand.set_reg(reg);
    memcpy(&buf_[pos_], operand.data(), operand.length());
    pos_ += operand.length();
  }

  void bind(Label* l) {
    ASSERT(!l->is_bound());
    for (int i = 0; i < l->num_links(); ++i) {
      put_dword_at(l->link_pos(i), pos_ - (l->link_pos(i) + 4));
    }
    l->bind_to(pos_);
  }

  // TODO deprecate blah_imm and use blah(Immediate)

  void add(Register dst, Register src) {
    emit(0x01); emit(0xc0 | (src << 3) | dst);
  }
  void add_imm(Register dst, int d) {
    if (d >= -128 && d <= 127) {
      emit(0x83); emit(0xc0 | dst); emit(d & 0xff);
    } else {
      emit(0x81); emit(0xc0 | dst); emit_dword(d);
    }
  }

  void and_(Register r, unsigned int mask) {
    emit(0x81); emit(0xe0 | r); emit_dword(mask);
  }

  void call(Register r) {
    call(Operand(r));
  }
  void call(const Operand& dst) {
    emit(0xff); emit_operand(EDX, dst);
  }

  void cmp(Register r1, Register r2) {
    emit(0x39); emit(0xc0 | (r2 << 3) | r1);
  }

  void cmp_imm(Register r, int d) {
    if (d >= -128 && d <= 127) {
      emit(0x83); emit(0xf8 | r); emit(d & 0xff);
    } else {
      emit(0x81); emit(0xf8 | r); emit_dword(d);
    }
  }

  void fs() {
    emit(0x64);
  }

  // Atomically increment the dword at |mem| with the increment amount in the
  // register |inc|.  Will replace |inc| with the old unincremented value.
  void inc_atomic(Register mem, Register inc) {
    // lock xadd [mem], inc
    emit(0xF0); emit(0x0F); emit(0xC1); emit((inc << 3) | mem);
  }

  void int3() {
    emit(0xcc);
  }

  void jcc(Condition cc, Label* l) {
    emit(0x0f); emit(0x80 | cc);
    if (l->is_bound()) {
      emit_dword(l->binding_pos() - (pos_ + 4));
    } else {
      // Will fix up when the label is bound.
      l->link_to(pos_);
      emit_dword(0);
    }
  }

  void jmp(Register r) {
    emit(0xff); emit(0xe0 | r);
  }

  void jmp(Label* l) {
    if (l->is_bound()) {
      jmp_rel(l->binding_pos() - (pos_ + 5));
    } else {
      // Will fix up when the label is bound.
      l->link_to(pos_ + 1);
      jmp_rel(0);
    }
  }

  void jmp_rel(int i) {
    emit(0xe9); emit_dword(i);
  }

  void jmp_rel_short(char c) {
    emit(0xeb); emit(c);
  }

  void lea(Register dst, const Operand& src) {
    emit(0x8d); emit_operand(dst, src);
  }

  void lodsb() {
    emit(0xac);
  }
  void lodsd() {
    emit(0xad);
  }

  void loop(Label* l) {
    ASSERT(l->is_bound());
    int pos = l->binding_pos() - (pos_ + 2);
    ASSERT(pos >= -128 && pos < 0);

    emit(0xe2); emit(pos & 0xff);
  }

  void mov(Register dst, Register src) {
    emit(0x89); emit(0xc0 | (src << 3) | dst);
  }
  void mov(Register dst, const Operand& src) {
    emit(0x8b); emit_operand(dst, src);
  }
  void mov_imm(Register r, unsigned int d) {
    emit(0xb8 | r); emit_dword(d);
  }

  void movsb() {
    emit(0xa4);
  }
  void movsd() {
    emit(0xa5);
  }

  void or_(Register r, unsigned int mask) {
    emit(0x81); emit(0xc8 | r); emit_dword(mask);
  }

  void pop(Register r) {
    emit(0x58 | r);
  }
  void pop(const Operand& dst) {
    emit(0x8f); emit_operand(EAX, dst);
  }

  void push(Register r) {
    emit(0x50 | r);
  }
  void push(const Operand& src) {
    emit(0xff); emit_operand(ESI, src);
  }
  void push_imm(int i) {
    if (i >= -128 && i <= 127) {
      emit(0x6a); emit(i & 0xff);
    } else {
      emit(0x68); emit_dword(i);
    }
  }

  // Puts the cycle counter into edx:eax.
  void rdtsc() {
    emit(0x0F); emit(0x31);
  }

  void rep() {
    emit(0xf3);
  }

  void ret() {
    ret(0);
  }
  void ret(short c) {
    if (c == 0) {
      emit(0xc3);
    } else {
      emit(0xc2); emit_word(c);
    }
  }

  void spin() {
    jmp_rel_short(-2);
  }

  void stosb() {
    emit(0xaa);
  }
  void stosd() {
    emit(0xab);
  }

  void sysenter() {
    emit(0x0f); emit(0x34);
  }

  // Puts a unique cpu identifier into eax, using sidt to fingerprint cores.
  void which_cpu() {
    // Make space
    push(EAX);
    push(EAX);
    // sidt [esp+2]
    emit(0x0f); emit(0x01); emit_operand(ECX, Operand(ESP, 2));
    pop(EAX);
    pop(EAX);  // sidt address
  }

  // Puts a unique identifier for the thread we're executing on into eax.
  void which_thread() {
    // mov eax, [fs:0x24]
    emit(0x64); emit(0xa1); emit_dword(0x24);
    // TODO: We could do this but it will use an encoding that is 1 byte bigger.
    // fs(); mov(EAX, Operand(0x24));
  }

  void xchg(Register r1, Register r2) {
    if (r1 == EAX) {
      emit(0x90 | r2);
    } else if (r2 == EAX) {
      emit(0x90 | r1);
    } else {
      xchg(r1, Operand(r2));
    }
  }
  void xchg(Register r1, const Operand& oper) {
    emit(0x87); emit_operand(r1, oper);
  }

 private:
  int pos_;
  char* buf_;
};

#endif  // TRACELINE_ASSEMBLER_H_
