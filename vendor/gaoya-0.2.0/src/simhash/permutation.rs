use crate::simhash::SimHashBits;
use core::mem;
use itertools::Itertools;
use std::fmt::Debug;
use std::ops::BitOrAssign;

#[derive(Debug)]
pub struct Permutation<S>
where
    S: SimHashBits,
{
    offsets_masks: Vec<(S, isize)>,
    search_mask: S,
    pub simple_mask: S,
    pub width: usize,
}

impl<S> Permutation<S>
where
    S: SimHashBits,
{
    pub fn new(diff_bits: usize, masks: Vec<S>, choice: Vec<S>) -> Self {
        let len = masks.len();
        let mut p = Permutation {
            offsets_masks: Vec::new(),
            search_mask: S::zero(),
            simple_mask: S::zero(),
            width: 0,
        };

        let type_width = S::bit_length();
        let mut width: usize = 0;
        let mut widths: Vec<usize> = Vec::new();
        let mut offsets: Vec<isize> = vec![0; len];

        for mask_index in 0..len {
            let mask = masks[mask_index];

            let mut i = 0;
            let mut j;

            while (S::one() << i) & mask == S::zero() {
                i += 1;
            }

            j = i;
            while j < type_width && ((S::one() << j) & mask != S::zero()) {
                j += 1;
            }

            width += (j - i);
            widths.push(j - i);

            let offset: isize = type_width as isize - width as isize - i as isize;
            offsets[mask_index] = offset;
        }

        //println!("{:?}", widths);

        width = 0;
        for i in (0..widths.len() - diff_bits) {
            width += widths[i];
            p.offsets_masks.push((masks[i], offsets[i]))
        }
        p.width = width;
        for _i in 0..width {
            p.search_mask = (p.search_mask << 1) | S::one();
        }
        for _i in width..type_width {
            p.search_mask = p.search_mask << 1;
        }
        for mask in choice {
            p.simple_mask |= mask;
        }
        p
    }

    pub fn create(num_blocks: usize, diff_bits: usize) -> Vec<Permutation<S>>
    where
        S: SimHashBits,
    {
        let mut permutations = Vec::new();
        let mut blocks = Vec::new();

        for i in 0..num_blocks {
            let mut mask: S = S::zero();
            let start = (i * S::bit_length()) / num_blocks;
            let end = ((i + 1) * S::bit_length()) / num_blocks;
            for j in start..end {
                mask |= S::one() << j;
            }
            blocks.push(mask);
        }
        let count = num_blocks - diff_bits;

        for mut combination in blocks.clone().into_iter().combinations(count) {
            let mut all_blocks = combination.clone();
            for v in &blocks {
                if !all_blocks.contains(&v) {
                    all_blocks.push(*v);
                }
            }
            permutations.push(Permutation::new(diff_bits, all_blocks, combination));
        }
        permutations
    }

    pub fn permute(&self, sim_hash: &S) -> S {
        let mut result: S = S::zero();

        for pair in &self.offsets_masks {
            //println!("{:#0130b}", result.to_u128().unwrap());
            let offset = pair.1;
            let mask = pair.0;
            if offset > 0 {
                result |= (*sim_hash & mask) << offset as usize;
            } else {
                result |= (*sim_hash & mask) >> (0 - offset) as usize;
            }
        }
        result & self.search_mask
    }
}

#[cfg(test)]
mod tests {
    use super::Permutation;

    #[test]
    pub fn test_permutations() {
        let permutations = Permutation::<u64>::create(6, 3);
        println!("{}", permutations.len());
        let u = 709743298743209799u64;
        let p = &permutations[0];
        println!("{:b} {}", p.search_mask, p.width);

        for p in &permutations {
            println!("{:b} {}", p.search_mask, p.width);
        }
    }

    #[test]
    pub fn test_permutations_128() {
        let permutations = Permutation::<u128>::create(12, 9);
        println!("{}", permutations.len());
        let u = 709743298743202973247987324239799u128;

        println!("");
        let p = &permutations[2];
        println!("{:?}", p);

        println!("");
        let p = &permutations[1];
        println!("{:?}", p);
        println!("{:b}", p.search_mask);
        println!("{:b}", p.permute(&u));
    }
}
