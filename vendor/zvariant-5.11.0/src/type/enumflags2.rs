use crate::{Signature, Type};

// BitFlags
impl<F> Type for enumflags2::BitFlags<F>
where
    F: Type + enumflags2::BitFlag,
{
    const SIGNATURE: &'static Signature = F::SIGNATURE;
}
