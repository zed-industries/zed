#![forbid(unsafe_code)]

use core::cmp::Ordering;
use core::ffi::{c_int, c_uint};

use crate::{
    assert_h,
    bzlib::{Arr2, EState, BZ_N_OVERSHOOT, BZ_N_QSORT, BZ_N_RADIX, FTAB_LEN},
};
use crate::{debug_log, debug_logln};

/// Fallback O(N log(N)^2) sorting algorithm, for repetitive blocks
#[inline]
fn fallbackSimpleSort(fmap: &mut [u32], eclass: &[u32], lo: i32, hi: i32) {
    let mut j: i32;
    let mut tmp: i32;
    let mut ec_tmp: u32;

    if lo == hi {
        return;
    }

    if hi - lo > 3 {
        for i in (lo..=hi - 4).rev() {
            tmp = fmap[i as usize] as i32;
            ec_tmp = eclass[tmp as usize];
            j = i + 4;
            while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
                fmap[(j - 4) as usize] = fmap[j as usize];
                j += 4;
            }
            fmap[(j - 4) as usize] = tmp as u32;
        }
    }

    for i in (lo..=hi - 1).rev() {
        tmp = fmap[i as usize] as i32;
        ec_tmp = eclass[tmp as usize];
        j = i + 1;
        while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
            fmap[(j - 1) as usize] = fmap[j as usize];
            j += 1;
        }
        fmap[(j - 1) as usize] = tmp as u32;
    }
}

const FALLBACK_QSORT_SMALL_THRESH: i32 = 10;
const FALLBACK_QSORT_STACK_SIZE: usize = 100;

fn fallbackQSort3(fmap: &mut [u32], eclass: &[u32], loSt: i32, hiSt: i32) {
    let mut unLo: i32;
    let mut unHi: i32;
    let mut ltLo: i32;
    let mut gtHi: i32;
    let mut n: i32;
    let mut m: i32;
    let mut sp: usize;
    let mut lo: i32;
    let mut hi: i32;
    let mut stackLo: [i32; FALLBACK_QSORT_STACK_SIZE] = [0; FALLBACK_QSORT_STACK_SIZE];
    let mut stackHi: [i32; FALLBACK_QSORT_STACK_SIZE] = [0; FALLBACK_QSORT_STACK_SIZE];

    macro_rules! fpush {
        ($lz:expr, $hz:expr) => {
            stackLo[sp] = $lz;
            stackHi[sp] = $hz;
            sp += 1;
        };
    }

    macro_rules! fvswap {
        ($zzp1:expr, $zzp2:expr, $zzn:expr) => {
            let mut yyp1: i32 = $zzp1;
            let mut yyp2: i32 = $zzp2;
            let mut yyn: i32 = $zzn;

            while (yyn > 0) {
                fmap.swap(yyp1 as usize, yyp2 as usize);
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1;
            }
        };
    }

    let mut r = 0u32;

    sp = 0;
    fpush!(loSt, hiSt);

    while sp > 0 {
        assert_h!(sp < FALLBACK_QSORT_STACK_SIZE - 1, 1004);

        // the `fpop` macro has one occurence, so it was inlined here
        sp -= 1;
        lo = stackLo[sp];
        hi = stackHi[sp];

        if hi - lo < FALLBACK_QSORT_SMALL_THRESH {
            fallbackSimpleSort(fmap, eclass, lo, hi);
            continue;
        }

        /* Random partitioning.  Median of 3 sometimes fails to
            avoid bad cases.  Median of 9 seems to help but
            looks rather expensive.  This too seems to work but
            is cheaper.  Guidance for the magic constants
            7621 and 32768 is taken from Sedgewick's algorithms
            book, chapter 35.
        */
        r = r.wrapping_mul(7621).wrapping_add(1).wrapping_rem(32768);
        let index = match r.wrapping_rem(3) {
            0 => fmap[lo as usize],
            1 => fmap[((lo + hi) >> 1) as usize],
            _ => fmap[hi as usize],
        };
        let med = eclass[index as usize];

        ltLo = lo;
        unLo = lo;

        gtHi = hi;
        unHi = hi;

        loop {
            while unLo <= unHi {
                let a = eclass[fmap[unLo as usize] as usize];
                let b = med;

                if a > b {
                    break;
                } else if a == b {
                    fmap.swap(unLo as usize, ltLo as usize);
                    ltLo += 1;
                    unLo += 1;
                } else {
                    unLo += 1;
                }
            }

            while unLo <= unHi {
                let a = eclass[fmap[unHi as usize] as usize];
                let b = med;

                if a < b {
                    break;
                } else if a == b {
                    fmap.swap(unHi as usize, gtHi as usize);
                    gtHi -= 1;
                    unHi -= 1;
                } else {
                    unHi -= 1;
                }
            }

            if unLo > unHi {
                break;
            }

            fmap.swap(unLo as usize, unHi as usize);
            unLo += 1;
            unHi -= 1;
        }

        debug_assert_eq!(unHi, unLo - 1, "fallbackQSort3(2)");

        if gtHi < ltLo {
            continue;
        }

        n = Ord::min(ltLo - lo, unLo - ltLo);
        fvswap!(lo, unLo - n, n);
        m = Ord::min(hi - gtHi, gtHi - unHi);
        fvswap!(unLo, hi - m + 1, m);

        n = lo + unLo - ltLo - 1;
        m = hi - (gtHi - unHi) + 1;

        if n - lo > hi - m {
            fpush!(lo, n);
            fpush!(m, hi);
        } else {
            fpush!(m, hi);
            fpush!(lo, n);
        }
    }
}

