// Experimental
// SuperMinHash – A New Minwise Hashing Algorithm for Jaccard Similarity Estimation
// https://arxiv.org/pdf/1706.05698.pdf

use std::cmp::min;
use std::hash::{BuildHasher, Hash, Hasher};
use fnv::FnvBuildHasher;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;
use crate::minhash::MinHasher;

pub struct SuperMinHasher32V1<B: BuildHasher> {
    build_hasher: B,
    num_hashes: usize,
}

impl SuperMinHasher32V1<FnvBuildHasher> {
    pub fn new(num_hashes: usize) -> Self {
        return SuperMinHasher32V1::new_with_hasher(num_hashes, FnvBuildHasher::default());
    }

}

impl<B: BuildHasher> SuperMinHasher32V1<B> {

    pub fn new_with_hasher(num_hashes: usize, build_hasher: B) -> Self {
        SuperMinHasher32V1 {
            build_hasher: build_hasher,
            num_hashes,
        }
    }
}

impl<B: BuildHasher> MinHasher for SuperMinHasher32V1<B> {
    type V = u32;
    fn create_signature<T, U>(&self, iter: T) -> Vec<u32>
        where
            T: Iterator<Item = U>,
            U: Hash,
    {
        let mut minhash = vec![99999999f32; self.num_hashes];
        for item in iter {
            let mut hasher = self.build_hasher.build_hasher();
            item.hash(&mut hasher);
            let h = hasher.finish();
            let mut rng = StdRng::seed_from_u64(h);
            let mut p: Vec<u32> = (0..(self.num_hashes) as u32).collect();
            p.shuffle(&mut rng);
            let rand_range = Uniform::from(0f32..1.0f32);
            for j in (0..self.num_hashes) {
                let r = rand_range.sample(&mut rng);
                let x = minhash[j].min(r + p[j] as f32);
                minhash[j] = x;
            }
        }
        minhash.into_iter().map(|h| h as u32).collect()
    }
}

pub struct SuperMinHash32V2<B: BuildHasher> {
    build_hasher: B,
    num_hashes: usize,
}

impl SuperMinHash32V2<FnvBuildHasher> {
    pub fn new(num_hashes: usize) -> Self {
        return SuperMinHash32V2::new_with_hasher(num_hashes, FnvBuildHasher::default());
    }
}

impl<B: BuildHasher> SuperMinHash32V2<B> {

    pub fn new_with_hasher(num_hashes: usize, build_hasher: B) -> Self {
        SuperMinHash32V2 {
            build_hasher: build_hasher,
            num_hashes,
        }
    }

}

impl<B: BuildHasher> MinHasher for SuperMinHash32V2<B> {
    type V = u32;
    fn create_signature<T, U>(&self, iter: T) -> Vec<u32>
        where
            T: Iterator<Item = U>,
            U: Hash,
    {
        let mut h = vec![99999999f32; self.num_hashes];
        let m = self.num_hashes;
        let mut a = m - 1;
        let unit_range = Uniform::<f32>::new(0., 1.);
        let mut q = vec![0; m];
        let mut p: Vec<usize> = vec![0; m];
        let mut b: Vec<isize> = vec![-1; m];
        b[m - 1] = m as isize;

        for item in iter.enumerate() {
            let mut hasher = self.build_hasher.build_hasher();
            item.1.hash(&mut hasher);
            let mut rng = StdRng::seed_from_u64(hasher.finish());
            let mut j: usize = 0;
            let i = item.0;
            while j <= a {
                let r: f32 = unit_range.sample(&mut rng);
                let k = Uniform::<usize>::new(j, m).sample(&mut rng);
                if q[j] != i {
                    q[j] = i;
                    p[j] = j;
                }
                if q[k] != i {
                    q[k] = i;
                    p[k] = k;
                }
                let tmp_swap = p[j];
                p[j] = p[k];
                p[k] = tmp_swap;
                let rpj = r + (j as f32);
                if rpj < h[p[j]] {
                    let j2 = min(h[p[j]] as usize, m - 1);
                    h[p[j]] = rpj;
                    if j < j2 {
                        b[j2] -= 1;
                        b[j] += 1;
                        while b[a] == 0 {
                            a -= 1;
                        }
                    }
                }
                j += 1;
            }
        }
        h.into_iter().map(|h| h as u32).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::minhash::{compute_jaccard_similarity, MinHasher};
    use crate::minhash::super_min_hash::{SuperMinHash32V2, SuperMinHasher32V1};
    use crate::text::whitespace_split;

    static S1: &'static str = "local sensitive hashing is cool";
    static S2: &'static str = "local sensitive hashing is great";
    static S3: &'static str = "local sensitive hashing is awesome";

    static S10: &'static str = "If you're still searching, we can visit a few open houses together in the next few weeks. It might help give clarity on what you're looking for. What do you think? - Gail's assistant w/eXp Realty";
    static S11: &'static str = "If you're still searching, we can visit a few open houses together in the next few weeks. It might help give clarity on what you're looking for. What do you think? - Elle's assistant w/Bright Birch Real Estate";


    fn test_min_hash<M: MinHasher>(min_hash: &M) {
        let similarity = min_hash.compute_similarity(whitespace_split(S10), whitespace_split(S11)) as f32;
        let actual_similarity = compute_jaccard_similarity(whitespace_split(S10), whitespace_split(S11));
        println!("actual {} estimated {} ", actual_similarity, similarity);
        assert!(f32::abs(similarity - 0.75) < 0.15);

        let estimated_similarity =
            min_hash.compute_similarity(whitespace_split(S1), whitespace_split(S3)) as f32;
        let actual_similarity = compute_jaccard_similarity(whitespace_split(S1), whitespace_split(S3));
        println!(
            "actual {} estimated {}",
            actual_similarity, estimated_similarity
        );
        assert!(f32::abs(estimated_similarity - actual_similarity) < 0.15);
    }

    #[test]
    fn test_super_min_hash_v1() {
        let min_hash = SuperMinHasher32V1::new(128);
        test_min_hash(&min_hash);
    }

    #[test]
    fn test_super_min_hash_v2() {
        let min_hash = SuperMinHash32V2::new(128);
        test_min_hash(&min_hash);
    }

}