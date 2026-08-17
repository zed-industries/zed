use vm_types::integer_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct time_value {
    pub seconds: integer_t,
    pub microseconds: integer_t,
}
pub type time_value_t = time_value;

pub const TIME_MICROS_MAX: integer_t = 1000000;
