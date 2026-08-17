use crate::CmpDisplay;
use core::fmt;

pub mod by_val;
pub mod debug;
pub mod sized;

impl<C, E: CmpDisplay<C, dyn fmt::Debug, dyn fmt::Debug>>
    CmpDisplay<
        by_val::CmpByValWrapper<sized::CmpSizedWrapper<debug::CmpDebugWrapper<C>>>,
        dyn fmt::Debug + 'static,
        dyn fmt::Debug + 'static,
    > for by_val::CmpByValWrapper<sized::CmpSizedWrapper<debug::CmpDebugWrapper<E>>>
{
    fn fmt(
        &self,
        cmp: &by_val::CmpByValWrapper<sized::CmpSizedWrapper<debug::CmpDebugWrapper<C>>>,
        lhs: &(dyn fmt::Debug + 'static),
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &(dyn fmt::Debug + 'static),
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        self.0 .0 .0.fmt(
            &cmp.0 .0 .0,
            lhs,
            lhs_source,
            lhs_debug,
            rhs,
            rhs_source,
            rhs_debug,
            f,
        )
    }
}

#[repr(transparent)]
pub struct Wrapper<T: ?Sized>(pub T);
