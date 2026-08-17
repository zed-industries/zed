/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2019, Martin Storsjo
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#include "src/arm/asm.S"
#include "util.S"

// depending on how many pixels need to be stored, returns:
// x14 = (1 << 0) : 0 pixels
// x14 = (1 << 4) : inner 4 pixels
// x14 = (1 << 6) : inner 6 pixels
// x14 = 0 : all pixels
.macro loop_filter wd
function lpf_16_wd\wd\()_neon
        uabd            v0.16b,  v22.16b, v23.16b // abs(p1 - p0)
        uabd            v1.16b,  v25.16b, v24.16b // abs(q1 - q0)
        uabd            v2.16b,  v23.16b, v24.16b // abs(p0 - q0)
        uabd            v3.16b,  v22.16b, v25.16b // abs(p1 - q1)
.if \wd >= 6
        uabd            v4.16b,  v21.16b, v22.16b // abs(p2 - p1)
        uabd            v5.16b,  v26.16b, v25.16b // abs(q2 - q1)
.endif
.if \wd >= 8
        uabd            v6.16b,  v20.16b, v21.16b // abs(p3 - p2)
        uabd            v7.16b,  v27.16b, v26.16b // abs(q3 - q3)
.endif
.if \wd >= 6
        umax            v4.16b,  v4.16b,  v5.16b
.endif
        uqadd           v2.16b,  v2.16b,  v2.16b  // abs(p0 - q0) * 2
.if \wd >= 8
        umax            v6.16b,  v6.16b,  v7.16b
.endif
        ushr            v3.16b,  v3.16b,  #1
.if \wd >= 8
        umax            v4.16b,  v4.16b,  v6.16b
.endif
.if \wd >= 6
        and             v4.16b,  v4.16b,  v14.16b
.endif
        umax            v0.16b,  v0.16b,  v1.16b  // max(abs(p1 - p0), abs(q1 - q0))
        uqadd           v2.16b,  v2.16b,  v3.16b  // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1
.if \wd >= 6
        umax            v4.16b,  v0.16b,  v4.16b
        cmhs            v1.16b,  v11.16b, v4.16b  // max(abs(p1 - p0), abs(q1 - q0), abs(), abs(), ...) <= I
.else
        cmhs            v1.16b,  v11.16b, v0.16b  // max(abs(p1 - p0), abs(q1 - q0)) <= I
.endif
        cmhs            v2.16b,  v10.16b, v2.16b  // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1 <= E
        and             v1.16b,  v1.16b,  v2.16b  // fm
        and             v1.16b,  v1.16b,  v13.16b // fm && wd >= 4
.if \wd >= 6
        and             v14.16b, v14.16b, v1.16b  // fm && wd > 4
.endif
.if \wd >= 16
        and             v15.16b, v15.16b, v1.16b  // fm && wd == 16
.endif

        mov             x16, v1.d[0]
        mov             x17, v1.d[1]
        adds            x16, x16, x17
        b.ne            9f                        // if (!fm || wd < 4) return;
        mov             x14, #(1 << 0)
        ret
9:
.if \wd >= 6
        movi            v10.16b, #1
        uabd            v2.16b,  v21.16b, v23.16b // abs(p2 - p0)
        uabd            v3.16b,  v22.16b, v23.16b // abs(p1 - p0)
        uabd            v4.16b,  v25.16b, v24.16b // abs(q1 - q0)
        uabd            v5.16b,  v26.16b, v24.16b // abs(q2 - q0)
.if \wd >= 8
        uabd            v6.16b,  v20.16b, v23.16b // abs(p3 - p0)
        uabd            v7.16b,  v27.16b, v24.16b // abs(q3 - q0)
.endif
        umax            v2.16b,  v2.16b,  v3.16b
        umax            v4.16b,  v4.16b,  v5.16b
.if \wd >= 8
        umax            v6.16b,  v6.16b,  v7.16b
.endif
        umax            v2.16b,  v2.16b,  v4.16b
.if \wd >= 8
        umax            v2.16b,  v2.16b,  v6.16b
.endif

.if \wd == 16
        uabd            v3.16b,  v17.16b, v23.16b // abs(p6 - p0)
        uabd            v4.16b,  v18.16b, v23.16b // abs(p5 - p0)
        uabd            v5.16b,  v19.16b, v23.16b // abs(p4 - p0)
.endif
        cmhs            v2.16b,  v10.16b, v2.16b  // flat8in
.if \wd == 16
        uabd            v6.16b,  v28.16b, v24.16b // abs(q4 - q0)
        uabd            v7.16b,  v29.16b, v24.16b // abs(q5 - q0)
        uabd            v8.16b,  v30.16b, v24.16b // abs(q6 - q0)
.endif
        and             v14.16b, v2.16b,  v14.16b // flat8in && fm && wd > 4
        bic             v1.16b,  v1.16b,  v14.16b // fm && wd >= 4 && !flat8in
.if \wd == 16
        umax            v3.16b,  v3.16b,  v4.16b
        umax            v5.16b,  v5.16b,  v6.16b
.endif
        mov             x16, v1.d[0]
        mov             x17, v1.d[1]
.if \wd == 16
        umax            v7.16b,  v7.16b,  v8.16b
        umax            v3.16b,  v3.16b,  v5.16b
        umax            v3.16b,  v3.16b,  v7.16b
        cmhs            v3.16b,  v10.16b, v3.16b  // flat8out
.endif
        adds            x16, x16, x17
.if \wd == 16
        and             v15.16b, v15.16b, v3.16b  // flat8out && fm && wd == 16
        and             v15.16b, v15.16b, v14.16b // flat8out && flat8in && fm && wd == 16
        bic             v14.16b, v14.16b, v15.16b // flat8in && fm && wd >= 4 && !flat8out
.endif
        b.eq            1f                        // skip wd == 4 case
