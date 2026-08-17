mod from_iter;
mod multi_arg;
#[cfg(feature = "experimental-overwritable")]
mod overwritable;
mod single_arg;
mod some;

struct IntoStrRef<'a>(&'a str);

impl<'a> From<IntoStrRef<'a>> for &'a str {
    fn from(val: IntoStrRef<'a>) -> Self {
        val.0
    }
}
