// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp::min;
use std::collections::HashSet;

use symphonia_core::errors::{decode_error, Error, Result};
use symphonia_core::io::{BitReaderRtl, ReadBitsRtl};

use super::codebook::VorbisCodebook;
use super::common::*;

/// As defined in section 10.1 of the Vorbis I specification.
#[allow(clippy::unreadable_literal)]
#[allow(clippy::excessive_precision)]
#[rustfmt::skip]
const FLOOR1_INVERSE_DB_TABLE: [f32; 256] = [
    1.0649863e-07, 1.1341951e-07, 1.2079015e-07, 1.2863978e-07,
    1.3699951e-07, 1.4590251e-07, 1.5538408e-07, 1.6548181e-07,
    1.7623575e-07, 1.8768855e-07, 1.9988561e-07, 2.1287530e-07,
    2.2670913e-07, 2.4144197e-07, 2.5713223e-07, 2.7384213e-07,
    2.9163793e-07, 3.1059021e-07, 3.3077411e-07, 3.5226968e-07,
    3.7516214e-07, 3.9954229e-07, 4.2550680e-07, 4.5315863e-07,
    4.8260743e-07, 5.1396998e-07, 5.4737065e-07, 5.8294187e-07,
    6.2082472e-07, 6.6116941e-07, 7.0413592e-07, 7.4989464e-07,
    7.9862701e-07, 8.5052630e-07, 9.0579828e-07, 9.6466216e-07,
    1.0273513e-06, 1.0941144e-06, 1.1652161e-06, 1.2409384e-06,
    1.3215816e-06, 1.4074654e-06, 1.4989305e-06, 1.5963394e-06,
    1.7000785e-06, 1.8105592e-06, 1.9282195e-06, 2.0535261e-06,
    2.1869758e-06, 2.3290978e-06, 2.4804557e-06, 2.6416497e-06,
    2.8133190e-06, 2.9961443e-06, 3.1908506e-06, 3.3982101e-06,
    3.6190449e-06, 3.8542308e-06, 4.1047004e-06, 4.3714470e-06,
    4.6555282e-06, 4.9580707e-06, 5.2802740e-06, 5.6234160e-06,
    5.9888572e-06, 6.3780469e-06, 6.7925283e-06, 7.2339451e-06,
    7.7040476e-06, 8.2047000e-06, 8.7378876e-06, 9.3057248e-06,
    9.9104632e-06, 1.0554501e-05, 1.1240392e-05, 1.1970856e-05,
    1.2748789e-05, 1.3577278e-05, 1.4459606e-05, 1.5399272e-05,
    1.6400004e-05, 1.7465768e-05, 1.8600792e-05, 1.9809576e-05,
    2.1096914e-05, 2.2467911e-05, 2.3928002e-05, 2.5482978e-05,
    2.7139006e-05, 2.8902651e-05, 3.0780908e-05, 3.2781225e-05,
    3.4911534e-05, 3.7180282e-05, 3.9596466e-05, 4.2169667e-05,
    4.4910090e-05, 4.7828601e-05, 5.0936773e-05, 5.4246931e-05,
    5.7772202e-05, 6.1526565e-05, 6.5524908e-05, 6.9783085e-05,
    7.4317983e-05, 7.9147585e-05, 8.4291040e-05, 8.9768747e-05,
    9.5602426e-05, 0.00010181521, 0.00010843174, 0.00011547824,
    0.00012298267, 0.00013097477, 0.00013948625, 0.00014855085,
    0.00015820453, 0.00016848555, 0.00017943469, 0.00019109536,
    0.00020351382, 0.00021673929, 0.00023082423, 0.00024582449,
    0.00026179955, 0.00027881276, 0.00029693158, 0.00031622787,
    0.00033677814, 0.00035866388, 0.00038197188, 0.00040679456,
    0.00043323036, 0.00046138411, 0.00049136745, 0.00052329927,
    0.00055730621, 0.00059352311, 0.00063209358, 0.00067317058,
    0.00071691700, 0.00076350630, 0.00081312324, 0.00086596457,
    0.00092223983, 0.00098217216, 0.0010459992,  0.0011139742,
    0.0011863665,  0.0012634633,  0.0013455702,  0.0014330129,
    0.0015261382,  0.0016253153,  0.0017309374,  0.0018434235,
    0.0019632195,  0.0020908006,  0.0022266726,  0.0023713743,
    0.0025254795,  0.0026895994,  0.0028643847,  0.0030505286,
    0.0032487691,  0.0034598925,  0.0036847358,  0.0039241906,
    0.0041792066,  0.0044507950,  0.0047400328,  0.0050480668,
    0.0053761186,  0.0057254891,  0.0060975636,  0.0064938176,
    0.0069158225,  0.0073652516,  0.0078438871,  0.0083536271,
    0.0088964928,  0.009474637,   0.010090352,   0.010746080,
    0.011444421,   0.012188144,   0.012980198,   0.013823725,
    0.014722068,   0.015678791,   0.016697687,   0.017782797,
    0.018938423,   0.020169149,   0.021479854,   0.022875735,
    0.024362330,   0.025945531,   0.027631618,   0.029427276,
    0.031339626,   0.033376252,   0.035545228,   0.037855157,
    0.040315199,   0.042935108,   0.045725273,   0.048696758,
    0.051861348,   0.055231591,   0.058820850,   0.062643361,
    0.066714279,   0.071049749,   0.075666962,   0.080584227,
    0.085821044,   0.091398179,   0.097337747,   0.10366330,
    0.11039993,    0.11757434,    0.12521498,    0.13335215,
    0.14201813,    0.15124727,    0.16107617,    0.17154380,
    0.18269168,    0.19456402,    0.20720788,    0.22067342,
    0.23501402,    0.25028656,    0.26655159,    0.28387361,
    0.30232132,    0.32196786,    0.34289114,    0.36517414,
    0.38890521,    0.41417847,    0.44109412,    0.46975890,
    0.50028648,    0.53279791,    0.56742212,    0.60429640,
    0.64356699,    0.68538959,    0.72993007,    0.77736504,
    0.82788260,    0.88168307,    0.9389798,     1.0,
];