.endif
        movi            v3.16b,  #128
        eor             v2.16b,  v22.16b, v3.16b  // p1 - 128
        eor             v3.16b,  v25.16b, v3.16b  // q1 - 128
        cmhi            v0.16b,  v0.16b,  v12.16b // hev
        sqsub           v2.16b,  v2.16b,  v3.16b  // iclip_diff(p1 - q1)
        and             v4.16b,  v2.16b,  v0.16b  // if (hev) iclip_diff(p1 - q1)
        bic             v0.16b,  v1.16b,  v0.16b  // (fm && wd >= 4 && !hev)
        usubl           v2.8h,   v24.8b,  v23.8b
        movi            v5.8h,   #3
        usubl2          v3.8h,   v24.16b, v23.16b
        mul             v2.8h,   v2.8h,   v5.8h
        mul             v3.8h,   v3.8h,   v5.8h
        movi            v6.16b,  #4
        saddw           v2.8h,   v2.8h,   v4.8b
        saddw2          v3.8h,   v3.8h,   v4.16b
        movi            v7.16b,  #3
        sqxtn           v2.8b,   v2.8h            // f
        sqxtn2          v2.16b,  v3.8h
        sqadd           v4.16b,  v6.16b,  v2.16b  // imin(f + 4, 127)
        sqadd           v5.16b,  v7.16b,  v2.16b  // imin(f + 3, 127)
        sshr            v4.16b,  v4.16b,  #3      // f1
        sshr            v5.16b,  v5.16b,  #3      // f2
        mov             v2.16b,  v23.16b          // p0
        mov             v3.16b,  v24.16b          // q0
        neg             v6.16b,  v4.16b           // -f1
        srshr           v4.16b,  v4.16b,  #1      // (f1 + 1) >> 1
        // p0 + f2, q0 - f1
        usqadd          v2.16b,  v5.16b           // out p0
        usqadd          v3.16b,  v6.16b           // out q0
        neg             v6.16b,  v4.16b           // -((f1 + 1) >> 1)
        bit             v23.16b, v2.16b,  v1.16b  // if (fm && wd >= 4)
        bit             v24.16b, v3.16b,  v1.16b  // if (fm && wd >= 4)
        mov             v2.16b,  v22.16b          // p1
        mov             v3.16b,  v25.16b          // q1
        // p1 + ((f1 + 1) >> 1), q1 - ((f1 + 1) >> 1)
        usqadd          v2.16b,  v4.16b           // out p1
        usqadd          v3.16b,  v6.16b           // out q1
        bit             v22.16b, v2.16b,  v0.16b  // if (fm && wd >= 4 && !hev)
        bit             v25.16b, v3.16b,  v0.16b  // if (fm && wd >= 4 && !hev)
1:

.if \wd == 6
        mov             x16, v14.d[0]
        mov             x17, v14.d[1]
        adds            x16, x16, x17
        b.eq            2f                        // skip if there's no flat8in

        uaddl           v0.8h,   v21.8b,  v21.8b  // p2 * 2
        uaddl2          v1.8h,   v21.16b, v21.16b
        uaddl           v2.8h,   v21.8b,  v22.8b  // p2 + p1
        uaddl2          v3.8h,   v21.16b, v22.16b
        uaddl           v4.8h,   v22.8b,  v23.8b  // p1 + p0
        uaddl2          v5.8h,   v22.16b, v23.16b
        uaddl           v6.8h,   v23.8b,  v24.8b  // p0 + q0
        uaddl2          v7.8h,   v23.16b, v24.16b
        add             v8.8h,   v0.8h,   v2.8h
        add             v9.8h,   v1.8h,   v3.8h
        add             v10.8h,  v4.8h,   v6.8h
        add             v11.8h,  v5.8h,   v7.8h
        uaddl           v12.8h,  v24.8b,  v25.8b  // q0 + q1
        uaddl2          v13.8h,  v24.16b, v25.16b
        add             v8.8h,   v8.8h,   v10.8h
        add             v9.8h,   v9.8h,   v11.8h
        sub             v12.8h,  v12.8h,  v0.8h
        sub             v13.8h,  v13.8h,  v1.8h
        uaddl           v10.8h,  v25.8b,  v26.8b  // q1 + q2
        uaddl2          v11.8h,  v25.16b, v26.16b
        rshrn           v0.8b,   v8.8h,   #3      // out p1
        rshrn2          v0.16b,  v9.8h,   #3

        add             v8.8h,   v8.8h,   v12.8h
        add             v9.8h,   v9.8h,   v13.8h
        sub             v10.8h,  v10.8h,  v2.8h
        sub             v11.8h,  v11.8h,  v3.8h
        uaddl           v12.8h,  v26.8b,  v26.8b  // q2 + q2
        uaddl2          v13.8h,  v26.16b, v26.16b
        rshrn           v1.8b,   v8.8h,   #3      // out p0
        rshrn2          v1.16b,  v9.8h,   #3

        add             v8.8h,   v8.8h,   v10.8h
        add             v9.8h,   v9.8h,   v11.8h
        sub             v12.8h,  v12.8h,  v4.8h
        sub             v13.8h,  v13.8h,  v5.8h
        rshrn           v2.8b,   v8.8h,   #3      // out q0
        rshrn2          v2.16b,  v9.8h,   #3

        bit             v22.16b, v0.16b,  v14.16b // p1 if (flat8in)
        add             v8.8h,   v8.8h,   v12.8h
        add             v9.8h,   v9.8h,   v13.8h
        bit             v23.16b, v1.16b,  v14.16b // p0 if (flat8in)
        rshrn           v3.8b,   v8.8h,   #3      // out q1
        rshrn2          v3.16b,  v9.8h,   #3
        bit             v24.16b, v2.16b,  v14.16b // q0 if (flat8in)
        bit             v25.16b, v3.16b,  v14.16b // q1 if (flat8in)
.elseif \wd >= 8
        mov             x16, v14.d[0]
        mov             x17, v14.d[1]
        adds            x16, x16, x17
.if \wd == 8
        b.eq            8f                        // skip if there's no flat8in
.else
        b.eq            2f                        // skip if there's no flat8in
