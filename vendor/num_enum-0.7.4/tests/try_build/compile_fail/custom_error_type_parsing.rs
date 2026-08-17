#[derive(num_enum::TryFromPrimitive)]
#[num_enum(error_type(name = CustomError))]
#[repr(u8)]
enum MissingConstructor {
    Zero,
    One,
    Two,
}

#[derive(num_enum::TryFromPrimitive)]
#[num_enum(error_type(constructor = CustomError::new))]
#[repr(u8)]
enum MissingName {
    Zero,
    One,
    Two,
}

#[derive(num_enum::TryFromPrimitive)]
#[num_enum(error_type(name = CustomError, constructor = CustomError::new, extra = something))]
#[repr(u8)]
enum ExtraAttr {
    Zero,
    One,
    Two,
}

#[derive(num_enum::TryFromPrimitive)]
#[num_enum(error_type(name = CustomError, constructor = CustomError::new), error_type(name = CustomError, constructor = CustomError::new))]
#[repr(u8)]
enum TwoErrorTypes {
    Zero,
    One,
    Two,
}

#[derive(num_enum::TryFromPrimitive)]
#[num_enum(error_type(name = CustomError, constructor = CustomError::new))]
#[num_enum(error_type(name = CustomError, constructor = CustomError::new))]
#[repr(u8)]
enum TwoAttrs {
    Zero,
    One,
    Two,
}

struct CustomError {}

impl CustomError {
    fn new(_: u8) -> CustomError {
        CustomError{}
    }
}

fn main() {
}
