#![forbid(unsafe_code)]

use crate::blocksort::block_sort;
use crate::bzlib::{EState, BZ_MAX_SELECTORS, BZ_N_GROUPS, BZ_N_ITERS, BZ_RUNA, BZ_RUNB};
use crate::{assert_h, debug_log, debug_logln, huffman};

pub(crate) struct EWriter {
    pub num_z: u32,
    bs_live: i32,
    bs_buff: u32,
}

pub(crate) struct LiveWriter<'a> {
    zbits: &'a mut [u8],
    writer: &'a mut EWriter,
    num_z: u32,
    bs_live: i32,
    bs_buff: u32,
}

impl Drop for LiveWriter<'_> {
    fn drop(&mut self) {
        self.writer.num_z = self.num_z;
        self.writer.bs_buff = self.bs_buff;
        self.writer.bs_live = self.bs_live;
    }
}

impl<'a> LiveWriter<'a> {
    fn new(writer: &'a mut EWriter, zbits: &'a mut [u8]) -> Self {
        Self {
            num_z: writer.num_z,
            bs_live: writer.bs_live,
            bs_buff: writer.bs_buff,
            zbits,
            writer,
        }
    }

    fn initialize(&mut self) {
        self.bs_live = 0;
        self.bs_buff = 0;
    }

    #[inline]
    fn finish(&mut self) {
        while self.bs_live > 0 {
            self.zbits[self.num_z as usize] = (self.bs_buff >> 24) as u8;
            self.num_z += 1;
            self.bs_buff <<= 8;
            self.bs_live -= 8;
        }
    }

    #[inline]
    fn flush_whole_bytes(&mut self) {
        let range = self.num_z as usize..self.num_z as usize + 4;
        if let Some(dst) = self.zbits.get_mut(range) {
            dst.copy_from_slice(&self.bs_buff.to_be_bytes());
            let bits_written = self.bs_live & !7;
            self.bs_buff <<= bits_written;
            self.bs_live -= bits_written;
            self.num_z += (bits_written / 8) as u32;
        }

        while self.bs_live >= 8 {
            self.zbits[self.num_z as usize] = (self.bs_buff >> 24) as u8;
            self.num_z += 1;
            self.bs_buff <<= 8;
            self.bs_live -= 8;
        }
    }

    fn write(&mut self, n: u8, v: u32) {
        self.flush_whole_bytes();

        self.bs_buff |= v << (32 - self.bs_live - i32::from(n));
        self.bs_live += i32::from(n);
    }

    fn write_u8(&mut self, c: u8) {
        self.write(8, c as u32);
    }

    fn write_u32(&mut self, u: u32) {
        let [a, b, c, d] = u.to_le_bytes();

        self.write(8, d as u32);
        self.write(8, c as u32);
        self.write(8, b as u32);
        self.write(8, a as u32);
    }
}

fn make_maps_e(s: &mut EState) {
    s.nInUse = 0;
    for (i, in_use) in s.inUse.iter().enumerate() {
        if *in_use {
            s.unseqToSeq[i] = s.nInUse as u8;
            s.nInUse += 1;
        }
    }
}

