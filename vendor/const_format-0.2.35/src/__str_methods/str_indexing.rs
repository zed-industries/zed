pub struct StrIndexArgsConv<T> {
    pub str: &'static str,
    pub arg: T,
}

#[allow(non_snake_case)]
pub const fn StrIndexArgsConv<T>(str: &'static str, arg: T) -> StrIndexArgsConv<T> {
    StrIndexArgsConv { str, arg }
}

pub struct StrIndexArgs {
    pub str: &'static str,
    pub index_validity: IndexValidity,
    pub used_rstart: usize,
    pub used_rlen: usize,
    pub used_rend: usize,
}

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum IndexValidity {
    Valid,
    StartOob(usize),
    StartInsideChar(usize),
    EndOob(usize),
    EndInsideChar(usize),
}

impl IndexValidity {
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }

    pub const fn assert_valid(self) {
        match self {
            Self::Valid => (),
            Self::StartOob(index) => [/*start index is out of bounds*/][index],
            Self::StartInsideChar(index) => [/*start index is not on a char boundary*/][index],
            Self::EndOob(index) => [/*end index is out of bounds*/][index],
            Self::EndInsideChar(index) => [/*end index is not on a char boundary*/][index],
        }
    }
}

macro_rules! pass_range_types {
    ($macro:ident) => {
        const _: () = {
            use core::ops;

            #[allow(unused_imports)]
            use crate::__hidden_utils::{is_char_boundary_no_len_check, max_usize, saturating_add};

            $macro! {
                fn(self, usize) {
                    let mut end = saturating_add(self.arg, 1);
                    let bytes = self.str.as_bytes();

                    if end < self.str.len() {
                        while !is_char_boundary_no_len_check(bytes, end) {
                            end = saturating_add(end, 1);
                        }
                    }

                    self.arg .. end
                }

                fn(self, ops::Range<usize>) {
                    let ops::Range{start, end} = self.arg;
                    start .. max_usize(start, end)
                }

                fn(self, ops::RangeTo<usize>) {
                    0..self.arg.end
                }

                fn(self, ops::RangeFrom<usize>) {
                    self.arg.start..self.str.len()
                }

                fn(self, ops::RangeInclusive<usize>) {
                    let start = *self.arg.start();
                    start .. max_usize(saturating_add(*self.arg.end(), 1), start)
                }

                fn(self, ops::RangeToInclusive<usize>) {
                    0 .. saturating_add(self.arg.end, 1)
                }

                fn(self, ops::RangeFull) {
                    0 .. self.str.len()
                }
            }
        };
    };
}
pub(super) use pass_range_types;

macro_rules! define_conversions {
    (
        $( fn($self:ident, $ty:ty) $block:block )*
    ) => {

        $(
            impl StrIndexArgsConv<$ty> {
                pub const fn conv($self) -> StrIndexArgs {
                    use crate::__hidden_utils::is_char_boundary_no_len_check;

                    let range = $block;

                    let str_len = $self.str.len();

                    let mut used_rstart = 0;
                    let mut used_rend = str_len;

                    let mut index_validity = IndexValidity::Valid;
                    let bytes = $self.str.as_bytes();

                    if range.end > str_len {
                        index_validity = IndexValidity::EndOob(range.end);
                    } else if is_char_boundary_no_len_check(bytes, range.end) {
                        used_rend = range.end;
                    } else {
                        index_validity = IndexValidity::EndInsideChar(range.end);
                    };

                    if range.start > str_len {
                        index_validity = IndexValidity::StartOob(range.start);
                    } else if is_char_boundary_no_len_check(bytes, range.start) {
                        used_rstart = range.start;
                    } else {
                        index_validity = IndexValidity::StartInsideChar(range.start);
                    };

                    StrIndexArgs {
                        str: $self.str,
                        index_validity,
                        used_rstart,
                        used_rend,
                        used_rlen: used_rend - used_rstart,
                    }
                }
            }
        )*
    };
}

pass_range_types! {define_conversions}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_validity_test() {
        macro_rules! miv {
            ($str:expr, $range:expr) => {
                StrIndexArgsConv($str, $range).conv().index_validity
            };
        }

        assert_eq!(miv!("効率的", 3), IndexValidity::Valid);
        assert_eq!(miv!("効率的", 6), IndexValidity::Valid);
        assert_eq!(miv!("効率的", 3..6), IndexValidity::Valid);

        assert_eq!(miv!("効率的", 4..6), IndexValidity::StartInsideChar(4));
        assert_eq!(miv!("効率的", 3..5), IndexValidity::EndInsideChar(5));
        assert_eq!(miv!("効率的", 7..9), IndexValidity::StartInsideChar(7));

        assert_eq!(miv!("効率的", 100..9), IndexValidity::StartOob(100));
        assert_eq!(miv!("効率的", 3..10), IndexValidity::EndOob(10));
        assert_eq!(miv!("効率的", 9), IndexValidity::EndOob(10));
        assert_eq!(miv!("効率的", 10), IndexValidity::StartOob(10));
        assert_eq!(miv!("効率的", 100..900), IndexValidity::StartOob(100));
    }
}