macro_rules! io_try_or_ret {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            // An end-of-bitstream error is classified under ErrorKind::Other. This condition
            // should not be treated as an error, rather, it should return from the function
            // immediately without error.
            Err(ref e) if e.kind() == std::io::ErrorKind::Other => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    };
}

macro_rules! try_or_ret {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            // An end-of-bitstream error is classified under ErrorKind::Other. This condition
            // should not be treated as an error, rather, it should return from the function
            // immediately without error.
            Err(Error::IoError(ref e)) if e.kind() == std::io::ErrorKind::Other => return Ok(()),
            Err(e) => return Err(e),
        }
    };
}

pub trait Floor: Send + Sync {
    fn read_channel(
        &mut self,
        bs: &mut BitReaderRtl<'_>,
        codebooks: &[VorbisCodebook],
    ) -> Result<()>;
    fn is_unused(&self) -> bool;
    fn synthesis(&mut self, bs_exp: u8, floor: &mut [f32]) -> Result<()>;
}

#[derive(Debug)]
struct Floor0Setup {
    floor0_order: u8,
    floor0_bark_map_size: u16,
    floor0_amplitude_bits: u8,
    floor0_amplitude_offset: u8,
    floor0_number_of_books: u8,
    floor0_book_list: [u8; 16],
    // The block size of the short Bark map.
    floor0_map_short_bs_exp: u8,
    // Pre-computed Bark map for short blocks.
    floor0_map_short: Vec<i32>,
    // Pre-computed Bark map for long blocks.
    floor0_map_long: Vec<i32>,
}

#[derive(Debug)]
pub struct Floor0 {
    setup: Floor0Setup,
    is_unused: bool,
    amplitude: u64,
    coeffs: [f32; 256],
}

impl Floor0 {
    pub fn try_read(
        bs: &mut BitReaderRtl<'_>,
        bs0_exp: u8,
        bs1_exp: u8,
        max_codebook: u8,
    ) -> Result<Box<dyn Floor>> {
        let setup = Self::read_setup(bs, bs0_exp, bs1_exp, max_codebook)?;

        Ok(Box::new(Floor0 { setup, is_unused: false, amplitude: 0, coeffs: [0.0; 256] }))
    }

