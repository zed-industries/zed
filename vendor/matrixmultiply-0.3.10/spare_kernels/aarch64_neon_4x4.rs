#[cfg(target_arch="aarch64")]
struct KernelArmNeon;

#[cfg(target_arch="aarch64")]
impl GemmKernel for KernelArmNeon {
    type Elem = T;

    type MRTy = U4;
    type NRTy = U4;

    #[inline(always)]
    fn align_to() -> usize { 16 }

    #[inline(always)]
    fn always_masked() -> bool { false }

    #[inline(always)]
    fn nc() -> usize { archparam::S_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::S_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::S_MC }

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T, rsc: isize, csc: isize) {
        kernel_target_arm_neon(k, alpha, a, b, beta, c, rsc, csc)
    }
}

// 4x4 neon kernel unrolled developed for apple silicon M1
#[cfg(target_arch="aarch64")]
#[target_feature(enable="neon")]
unsafe fn kernel_target_arm_neon(k: usize, alpha: T, a: *const T, b: *const T,
                                 beta: T, c: *mut T, rsc: isize, csc: isize)
{
    use core::arch::aarch64::*;
    const MR: usize = KernelArmNeon::MR;
    const NR: usize = KernelArmNeon::NR;

    let (mut a, mut b, rsc, csc) = if rsc == 1 { (b, a, csc, rsc) } else { (a, b, rsc, csc) };

    let mut ab = [vmovq_n_f32(0.); MR];
    let mut ab2 = [vmovq_n_f32(0.); MR];
    let mut ab3 = [vmovq_n_f32(0.); MR];
    let mut ab4 = [vmovq_n_f32(0.); MR];
    let use_fma = true;

    // Compute
    // ab_ij = a_i * b_j for all i, j
    macro_rules! ab_ij_equals_ai_bj {
        ($dest:ident, $av:expr, $bv:expr) => {
            if use_fma {
                $dest[0] = vfmaq_laneq_f32($dest[0], $bv, $av, 0);
                $dest[1] = vfmaq_laneq_f32($dest[1], $bv, $av, 1);
                $dest[2] = vfmaq_laneq_f32($dest[2], $bv, $av, 2);
                $dest[3] = vfmaq_laneq_f32($dest[3], $bv, $av, 3);
            } else {
                $dest[0] = vaddq_f32($dest[0], vmulq_laneq_f32($bv, $av, 0));
                $dest[1] = vaddq_f32($dest[1], vmulq_laneq_f32($bv, $av, 1));
                $dest[2] = vaddq_f32($dest[2], vmulq_laneq_f32($bv, $av, 2));
                $dest[3] = vaddq_f32($dest[3], vmulq_laneq_f32($bv, $av, 3));
            }
        }
    }

    const UNROLL_BY: usize = 4;

    for _ in 0..k / UNROLL_BY {
        let av = vld1q_f32(a);
        let bv = vld1q_f32(b);
        // eprintln!("a: {av:?}");
        // eprintln!("b: {bv:?}");

        // FMLA instruction
        // Cortex 7A: FMA has 7 cycles latency or 3 cycles when the dependency is on the accumulator
        // M1: Latency 3, throughput 0.25
        ab_ij_equals_ai_bj!(ab, av, bv);

        let av = vld1q_f32(a.add(4));
        let bv = vld1q_f32(b.add(4));

        ab_ij_equals_ai_bj!(ab2, av, bv);

        if UNROLL_BY > 2 {

        let av = vld1q_f32(a.add(8));
        let bv = vld1q_f32(b.add(8));

        ab_ij_equals_ai_bj!(ab3, av, bv);

        let av = vld1q_f32(a.add(12));
        let bv = vld1q_f32(b.add(12));

        ab_ij_equals_ai_bj!(ab4, av, bv);

        }

        a = a.offset(UNROLL_BY as isize * MR as isize);
        b = b.offset(UNROLL_BY as isize * NR as isize);
    }

    for _ in 0..k % UNROLL_BY {
        let av = vld1q_f32(a);
        let bv = vld1q_f32(b);

        ab_ij_equals_ai_bj!(ab, av, bv);

        a = a.offset(MR as isize);
        b = b.offset(NR as isize);
    }

    macro_rules! c {
        ($i:expr, $j:expr) => (c.offset(rsc * $i as isize + csc * $j as isize));
    }

    macro_rules! extract {
        ($v:expr, $imm:expr) => (
            f32::from_bits(vgetq_lane_u32(core::mem::transmute::<_, uint32x4_t>($v), $imm))
        )
    }

    // Combine accumulators and multiply by alpha
    loop4!(i, ab[i] = vaddq_f32(vaddq_f32(ab[i], ab2[i]), vaddq_f32(ab3[i], ab4[i])));
    loop4!(i, ab[i] = vmulq_n_f32(ab[i], alpha));

    if beta == 0. {
        // set C = α A B
        if csc == 1 {
            loop4!(i, vst1q_f32(c![i, 0], ab[i]));
        } else {
            loop4!(i, vst1q_lane_f32(c![i, 0], ab[i], 0));
            loop4!(i, vst1q_lane_f32(c![i, 1], ab[i], 1));
            loop4!(i, vst1q_lane_f32(c![i, 2], ab[i], 2));
            loop4!(i, vst1q_lane_f32(c![i, 3], ab[i], 3));
        }
    } else {
        // set C = α A B + beta C
        loop4!(i, *c![i, 0] = *c![i, 0] * beta + extract!(ab[i], 0));
        loop4!(i, *c![i, 1] = *c![i, 1] * beta + extract!(ab[i], 1));
        loop4!(i, *c![i, 2] = *c![i, 2] * beta + extract!(ab[i], 2));
        loop4!(i, *c![i, 3] = *c![i, 3] * beta + extract!(ab[i], 3));
    }
}