fn generate_mtf_values(s: &mut EState) {
    /*
       After sorting (eg, here),
          s.arr1 [ 0 .. s->nblock-1 ] holds sorted order,
          and
          s.arr2 [ 0 .. s->nblock-1 ]
          holds the original block data.

       The first thing to do is generate the MTF values,
       and put them in
          s.arr1 [ 0 .. s->nblock-1 ].
       Because there are strictly fewer or equal MTF values
       than block values, ptr values in this area are overwritten
       with MTF values only when they are no longer needed.

       The final compressed bitstream is generated into the
       area starting at
          (UChar*) (&((UChar*)s->arr2)[s->nblock])

       These storage aliases are set up in bzCompressInit(),
       except for the last one, which is arranged in
       compressBlock().
    */

    make_maps_e(s);
    let EOB = s.nInUse + 1;

    s.mtfFreq[..=EOB as usize].fill(0);

    let mut wr = 0;
    let mut zPend = 0;

    let mut yy: [u8; 256] = [0; 256];
    for i in 0..s.nInUse {
        yy[i as usize] = i as u8;
    }

    for i in 0..s.nblock {
        debug_assert!(wr <= i, "generateMTFValues(1)");
        let mut j = s.arr1.ptr()[i as usize].wrapping_sub(1) as i32;
        if j < 0 {
            j += s.nblock;
        }
        let ll_i: u8 = s.unseqToSeq[s.arr2.block(s.nblock as usize)[j as usize] as usize];
        debug_assert!((ll_i as i32) < s.nInUse, "generateMTFValues(2a)");

        if yy[0] == ll_i {
            zPend += 1;
        } else {
            if zPend > 0 {
                zPend -= 1;
                loop {
                    if zPend & 1 != 0 {
                        s.arr1.mtfv()[wr as usize] = 1;
                        wr += 1;
                        s.mtfFreq[1] += 1;
                    } else {
                        s.arr1.mtfv()[wr as usize] = 0;
                        wr += 1;
                        s.mtfFreq[0] += 1;
                    }
                    if zPend < 2 {
                        break;
                    }
                    zPend = (zPend - 2) / 2;
                }
                zPend = 0;
            }

            {
                let mut rtmp: u8;
                rtmp = yy[1];
                yy[1] = yy[0];
                j = 1;
                while ll_i != rtmp {
                    j += 1;
                    core::mem::swap(&mut rtmp, &mut yy[j as usize]);
                }
                yy[0] = rtmp;
                s.arr1.mtfv()[wr as usize] = (j + 1) as u16;
                wr += 1;
                s.mtfFreq[(j + 1) as usize] += 1;
            }
        }
    }

    if zPend > 0 {
        zPend -= 1;
        loop {
            if zPend & 1 != 0 {
                s.arr1.mtfv()[wr as usize] = BZ_RUNB;
                wr += 1;
                s.mtfFreq[BZ_RUNB as usize] += 1;
            } else {
                s.arr1.mtfv()[wr as usize] = BZ_RUNA;
                wr += 1;
                s.mtfFreq[BZ_RUNA as usize] += 1;
            }
            if zPend < 2 {
                break;
            }
            zPend = (zPend - 2) / 2;
        }
    }

    s.arr1.mtfv()[wr as usize] = EOB as u16;
    wr += 1;
    s.mtfFreq[EOB as usize] += 1;

    s.nMTF = wr;
}