fn fallbackSort(
    fmap: &mut [u32],
    arr2: &mut Arr2,
    bhtab: &mut [u32; FTAB_LEN],
    nblock: i32,
    verb: i32,
) {
    macro_rules! SET_BH {
        ($zz:expr) => {
            bhtab[$zz as usize >> 5] |= 1 << ($zz & 31);
        };
    }

    macro_rules! CLEAR_BH {
        ($zz:expr) => {
            bhtab[$zz as usize >> 5] &= !(1 << ($zz & 31));
        };
    }

    macro_rules! ISSET_BH {
        ($zz:expr) => {
            bhtab[$zz as usize >> 5] & 1u32 << ($zz & 31) != 0
        };
    }

    macro_rules! UNALIGNED_BH {
        ($zz:expr) => {
            ($zz & 0x01f) != 0
        };
    }

    macro_rules! WORD_BH {
        ($zz:expr) => {
            bhtab[$zz as usize >> 5]
        };
    }

    let mut ftab: [i32; 257] = [0; 257];
    let mut ftabCopy: [i32; 256] = [0; 256];
    let mut H: i32;
    let mut k: i32;
    let mut l: i32;

    /*--
       Initial 1-char radix sort to generate
       initial fmap and initial BH bits.
    --*/
    if verb >= 4 {
        debug_logln!("        bucket sorting ...");
    }

    {
        let eclass8 = arr2.block(nblock as usize);

        for e in eclass8.iter() {
            ftab[usize::from(*e)] += 1;
        }

        ftabCopy[0..256].copy_from_slice(&ftab[0..256]);

        for i in 1..257 {
            ftab[i] += ftab[i - 1];
        }

        for (i, e) in eclass8.iter().enumerate() {
            let j = usize::from(*e);
            k = ftab[j] - 1;
            ftab[j] = k;
            fmap[k as usize] = i as u32;
        }
    }

    bhtab[0..2 + nblock as usize / 32].fill(0);

    for i in 0..256 {
        SET_BH!(ftab[i]);
    }

    /*--
       Inductively refine the buckets.  Kind-of an
       "exponential radix sort" (!), inspired by the
       Manber-Myers suffix array construction algorithm.
    --*/

    /*-- set sentinel bits for block-end detection --*/
    for i in 0..32 {
        SET_BH!(nblock + 2 * i);
        CLEAR_BH!(nblock + 2 * i + 1);
    }

    /*-- the log(N) loop --*/
    H = 1;
    loop {
        if verb >= 4 {
            debug_log!("        depth {:>6} has ", H);
        }
        let mut j = 0;
        for (i, x) in fmap[..nblock as usize].iter().enumerate() {
            if ISSET_BH!(i) {
                j = i;
            }
            k = x.wrapping_sub(H as c_uint) as i32;
            if k < 0 {
                k += nblock;
            }
            arr2.eclass()[k as usize] = j as u32;
        }

        let mut nNotDone = 0;
        let mut r = -1;
        loop {
            /*-- find the next non-singleton bucket --*/
            k = r + 1;
            while ISSET_BH!(k) && UNALIGNED_BH!(k) {
                k += 1;
            }
            if ISSET_BH!(k) {
                while WORD_BH!(k) == 0xffffffff {
                    k += 32;
                }
                while ISSET_BH!(k) {
                    k += 1;
                }
            }
            l = k - 1;
            if l >= nblock {
                break;
            }
            while !ISSET_BH!(k) && UNALIGNED_BH!(k) {
                k += 1;
            }
            if !ISSET_BH!(k) {
                while WORD_BH!(k) == 0x00000000 {
                    k += 32;
                }
                while !ISSET_BH!(k) {
                    k += 1;
                }
            }
            r = k - 1;
            if r >= nblock {
                break;
            }

            /*-- now [l, r] bracket current bucket --*/
            if r > l {
                nNotDone += r - l + 1;
                fallbackQSort3(fmap, arr2.eclass(), l, r);

                /*-- scan bucket and generate header bits-- */
                let mut cc = -1;
                for (i, x) in fmap[l as usize..=r as usize].iter().enumerate() {
                    let cc1 = arr2.eclass()[*x as usize] as i32;
                    if cc != cc1 {
                        SET_BH!(l + i as i32);
                        cc = cc1;
                    }
                }
            }
        }
        if verb >= 4 {
            debug_logln!("{:>6} unresolved strings", nNotDone);
        }
        H *= 2;
        if H > nblock || nNotDone == 0 {
            break;
        }
    }

    if verb >= 4 {
        debug_logln!("        reconstructing block ...");
    }

    {
        let eclass8 = arr2.block(nblock as usize);

        let mut j = 0;
        for i in 0..nblock {
            while ftabCopy[j] == 0 {
                j += 1;
            }
            ftabCopy[j] -= 1;
            eclass8[fmap[i as usize] as usize] = j as u8;
        }

        assert_h!(j < 256, 1005);
    }
}