    fn read_setup(
        bs: &mut BitReaderRtl<'_>,
        bs0_exp: u8,
        bs1_exp: u8,
        max_codebook: u8,
    ) -> Result<Floor0Setup> {
        let floor0_order = bs.read_bits_leq32(8)? as u8;
        let floor0_rate = bs.read_bits_leq32(16)? as u16;
        let floor0_bark_map_size = bs.read_bits_leq32(16)? as u16;
        let floor0_amplitude_bits = bs.read_bits_leq32(6)? as u8;
        let floor0_amplitude_offset = bs.read_bits_leq32(8)? as u8;
        let floor0_number_of_books = bs.read_bits_leq32(4)? as u8 + 1;
        let mut floor0_book_list = [0; 16];

        let end = usize::from(floor0_number_of_books);

        for book in &mut floor0_book_list[..end] {
            *book = bs.read_bits_leq32(8)? as u8;

            if *book >= max_codebook {
                return decode_error("vorbis: floor0, invalid codebook number");
            }
        }

        // Pre-compute the Bark-scale maps.
        let floor0_map_short = bark_map(1 << (bs0_exp - 1), floor0_rate, floor0_bark_map_size);
        let floor0_map_long = bark_map(1 << (bs1_exp - 1), floor0_rate, floor0_bark_map_size);

        let floor_type0 = Floor0Setup {
            floor0_order,
            floor0_bark_map_size,
            floor0_amplitude_bits,
            floor0_amplitude_offset,
            floor0_number_of_books,
            floor0_book_list,
            floor0_map_short_bs_exp: bs0_exp,
            floor0_map_short,
            floor0_map_long,
        };

        Ok(floor_type0)
    }
}

impl Floor for Floor0 {
    fn read_channel(
        &mut self,
        bs: &mut BitReaderRtl<'_>,
        codebooks: &[VorbisCodebook],
    ) -> Result<()> {
        // Assume the floor is unused until it is decoded successfully.
        self.is_unused = true;

        self.amplitude =
            io_try_or_ret!(bs.read_bits_leq64(u32::from(self.setup.floor0_amplitude_bits)));

        if self.amplitude != 0 {
            // Read the index into the floor's codebook list that contains the actual codebook
            // index.
            let floor_book_idx_bits = ilog(u32::from(self.setup.floor0_number_of_books));
            let floor_book_idx = io_try_or_ret!(bs.read_bits_leq32(floor_book_idx_bits)) as usize;

            // Get the actual codebook index from the floor's codebook list.
            let codebook_idx = self.setup.floor0_book_list[floor_book_idx] as usize;

            if codebook_idx >= codebooks.len() {
                return decode_error("vorbis: floor0, invalid codebook");
            }

            // Get the codebook for this floor.
            let codebook = &codebooks[codebook_idx];

            let order = usize::from(self.setup.floor0_order);
            let mut i = 0;
            let mut last = 0.0;

            while i < order {
                let i0 = i;

                // Read and obtain the VQ vector from the codebook.
                let vq = try_or_ret!(codebook.read_vq(bs));

                // The VQ vector may be much larger (up-to 65535 scalars) than the remaining number
                // of coefficients (up-to 255 scalars). Cap the amount of coefficients to be
                // processed.
                i += min(order - i0, vq.len());

                // Add the value of last coefficient from the previous iteration to each scalar
                // value read from the VQ vector and append those valeus to the coefficient vector.
                for (c, &vq) in self.coeffs[i0..i].iter_mut().zip(vq) {
                    *c = last + vq;
                }

                // Store the value of the last coefficient in the coefficient vector for the next
                // iteration.
                last = self.coeffs[i - 1];
            }

            // Pre-compute the 2 times the cosine of all coefficients.
            for coeff in self.coeffs[..order].iter_mut() {
                *coeff = 2.0 * coeff.cos();
            }
        }

        // The floor is used if the amplitude is not 0.
        self.is_unused = self.amplitude == 0;

        Ok(())
    }

    fn is_unused(&self) -> bool {
        self.is_unused
    }

