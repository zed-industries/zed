#![forbid(unsafe_code)]

use crate::{
    assert_h,
    bzlib::{BZ_MAX_ALPHA_SIZE, BZ_MAX_CODE_LEN},
};

#[inline]
const fn weight_of(zz0: i32) -> i32 {
    zz0 & 0xffffff00u32 as i32
}

#[inline]
const fn depth_of(zz1: i32) -> i32 {
    zz1 & 0xff
}

#[inline]
fn add_weights(zw1: i32, zw2: i32) -> i32 {
    (weight_of(zw1)).wrapping_add(weight_of(zw2)) | (1 + Ord::max(depth_of(zw1), depth_of(zw2)))
}

#[inline]
fn upheap(
    heap: &mut [i32; BZ_MAX_ALPHA_SIZE + 2],
    weight: &mut [i32; BZ_MAX_ALPHA_SIZE * 2],
    mut z: usize,
) {
    let tmp = heap[z];
    while weight[tmp as usize] < weight[heap[z >> 1] as usize] {
        heap[z] = heap[z >> 1];
        z >>= 1;
    }
    heap[z] = tmp;
}

#[inline]
fn downheap(
    heap: &mut [i32; BZ_MAX_ALPHA_SIZE + 2],
    weight: &mut [i32; BZ_MAX_ALPHA_SIZE * 2],
    nHeap: usize,
    mut z: usize,
) {
    let tmp = heap[z];
    loop {
        let mut yy = z << 1;
        if yy > nHeap {
            break;
        }
        if yy < nHeap && weight[heap[yy + 1] as usize] < weight[heap[yy] as usize] {
            yy += 1;
        }
        if weight[tmp as usize] < weight[heap[yy] as usize] {
            break;
        }
        heap[z] = heap[yy];
        z = yy;
    }
    heap[z] = tmp;
}

pub(crate) fn make_code_lengths(len: &mut [u8], freq: &[i32], alphaSize: usize, maxLen: i32) {
    /*--
       Nodes and heap entries run from 1.  Entry 0
       for both the heap and nodes is a sentinel.
    --*/
    let mut nNodes: usize;
    let mut nHeap: usize;
    let mut j: i32;
    let mut heap = [0i32; BZ_MAX_ALPHA_SIZE + 2];
    let mut weight = [0i32; BZ_MAX_ALPHA_SIZE * 2];
    let mut parent = [0i32; BZ_MAX_ALPHA_SIZE * 2];

    for i in 0..alphaSize {
        weight[i + 1] = (if freq[i] == 0 { 1 } else { freq[i] }) << 8;
    }

    loop {
        nNodes = alphaSize;
        nHeap = 0;

        heap[0] = 0;
        weight[0] = 0;
        parent[0] = -2;

        parent[1..=alphaSize].fill(-1);

        for i in 1..=alphaSize {
            nHeap += 1;
            heap[nHeap] = i as i32;
            upheap(&mut heap, &mut weight, nHeap);
        }

        assert_h!(nHeap < (BZ_MAX_ALPHA_SIZE + 2), 2001);

        while nHeap > 1 {
            let n1 = heap[1] as usize;
            heap[1] = heap[nHeap];
            nHeap -= 1;
            downheap(&mut heap, &mut weight, nHeap, 1);
            let n2 = heap[1] as usize;
            heap[1] = heap[nHeap];
            nHeap -= 1;
            downheap(&mut heap, &mut weight, nHeap, 1);
            nNodes += 1;
            parent[n1] = nNodes as i32;
            parent[n2] = nNodes as i32;
            weight[nNodes] = add_weights(weight[n1], weight[n2]);
            parent[nNodes] = -1;
            nHeap += 1;
            heap[nHeap] = nNodes as i32;
            upheap(&mut heap, &mut weight, nHeap);
        }

        assert_h!(nNodes < (BZ_MAX_ALPHA_SIZE * 2), 2002);

        let mut tooLong = false;
        for i in 1..=alphaSize {
            j = 0;
            let mut k = i;
            while parent[k] >= 0 {
                k = parent[k] as usize;
                j += 1;
            }
            len[i - 1] = j as u8;
            if j > maxLen {
                tooLong = true;
            }
        }

        if !tooLong {
            break;
        }

        /* 17 Oct 04: keep-going condition for the following loop used
        to be 'i < alphaSize', which missed the last element,
        theoretically leading to the possibility of the compressor
        looping.  However, this count-scaling step is only needed if
        one of the generated Huffman code words is longer than
        maxLen, which up to and including version 1.0.2 was 20 bits,
        which is extremely unlikely.  In version 1.0.3 maxLen was
        changed to 17 bits, which has minimal effect on compression
        ratio, but does mean this scaling step is used from time to
        time, enough to verify that it works.

        This means that bzip2-1.0.3 and later will only produce
        Huffman codes with a maximum length of 17 bits.  However, in
        order to preserve backwards compatibility with bitstreams
        produced by versions pre-1.0.3, the decompressor must still
        handle lengths of up to 20. */

        for weight in weight[1..=alphaSize].iter_mut() {
            *weight = (1 + (*weight >> 8) / 2) << 8;
        }
    }
}

#[inline]
pub(crate) fn assign_codes(code: &mut [u32], length: &[u8], minLen: u8, maxLen: u8) {
    let mut vec: u32 = 0;
    for n in minLen..=maxLen {
        for (i, &l) in length.iter().enumerate() {
            if l == n {
                code[i] = vec;
                vec += 1;
            }
        }
        vec <<= 1;
    }
}

#[inline(always)]
pub(crate) fn create_decode_tables(
    limit: &mut [i32; 258],
    base: &mut [i32; 258],
    perm: &mut [u16; 258],
    length: &[u8],
    minLen: u8,
    maxLen: u8,
) {
    assert!(length.len() <= 258);

    let mut pp = 0;
    for i in minLen..=maxLen {
        for (j, e) in length.iter().enumerate() {
            if *e == i {
                perm[pp] = j as u16;
                pp += 1;
            }
        }
    }

    base[0..BZ_MAX_CODE_LEN].fill(0);

    for l in length {
        base[usize::from(*l) + 1] += 1;
    }

    for i in 1..BZ_MAX_CODE_LEN {
        base[i] += base[i - 1];
    }

    limit[0..BZ_MAX_CODE_LEN].fill(0);

    let mut vec = 0;
    for i in usize::from(minLen)..=usize::from(maxLen) {
        vec += base[i + 1] - base[i];
        limit[i] = vec - 1;
        vec <<= 1;
    }

    for i in usize::from(minLen)..usize::from(maxLen) {
        base[i + 1] = (2 * (limit[i] + 1)) - base[i + 1];
    }
}