.endif

        uaddl           v0.8h,   v20.8b,  v21.8b  // p3 + p2
        uaddl2          v1.8h,   v20.16b, v21.16b
        uaddl           v2.8h,   v22.8b,  v25.8b  // p1 + q1
        uaddl2          v3.8h,   v22.16b, v25.16b
        uaddl           v4.8h,   v20.8b,  v22.8b  // p3 + p1
        uaddl2          v5.8h,   v20.16b, v22.16b
        uaddl           v6.8h,   v23.8b,  v26.8b  // p0 + q2
        uaddl2          v7.8h,   v23.16b, v26.16b
        add             v8.8h,   v0.8h,   v0.8h   // 2 * (p3 + p2)
        add             v9.8h,   v1.8h,   v1.8h
        uaddw           v8.8h,   v8.8h,   v23.8b  // + p0
        uaddw2          v9.8h,   v9.8h,   v23.16b
        uaddw           v8.8h,   v8.8h,   v24.8b  // + q0
        uaddw2          v9.8h,   v9.8h,   v24.16b
        add             v8.8h,   v8.8h,   v4.8h
        add             v9.8h,   v9.8h,   v5.8h   // + p3 + p1
        sub             v2.8h,   v2.8h,   v0.8h   // p1 + q1 - p3 - p2
        sub             v3.8h,   v3.8h,   v1.8h
        sub             v6.8h,   v6.8h,   v4.8h   // p0 + q2 - p3 - p1
        sub             v7.8h,   v7.8h,   v5.8h
        rshrn           v10.8b,  v8.8h,   #3      // out p2
        rshrn2          v10.16b, v9.8h,   #3

        add             v8.8h,   v8.8h,   v2.8h
        add             v9.8h,   v9.8h,   v3.8h
        uaddl           v0.8h,   v20.8b,  v23.8b  // p3 + p0
        uaddl2          v1.8h,   v20.16b, v23.16b
        uaddl           v2.8h,   v24.8b,  v27.8b  // q0 + q3
        uaddl2          v3.8h,   v24.16b, v27.16b
        rshrn           v11.8b,  v8.8h,   #3      // out p1
        rshrn2          v11.16b, v9.8h,   #3

        add             v8.8h,   v8.8h,   v6.8h
        add             v9.8h,   v9.8h,   v7.8h
        sub             v2.8h,   v2.8h,   v0.8h   // q0 + q3 - p3 - p0
        sub             v3.8h,   v3.8h,   v1.8h
        uaddl           v4.8h,   v21.8b,  v24.8b  // p2 + q0
        uaddl2          v5.8h,   v21.16b, v24.16b
        uaddl           v6.8h,   v25.8b,  v27.8b  // q1 + q3
        uaddl2          v7.8h,   v25.16b, v27.16b
        rshrn           v12.8b,  v8.8h,   #3      // out p0
        rshrn2          v12.16b, v9.8h,   #3

        add             v8.8h,   v8.8h,   v2.8h
        add             v9.8h,   v9.8h,   v3.8h
        sub             v6.8h,   v6.8h,   v4.8h   // q1 + q3 - p2 - q0
        sub             v7.8h,   v7.8h,   v5.8h
        uaddl           v0.8h,   v22.8b,  v25.8b  // p1 + q1
        uaddl2          v1.8h,   v22.16b, v25.16b
        uaddl           v2.8h,   v26.8b,  v27.8b  // q2 + q3
        uaddl2          v3.8h,   v26.16b, v27.16b
        rshrn           v13.8b,  v8.8h,   #3      // out q0
        rshrn2          v13.16b, v9.8h,   #3

        add             v8.8h,   v8.8h,   v6.8h
        add             v9.8h,   v9.8h,   v7.8h
        sub             v2.8h,   v2.8h,   v0.8h   // q2 + q3 - p1 - q1
        sub             v3.8h,   v3.8h,   v1.8h
        rshrn           v0.8b,   v8.8h,   #3      // out q1
        rshrn2          v0.16b,  v9.8h,   #3

        add             v8.8h,   v8.8h,   v2.8h
        add             v9.8h ,  v9.8h,   v3.8h

        bit             v21.16b, v10.16b, v14.16b
        bit             v22.16b, v11.16b, v14.16b
        bit             v23.16b, v12.16b, v14.16b
        rshrn           v1.8b,   v8.8h,   #3      // out q2
        rshrn2          v1.16b,  v9.8h,   #3
        bit             v24.16b, v13.16b, v14.16b
        bit             v25.16b, v0.16b,  v14.16b
        bit             v26.16b, v1.16b,  v14.16b
.endif
2:
.if \wd == 16
        mov             x16, v15.d[0]
        mov             x17, v15.d[1]
        adds            x16, x16, x17
        b.ne            1f                        // check if flat8out is needed
        mov             x16, v14.d[0]
        mov             x17, v14.d[1]
        adds            x16, x16, x17
        b.eq            8f                        // if there was no flat8in, just write the inner 4 pixels
        b               7f                        // if flat8in was used, write the inner 6 pixels