    fn synthesis(&mut self, bs_exp: u8, floor: &mut [f32]) -> Result<()> {
        debug_assert!(!self.is_unused);

        // Block size.
        let n = (1 << bs_exp) >> 1;

        // Select the correct Bark-scale map based on the block-size exponent.
        let map = if bs_exp == self.setup.floor0_map_short_bs_exp {
            &self.setup.floor0_map_short
        }
        else {
            &self.setup.floor0_map_long
        };

        let omega_step = std::f32::consts::PI / f32::from(self.setup.floor0_bark_map_size);

        let mut i = 0;

        loop {
            let iter_cond = map[i];

            let omega = omega_step * iter_cond as f32;
            let cos_omega = omega.cos();
            let two_cos_omega = 2.0 * cos_omega;

            let mut p = 1.0;
            let mut q = 1.0;

            let mut iter = self.coeffs[..usize::from(self.setup.floor0_order)].chunks_exact(2);

            // Calculate p using coefficients with odd indicies, and q using coefficients with even
            // indicies.
            for coeffs in &mut iter {
                p *= coeffs[1] - two_cos_omega;
                q *= coeffs[0] - two_cos_omega;
            }

            // If order is an odd number, then there should be exactly one extra coefficient.
            let last_coeff = iter.remainder();

            if !last_coeff.is_empty() {
                q *= last_coeff[0] - two_cos_omega;

                p = p * p * (1.0 - (cos_omega * cos_omega));
                q = q * q * 0.25;
            }
            else {
                p = p * p * ((1.0 - cos_omega) / 2.0);
                q = q * q * ((1.0 + cos_omega) / 2.0);
            }

            if p + q == 0.0 {
                return decode_error("vorbis: invalid floor0 coefficients");
            }

            let linear_floor_value = linear_floor0_value(
                p,
                q,
                self.amplitude,
                self.setup.floor0_amplitude_bits,
                self.setup.floor0_amplitude_offset,
            );

            // Fill in the floor values where the map value is the same.
            for (floor, &map) in floor[i..n].iter_mut().zip(&map[i..n]) {
                if map != iter_cond {
                    break;
                }

                *floor = linear_floor_value;
                i += 1;
            }

            if i >= n {
                break;
            }
        }

        Ok(())
    }
}

/// Vorbis I specification, section 6.2.3.
#[inline(always)]
fn bark(x: f64) -> f64 {
    (13.1 * (0.00074 * x).atan()) + (2.24 * (0.0000000185 * x * x).atan()) + (0.0001 * x)
}

fn bark_map(n: u32, floor0_rate: u16, floor0_bark_map_size: u16) -> Vec<i32> {
    let mut map = Vec::with_capacity(n as usize);

    let foobar_min = i32::from(floor0_bark_map_size) - 1;
    let rate = f64::from(floor0_rate);
    let rate_by_2n = rate / (2.0 * f64::from(n));

    let c = f64::from(floor0_bark_map_size) / bark(0.5 * rate);

    // Compute 0 to N-1 elements.
    for i in 0..n {
        let foobar = (bark(rate_by_2n * f64::from(i)) * c).floor() as i32;
        map.push(foobar.min(foobar_min));
    }

    map
}

/// Calculate the linear floor value as per Vorbis I specification, section 6.2.3.
#[inline(always)]
fn linear_floor0_value(
    p: f32,
    q: f32,
    amplitude: u64,
    amplitude_bits: u8,
    amplitude_offset: u8,
) -> f32 {
    // Amplitude could be up-to 63-bits, and amplitude offset 8-bits. Therefore, 71-bits is required
    // in total. This is unreasonable since even an f64 can't represent more than 56-bits. Since
    // large values like this shouldn't happen in practice, use wrapping arithmetic to prevent
    // panics and truncate to 32-bits in the f32 conversion.
    let a = amplitude.wrapping_mul(u64::from(amplitude_offset)) as f32;
    let b = (p + q).sqrt() * ((1u64 << amplitude_bits) - 1) as f32;

    (0.11512925 * ((a / b) - f32::from(amplitude_offset))).exp()
}

#[derive(Debug, Default)]
struct Floor1Class {
    /// Main codebook index.
    mainbook: u8,
    /// Class dimensions.
    dimensions: u8,
    /// Number of sub-classes expressed as a power-of-2 exponent (2 ^ subclass_bits).
    subclass_bits: u8,
    /// Codebook index for each sub-class.
    subbooks: [u8; 8],
    /// Bitset marking if a sub-class codebook is used or not.
    is_subbook_used: u8,
}

