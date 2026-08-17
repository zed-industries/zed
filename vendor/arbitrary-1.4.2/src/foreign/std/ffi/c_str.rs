// impl Arbitrary for Box<CStr> {
//     fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
//         <CString as Arbitrary>::arbitrary(u).map(|x| x.into_boxed_c_str())
//     }
// }
