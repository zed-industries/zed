use cxx::{CxxString, UniquePtr};

#[test]
#[should_panic = "called deref on a null UniquePtr<CxxString>"]
fn test_deref_null() {
    let unique_ptr = UniquePtr::<CxxString>::null();
    let _: &CxxString = &unique_ptr;
}
