use crate::{Matrix2, RealField, SVD, U2, Vector2};

// Implementation of the 2D SVD from https://ieeexplore.ieee.org/document/486688
// See also https://scicomp.stackexchange.com/questions/8899/robust-algorithm-for-2-times-2-svd
pub fn svd_ordered2<T: RealField>(
    m: &Matrix2<T>,
    compute_u: bool,
    compute_v: bool,
) -> SVD<T, U2, U2> {
    let half: T = crate::convert(0.5);
    let one: T = crate::convert(1.0);

    let e = (m.m11.clone() + m.m22.clone()) * half.clone();
    let f = (m.m11.clone() - m.m22.clone()) * half.clone();
    let g = (m.m21.clone() + m.m12.clone()) * half.clone();
    let h = (m.m21.clone() - m.m12.clone()) * half.clone();
    let q = (e.clone() * e.clone() + h.clone() * h.clone()).sqrt();
    let r = (f.clone() * f.clone() + g.clone() * g.clone()).sqrt();

    // Note that the singular values are always sorted because sx >= sy
    // because q >= 0 and r >= 0.
    let sx = q.clone() + r.clone();
    let sy = q - r;
    let sy_sign = if sy < T::zero() { -one } else { one };
    let singular_values = Vector2::new(sx, sy * sy_sign.clone());

    if compute_u || compute_v {
        let a1 = g.atan2(f);
        let a2 = h.atan2(e);
        let theta = (a2.clone() - a1.clone()) * half.clone();
        let phi = (a2 + a1) * half;
        let (st, ct) = theta.sin_cos();
        let (sp, cp) = phi.sin_cos();

        let u = Matrix2::new(cp.clone(), -sp.clone(), sp, cp);
        let v_t = Matrix2::new(ct.clone(), -st.clone(), st * sy_sign.clone(), ct * sy_sign);

        SVD {
            u: if compute_u { Some(u) } else { None },
            singular_values,
            v_t: if compute_v { Some(v_t) } else { None },
        }
    } else {
        SVD {
            u: None,
            singular_values,
            v_t: None,
        }
    }
}
