pub enum SetEqError<T> {
    LeftMissing(T),
    RightMissing(T),
}

impl<T> SetEqError<T> {
    pub fn map<R, F: FnOnce(T) -> R>(self, update: F) -> SetEqError<R> {
        match self {
            SetEqError::LeftMissing(missing) => SetEqError::LeftMissing(update(missing)),
            SetEqError::RightMissing(missing) => SetEqError::RightMissing(update(missing)),
        }
    }
}

#[macro_export]
macro_rules! set_eq {
    ($left:expr,$right:expr) => {{
        use util::test::*;

        let left = $left;
        let right = $right;

        let mut result = Ok(());
        for right_value in right.iter() {
            if !left.contains(right_value) {
                result = Err(SetEqError::LeftMissing(right_value.clone()));
                break;
            }
        }

        if result.is_ok() {
            for left_value in left.iter() {
                if !right.contains(left_value) {
                    result = Err(SetEqError::RightMissing(left_value.clone()));
                }
            }
        }

        result
    }};
}

#[macro_export]
macro_rules! assert_set_eq {
    ($left:expr,$right:expr) => {{
        use util::test::*;
        use util::set_eq;

        let left = $left;
        let right = $right;

        match set_eq!(left, right) {
            Err(SetEqError::LeftMissing(missing)) => {
                panic!("assertion failed: `(left == right)`\n left: {:?}\nright: {:?}\nright does not contain {:?}", left, right, missing);
            },
            Err(SetEqError::RightMissing(missing)) => {
                panic!("assertion failed: `(left == right)`\n left: {:?}\nright: {:?}\nleft does not contain {:?}", left, right, missing);
            },
            _ => {}
        }
    }};
}
