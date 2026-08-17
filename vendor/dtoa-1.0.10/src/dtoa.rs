// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// ---
//
// The C++ implementation preserved here in comments is licensed as follows:
//
// Tencent is pleased to support the open source community by making RapidJSON
// available.
//
// Copyright (C) 2015 THL A29 Limited, a Tencent company, and Milo Yip. All
// rights reserved.
//
// Licensed under the MIT License (the "License"); you may not use this file
// except in compliance with the License. You may obtain a copy of the License
// at
//
// http://opensource.org/licenses/MIT
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

use core::ptr;
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

/*
inline unsigned CountDecimalDigit32(uint32_t n) {
    // Simple pure C++ implementation was faster than __builtin_clz version in this situation.
    if (n < 10) return 1;
    if (n < 100) return 2;
    if (n < 1000) return 3;
    if (n < 10000) return 4;
    if (n < 100000) return 5;
    if (n < 1000000) return 6;
    if (n < 10000000) return 7;
    if (n < 100000000) return 8;
    // Will not reach 10 digits in DigitGen()
    //if (n < 1000000000) return 9;
    //return 10;
    return 9;
}
*/

#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
pub fn count_decimal_digit32(n: u32) -> usize {
    if n < 10 {
        1
    } else if n < 100 {
        2
    } else if n < 1000 {
        3
    } else if n < 10000 {
        4
    } else if n < 100000 {
        5
    } else if n < 1000000 {
        6
    } else if n < 10000000 {
        7
    } else if n < 100000000 {
        8
    }
    // Will not reach 10 digits in digit_gen()
    else {
        9
    }
}

/*
inline char* WriteExponent(int K, char* buffer) {
    if (K < 0) {
        *buffer++ = '-';
        K = -K;
    }

    if (K >= 100) {
        *buffer++ = static_cast<char>('0' + static_cast<char>(K / 100));
        K %= 100;
        const char* d = GetDigitsLut() + K * 2;
        *buffer++ = d[0];
        *buffer++ = d[1];
    }
    else if (K >= 10) {
        const char* d = GetDigitsLut() + K * 2;
        *buffer++ = d[0];
        *buffer++ = d[1];
    }
    else
        *buffer++ = static_cast<char>('0' + static_cast<char>(K));

    return buffer;
}
*/

#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
unsafe fn write_exponent(mut k: isize, mut buffer: *mut u8) -> *mut u8 {
    if k < 0 {
        *buffer = b'-';
        buffer = buffer.offset(1);
        k = -k;
    }

    if k >= 100 {
        *buffer = b'0' + (k / 100) as u8;
        k %= 100;
        let d = crate::DEC_DIGITS_LUT.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, buffer.offset(1), 2);
        buffer.offset(3)
    } else if k >= 10 {
        let d = crate::DEC_DIGITS_LUT.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, buffer, 2);
        buffer.offset(2)
    } else {
        *buffer = b'0' + k as u8;
        buffer.offset(1)
    }
}

/*
inline char* Prettify(char* buffer, int length, int k, int maxDecimalPlaces) {
    const int kk = length + k;  // 10^(kk-1) <= v < 10^kk
*/