#[inline]
fn mainGtU(
    mut i1: u32,
    mut i2: u32,
    block: &[u8],
    quadrant: &[u16],
    nblock: u32,
    budget: &mut i32,
) -> bool {
    debug_assert_ne!(i1, i2, "mainGtU");

    let chunk1 = &block[i1 as usize..][..12];
    let chunk2 = &block[i2 as usize..][..12];

    for (c1, c2) in chunk1.chunks_exact(4).zip(chunk2.chunks_exact(4)) {
        let c1 = u32::from_be_bytes(c1[..4].try_into().unwrap());
        let c2 = u32::from_be_bytes(c2[..4].try_into().unwrap());

        if c1 != c2 {
            return c1 > c2;
        }
    }

    i1 += 12;
    i2 += 12;

    for _ in 0..nblock.div_ceil(8) {
        let b1 = &block[i1 as usize..][..8];
        let b2 = &block[i2 as usize..][..8];

        let q1 = &quadrant[i1 as usize..][..8];
        let q2 = &quadrant[i2 as usize..][..8];

        if b1 != b2 || q1 != q2 {
            for (((c1, c2), s1), s2) in b1.iter().zip(b2).zip(q1).zip(q2) {
                if c1 != c2 {
                    return c1 > c2;
                }
                if s1 != s2 {
                    return s1 > s2;
                }
            }
        }

        i1 += 8;
        i2 += 8;

        if i1 >= nblock {
            i1 = i1.wrapping_sub(nblock);
        }
        if i2 >= nblock {
            i2 = i2.wrapping_sub(nblock);
        }

        *budget -= 1;
    }

    false
}

static INCS: [i32; 14] = [
    1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161, 2391484,
];