#[derive(Debug)]
struct Floor1Setup {
    /// Number of partitions, range limited to 0..32.
    floor1_partitions: usize,
    /// Class index (range limited to 0..16), associated with each partition.
    floor1_partition_class_list: [u8; 32],
    /// Classes.
    floor1_classes: [Floor1Class; 16],
    /// Floor multiplier, range limited to 1..5.
    floor1_multiplier: u8,
    /// X-list.
    floor1_x_list: Vec<u32>,
    // Precomputed x-list sort order.
    floor1_x_list_sort_order: Vec<u8>,
    // Precomputed x-list neighbours.
    floor1_x_list_neighbors: Vec<(usize, usize)>,
}

#[derive(Debug)]
pub struct Floor1 {
    setup: Floor1Setup,
    is_unused: bool,
    floor_y: Vec<u32>,
    floor_final_y: Vec<i32>,
    floor_step2_flag: Vec<bool>,
}

impl Floor1 {
    pub fn try_read(bs: &mut BitReaderRtl<'_>, max_codebook: u8) -> Result<Box<dyn Floor>> {
        let setup = Self::read_setup(bs, max_codebook)?;

        let x_list_len = setup.floor1_x_list.len();

        Ok(Box::new(Floor1 {
            setup,
            is_unused: false,
            floor_y: vec![0; x_list_len],
            floor_final_y: vec![0; x_list_len],
            floor_step2_flag: vec![false; x_list_len],
        }))
    }

    fn read_setup(bs: &mut BitReaderRtl<'_>, max_codebook: u8) -> Result<Floor1Setup> {
        // The number of partitions. 5-bit value, 0..31 range.
        let floor1_partitions = bs.read_bits_leq32(5)? as usize;

        // Parition list of up-to 32 partitions (floor1_partitions), with each partition indicating
        // a 4-bit class (0..16) identifier.
        let mut floor1_partition_class_list = [0; 32];

        let mut floor1_classes: [Floor1Class; 16] = Default::default();

        if floor1_partitions > 0 {
            let mut max_class = 0; // 4-bits, 0..15

            // Read the partition class list.
            for class_idx in &mut floor1_partition_class_list[..floor1_partitions] {
                *class_idx = bs.read_bits_leq32(4)? as u8;

                // Find the maximum class value.
                max_class = max_class.max(*class_idx);
            }

            let num_classes = usize::from(1 + max_class);

            for class in floor1_classes[..num_classes].iter_mut() {
                class.dimensions = bs.read_bits_leq32(3)? as u8 + 1;
                class.subclass_bits = bs.read_bits_leq32(2)? as u8;

                if class.subclass_bits != 0 {
                    let masterbook = bs.read_bits_leq32(8)? as u8;

                    if masterbook >= max_codebook {
                        return decode_error("vorbis: floor1, invalid codebook for class");
                    }

                    class.mainbook = masterbook;
                }

                let num_subclasses = 1 << class.subclass_bits;

                for (i, book) in class.subbooks[..num_subclasses].iter_mut().enumerate() {
                    // Read the codebook number.
                    *book = bs.read_bits_leq32(8)? as u8;

                    // A codebook number > 0 indicates a codebook is used.
                    if *book > 0 {
                        // The actual codebook used is the number read minus one.
                        *book -= 1;

                        if *book >= max_codebook {
                            return decode_error("vorbis: floor1, invalid codebook for subclass");
                        }

                        class.is_subbook_used |= 1 << i;
                    }
                }
            }
        }

        let floor1_multiplier = bs.read_bits_leq32(2)? as u8 + 1;

        let rangebits = bs.read_bits_leq32(4)?;

        let mut floor1_x_list = Vec::new();
        let mut floor1_x_list_unique = HashSet::new();

        floor1_x_list.push(0);
        floor1_x_list.push(1 << rangebits);

        for &class_idx in &floor1_partition_class_list[..floor1_partitions] {
            let class = &floor1_classes[class_idx as usize];

            // No more than 65 elements are allowed.
            if floor1_x_list.len() + usize::from(class.dimensions) > 65 {
                return decode_error("vorbis: floor1, x_list too long");
            }

            for _ in 0..class.dimensions {
                let x = bs.read_bits_leq32(rangebits)?;

                // All elements in the x list must be unique.
                if !floor1_x_list_unique.insert(x) {
                    return decode_error("vorbis: floor1, x_list is not unique");
                }

                floor1_x_list.push(x);
            }
        }

        let mut floor1_x_list_neighbors = Vec::with_capacity(floor1_x_list.len());
        let mut floor1_x_list_sort_order = Vec::with_capacity(floor1_x_list.len());

        // Precompute neighbors.
        for i in 0..floor1_x_list.len() {
            floor1_x_list_neighbors.push(find_neighbors(&floor1_x_list, i));
            floor1_x_list_sort_order.push(i as u8);
        }

        // Precompute sort-order.
        floor1_x_list_sort_order.sort_by_key(|&i| floor1_x_list[i as usize]);

        let floor_type1 = Floor1Setup {
            floor1_partitions,
            floor1_partition_class_list,
            floor1_classes,
            floor1_multiplier,
            floor1_x_list,
            floor1_x_list_neighbors,
            floor1_x_list_sort_order,
        };

        Ok(floor_type1)
    }

