#[macro_use]
mod action;

#[macro_use]
mod action_list;

#[macro_use]
mod state_body;

#[macro_use]
mod state;

#[macro_use]
mod arm_pattern;

macro_rules! define_state_group {
    ( $name:ident = { $($states:tt)+ } ) => {
        macro_rules! $name {
            () => {
                state!($($states)+);
            };
        }
    };
}