fn mainSimpleSort(
    ptr: &mut [u32],
    block: &[u8],
    quadrant: &[u16],
    nblock: i32,
    lo: i32,
    hi: i32,
    d: i32,
    budget: &mut i32,
) {
    let bigN = hi - lo + 1;

    let Some(index) = INCS.iter().position(|&e| e >= bigN) else {
        return;
    };

    for &h in INCS[..index].iter().rev() {
        for i in lo + h..=hi {
            let v = ptr[i as usize];
            let mut j = i;
            while mainGtU(
                (ptr[(j - h) as usize]).wrapping_add(d as u32),
                v.wrapping_add(d as u32),
                block,
                quadrant,
                nblock as u32,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j -= h;
                if j < lo + h {
                    break;
                }
            }
            ptr[j as usize] = v;
            if *budget < 0 {
                return;
            }
        }
    }
}

#[inline]
fn median_of_3(mut a: u8, mut b: u8, mut c: u8) -> u8 {
    if a > b {
        (a, b) = (b, a);
    }
    if a > c {
        (_, c) = (c, a);
    }
    if b > c {
        (b, _) = (c, b);
    }

    debug_assert!(a <= b && b <= c);

    b
}

const MAIN_QSORT_SMALL_THRESH: i32 = 20;
const MAIN_QSORT_DEPTH_THRESH: i32 = BZ_N_RADIX + BZ_N_QSORT;
const MAIN_QSORT_STACK_SIZE: i32 = 100;