    fn synthesis_step1(&mut self) {
        // Step 1.
        let range = get_range(self.setup.floor1_multiplier);

        self.floor_step2_flag[0] = true;
        self.floor_step2_flag[1] = true;

        self.floor_final_y[0] = self.floor_y[0] as i32;
        self.floor_final_y[1] = self.floor_y[1] as i32;

        for i in 2..self.setup.floor1_x_list.len() {
            // Find the neighbours.
            let (low_neighbor_offset, high_neighbor_offset) = self.setup.floor1_x_list_neighbors[i];

            let predicted = render_point(
                self.setup.floor1_x_list[low_neighbor_offset],
                self.floor_final_y[low_neighbor_offset],
                self.setup.floor1_x_list[high_neighbor_offset],
                self.floor_final_y[high_neighbor_offset],
                self.setup.floor1_x_list[i],
            );

            let val = self.floor_y[i] as i32;
            let highroom = range as i32 - predicted;
            let lowroom = predicted;

            if val != 0 {
                let room = 2 * if highroom < lowroom { highroom } else { lowroom };

                self.floor_step2_flag[low_neighbor_offset] = true;
                self.floor_step2_flag[high_neighbor_offset] = true;
                self.floor_step2_flag[i] = true;

                self.floor_final_y[i] = if val >= room {
                    if highroom > lowroom {
                        val - lowroom + predicted
                    }
                    else {
                        predicted - val + highroom - 1
                    }
                }
                else {
                    // If val is odd.
                    if val & 1 == 1 {
                        predicted - ((val + 1) / 2)
                    }
                    else {
                        // If val is even.
                        predicted + (val / 2)
                    }
                }
            }
            else {
                self.floor_step2_flag[i] = false;
                self.floor_final_y[i] = predicted;
            }
        }
    }

    fn synthesis_step2(&mut self, n: u32, floor: &mut [f32]) {
        let multiplier = self.setup.floor1_multiplier as i32;

        let floor_final_y0 = self.floor_final_y[self.setup.floor1_x_list_sort_order[0] as usize];

        let mut hx = 0;
        let mut hy = 0;
        let mut lx = 0;
        let mut ly = floor_final_y0 * multiplier;

        // Iterate in sort-order.
        for i in self.setup.floor1_x_list_sort_order[1..].iter().map(|i| *i as usize) {
            if self.floor_step2_flag[i] {
                hy = self.floor_final_y[i] * multiplier;
                hx = self.setup.floor1_x_list[i];

                render_line(lx, ly, hx, hy, n as usize, floor);

                lx = hx;
                ly = hy;
            }
        }

        if hx < n {
            render_line(hx, hy, n, hy, n as usize, floor);
        }
    }
}

