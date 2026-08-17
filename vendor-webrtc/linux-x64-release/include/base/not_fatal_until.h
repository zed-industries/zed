// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_NOT_FATAL_UNTIL_H_
#define BASE_NOT_FATAL_UNTIL_H_

namespace base {

// Add new entries a few milestones into the future whenever necessary.
// M here refers to milestones, see chrome/VERSION's MAJOR field that updates
// when chromium branches.
//
// To clean up old entries remove the already-fatal argument from CHECKs as well
// as from this list. This generates better-optimized CHECKs in official builds.
enum class NotFatalUntil {
  NoSpecifiedMilestoneInternal = -1,
  M120 = 120,
  M121 = 121,
  M122 = 122,
  M123 = 123,
  M124 = 124,
  M125 = 125,
  M126 = 126,
  M127 = 127,
  M128 = 128,
  M129 = 129,
  M130 = 130,
  M131 = 131,
  M132 = 132,
  M133 = 133,
  M134 = 134,
  M135 = 135,
  M136 = 136,
  M137 = 137,
  M138 = 138,
  M139 = 139,
  M140 = 140,
  M141 = 141,
  M142 = 142,
  M143 = 143,
  M144 = 144,
  M145 = 145,
  M146 = 146,
  M147 = 147,
  M148 = 148,
  M149 = 149,
  M150 = 150,
  M151 = 151,
  M152 = 152,
  M153 = 153,
  M154 = 154,
  M155 = 155,
  M156 = 156,
  M157 = 157,
  M158 = 158,
  M159 = 159,
  M160 = 160,
  M161 = 161,
  M162 = 162,
  M163 = 163,
  M164 = 164,
  M165 = 165,
  M166 = 166,
  M167 = 167,
  M168 = 168,
  M169 = 169,
  M170 = 170,
  M171 = 171,
  M172 = 172,
  M173 = 173,
  M174 = 174,
  M175 = 175,
  M176 = 176,
  M177 = 177,
  M178 = 178,
  M179 = 179,
  M180 = 180,
  M181 = 181,
  M182 = 182,
  M183 = 183,
  M184 = 184,
  M185 = 185,
  M186 = 186,
  M187 = 187,
  M188 = 188,
  M189 = 189,
  M190 = 190,
  M191 = 191,
  M192 = 192,
  M193 = 193,
  M194 = 194,
  M195 = 195,
  M196 = 196,
  M197 = 197,
  M198 = 198,
  M199 = 199,
  M200 = 200,
};

}  // namespace base

#endif  // BASE_NOT_FATAL_UNTIL_H_