fn mainQSort3(
    ptr: &mut [u32],
    block: &[u8],
    quadrant: &[u16],
    nblock: i32,
    loSt: i32,
    hiSt: i32,
    dSt: i32,
    budget: &mut i32,
) {
    let mut unLo: i32;
    let mut unHi: i32;
    let mut ltLo: i32;
    let mut gtHi: i32;
    let mut n: i32;
    let mut m: i32;
    let mut med: i32;

    let mut stack = [(0i32, 0i32, 0i32); 100];

    stack[0] = (loSt, hiSt, dSt);

    let mut sp = 1;
    while sp > 0 {
        assert_h!(sp < MAIN_QSORT_STACK_SIZE as usize - 2, 1001);

        sp -= 1;

        let (lo, hi, d) = stack[sp];

        if hi - lo < MAIN_QSORT_SMALL_THRESH || d > MAIN_QSORT_DEPTH_THRESH {
            mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 {
                return;
            }
        } else {
            med = median_of_3(
                block[(ptr[lo as usize]).wrapping_add(d as c_uint) as usize],
                block[(ptr[hi as usize]).wrapping_add(d as c_uint) as usize],
                block[((ptr[((lo + hi) >> 1) as usize]).wrapping_add(d as c_uint) as isize)
                    as usize],
            ) as i32;
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            loop {
                while unLo <= unHi {
                    n = block[(ptr[unLo as usize]).wrapping_add(d as c_uint) as usize] as i32 - med;
                    match n.cmp(&0) {
                        Ordering::Greater => break,
                        Ordering::Equal => {
                            ptr.swap(unLo as usize, ltLo as usize);
                            ltLo += 1;
                            unLo += 1;
                        }
                        Ordering::Less => unLo += 1,
                    }
                }
                while unLo <= unHi {
                    n = block[(ptr[unHi as usize]).wrapping_add(d as c_uint) as usize] as i32 - med;
                    match n.cmp(&0) {
                        Ordering::Less => break,
                        Ordering::Equal => {
                            ptr.swap(unHi as usize, gtHi as usize);
                            gtHi -= 1;
                            unHi -= 1;
                        }
                        Ordering::Greater => unHi -= 1,
                    }
                }
                if unLo > unHi {
                    break;
                }
                ptr.swap(unLo as usize, unHi as usize);
                unLo += 1;
                unHi -= 1;
            }
            if gtHi < ltLo {
                stack[sp] = (lo, hi, d + 1);
                sp += 1;
            } else {
                n = Ord::min(ltLo - lo, unLo - ltLo);
                let mut yyp1: i32 = lo;
                let mut yyp2: i32 = unLo - n;
                for _ in 0..n {
                    ptr.swap(yyp1 as usize, yyp2 as usize);
                    yyp1 += 1;
                    yyp2 += 1;
                }

                m = Ord::min(hi - gtHi, gtHi - unHi);
                let mut yyp1_0: i32 = unLo;
                let mut yyp2_0: i32 = hi - m + 1;
                for _ in 0..m {
                    ptr.swap(yyp1_0 as usize, yyp2_0 as usize);
                    yyp1_0 += 1;
                    yyp2_0 += 1;
                }

                n = lo + unLo - ltLo - 1;
                m = hi - (gtHi - unHi) + 1;

                let mut next = [(lo, n, d), (m, hi, d), (n + 1, m - 1, d + 1)];

                if next[0].1 - next[0].0 < next[1].1 - next[1].0 {
                    next.swap(0, 1);
                }

                if next[1].1 - next[1].0 < next[2].1 - next[2].0 {
                    next.swap(1, 2);
                }

                if next[0].1 - next[0].0 < next[1].1 - next[1].0 {
                    next.swap(0, 1);
                }

                stack[sp..][..next.len()].copy_from_slice(&next);
                sp += next.len();
            }
        }
    }
}
fn mainSort(
    ptr: &mut [u32],
    block: &mut [u8],
    quadrant: &mut [u16],
    ftab: &mut [u32; FTAB_LEN],
    nblock: i32,
    verb: i32,
    budget: &mut i32,
) {
    let mut j: i32;
    let mut k: i32;
    let mut ss: i32;
    let mut sb: i32;
    let mut bigDone: [bool; 256] = [false; 256];
    let mut copyStart: [i32; 256] = [0; 256];
    let mut copyEnd: [i32; 256] = [0; 256];
    let mut c1: u8;
    let mut numQSorted: i32;
    let mut s: u16;
    if verb >= 4 as c_int {
        debug_logln!("        main sort initialise ...");
    }

    /*-- set up the 2-byte frequency table --*/
    ftab.fill(0);

    j = (block[0] as i32) << 8;
    for &block in block[..nblock as usize].iter().rev() {
        j = (j >> 8) | (i32::from(block) << 8);
        ftab[j as usize] += 1;
    }

    for i in 0..BZ_N_OVERSHOOT {
        block[nblock as usize + i] = block[i];
    }

    if verb >= 4 as c_int {
        debug_logln!("        bucket sorting ...");
    }

    /*-- Complete the initial radix sort --*/
    for i in 1..=65536 {
        ftab[i] += ftab[i - 1];
    }

    s = ((block[0 as c_int as usize] as c_int) << 8 as c_int) as u16;

    for (i, &block) in block[..nblock as usize].iter().enumerate().rev() {
        s = (s >> 8) | (u16::from(block) << 8);
        j = ftab[usize::from(s)] as i32 - 1;
        ftab[usize::from(s)] = j as u32;
        ptr[j as usize] = i as u32;
    }

    bigDone.fill(false);
    let mut runningOrder: [i32; 256] = core::array::from_fn(|i| i as i32);

    let mut vv: i32;
    let mut h: i32 = 1 as c_int;
    loop {
        h = 3 as c_int * h + 1 as c_int;
        if h > 256 as c_int {
            break;
        }
    }

    macro_rules! BIGFREQ {
        ($b:expr) => {
            ftab[(($b) + 1) << 8] - ftab[($b) << 8]
        };
    }

    loop {
        h /= 3 as c_int;
        for i in h..256 {
            vv = runningOrder[i as usize];
            j = i;
            while BIGFREQ!(runningOrder[(j - h) as usize] as usize) > BIGFREQ!(vv as usize) {
                runningOrder[j as usize] = runningOrder[(j - h) as usize];
                j -= h;
                if j <= h - 1 as c_int {
                    break;
                }
            }
            runningOrder[j as usize] = vv;
        }
        if h == 1 as c_int {
            break;
        }
    }

    /*--
       The main sorting loop.
    --*/

    numQSorted = 0 as c_int;

    for i in 0..=255 {
        /*--
           Process big buckets, starting with the least full.
           Basically this is a 3-step process in which we call
           mainQSort3 to sort the small buckets [ss, j], but
           also make a big effort to avoid the calls if we can.
        --*/
        ss = runningOrder[i as usize];

        const SETMASK: u32 = 1 << 21;
        const CLEARMASK: u32 = !SETMASK;

        /*--
           Step 1:
           Complete the big bucket [ss] by quicksorting
           any unsorted small buckets [ss, j], for j != ss.
           Hopefully previous pointer-scanning phases have already
           completed many of the small buckets [ss, j], so
           we don't have to sort them at all.
        --*/
        for j in 0..=255 {
            if j != ss {
                sb = (ss << 8 as c_int) + j;
                if ftab[sb as usize] & SETMASK == 0 {
                    let lo: i32 = (ftab[sb as usize] & CLEARMASK) as i32;
                    let hi: i32 = ((ftab[sb as usize + 1] & CLEARMASK).wrapping_sub(1)) as i32;

                    if hi > lo {
                        if verb >= 4 as c_int {
                            debug_logln!(
                                "        qsort [{:#x}, {:#x}]   done {}   this {}",
                                ss,
                                j,
                                numQSorted,
                                hi - lo + 1 as c_int,
                            );
                        }
                        mainQSort3(ptr, block, quadrant, nblock, lo, hi, 2 as c_int, budget);
                        numQSorted += hi - lo + 1 as c_int;
                        if *budget < 0 as c_int {
                            return;
                        }
                    }
                }
                ftab[sb as usize] |= SETMASK;
            }
        }
        assert_h!(!bigDone[ss as usize], 1006);

        /*--
           Step 2:
           Now scan this big bucket [ss] so as to synthesise the
           sorted order for small buckets [t, ss] for all t,
           including, magically, the bucket [ss,ss] too.
           This will avoid doing Real Work in subsequent Step 1's.
        --*/
        {
            for j in 0..=255 {
                copyStart[j] = (ftab[(j << 8) + ss as usize] & CLEARMASK) as i32;
                copyEnd[j] = (ftab[(j << 8) + ss as usize + 1] & CLEARMASK) as i32 - 1;
            }

            j = (ftab[(ss as usize) << 8] & CLEARMASK) as i32;
            while j < copyStart[ss as usize] {
                k = (ptr[j as usize]).wrapping_sub(1) as i32;
                if k < 0 as c_int {
                    k += nblock;
                }
                c1 = block[k as usize];
                if !bigDone[c1 as usize] {
                    let fresh11 = copyStart[c1 as usize];
                    copyStart[c1 as usize] += 1;
                    ptr[fresh11 as usize] = k as u32;
                }
                j += 1;
            }

            j = (ftab[(ss as usize + 1) << 8] & CLEARMASK) as i32 - 1;
            while j > copyEnd[ss as usize] {
                k = (ptr[j as usize]).wrapping_sub(1) as i32;
                if k < 0 as c_int {
                    k += nblock;
                }
                c1 = block[k as usize];
                if !bigDone[c1 as usize] {
                    let fresh12 = copyEnd[c1 as usize];
                    copyEnd[c1 as usize] -= 1;
                    ptr[fresh12 as usize] = k as u32;
                }
                j -= 1;
            }
        }

        assert_h!(
            (copyStart[ss as usize]-1 == copyEnd[ss as usize])
                ||
                /* Extremely rare case missing in bzip2-1.0.0 and 1.0.1.
                   Necessity for this case is demonstrated by compressing
                   a sequence of approximately 48.5 million of character
                   251; 1.0.0/1.0.1 will then die here. */
                (copyStart[ss as usize] == 0 && copyEnd[ss as usize] == nblock-1),
            1007
        );

        for j in 0..=255 {
            ftab[(j << 8) + ss as usize] |= SETMASK
        }

        /*--
           Step 3:
           The [ss] big bucket is now done.  Record this fact,
           and update the quadrant descriptors.  Remember to
           update quadrants in the overshoot area too, if
           necessary.  The "if (i < 255)" test merely skips
           this updating for the last bucket processed, since
           updating for the last bucket is pointless.

           The quadrant array provides a way to incrementally
           cache sort orderings, as they appear, so as to
           make subsequent comparisons in fullGtU() complete
           faster.  For repetitive blocks this makes a big
           difference (but not big enough to be able to avoid
           the fallback sorting mechanism, exponential radix sort).

           The precise meaning is: at all times:

              for 0 <= i < nblock and 0 <= j <= nblock

              if block[i] != block[j],

                 then the relative values of quadrant[i] and
                      quadrant[j] are meaningless.

                 else {
                    if quadrant[i] < quadrant[j]
                       then the string starting at i lexicographically
                       precedes the string starting at j

                    else if quadrant[i] > quadrant[j]
                       then the string starting at j lexicographically
                       precedes the string starting at i

                    else
                       the relative ordering of the strings starting
                       at i and j has not yet been determined.
                 }
        --*/
        bigDone[ss as usize] = true;

        if i < 255 as c_int {
            let bbStart: i32 = (ftab[(ss as usize) << 8] & CLEARMASK) as i32;
            let bbSize: i32 = (ftab[(ss as usize + 1) << 8] & CLEARMASK) as i32 - bbStart;
            let mut shifts: i32 = 0 as c_int;

            while bbSize >> shifts > 65534 as c_int {
                shifts += 1;
            }

            j = bbSize - 1 as c_int;
            while j >= 0 as c_int {
                let a2update: i32 = ptr[(bbStart + j) as usize] as i32;
                let qVal: u16 = (j >> shifts) as u16;
                quadrant[a2update as usize] = qVal;
                if (a2update as usize) < BZ_N_OVERSHOOT {
                    quadrant[(a2update + nblock) as usize] = qVal;
                }
                j -= 1;
            }

            assert_h!(((bbSize - 1) >> shifts) <= 65535, 1002);
        }
    }
    if verb >= 4 as c_int {
        debug_logln!(
            "        {} pointers, {} sorted, {} scanned",
            nblock,
            numQSorted,
            nblock - numQSorted,
        );
    }
}

