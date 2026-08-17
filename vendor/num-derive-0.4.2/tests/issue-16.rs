macro_rules! get_an_isize {
    () => {
        0_isize
    };
}

#[derive(num_derive::FromPrimitive)]
pub enum CLikeEnum {
    VarA = get_an_isize!(),
    VarB = 2,
}
