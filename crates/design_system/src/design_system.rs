use std::marker::PhantomData;

mod components;

pub use components::*;

pub enum DesignSystem<Tag> {
    _PD(PhantomData<*const Tag>),
}