/// Pre:
///    nblock > 0
///    arr2 exists for [0 .. nblock-1 +N_OVERSHOOT]
///    ((UChar*)arr2)  [0 .. nblock-1] holds block
///    arr1 exists for [0 .. nblock-1]
///
/// Post:
///    ((UChar*)arr2) [0 .. nblock-1] holds block
///    All other areas of block destroyed
///    ftab [ 0 .. 65536 ] destroyed
///    arr1 [0 .. nblock-1] holds sorted order
pub(crate) fn block_sort(s: &mut EState) {
    let nblock = usize::try_from(s.nblock).unwrap();

    let ptr = s.arr1.ptr();
    let ftab = s.ftab.ftab();

    BZ2_blockSortHelp(ptr, &mut s.arr2, ftab, nblock, s.workFactor, s.verbosity);

    s.origPtr = -1 as c_int;
    for i in 0..s.nblock {
        if ptr[i as usize] == 0 {
            s.origPtr = i;
            break;
        }
    }

    assert_h!(s.origPtr != -1, 1003);
}

fn BZ2_blockSortHelp(
    ptr: &mut [u32],
    arr2: &mut Arr2,
    ftab: &mut [u32; FTAB_LEN],
    nblock: usize,
    workFactor: i32,
    verbosity: i32,
) {
    if nblock < 10000 {
        fallbackSort(ptr, arr2, ftab, nblock as i32, verbosity);
    } else {
        let (block, quadrant) = arr2.block_and_quadrant(nblock);

        /* (wfact-1) / 3 puts the default-factor-30
           transition point at very roughly the same place as
           with v0.1 and v0.9.0.
           Not that it particularly matters any more, since the
           resulting compressed stream is now the same regardless
           of whether or not we use the main sort or fallback sort.
        */
        let wfact = workFactor.clamp(1, 100);
        let budgetInit = nblock as i32 * ((wfact - 1) / 3);
        let mut budget = budgetInit;

        mainSort(
            ptr,
            block,
            quadrant,
            ftab,
            nblock as i32,
            verbosity,
            &mut budget,
        );

        if verbosity >= 3 {
            debug_logln!(
                "      {} work, {} block, ratio {:5.2}",
                budgetInit - budget,
                nblock,
                (budgetInit - budget) as f64 / (if nblock == 0 { 1 } else { nblock }) as f64
            );
        }

        if budget < 0 {
            if verbosity >= 2 as c_int {
                debug_logln!("    too repetitive; using fallback sorting algorithm");
            }

            fallbackSort(ptr, arr2, ftab, nblock as i32, verbosity);
        }
    }
}