#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
pub unsafe fn prettify(buffer: *mut u8, length: isize, k: isize) -> *mut u8 {
    let kk = length + k; // 10^(kk-1) <= v < 10^kk

    /*
    if (0 <= k && kk <= 21) {
        // 1234e7 -> 12340000000
        for (int i = length; i < kk; i++)
            buffer[i] = '0';
        buffer[kk] = '.';
        buffer[kk + 1] = '0';
        return &buffer[kk + 2];
    }
    */
    if 0 <= k && kk <= 21 {
        // 1234e7 -> 12340000000
        for i in length..kk {
            *buffer.offset(i) = b'0';
        }
        *buffer.offset(kk) = b'.';
        *buffer.offset(kk + 1) = b'0';
        buffer.offset(kk + 2)
    }
    /*
    else if (0 < kk && kk <= 21) {
        // 1234e-2 -> 12.34
        std::memmove(&buffer[kk + 1], &buffer[kk], static_cast<size_t>(length - kk));
        buffer[kk] = '.';
        if (0 > k + maxDecimalPlaces) {
            // When maxDecimalPlaces = 2, 1.2345 -> 1.23, 1.102 -> 1.1
            // Remove extra trailing zeros (at least one) after truncation.
            for (int i = kk + maxDecimalPlaces; i > kk + 1; i--)
                if (buffer[i] != '0')
                    return &buffer[i + 1];
            return &buffer[kk + 2]; // Reserve one zero
        }
        else
            return &buffer[length + 1];
    }
    */
    else if 0 < kk && kk <= 21 {
        // 1234e-2 -> 12.34
        ptr::copy(
            buffer.offset(kk),
            buffer.offset(kk + 1),
            (length - kk) as usize,
        );
        *buffer.offset(kk) = b'.';
        if 0 > k + crate::MAX_DECIMAL_PLACES {
            // When MAX_DECIMAL_PLACES = 2, 1.2345 -> 1.23, 1.102 -> 1.1
            // Remove extra trailing zeros (at least one) after truncation.
            for i in (kk + 2..kk + crate::MAX_DECIMAL_PLACES + 1).rev() {
                if *buffer.offset(i) != b'0' {
                    return buffer.offset(i + 1);
                }
            }
            buffer.offset(kk + 2) // Reserve one zero
        } else {
            buffer.offset(length + 1)
        }
    }
    /*
    else if (-6 < kk && kk <= 0) {
        // 1234e-6 -> 0.001234
        const int offset = 2 - kk;
        std::memmove(&buffer[offset], &buffer[0], static_cast<size_t>(length));
        buffer[0] = '0';
        buffer[1] = '.';
        for (int i = 2; i < offset; i++)
            buffer[i] = '0';
        if (length - kk > maxDecimalPlaces) {
            // When maxDecimalPlaces = 2, 0.123 -> 0.12, 0.102 -> 0.1
            // Remove extra trailing zeros (at least one) after truncation.
            for (int i = maxDecimalPlaces + 1; i > 2; i--)
                if (buffer[i] != '0')
                    return &buffer[i + 1];
            return &buffer[3]; // Reserve one zero
        }
        else
            return &buffer[length + offset];
    }
    */
    else if -6 < kk && kk <= 0 {
        // 1234e-6 -> 0.001234
        let offset = 2 - kk;
        ptr::copy(buffer, buffer.offset(offset), length as usize);
        *buffer = b'0';
        *buffer.offset(1) = b'.';
        for i in 2..offset {
            *buffer.offset(i) = b'0';
        }
        if length - kk > crate::MAX_DECIMAL_PLACES {
            // When MAX_DECIMAL_PLACES = 2, 0.123 -> 0.12, 0.102 -> 0.1
            // Remove extra trailing zeros (at least one) after truncation.
            for i in (3..crate::MAX_DECIMAL_PLACES + 2).rev() {
                if *buffer.offset(i) != b'0' {
                    return buffer.offset(i + 1);
                }
            }
            buffer.offset(3) // Reserve one zero
        } else {
            buffer.offset(length + offset)
        }
    }
    /*
    else if (kk < -maxDecimalPlaces) {
        // Truncate to zero
        buffer[0] = '0';
        buffer[1] = '.';
        buffer[2] = '0';
        return &buffer[3];
    }
    */
    else if kk < -crate::MAX_DECIMAL_PLACES {
        *buffer = b'0';
        *buffer.offset(1) = b'.';
        *buffer.offset(2) = b'0';
        buffer.offset(3)
    }
    /*
    else if (length == 1) {
        // 1e30
        buffer[1] = 'e';
        return WriteExponent(kk - 1, &buffer[2]);
    }
    */
    else if length == 1 {
        // 1e30
        *buffer.offset(1) = b'e';
        write_exponent(kk - 1, buffer.offset(2))
    }
    /*
    else {
        // 1234e30 -> 1.234e33
        std::memmove(&buffer[2], &buffer[1], static_cast<size_t>(length - 1));
        buffer[1] = '.';
        buffer[length + 1] = 'e';
        return WriteExponent(kk - 1, &buffer[0 + length + 2]);
    }
    */
    else {
        // 1234e30 -> 1.234e33
        ptr::copy(buffer.offset(1), buffer.offset(2), (length - 1) as usize);
        *buffer.offset(1) = b'.';
        *buffer.offset(length + 1) = b'e';
        write_exponent(kk - 1, buffer.offset(length + 2))
    }
}

