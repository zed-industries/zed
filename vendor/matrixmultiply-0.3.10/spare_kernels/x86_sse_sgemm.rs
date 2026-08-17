
// 4x4 sse sgemm
macro_rules! mm_transpose4 {
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr) => {{
        // This is _MM_TRANSPOSE4_PS except we take variables, not references
        let tmp0 = _mm_unpacklo_ps($c0, $c1);
        let tmp2 = _mm_unpacklo_ps($c2, $c3);
        let tmp1 = _mm_unpackhi_ps($c0, $c1);
        let tmp3 = _mm_unpackhi_ps($c2, $c3);

        $c0 = _mm_movelh_ps(tmp0, tmp2);
        $c1 = _mm_movehl_ps(tmp2, tmp0);
        $c2 = _mm_movelh_ps(tmp1, tmp3);
        $c3 = _mm_movehl_ps(tmp3, tmp1);
    }}
}

#[inline(always)]
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
unsafe fn kernel_x86_sse(k: usize, alpha: T, a: *const T, b: *const T,
                         beta: T, c: *mut T, rsc: isize, csc: isize)
{
    let mut ab = [_mm_setzero_ps(); MR];

    let mut bv;
    let (mut a, mut b) = (a, b);

    // Compute A B
    for _ in 0..k {
        bv = _mm_load_ps(b as _); // aligned due to GemmKernel::align_to

        loop_m!(i, {
            // Compute ab_i += [ai b_j+0, ai b_j+1, ai b_j+2, ai b_j+3]
            let aiv = _mm_set1_ps(at(a, i));
            ab[i] = _mm_add_ps(ab[i], _mm_mul_ps(aiv, bv));
        });

        a = a.add(MR);
        b = b.add(NR);
    }

    // Compute α (A B)
    let alphav = _mm_set1_ps(alpha);
    loop_m!(i, ab[i] = _mm_mul_ps(alphav, ab[i]));

    macro_rules! c {
        ($i:expr, $j:expr) => (c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // C ← α A B + β C
    let mut c = [_mm_setzero_ps(); MR];
    let betav = _mm_set1_ps(beta);
    if beta != 0. {
        // Read C
        if csc == 1 {
            loop_m!(i, c[i] = _mm_loadu_ps(c![i, 0]));
        } else if rsc == 1 {
            loop_m!(i, c[i] = _mm_loadu_ps(c![0, i]));
            mm_transpose4!(c[0], c[1], c[2], c[3]);
        } else {
            loop_m!(i, c[i] = _mm_set_ps(*c![i, 3], *c![i, 2], *c![i, 1], *c![i, 0]));
        }
        // Compute β C
        loop_m!(i, c[i] = _mm_mul_ps(c[i], betav));
    }

    // Compute (α A B) + (β C)
    loop_m!(i, c[i] = _mm_add_ps(c[i], ab[i]));

    // Store C back to memory
    if csc == 1 {
        loop_m!(i, _mm_storeu_ps(c![i, 0], c[i]));
    } else if rsc == 1 {
        mm_transpose4!(c[0], c[1], c[2], c[3]);
        loop_m!(i, _mm_storeu_ps(c![0, i], c[i]));
    } else {
        // extract the nth value of a vector using _mm_cvtss_f32 (extract lowest)
        // in combination with shuffle (move nth value to first position)
        loop_m!(i, *c![i, 0] = _mm_cvtss_f32(c[i]));
        loop_m!(i, *c![i, 1] = _mm_cvtss_f32(_mm_shuffle_ps(c[i], c[i], 1)));
        loop_m!(i, *c![i, 2] = _mm_cvtss_f32(_mm_shuffle_ps(c[i], c[i], 2)));
        loop_m!(i, *c![i, 3] = _mm_cvtss_f32(_mm_shuffle_ps(c[i], c[i], 3)));
    }
}