fn send_mtf_values(s: &mut EState) {
    const BZ_LESSER_ICOST: u8 = 0;
    const BZ_GREATER_ICOST: u8 = 15;

    let mut gs: i32;
    let mut ge: i32;
    let mut totc: i32;
    let mut bt: i32;
    let mut bc: i32;
    let mut nSelectors: usize = 0;
    let mut selCtr: usize;
    let mut nBytes: i32;

    /*--
    s.len: [[u8; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS];
    is a global because the decoder also needs it.

    s.code: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS];
    s.rfreq: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS];

    are also globals only used in this proc.
    Made global to keep stack frame size small.
    --*/

    let mtfv = s.arr1.mtfv();

    if s.verbosity >= 3 {
        debug_logln!(
            "      {} in block, {} after MTF & 1-2 coding, {}+2 syms in use",
            s.nblock,
            s.nMTF,
            s.nInUse,
        );
    }

    let alphaSize = usize::try_from(s.nInUse + 2).unwrap_or(0);

    for t in s.len.iter_mut() {
        t[..alphaSize].fill(BZ_GREATER_ICOST);
    }

    /*--- Decide how many coding tables to use ---*/
    assert_h!(s.nMTF > 0, 3001);
    let nGroups: usize = match s.nMTF {
        0..200 => 2,
        200..600 => 3,
        600..1200 => 4,
        1200..2400 => 5,
        _ => 6,
    };

    let mut cost: [u16; 6] = [0; 6];
    let cost = &mut cost[..nGroups];

    let mut fave: [i32; 6] = [0; 6];
    let fave = &mut fave[..nGroups];

    /*--- Generate an initial set of coding tables ---*/
    {
        let mut tFreq: i32;
        let mut aFreq: i32;

        let mut nPart = nGroups;
        let mut remF = s.nMTF;
        let mut gs = 0i32;

        while nPart > 0 {
            tFreq = remF / nPart as i32;
            ge = gs - 1;
            aFreq = 0;
            while aFreq < tFreq && ge < alphaSize as i32 - 1 {
                ge += 1;
                aFreq += s.mtfFreq[ge as usize];
            }
            if ge > gs && nPart != nGroups && nPart != 1 && (nGroups - nPart) % 2 == 1 {
                aFreq -= s.mtfFreq[ge as usize];
                ge -= 1;
            }

            if s.verbosity >= 3 {
                debug_logln!(
                    "      initial group {}, [{} .. {}], has {} syms ({:4.1}%)",
                    nPart,
                    gs,
                    ge,
                    aFreq,
                    100.0f64 * aFreq as f64 / s.nMTF as f64,
                );
            }

            for v in 0..alphaSize {
                s.len[nPart - 1][v] = if (gs..=ge).contains(&(v as i32)) {
                    BZ_LESSER_ICOST
                } else {
                    BZ_GREATER_ICOST
                };
            }
            nPart -= 1;
            gs = ge + 1;
            remF -= aFreq;
        }
    }

    /*---
       Iterate up to BZ_N_ITERS times to improve the tables.
    ---*/
    for iter in 0..BZ_N_ITERS {
        fave.fill(0);

        for t in 0..nGroups {
            s.rfreq[t][..alphaSize].fill(0);
        }

        /*---
          Set up an auxiliary length table which is used to fast-track
          the common case (nGroups == 6).
        ---*/
        if nGroups == 6 {
            for v in 0..alphaSize {
                s.len_pack[v][0] = ((s.len[1][v] as u32) << 16) | (s.len[0][v] as u32);
                s.len_pack[v][1] = ((s.len[3][v] as u32) << 16) | (s.len[2][v] as u32);
                s.len_pack[v][2] = ((s.len[5][v] as u32) << 16) | (s.len[4][v] as u32);
            }
        }

        nSelectors = 0;
        totc = 0;
        gs = 0;
        loop {
            /*--- Set group start & end marks. --*/
            if gs >= s.nMTF {
                break;
            }
            ge = gs + 50 - 1;
            if ge >= s.nMTF {
                ge = s.nMTF - 1;
            }

            /*--
               Calculate the cost of this group as coded
               by each of the coding tables.
            --*/
            cost.fill(0);

            if nGroups == 6 && 50 == ge - gs + 1 {
                let mut cost01: u32 = 0;
                let mut cost23: u32 = 0;
                let mut cost45: u32 = 0;

                for chunk in mtfv[gs as usize..][..50].chunks_exact(10) {
                    for icv in chunk {
                        let [a, b, c, _] = s.len_pack[usize::from(*icv)];
                        cost01 = cost01.wrapping_add(a);
                        cost23 = cost23.wrapping_add(b);
                        cost45 = cost45.wrapping_add(c);
                    }
                }

                cost[0] = (cost01 & 0xffff) as u16;
                cost[1] = (cost01 >> 16) as u16;
                cost[2] = (cost23 & 0xffff) as u16;
                cost[3] = (cost23 >> 16) as u16;
                cost[4] = (cost45 & 0xffff) as u16;
                cost[5] = (cost45 >> 16) as u16;
            } else {
                /*--- slow version which correctly handles all situations ---*/
                for i in gs..=ge {
                    let icv_0: u16 = mtfv[i as usize];

                    for (t, c) in cost.iter_mut().enumerate() {
                        *c = (*c as i32 + s.len[t][icv_0 as usize] as i32) as u16;
                    }
                }
            }

            /*--
               Find the coding table which is best for this group,
               and record its identity in the selector table.
            --*/
            bc = 999999999;
            bt = -1;
            for (t, &c) in cost.iter().enumerate() {
                if (c as i32) < bc {
                    bc = c as i32;
                    bt = t as i32;
                }
            }
            totc += bc;
            fave[bt as usize] += 1;
            s.selector[nSelectors] = bt as u8;
            nSelectors += 1;

            if nGroups == 6 && 50 == ge - gs + 1 {
                for chunk in mtfv[gs as usize..][..50].chunks_exact(10) {
                    for &mtfv_i in chunk {
                        s.rfreq[bt as usize][usize::from(mtfv_i)] += 1;
                    }
                }
            } else {
                for i in gs..=ge {
                    s.rfreq[bt as usize][mtfv[i as usize] as usize] += 1;
                }
            }

            gs = ge + 1;
        }

        if s.verbosity >= 3 {
            debug_log!(
                "      pass {}: size is {}, grp uses are ",
                iter + 1,
                totc / 8,
            );
            for f in fave.iter() {
                debug_log!("{} ", f);
            }
            debug_logln!();
        }

        /*--
          Recompute the tables based on the accumulated frequencies.
        --*/
        /* maxLen was changed from 20 to 17 in bzip2-1.0.3.  See
        comment in huffman.c for details. */
        for t in 0..nGroups {
            huffman::make_code_lengths(&mut s.len[t], &s.rfreq[t], alphaSize, 17);
        }
    }

    assert_h!(nGroups < 8, 3002);
    assert_h!(nSelectors < 32768, 3003);
    assert_h!(nSelectors <= usize::from(BZ_MAX_SELECTORS), 3003);

    /*--- Compute MTF values for the selectors. ---*/
    {
        let mut pos: [u8; BZ_N_GROUPS] = [0, 1, 2, 3, 4, 5];

        let mut tmp2: u8;
        let mut tmp: u8;

        for (i, &ll_i) in s.selector[..nSelectors].iter().enumerate() {
            let mut j = 0;
            tmp = pos[j as usize];
            while ll_i != tmp {
                j += 1;
                tmp2 = tmp;
                tmp = pos[j as usize];
                pos[j as usize] = tmp2;
            }
            pos[0] = tmp;
            s.selectorMtf[i] = j as u8;
        }
    }

    /*--- Assign actual codes for the tables. --*/
    for (t, len) in s.len[..nGroups].iter().enumerate() {
        let len = &len[..alphaSize];

        let mut minLen = 32;
        let mut maxLen = 0;

        for &l in len {
            maxLen = Ord::max(maxLen, l);
            minLen = Ord::min(minLen, l);
        }

        assert_h!(maxLen <= 17, 3004);
        assert_h!(minLen >= 1, 3005);

        huffman::assign_codes(&mut s.code[t], len, minLen, maxLen);
    }

    /*--- Transmit the mapping table. ---*/
    let mut writer = LiveWriter::new(&mut s.writer, s.arr2.zbits(s.nblock as usize));

    {
        let inUse16: [bool; 16] =
            core::array::from_fn(|i| s.inUse[i * 16..][..16].iter().any(|x| *x));

        nBytes = writer.num_z as i32;
        for in_use in inUse16 {
            writer.write(1, in_use as u32);
        }
        for (i, any_in_use) in inUse16.iter().enumerate() {
            if *any_in_use {
                for j in 0..16 {
                    writer.write(1, s.inUse[i * 16 + j] as u32);
                }
            }
        }
        if s.verbosity >= 3 {
            debug_log!("      bytes: mapping {}, ", writer.num_z as i32 - nBytes,);
        }
    }

    /*--- Now the selectors. ---*/
    nBytes = writer.num_z as i32;
    writer.write(3, nGroups as u32);
    writer.write(15, nSelectors as u32);

    for i in 0..nSelectors {
        for _ in 0..s.selectorMtf[i] {
            writer.write(1, 1);
        }
        writer.write(1, 0);
    }
    if s.verbosity >= 3 {
        debug_log!("selectors {}, ", writer.num_z as i32 - nBytes);
    }

    /*--- Now the coding tables. ---*/
    nBytes = writer.num_z as i32;

    for t in 0..nGroups {
        let mut curr = s.len[t][0];
        writer.write(5, curr as u32);
        for i in 0..alphaSize {
            while curr < s.len[t][i] {
                writer.write(2, 2);
                curr += 1;
            }
            while curr > s.len[t][i] {
                writer.write(2, 3);
                curr -= 1;
            }
            writer.write(1, 0);
        }
    }
    if s.verbosity >= 3 {
        debug_log!("code lengths {}, ", writer.num_z as i32 - nBytes);
    }

    /*--- And finally, the block data proper ---*/
    nBytes = writer.num_z as i32;
    selCtr = 0;
    gs = 0;
    loop {
        if gs >= s.nMTF {
            break;
        }
        ge = gs + 50 - 1;
        if ge >= s.nMTF {
            ge = s.nMTF - 1;
        }
        assert_h!((s.selector[selCtr] as usize) < nGroups, 3006);
        if nGroups == 6 && 50 == ge - gs + 1 {
            /*--- fast track the common case ---*/

            let s_len_sel_selCtr = s.len[s.selector[selCtr] as usize];
            let s_code_sel_selCtr = s.code[s.selector[selCtr] as usize];

            for chunk in mtfv[gs as usize..][..50].chunks_exact(10) {
                for &mtfv_i in chunk {
                    writer.write(
                        s_len_sel_selCtr[usize::from(mtfv_i)],
                        s_code_sel_selCtr[usize::from(mtfv_i)],
                    );
                }
            }
        } else {
            /*--- slow version which correctly handles all situations ---*/
            for i in gs..=ge {
                writer.write(
                    s.len[s.selector[selCtr] as usize][mtfv[i as usize] as usize],
                    s.code[s.selector[selCtr] as usize][mtfv[i as usize] as usize],
                );
            }
        }
        gs = ge + 1;
        selCtr += 1;
    }
    assert_h!(selCtr == nSelectors, 3007);

    if s.verbosity >= 3 {
        debug_logln!("codes {}", writer.num_z as i32 - nBytes);
    }
}