1:

        uaddl           v2.8h,   v17.8b,  v17.8b  // p6 + p6
        uaddl2          v3.8h,   v17.16b, v17.16b
        uaddl           v4.8h,   v17.8b,  v18.8b  // p6 + p5
        uaddl2          v5.8h,   v17.16b, v18.16b
        uaddl           v6.8h,   v17.8b,  v19.8b  // p6 + p4
        uaddl2          v7.8h,   v17.16b, v19.16b
        uaddl           v8.8h,   v17.8b,  v20.8b  // p6 + p3
        uaddl2          v9.8h,   v17.16b, v20.16b
        add             v12.8h,  v2.8h,   v4.8h
        add             v13.8h,  v3.8h,   v5.8h
        add             v10.8h,  v6.8h,   v8.8h
        add             v11.8h,  v7.8h,   v9.8h
        uaddl           v6.8h,   v17.8b,  v21.8b  // p6 + p2
        uaddl2          v7.8h,   v17.16b, v21.16b
        add             v12.8h,  v12.8h,  v10.8h
        add             v13.8h,  v13.8h,  v11.8h
        uaddl           v8.8h,   v17.8b,  v22.8b  // p6 + p1
        uaddl2          v9.8h,   v17.16b, v22.16b
        uaddl           v10.8h,  v18.8b,  v23.8b  // p5 + p0
        uaddl2          v11.8h,  v18.16b, v23.16b
        add             v6.8h,   v6.8h,   v8.8h
        add             v7.8h,   v7.8h,   v9.8h
        uaddl           v8.8h,   v19.8b,  v24.8b  // p4 + q0
        uaddl2          v9.8h,   v19.16b, v24.16b
        add             v12.8h,  v12.8h,  v6.8h
        add             v13.8h,  v13.8h,  v7.8h
        add             v10.8h,  v10.8h,  v8.8h
        add             v11.8h,  v11.8h,  v9.8h
        uaddl           v6.8h,   v20.8b,  v25.8b  // p3 + q1
        uaddl2          v7.8h,   v20.16b, v25.16b
        add             v12.8h,  v12.8h,  v10.8h
        add             v13.8h,  v13.8h,  v11.8h
        sub             v6.8h,   v6.8h,   v2.8h
        sub             v7.8h,   v7.8h,   v3.8h
        uaddl           v2.8h,   v21.8b,  v26.8b  // p2 + q2
        uaddl2          v3.8h,   v21.16b, v26.16b
        rshrn           v0.8b,   v12.8h,  #4      // out p5
        rshrn2          v0.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v6.8h   // - (p6 + p6) + (p3 + q1)
        add             v13.8h,  v13.8h,  v7.8h
        sub             v2.8h,   v2.8h,   v4.8h
        sub             v3.8h,   v3.8h,   v5.8h
        uaddl           v4.8h,   v22.8b,  v27.8b  // p1 + q3
        uaddl2          v5.8h,   v22.16b, v27.16b
        uaddl           v6.8h,   v17.8b,  v19.8b  // p6 + p4
        uaddl2          v7.8h,   v17.16b, v19.16b
        rshrn           v1.8b,   v12.8h,  #4      // out p4
        rshrn2          v1.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v2.8h   // - (p6 + p5) + (p2 + q2)
        add             v13.8h,  v13.8h,  v3.8h
        sub             v4.8h,   v4.8h,   v6.8h
        sub             v5.8h,   v5.8h,   v7.8h
        uaddl           v6.8h,   v23.8b,  v28.8b  // p0 + q4
        uaddl2          v7.8h,   v23.16b, v28.16b
        uaddl           v8.8h,   v17.8b,  v20.8b  // p6 + p3
        uaddl2          v9.8h,   v17.16b, v20.16b
        rshrn           v2.8b,   v12.8h,  #4      // out p3
        rshrn2          v2.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v4.8h   // - (p6 + p4) + (p1 + q3)
        add             v13.8h,  v13.8h,  v5.8h
        sub             v6.8h,   v6.8h,   v8.8h
        sub             v7.8h,   v7.8h,   v9.8h
        uaddl           v8.8h,   v24.8b,  v29.8b  // q0 + q5
        uaddl2          v9.8h,   v24.16b, v29.16b
        uaddl           v4.8h,   v17.8b,  v21.8b  // p6 + p2
        uaddl2          v5.8h,   v17.16b, v21.16b
        rshrn           v3.8b,   v12.8h,  #4      // out p2
        rshrn2          v3.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v6.8h   // - (p6 + p3) + (p0 + q4)
        add             v13.8h,  v13.8h,  v7.8h
        sub             v8.8h,   v8.8h,   v4.8h
        sub             v9.8h,   v9.8h,   v5.8h
        uaddl           v6.8h,   v25.8b,  v30.8b  // q1 + q6
        uaddl2          v7.8h,   v25.16b, v30.16b
        uaddl           v10.8h,  v17.8b,  v22.8b  // p6 + p1
        uaddl2          v11.8h,  v17.16b, v22.16b
        rshrn           v4.8b,   v12.8h,  #4      // out p1
        rshrn2          v4.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v8.8h   // - (p6 + p2) + (q0 + q5)
        add             v13.8h,  v13.8h,  v9.8h
        sub             v6.8h,   v6.8h,   v10.8h
        sub             v7.8h,   v7.8h,   v11.8h
        uaddl           v8.8h,   v26.8b,  v30.8b  // q2 + q6
        uaddl2          v9.8h,   v26.16b, v30.16b
        bif             v0.16b,  v18.16b, v15.16b // out p5
        uaddl           v10.8h,  v18.8b,  v23.8b  // p5 + p0
        uaddl2          v11.8h,  v18.16b, v23.16b
        rshrn           v5.8b,   v12.8h,  #4      // out p0
        rshrn2          v5.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v6.8h   // - (p6 + p1) + (q1 + q6)
        add             v13.8h,  v13.8h,  v7.8h
        sub             v8.8h,   v8.8h,   v10.8h
        sub             v9.8h,   v9.8h,   v11.8h
        uaddl           v10.8h,  v27.8b,  v30.8b  // q3 + q6
        uaddl2          v11.8h,  v27.16b, v30.16b
        bif             v1.16b,  v19.16b, v15.16b // out p4
        uaddl           v18.8h,  v19.8b,  v24.8b  // p4 + q0
        uaddl2          v19.8h,  v19.16b, v24.16b
        rshrn           v6.8b,   v12.8h,  #4      // out q0
        rshrn2          v6.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v8.8h   // - (p5 + p0) + (q2 + q6)
        add             v13.8h,  v13.8h,  v9.8h
        sub             v10.8h,  v10.8h,  v18.8h
        sub             v11.8h,  v11.8h,  v19.8h
        uaddl           v8.8h,   v28.8b,  v30.8b  // q4 + q6
        uaddl2          v9.8h,   v28.16b, v30.16b
        bif             v2.16b,  v20.16b, v15.16b // out p3
        uaddl           v18.8h,  v20.8b,  v25.8b  // p3 + q1
        uaddl2          v19.8h,  v20.16b, v25.16b
        rshrn           v7.8b,   v12.8h,  #4      // out q1
        rshrn2          v7.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v10.8h  // - (p4 + q0) + (q3 + q6)
        add             v13.8h,  v13.8h,  v11.8h
        sub             v18.8h,  v8.8h,   v18.8h
        sub             v19.8h,  v9.8h,   v19.8h
        uaddl           v10.8h,  v29.8b,  v30.8b  // q5 + q6
        uaddl2          v11.8h,  v29.16b, v30.16b
        bif             v3.16b,  v21.16b, v15.16b // out p2
        uaddl           v20.8h,  v21.8b,  v26.8b  // p2 + q2
        uaddl2          v21.8h,  v21.16b, v26.16b
        rshrn           v8.8b,   v12.8h,  #4      // out q2
        rshrn2          v8.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v18.8h  // - (p3 + q1) + (q4 + q6)
        add             v13.8h,  v13.8h,  v19.8h
        sub             v10.8h,  v10.8h,  v20.8h
        sub             v11.8h,  v11.8h,  v21.8h
        uaddl           v18.8h,  v30.8b,  v30.8b  // q6 + q6
        uaddl2          v19.8h,  v30.16b, v30.16b
        bif             v4.16b,  v22.16b, v15.16b // out p1
        uaddl           v20.8h,  v22.8b,  v27.8b  // p1 + q3
        uaddl2          v21.8h,  v22.16b, v27.16b
        rshrn           v9.8b,   v12.8h,  #4      // out q3
        rshrn2          v9.16b,  v13.8h,  #4
        add             v12.8h,  v12.8h,  v10.8h  // - (p2 + q2) + (q5 + q6)
        add             v13.8h,  v13.8h,  v11.8h
        sub             v18.8h,  v18.8h,  v20.8h
        sub             v19.8h,  v19.8h,  v21.8h
        bif             v5.16b,  v23.16b, v15.16b // out p0
        rshrn           v10.8b,  v12.8h,  #4      // out q4
        rshrn2          v10.16b, v13.8h,  #4
        add             v12.8h,  v12.8h,  v18.8h  // - (p1 + q3) + (q6 + q6)
        add             v13.8h,  v13.8h,  v19.8h
        rshrn           v11.8b,  v12.8h,  #4      // out q5
        rshrn2          v11.16b, v13.8h,  #4
        bif             v6.16b,  v24.16b, v15.16b // out q0
        bif             v7.16b,  v25.16b, v15.16b // out q1
        bif             v8.16b,  v26.16b, v15.16b // out q2
        bif             v9.16b,  v27.16b, v15.16b // out q3
        bif             v10.16b, v28.16b, v15.16b // out q4
        bif             v11.16b, v29.16b, v15.16b // out q5
.endif

        mov             x14, #0
        ret
