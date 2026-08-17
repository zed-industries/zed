#[doc(hidden)]
pub trait RuntimeName {
    // TODO: needs to use ConstBuffer like RuntimeType to allow generic interfaces to have names for GetRuntimeClassName
    const NAME: &'static str = "";
}
