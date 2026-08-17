#![cfg(feature = "proptest-support")]

use na::DMatrix;
use na::balancing;

use crate::proptest::*;
use proptest::{prop_assert_eq, proptest};

proptest! {
    #[test]
    fn balancing_parlett_reinsch(n in PROPTEST_MATRIX_DIM) {
        let m = DMatrix::<f64>::new_random(n, n);
        let mut balanced = m.clone();
        let d = balancing::balance_parlett_reinsch(&mut balanced);
        balancing::unbalance(&mut balanced, &d);

        prop_assert_eq!(balanced, m);
    }

    #[test]
    fn balancing_parlett_reinsch_static(m in matrix4()) {
        let mut balanced = m;
        let d = balancing::balance_parlett_reinsch(&mut balanced);
        balancing::unbalance(&mut balanced, &d);

        prop_assert_eq!(balanced, m);
    }
}