.if \wd == 16
7:
        // Return to a shorter epilogue, writing only the inner 6 pixels
        mov             x14, #(1 << 6)
        ret
.endif
.if \wd >= 8
8:
        // Return to a shorter epilogue, writing only the inner 4 pixels
        mov             x14, #(1 << 4)
        ret
.endif
endfunc
.endm

loop_filter 16
loop_filter 8
loop_filter 6
loop_filter 4

.macro lpf_16_wd16
        bl              lpf_16_wd16_neon
        cbz             x14, 1f
        tbnz            x14, #6, 7f
        tbnz            x14, #4, 8f
        ret             x15
1:
.endm

.macro lpf_16_wd8
        bl              lpf_16_wd8_neon
        cbz             x14, 1f
        tbnz            x14, #4, 8f
        ret             x15
1:
.endm

.macro lpf_16_wd6
        bl              lpf_16_wd6_neon
        cbz             x14, 1f
        ret             x15
1:
.endm

.macro lpf_16_wd4
        bl              lpf_16_wd4_neon
        cbz             x14, 1f
        ret             x15
1:
.endm

function lpf_v_4_16_neon
        mov             x15, x30
        sub             x16, x0,  x1, lsl #1
        ld1             {v22.16b}, [x16], x1 // p1
        ld1             {v24.16b}, [x0],  x1 // q0
        ld1             {v23.16b}, [x16], x1 // p0
        ld1             {v25.16b}, [x0],  x1 // q1
        sub             x0,  x0,  x1, lsl #1

        lpf_16_wd4

        sub             x16, x0,  x1, lsl #1
        st1             {v22.16b}, [x16], x1 // p1
        st1             {v24.16b}, [x0],  x1 // q0
        st1             {v23.16b}, [x16], x1 // p0
        st1             {v25.16b}, [x0],  x1 // q1
        sub             x0,  x0,  x1, lsl #1
        ret             x15
endfunc

function lpf_h_4_16_neon
        mov             x15, x30
        sub             x16, x0,  #2
        add             x0,  x16, x1, lsl #3
        ld1             {v22.s}[0], [x16], x1
        ld1             {v22.s}[2], [x0],  x1
        ld1             {v23.s}[0], [x16], x1
        ld1             {v23.s}[2], [x0],  x1
        ld1             {v24.s}[0], [x16], x1
        ld1             {v24.s}[2], [x0],  x1
        ld1             {v25.s}[0], [x16], x1
        ld1             {v25.s}[2], [x0],  x1
        ld1             {v22.s}[1], [x16], x1
        ld1             {v22.s}[3], [x0],  x1
        ld1             {v23.s}[1], [x16], x1
        ld1             {v23.s}[3], [x0],  x1
        ld1             {v24.s}[1], [x16], x1
        ld1             {v24.s}[3], [x0],  x1
        ld1             {v25.s}[1], [x16], x1
        ld1             {v25.s}[3], [x0],  x1
        add             x0,  x0,  #2

        transpose_4x16b v22, v23, v24, v25, v26, v27, v28, v29

        lpf_16_wd4

        sub             x16, x0,  x1, lsl #4
        sub             x16, x16, #2
        transpose_4x16b v22, v23, v24, v25, v26, v27, v28, v29
        add             x0,  x16, x1, lsl #3

        st1             {v22.s}[0], [x16], x1
        st1             {v22.s}[2], [x0],  x1
        st1             {v23.s}[0], [x16], x1
        st1             {v23.s}[2], [x0],  x1
        st1             {v24.s}[0], [x16], x1
        st1             {v24.s}[2], [x0],  x1
        st1             {v25.s}[0], [x16], x1
        st1             {v25.s}[2], [x0],  x1
        st1             {v22.s}[1], [x16], x1
        st1             {v22.s}[3], [x0],  x1
        st1             {v23.s}[1], [x16], x1
        st1             {v23.s}[3], [x0],  x1
        st1             {v24.s}[1], [x16], x1
        st1             {v24.s}[3], [x0],  x1
        st1             {v25.s}[1], [x16], x1
        st1             {v25.s}[3], [x0],  x1
        add             x0,  x0,  #2
        ret             x15
endfunc

function lpf_v_6_16_neon
        mov             x15, x30
        sub             x16, x0,  x1, lsl #1
        sub             x16, x16, x1
        ld1             {v21.16b}, [x16], x1 // p2
        ld1             {v24.16b}, [x0],  x1 // q0
        ld1             {v22.16b}, [x16], x1 // p1
        ld1             {v25.16b}, [x0],  x1 // q1
        ld1             {v23.16b}, [x16], x1 // p0
        ld1             {v26.16b}, [x0],  x1 // q2
        sub             x0,  x0,  x1, lsl #1
        sub             x0,  x0,  x1

        lpf_16_wd6

        sub             x16, x0,  x1, lsl #1
        st1             {v22.16b}, [x16], x1 // p1
        st1             {v24.16b}, [x0],  x1 // q0
        st1             {v23.16b}, [x16], x1 // p0
        st1             {v25.16b}, [x0],  x1 // q1
        sub             x0,  x0,  x1, lsl #1
        ret             x15
endfunc

function lpf_h_6_16_neon
        mov             x15, x30
        sub             x16, x0,  #4
        add             x0,  x16, x1, lsl #3
        ld1             {v20.d}[0], [x16], x1
        ld1             {v20.d}[1], [x0],  x1
        ld1             {v21.d}[0], [x16], x1
        ld1             {v21.d}[1], [x0],  x1
        ld1             {v22.d}[0], [x16], x1
        ld1             {v22.d}[1], [x0],  x1
        ld1             {v23.d}[0], [x16], x1
        ld1             {v23.d}[1], [x0],  x1
        ld1             {v24.d}[0], [x16], x1
        ld1             {v24.d}[1], [x0],  x1
        ld1             {v25.d}[0], [x16], x1
        ld1             {v25.d}[1], [x0],  x1
        ld1             {v26.d}[0], [x16], x1
        ld1             {v26.d}[1], [x0],  x1
        ld1             {v27.d}[0], [x16], x1
        ld1             {v27.d}[1], [x0],  x1
        add             x0,  x0,  #4

        transpose_8x16b v20, v21, v22, v23, v24, v25, v26, v27, v28, v29

        lpf_16_wd6

        sub             x16, x0,  x1, lsl #4
        sub             x16, x16, #2
        transpose_4x16b v22, v23, v24, v25, v26, v27, v28, v29
        add             x0,  x16, x1, lsl #3

        st1             {v22.s}[0], [x16], x1
        st1             {v22.s}[2], [x0],  x1
        st1             {v23.s}[0], [x16], x1
        st1             {v23.s}[2], [x0],  x1
        st1             {v24.s}[0], [x16], x1
        st1             {v24.s}[2], [x0],  x1
        st1             {v25.s}[0], [x16], x1
        st1             {v25.s}[2], [x0],  x1
        st1             {v22.s}[1], [x16], x1
        st1             {v22.s}[3], [x0],  x1
        st1             {v23.s}[1], [x16], x1
        st1             {v23.s}[3], [x0],  x1
        st1             {v24.s}[1], [x16], x1
        st1             {v24.s}[3], [x0],  x1
        st1             {v25.s}[1], [x16], x1
        st1             {v25.s}[3], [x0],  x1
        add             x0,  x0,  #2
        ret             x15
