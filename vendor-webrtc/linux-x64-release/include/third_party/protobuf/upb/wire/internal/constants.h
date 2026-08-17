// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_WIRE_INTERNAL_CONSTANTS_H_
#define UPB_WIRE_INTERNAL_CONSTANTS_H_

#define kUpb_WireFormat_DefaultDepthLimit 100

// MessageSet wire format is:
//   message MessageSet {
//     repeated group Item = 1 {
//       required int32 type_id = 2;
//       required bytes message = 3;
//     }
//   }

enum {
  kUpb_MsgSet_Item = 1,
  kUpb_MsgSet_TypeId = 2,
  kUpb_MsgSet_Message = 3,
};

#endif /* UPB_WIRE_INTERNAL_CONSTANTS_H_ */
