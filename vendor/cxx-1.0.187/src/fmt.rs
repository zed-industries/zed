use core::fmt::{self, Display};

pub(crate) fn display(fmt: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl Display {
    DisplayInvoke(fmt)
}

struct DisplayInvoke<T>(T);

impl<T> Display for DisplayInvoke<T>
where
    T: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(formatter)
    }
}