endfunc

function lpf_v_8_16_neon
        mov             x15, x30
        sub             x16, x0,  x1, lsl #2
        ld1             {v20.16b}, [x16], x1 // p3
        ld1             {v24.16b}, [x0],  x1 // q0
        ld1             {v21.16b}, [x16], x1 // p2
        ld1             {v25.16b}, [x0],  x1 // q1
        ld1             {v22.16b}, [x16], x1 // p1
        ld1             {v26.16b}, [x0],  x1 // q2
        ld1             {v23.16b}, [x16], x1 // p0
        ld1             {v27.16b}, [x0],  x1 // q3
        sub             x0,  x0,  x1, lsl #2

        lpf_16_wd8

        sub             x16, x0,  x1, lsl #1
        sub             x16, x16,  x1
        st1             {v21.16b}, [x16], x1 // p2
        st1             {v24.16b}, [x0],  x1 // q0
        st1             {v22.16b}, [x16], x1 // p1
        st1             {v25.16b}, [x0],  x1 // q1
        st1             {v23.16b}, [x16], x1 // p0
        st1             {v26.16b}, [x0],  x1 // q2
        sub             x0,  x0,  x1, lsl #1
        sub             x0,  x0,  x1
        ret             x15

8:
        sub             x16, x0,  x1, lsl #1
        st1             {v22.16b}, [x16], x1 // p1
        st1             {v24.16b}, [x0],  x1 // q0
        st1             {v23.16b}, [x16], x1 // p0
        st1             {v25.16b}, [x0],  x1 // q1
        sub             x0,  x0,  x1, lsl #1
        ret             x15
endfunc

function lpf_h_8_16_neon
        mov             x15, x30
        sub             x16, x0,  #4
        add             x0,  x16, x1, lsl #3
        ld1             {v20.d}[0], [x16], x1
        ld1             {v20.d}[1], [x0],  x1
        ld1             {v21.d}[0], [x16], x1
        ld1             {v21.d}[1], [x0],  x1
        ld1             {v22.d}[0], [x16], x1
        ld1             {v22.d}[1], [x0],  x1
        ld1             {v23.d}[0], [x16], x1
        ld1             {v23.d}[1], [x0],  x1
        ld1             {v24.d}[0], [x16], x1
        ld1             {v24.d}[1], [x0],  x1
        ld1             {v25.d}[0], [x16], x1
        ld1             {v25.d}[1], [x0],  x1
        ld1             {v26.d}[0], [x16], x1
        ld1             {v26.d}[1], [x0],  x1
        ld1             {v27.d}[0], [x16], x1
        ld1             {v27.d}[1], [x0],  x1
        add             x0,  x0,  #4

        transpose_8x16b v20, v21, v22, v23, v24, v25, v26, v27, v28, v29

        lpf_16_wd8

        sub             x16, x0,  x1, lsl #4
        sub             x16, x16, #4
        transpose_8x16b v20, v21, v22, v23, v24, v25, v26, v27, v28, v29
        add             x0,  x16, x1, lsl #3

        st1             {v20.d}[0], [x16], x1
        st1             {v20.d}[1], [x0],  x1
        st1             {v21.d}[0], [x16], x1
        st1             {v21.d}[1], [x0],  x1
        st1             {v22.d}[0], [x16], x1
        st1             {v22.d}[1], [x0],  x1
        st1             {v23.d}[0], [x16], x1
        st1             {v23.d}[1], [x0],  x1
        st1             {v24.d}[0], [x16], x1
        st1             {v24.d}[1], [x0],  x1
        st1             {v25.d}[0], [x16], x1
        st1             {v25.d}[1], [x0],  x1
        st1             {v26.d}[0], [x16], x1
        st1             {v26.d}[1], [x0],  x1
        st1             {v27.d}[0], [x16], x1
        st1             {v27.d}[1], [x0],  x1
        add             x0,  x0,  #4
        ret             x15
8:
        sub             x16, x0,  x1, lsl #4
        sub             x16, x16, #2
        transpose_4x16b v22, v23, v24, v25, v26, v27, v28, v29
        add             x0,  x16, x1, lsl #3

        st1             {v22.s}[0], [x16], x1
        st1             {v22.s}[2], [x0],  x1
        st1             {v23.s}[0], [x16], x1
        st1             {v23.s}[2], [x0],  x1
        st1             {v24.s}[0], [x16], x1
        st1             {v24.s}[2], [x0],  x1
        st1             {v25.s}[0], [x16], x1
        st1             {v25.s}[2], [x0],  x1
        st1             {v22.s}[1], [x16], x1
        st1             {v22.s}[3], [x0],  x1
        st1             {v23.s}[1], [x16], x1
        st1             {v23.s}[3], [x0],  x1
        st1             {v24.s}[1], [x16], x1
        st1             {v24.s}[3], [x0],  x1
        st1             {v25.s}[1], [x16], x1
        st1             {v25.s}[3], [x0],  x1
        add             x0,  x0,  #2
        ret             x15
endfunc

function lpf_v_16_16_neon
        mov             x15, x30

        sub             x16, x0,  x1, lsl #3
        add             x16, x16, x1
        ld1             {v17.16b}, [x16], x1 // p6
        ld1             {v24.16b}, [x0],  x1 // q0
        ld1             {v18.16b}, [x16], x1 // p5
        ld1             {v25.16b}, [x0],  x1 // q1
        ld1             {v19.16b}, [x16], x1 // p4
        ld1             {v26.16b}, [x0],  x1 // q2
        ld1             {v20.16b}, [x16], x1 // p3
        ld1             {v27.16b}, [x0],  x1 // q3
        ld1             {v21.16b}, [x16], x1 // p2
        ld1             {v28.16b}, [x0],  x1 // q4
        ld1             {v22.16b}, [x16], x1 // p1
        ld1             {v29.16b}, [x0],  x1 // q5
        ld1             {v23.16b}, [x16], x1 // p0
        ld1             {v30.16b}, [x0],  x1 // q6
        sub             x0,  x0,  x1, lsl #3
        add             x0,  x0,  x1

        lpf_16_wd16

        sub             x16, x0,  x1, lsl #2
        sub             x16, x16,  x1, lsl #1
        st1             {v0.16b},  [x16], x1 // p5
        st1             {v6.16b},  [x0],  x1 // q0
        st1             {v1.16b},  [x16], x1 // p4
        st1             {v7.16b},  [x0],  x1 // q1
        st1             {v2.16b},  [x16], x1 // p3
        st1             {v8.16b},  [x0],  x1 // q2
        st1             {v3.16b},  [x16], x1 // p2
        st1             {v9.16b},  [x0],  x1 // q3
        st1             {v4.16b},  [x16], x1 // p1
        st1             {v10.16b}, [x0],  x1 // q4
        st1             {v5.16b},  [x16], x1 // p0
        st1             {v11.16b}, [x0],  x1 // q5
        sub             x0,  x0,  x1, lsl #2
        sub             x0,  x0,  x1, lsl #1
        ret             x15