pub(crate) fn compress_block(s: &mut EState, is_last_block: bool) {
    if s.nblock > 0 {
        s.blockCRC = !s.blockCRC;
        s.combinedCRC = s.combinedCRC.rotate_left(1);
        s.combinedCRC ^= s.blockCRC;
        if s.blockNo > 1 {
            s.writer.num_z = 0;
        }

        if s.verbosity >= 2 {
            debug_logln!(
                "    block {}: crc = 0x{:08x}, combined CRC = 0x{:08x}, size = {}",
                s.blockNo,
                s.blockCRC,
                s.combinedCRC,
                s.nblock,
            );
        }

        block_sort(s);
    }

    {
        /*-- If this is the first block, create the stream header. --*/
        if s.blockNo == 1 {
            let mut writer = LiveWriter::new(&mut s.writer, s.arr2.zbits(s.nblock as usize));

            writer.initialize();
            writer.write_u8(b'B');
            writer.write_u8(b'Z');
            writer.write_u8(b'h');
            writer.write_u8(b'0' + s.blockSize100k as u8);
        }

        if s.nblock > 0 {
            let mut writer = LiveWriter::new(&mut s.writer, s.arr2.zbits(s.nblock as usize));

            writer.write_u8(0x31);
            writer.write_u8(0x41);
            writer.write_u8(0x59);
            writer.write_u8(0x26);
            writer.write_u8(0x53);
            writer.write_u8(0x59);

            /*-- Now the block's CRC, so it is in a known place. --*/
            writer.write_u32(s.blockCRC);

            /*--
               Now a single bit indicating (non-)randomisation.
               As of version 0.9.5, we use a better sorting algorithm
               which makes randomisation unnecessary.  So always set
               the randomised bit to 'no'.  Of course, the decoder
               still needs to be able to handle randomised blocks
               so as to maintain backwards compatibility with
               older versions of bzip2.
            --*/
            writer.write(1, 0);

            writer.write(24, s.origPtr as u32);

            drop(writer);

            generate_mtf_values(s);

            send_mtf_values(s);
        }
    }

    /*-- If this is the last block, add the stream trailer. --*/
    if is_last_block {
        let mut writer = LiveWriter::new(&mut s.writer, s.arr2.zbits(s.nblock as usize));

        writer.write_u8(0x17);
        writer.write_u8(0x72);
        writer.write_u8(0x45);
        writer.write_u8(0x38);
        writer.write_u8(0x50);
        writer.write_u8(0x90);
        writer.write_u32(s.combinedCRC);

        if s.verbosity >= 2 {
            debug_log!("    final combined CRC = 0x{:08x}\n   ", s.combinedCRC);
        }

        writer.finish();
    }
}