impl Floor for Floor1 {
    fn read_channel(
        &mut self,
        bs: &mut BitReaderRtl<'_>,
        codebooks: &[VorbisCodebook],
    ) -> Result<()> {
        // Assume the floor is unused until it is decoded successfully.
        self.is_unused = true;

        // First bit marks if this floor is used. Exit early if it is not.
        let is_used = io_try_or_ret!(bs.read_bool());

        if !is_used {
            return Ok(());
        }

        // Section 7.3.2
        let range = get_range(self.setup.floor1_multiplier);

        // The number of bits required to represent range.
        let range_bits = ilog(range - 1);

        self.floor_y[0] = io_try_or_ret!(bs.read_bits_leq32(range_bits));
        self.floor_y[1] = io_try_or_ret!(bs.read_bits_leq32(range_bits));

        let mut offset = 2;

        for &class_idx in &self.setup.floor1_partition_class_list[0..self.setup.floor1_partitions] {
            // The class.
            let class = &self.setup.floor1_classes[class_idx as usize];

            let cdim = class.dimensions as usize;
            let cbits = class.subclass_bits;
            let csub = (1 << cbits) - 1;

            let mut cval = 0;

            if cbits > 0 {
                let mainbook_idx = class.mainbook as usize;
                cval = try_or_ret!(codebooks[mainbook_idx].read_scalar(bs));
            }

            for floor_y in self.floor_y[offset..offset + cdim].iter_mut() {
                let subclass_idx = cval & csub;

                // Is the sub-book used for this sub-class.
                let is_subbook_used = class.is_subbook_used & (1 << subclass_idx) != 0;

                cval >>= cbits;

                *floor_y = if is_subbook_used {
                    let subbook_idx = class.subbooks[subclass_idx as usize] as usize;
                    try_or_ret!(codebooks[subbook_idx].read_scalar(bs))
                }
                else {
                    0
                };
            }

            offset += cdim;
        }

        // If this point is reached then the floor is used.
        self.is_unused = false;

        Ok(())
    }

    fn is_unused(&self) -> bool {
        self.is_unused
    }

    fn synthesis(&mut self, bs_exp: u8, floor: &mut [f32]) -> Result<()> {
        debug_assert!(!self.is_unused);
        self.synthesis_step1();
        self.synthesis_step2((1 << bs_exp) >> 1, floor);
        Ok(())
    }
}

#[inline(always)]
fn get_range(multiplier: u8) -> u32 {
    match multiplier - 1 {
        0 => 256,
        1 => 128,
        2 => 86,
        3 => 64,
        _ => unreachable!(),
    }
}

#[inline(always)]
fn find_neighbors(vec: &[u32], x: usize) -> (usize, usize) {
    let bound = vec[x];

    let mut low = u32::MIN; // TODO: Should be -1?
    let mut high = u32::MAX;

    let mut res: (usize, usize) = (0, 0);

    // Sections 9.2.4 and 9.2.5
    for (i, &xv) in vec[..x].iter().enumerate() {
        // low_neighbor(v,x) finds the position n in vector [v] of the greatest value scalar element
        // for which n is less than x and vector [v] element n is less than vector [v] element [x].
        if xv > low && xv < bound {
            low = xv;
            res.0 = i;
        }
        // high_neighbor(v,x) finds the position n in vector [v] of the lowest value scalar element
        // for which n is less than x and vector [v] element n is greater than vector [v] element [x].
        if xv < high && xv > bound {
            high = xv;
            res.1 = i;
        }
    }

    res
}

#[inline(always)]
fn render_point(x0: u32, y0: i32, x1: u32, y1: i32, x: u32) -> i32 {
    let dy = y1 - y0;
    let adx = x1 - x0;
    let err = dy.unsigned_abs() * (x - x0);
    let off = err / adx;
    if dy < 0 {
        y0 - off as i32
    }
    else {
        y0 + off as i32
    }
}

#[inline(always)]
fn render_line(x0: u32, y0: i32, x1: u32, y1: i32, n: usize, v: &mut [f32]) {
    let dy = y1 - y0;
    let adx = (x1 - x0) as i32;

    let base = dy / adx;

    let mut y = y0;

    let sy = if dy < 0 { base - 1 } else { base + 1 };

    let ady = dy.abs() - base.abs() * adx;

    v[x0 as usize] = FLOOR1_INVERSE_DB_TABLE[y as usize];

    let mut err = 0;

    let x_begin = x0 as usize + 1;
    let x_end = min(n, x1 as usize);

    for v in v[x_begin..x_end].iter_mut() {
        err += ady;

        y += if err >= adx {
            err -= adx;
            sy
        }
        else {
            base
        };

        *v = FLOOR1_INVERSE_DB_TABLE[y as usize];
    }
}