7:
        sub             x16, x0,  x1
        sub             x16, x16, x1, lsl #1
        st1             {v21.16b}, [x16], x1 // p2
        st1             {v24.16b}, [x0],  x1 // q0
        st1             {v22.16b}, [x16], x1 // p1
        st1             {v25.16b}, [x0],  x1 // q1
        st1             {v23.16b}, [x16], x1 // p0
        st1             {v26.16b}, [x0],  x1 // q2
        sub             x0,  x0,  x1, lsl #1
        sub             x0,  x0,  x1
        ret             x15

8:
        sub             x16, x0,  x1, lsl #1
        st1             {v22.16b}, [x16], x1 // p1
        st1             {v24.16b}, [x0],  x1 // q0
        st1             {v23.16b}, [x16], x1 // p0
        st1             {v25.16b}, [x0],  x1 // q1
        sub             x0,  x0,  x1, lsl #1
        ret             x15
endfunc

function lpf_h_16_16_neon
        mov             x15, x30
        sub             x16, x0,  #8
        ld1             {v16.d}[0], [x16], x1
        ld1             {v24.d}[0], [x0],  x1
        ld1             {v17.d}[0], [x16], x1
        ld1             {v25.d}[0], [x0],  x1
        ld1             {v18.d}[0], [x16], x1
        ld1             {v26.d}[0], [x0],  x1
        ld1             {v19.d}[0], [x16], x1
        ld1             {v27.d}[0], [x0],  x1
        ld1             {v20.d}[0], [x16], x1
        ld1             {v28.d}[0], [x0],  x1
        ld1             {v21.d}[0], [x16], x1
        ld1             {v29.d}[0], [x0],  x1
        ld1             {v22.d}[0], [x16], x1
        ld1             {v30.d}[0], [x0],  x1
        ld1             {v23.d}[0], [x16], x1
        ld1             {v31.d}[0], [x0],  x1
        ld1             {v16.d}[1], [x16], x1
        ld1             {v24.d}[1], [x0],  x1
        ld1             {v17.d}[1], [x16], x1
        ld1             {v25.d}[1], [x0],  x1
        ld1             {v18.d}[1], [x16], x1
        ld1             {v26.d}[1], [x0],  x1
        ld1             {v19.d}[1], [x16], x1
        ld1             {v27.d}[1], [x0],  x1
        ld1             {v20.d}[1], [x16], x1
        ld1             {v28.d}[1], [x0],  x1
        ld1             {v21.d}[1], [x16], x1
        ld1             {v29.d}[1], [x0],  x1
        ld1             {v22.d}[1], [x16], x1
        ld1             {v30.d}[1], [x0],  x1
        ld1             {v23.d}[1], [x16], x1
        ld1             {v31.d}[1], [x0],  x1

        transpose_8x16b v16, v17, v18, v19, v20, v21, v22, v23, v0,  v1
        transpose_8x16b v24, v25, v26, v27, v28, v29, v30, v31, v0,  v1

        lpf_16_wd16

        sub             x0,  x0,  x1, lsl #4
        sub             x16, x0,  #8

        transpose_8x16b v16, v17, v0,  v1,  v2,  v3,  v4,  v5,  v18, v19
        transpose_8x16b v6,  v7,  v8,  v9,  v10, v11, v30, v31, v18, v19

        st1             {v16.d}[0], [x16], x1
        st1             {v6.d}[0],  [x0],  x1
        st1             {v17.d}[0], [x16], x1
        st1             {v7.d}[0],  [x0],  x1
        st1             {v0.d}[0],  [x16], x1
        st1             {v8.d}[0],  [x0],  x1
        st1             {v1.d}[0],  [x16], x1
        st1             {v9.d}[0],  [x0],  x1
        st1             {v2.d}[0],  [x16], x1
        st1             {v10.d}[0], [x0],  x1
        st1             {v3.d}[0],  [x16], x1
        st1             {v11.d}[0], [x0],  x1
        st1             {v4.d}[0],  [x16], x1
        st1             {v30.d}[0], [x0],  x1
        st1             {v5.d}[0],  [x16], x1
        st1             {v31.d}[0], [x0],  x1
        st1             {v16.d}[1], [x16], x1
        st1             {v6.d}[1],  [x0],  x1
        st1             {v17.d}[1], [x16], x1
        st1             {v7.d}[1],  [x0],  x1
        st1             {v0.d}[1],  [x16], x1
        st1             {v8.d}[1],  [x0],  x1
        st1             {v1.d}[1],  [x16], x1
        st1             {v9.d}[1],  [x0],  x1
        st1             {v2.d}[1],  [x16], x1
        st1             {v10.d}[1], [x0],  x1
        st1             {v3.d}[1],  [x16], x1
        st1             {v11.d}[1], [x0],  x1
        st1             {v4.d}[1],  [x16], x1
        st1             {v30.d}[1], [x0],  x1
        st1             {v5.d}[1],  [x16], x1
        st1             {v31.d}[1], [x0],  x1
        ret             x15

7:
        sub             x16, x0,  x1, lsl #4
        sub             x16, x16, #4
        transpose_8x16b v20, v21, v22, v23, v24, v25, v26, v27, v28, v29
        add             x0,  x16, x1, lsl #3

        st1             {v20.d}[0], [x16], x1
        st1             {v20.d}[1], [x0],  x1
        st1             {v21.d}[0], [x16], x1
        st1             {v21.d}[1], [x0],  x1
        st1             {v22.d}[0], [x16], x1
        st1             {v22.d}[1], [x0],  x1
        st1             {v23.d}[0], [x16], x1
        st1             {v23.d}[1], [x0],  x1
        st1             {v24.d}[0], [x16], x1
        st1             {v24.d}[1], [x0],  x1
        st1             {v25.d}[0], [x16], x1
        st1             {v25.d}[1], [x0],  x1
        st1             {v26.d}[0], [x16], x1
        st1             {v26.d}[1], [x0],  x1
        st1             {v27.d}[0], [x16], x1
        st1             {v27.d}[1], [x0],  x1
        add             x0,  x0,  #4
        ret             x15
