#[macro_export]
macro_rules! assert_set_eq {
    ($left:expr,$right:expr) => {{
        let left = $left;
        let right = $right;

        for left_value in left.iter() {
            if !right.contains(left_value) {
                panic!("assertion failed: `(left == right)`\n left: {:?}\nright: {:?}\nright does not contain {:?}", left, right, left_value);
            }
        }

        for right_value in right.iter() {
            if !left.contains(right_value) {
                panic!("assertion failed: `(left == right)`\n left: {:?}\nright: {:?}\nleft does not contain {:?}", left, right, right_value);
            }
        }
    }};
}