macro_rules! dtoa {
    (
        floating_type: $fty:ty,
        significand_type: $sigty:ty,
        exponent_type: $expty:ty,
        $($diyfp_param:ident: $diyfp_value:tt,)*
    ) => {
        diyfp! {
            floating_type: $fty,
            significand_type: $sigty,
            exponent_type: $expty,
            $($diyfp_param: $diyfp_value,)*
        };

        /*
        inline void GrisuRound(char* buffer, int len, uint64_t delta, uint64_t rest, uint64_t ten_kappa, uint64_t wp_w) {
            while (rest < wp_w && delta - rest >= ten_kappa &&
                (rest + ten_kappa < wp_w ||  /// closer
                    wp_w - rest > rest + ten_kappa - wp_w)) {
                buffer[len - 1]--;
                rest += ten_kappa;
            }
        }
        */

        #[inline]
        #[cfg_attr(feature = "no-panic", no_panic)]
        unsafe fn grisu_round(buffer: *mut u8, len: isize, delta: $sigty, mut rest: $sigty, ten_kappa: $sigty, wp_w: $sigty) {
            while rest < wp_w && delta - rest >= ten_kappa &&
                (rest + ten_kappa < wp_w || // closer
                    wp_w - rest > rest + ten_kappa - wp_w) {
                *buffer.offset(len - 1) -= 1;
                rest += ten_kappa;
            }
        }

        /*
        inline void DigitGen(const DiyFp& W, const DiyFp& Mp, uint64_t delta, char* buffer, int* len, int* K) {
            static const uint32_t kPow10[] = { 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000 };
            const DiyFp one(uint64_t(1) << -Mp.e, Mp.e);
            const DiyFp wp_w = Mp - W;
            uint32_t p1 = static_cast<uint32_t>(Mp.f >> -one.e);
            uint64_t p2 = Mp.f & (one.f - 1);
            unsigned kappa = CountDecimalDigit32(p1); // kappa in [0, 9]
            *len = 0;
        */

        // Returns length and k.
        #[inline]
        #[cfg_attr(feature = "no-panic", no_panic)]
        unsafe fn digit_gen(w: DiyFp, mp: DiyFp, mut delta: $sigty, buffer: *mut u8, mut k: isize) -> (isize, isize) {
            static POW10: [$sigty; 10] = [ 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000 ];
            let one = DiyFp::new(1 << -mp.e, mp.e);
            let wp_w = mp - w;
            let mut p1 = (mp.f >> -one.e) as u32;
            let mut p2 = mp.f & (one.f - 1);
            let mut kappa = dtoa::count_decimal_digit32(p1); // kappa in [0, 9]
            let mut len = 0;

            /*
            while (kappa > 0) {
                uint32_t d = 0;
                switch (kappa) {
                    case  9: d = p1 /  100000000; p1 %=  100000000; break;
                    case  8: d = p1 /   10000000; p1 %=   10000000; break;
                    case  7: d = p1 /    1000000; p1 %=    1000000; break;
                    case  6: d = p1 /     100000; p1 %=     100000; break;
                    case  5: d = p1 /      10000; p1 %=      10000; break;
                    case  4: d = p1 /       1000; p1 %=       1000; break;
                    case  3: d = p1 /        100; p1 %=        100; break;
                    case  2: d = p1 /         10; p1 %=         10; break;
                    case  1: d = p1;              p1 =           0; break;
                    default:;
                }
                if (d || *len)
                    buffer[(*len)++] = static_cast<char>('0' + static_cast<char>(d));
                kappa--;
                uint64_t tmp = (static_cast<uint64_t>(p1) << -one.e) + p2;
                if (tmp <= delta) {
                    *K += kappa;
                    GrisuRound(buffer, *len, delta, tmp, static_cast<uint64_t>(kPow10[kappa]) << -one.e, wp_w.f);
                    return;
                }
            }
            */
            while kappa > 0 {
                let mut d = 0u32;
                match kappa {
                    9 => { d = p1 /  100000000; p1 %=  100000000; }
                    8 => { d = p1 /   10000000; p1 %=   10000000; }
                    7 => { d = p1 /    1000000; p1 %=    1000000; }
                    6 => { d = p1 /     100000; p1 %=     100000; }
                    5 => { d = p1 /      10000; p1 %=      10000; }
                    4 => { d = p1 /       1000; p1 %=       1000; }
                    3 => { d = p1 /        100; p1 %=        100; }
                    2 => { d = p1 /         10; p1 %=         10; }
                    1 => { d = p1;              p1 =           0; }
                    _ => {}
                }
                if d != 0 || len != 0 {
                    *buffer.offset(len) = b'0' + d as u8;
                    len += 1;
                }
                kappa -= 1;
                let tmp = ((p1 as $sigty) << -one.e) + p2;
                if tmp <= delta {
                    k += kappa as isize;
                    grisu_round(buffer, len, delta, tmp, *POW10.get_unchecked(kappa) << -one.e, wp_w.f);
                    return (len, k);
                }
            }

            // kappa = 0
            /*
            for (;;) {
                p2 *= 10;
                delta *= 10;
                char d = static_cast<char>(p2 >> -one.e);
                if (d || *len)
                    buffer[(*len)++] = static_cast<char>('0' + d);
                p2 &= one.f - 1;
                kappa--;
                if (p2 < delta) {
                    *K += kappa;
                    int index = -static_cast<int>(kappa);
                    GrisuRound(buffer, *len, delta, p2, one.f, wp_w.f * (index < 9 ? kPow10[-static_cast<int>(kappa)] : 0));
                    return;
                }
            }
            */
            loop {
                p2 *= 10;
                delta *= 10;
                let d = (p2 >> -one.e) as u8;
                if d != 0 || len != 0 {
                    *buffer.offset(len) = b'0' + d;
                    len += 1;
                }
                p2 &= one.f - 1;
                kappa = kappa.wrapping_sub(1);
                if p2 < delta {
                    k += kappa as isize;
                    let index = -(kappa as isize);
                    grisu_round(
                        buffer,
                        len,
                        delta,
                        p2,
                        one.f,
                        wp_w.f * if index < 9 {
                            *POW10.get_unchecked(-(kappa as isize) as usize)
                        } else {
                            0
                        },
                    );
                    return (len, k);
                }
            }
        }

        /*
        inline void Grisu2(double value, char* buffer, int* length, int* K) {
            const DiyFp v(value);
            DiyFp w_m, w_p;
            v.NormalizedBoundaries(&w_m, &w_p);

            const DiyFp c_mk = GetCachedPower(w_p.e, K);
            const DiyFp W = v.Normalize() * c_mk;
            DiyFp Wp = w_p * c_mk;
            DiyFp Wm = w_m * c_mk;
            Wm.f++;
            Wp.f--;
            DigitGen(W, Wp, Wp.f - Wm.f, buffer, length, K);
        }
        */

        // Returns length and k.
        #[inline]
        #[cfg_attr(feature = "no-panic", no_panic)]
        unsafe fn grisu2(value: $fty, buffer: *mut u8) -> (isize, isize) {
            let v = DiyFp::from(value);
            let (w_m, w_p) = v.normalized_boundaries();

            let (c_mk, k) = get_cached_power(w_p.e);
            let w = v.normalize() * c_mk;
            let mut wp = w_p * c_mk;
            let mut wm = w_m * c_mk;
            wm.f += 1;
            wp.f -= 1;
            digit_gen(w, wp, wp.f - wm.f, buffer, k)
        }

        /*
        inline char* dtoa(double value, char* buffer, int maxDecimalPlaces = 324) {
            RAPIDJSON_ASSERT(maxDecimalPlaces >= 1);
            Double d(value);
            if (d.IsZero()) {
                if (d.Sign())
                    *buffer++ = '-';     // -0.0, Issue #289
                buffer[0] = '0';
                buffer[1] = '.';
                buffer[2] = '0';
                return &buffer[3];
            }
            else {
                if (value < 0) {
                    *buffer++ = '-';
                    value = -value;
                }
                int length, K;
                Grisu2(value, buffer, &length, &K);
                return Prettify(buffer, length, K, maxDecimalPlaces);
            }
        }
        */

        #[inline]
        #[cfg_attr(feature = "no-panic", no_panic)]
        unsafe fn dtoa(buf: &mut Buffer, mut value: $fty) -> &str {
            if value == 0.0 {
                if value.is_sign_negative() {
                    "-0.0"
                } else {
                    "0.0"
                }
            } else {
                let start = buf.bytes.as_mut_ptr() as *mut u8;
                let mut buf_ptr = start;
                if value < 0.0 {
                    *buf_ptr = b'-';
                    buf_ptr = buf_ptr.offset(1);
                    value = -value;
                }
                let (length, k) = grisu2(value, buf_ptr);
                let end = dtoa::prettify(buf_ptr, length, k);
                let len = end as usize - start as usize;
                str::from_utf8_unchecked(slice::from_raw_parts(start, len))
            }
        }
    };
}