8:
        sub             x16, x0,  x1, lsl #4
        sub             x16, x16, #2
        transpose_4x16b v22, v23, v24, v25, v26, v27, v28, v29
        add             x0,  x16, x1, lsl #3

        st1             {v22.s}[0], [x16], x1
        st1             {v22.s}[2], [x0],  x1
        st1             {v23.s}[0], [x16], x1
        st1             {v23.s}[2], [x0],  x1
        st1             {v24.s}[0], [x16], x1
        st1             {v24.s}[2], [x0],  x1
        st1             {v25.s}[0], [x16], x1
        st1             {v25.s}[2], [x0],  x1
        st1             {v22.s}[1], [x16], x1
        st1             {v22.s}[3], [x0],  x1
        st1             {v23.s}[1], [x16], x1
        st1             {v23.s}[3], [x0],  x1
        st1             {v24.s}[1], [x16], x1
        st1             {v24.s}[3], [x0],  x1
        st1             {v25.s}[1], [x16], x1
        st1             {v25.s}[3], [x0],  x1
        add             x0,  x0,  #2
        ret             x15
endfunc

// void dav1d_lpf_v_sb_y_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                 const uint32_t *const vmask,
//                                 const uint8_t (*l)[4], ptrdiff_t b4_stride,
//                                 const Av1FilterLUT *lut, const int w)

.macro lpf_func dir, type
function lpf_\dir\()_sb_\type\()_8bpc_neon, export=1
        mov             x11, x30
        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]
        ldp             w6,  w7,  [x2]           // vmask[0], vmask[1]
.ifc \type, y
        ldr             w2,  [x2, #8]            // vmask[2]
.endif
        add             x5,  x5,  #128           // Move to sharp part of lut
.ifc \type, y
        orr             w7,  w7,  w2             // vmask[1] |= vmask[2]
.endif
.ifc \dir, v
        sub             x4,  x3,  x4, lsl #2
.else
        sub             x3,  x3,  #4
        lsl             x4,  x4,  #2
.endif
        orr             w6,  w6,  w7             // vmask[0] |= vmask[1]

1:
        tst             w6,  #0x0f
.ifc \dir, v
        ld1             {v0.16b}, [x4], #16
        ld1             {v1.16b}, [x3], #16
.else
        ld2             {v0.s,v1.s}[0], [x3], x4
        ld2             {v0.s,v1.s}[1], [x3], x4
        ld2             {v0.s,v1.s}[2], [x3], x4
        ld2             {v0.s,v1.s}[3], [x3], x4
.endif
        b.eq            7f                        // if (!(vm & bits)) continue;

        ld1r            {v5.16b}, [x5]            // sharp[0]
        add             x5,  x5,  #8
        movi            v2.4s,   #0xff
        dup             v13.4s,  w6               // vmask[0]

        and             v0.16b,  v0.16b,  v2.16b  // Keep only lowest byte in each 32 bit word
        and             v1.16b,  v1.16b,  v2.16b
        cmtst           v3.16b,  v1.16b,  v2.16b  // Check for nonzero values in l[0][0]
        movi            v4.16b,  #1
        ld1r            {v6.16b}, [x5]            // sharp[1]
        sub             x5,  x5,  #8
        bif             v1.16b,  v0.16b,  v3.16b  // if (!l[0][0]) L = l[offset][0]
        cmtst           v2.4s,   v1.4s,   v2.4s   // L != 0
        mul             v1.4s,   v1.4s,   v4.4s   // L
.ifc \type, y
        dup             v15.4s,  w2               // vmask[2]
.endif
        dup             v14.4s,  w7               // vmask[1]
        mov             x16, v2.d[0]
        mov             x17, v2.d[1]
        adds            x16, x16, x17
        b.eq            7f                        // if (!L) continue;
        neg             v5.16b,  v5.16b           // -sharp[0]
        movrel          x16,  word_1248
        ushr            v12.16b, v1.16b,  #4      // H
        ld1             {v16.4s}, [x16]
        sshl            v3.16b,  v1.16b,  v5.16b  // L >> sharp[0]
.ifc \type, y
        cmtst           v15.4s,  v15.4s,  v16.4s  // if (vmask[2] & bits)
.endif
        movi            v7.16b,  #2
        umin            v3.16b,  v3.16b,  v6.16b  // imin(L >> sharp[0], sharp[1])
        add             v0.16b,  v1.16b,  v7.16b  // L + 2
        umax            v11.16b, v3.16b,  v4.16b  // imax(imin(), 1) = limit = I
        add             v0.16b,  v0.16b,  v0.16b  // 2*(L + 2)
        cmtst           v14.4s,  v14.4s,  v16.4s  // if (vmask[1] & bits)
        add             v10.16b, v0.16b,  v11.16b // 2*(L + 2) + limit = E
        cmtst           v13.4s,  v13.4s,  v16.4s  // if (vmask[0] & bits)
        and             v13.16b, v13.16b, v2.16b  // vmask[0] &= L != 0

.ifc \type, y
        tst             w2,  #0x0f
        b.eq            2f
        // wd16
        bl              lpf_\dir\()_16_16_neon
        b               8f
2:
.endif
        tst             w7,  #0x0f
        b.eq            3f
.ifc \type, y
        // wd8
        bl              lpf_\dir\()_8_16_neon
.else
        // wd6
        bl              lpf_\dir\()_6_16_neon
.endif
        b               8f
3:
        // wd4
        bl              lpf_\dir\()_4_16_neon
.ifc \dir, h
        b               8f
7:
        // For dir h, the functions above increment x0.
        // If the whole function is skipped, increment it here instead.
        add             x0,  x0,  x1,  lsl #4
.else
7:
.endif
8:
        lsr             w6,  w6,  #4              // vmask[0] >>= 4
        lsr             w7,  w7,  #4              // vmask[1] >>= 4
.ifc \type, y
        lsr             w2,  w2,  #4              // vmask[2] >>= 4
.endif
.ifc \dir, v
        add             x0,  x0,  #16
.else
        // For dir h, x0 is returned incremented
.endif
        cbnz            w6,  1b

        ldp             d14, d15, [sp, #0x30]
        ldp             d12, d13, [sp, #0x20]
        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x40
        ret             x11
endfunc
.endm

lpf_func v, y
lpf_func h, y
lpf_func v, uv
lpf_func h, uv

const word_1248
        .word 1, 2, 4, 8
endconst
