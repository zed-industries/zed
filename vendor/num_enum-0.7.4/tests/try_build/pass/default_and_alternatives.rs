#[derive(num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Default {
    #[num_enum(default)]
    Foo = 0,
    Bar = 1,
}

#[derive(num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Alternatives {
    #[num_enum(alternatives = [])]
    Foo = 0,
    #[num_enum(alternatives = [3])]
    Bar = 1,
    #[num_enum(alternatives = [4, 5])]
    Baz = 2,
    #[num_enum(alternatives = [7])]
    #[num_enum(alternatives = [8])]
    Blee = 6,
}

#[derive(num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Both {
    #[num_enum(default)]
    Foo = 0,
    #[num_enum(alternatives = [3])]
    Bar = 1,
}

mod mixed {
    #[derive(num_enum::TryFromPrimitive)]
    #[repr(u8)]
    enum AlternativesFollowedByDefaultInSingleAttribute {
        #[num_enum(alternatives = [1, 2], default)]
        Foo = 0,
    }

    #[derive(num_enum::TryFromPrimitive)]
    #[repr(u8)]
    enum DefaultFollowedByAlternativesInSingleAttribute {
        #[num_enum(default, alternatives = [1, 2])]
        Foo = 0,
    }

    #[derive(num_enum::TryFromPrimitive)]
    #[repr(u8)]
    enum AlternativesFollowedByDefaultInMultipleAttributes {
        #[num_enum(alternatives = [1, 2])]
        #[num_enum(default)]
        Foo = 0,
    }

    #[derive(num_enum::TryFromPrimitive)]
    #[repr(u8)]
    enum DefaultFollowedByAlternativesInMultipleAttributes {
        #[num_enum(default)]
        #[num_enum(alternatives = [1, 2])]
        Foo = 0,
    }
}

fn main() {

}
