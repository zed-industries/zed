use super::*;

impl Config<'_> {
    pub fn write_cpp_const_guid(&self, name: TokenStream, value: &GUID) -> TokenStream {
        let crate_name = self.write_core();
        let value = self.write_guid_u128(value);

        quote! {
            pub const #name: #crate_name GUID = #crate_name GUID::from_u128(#value);
        }
    }

    pub fn write_guid_u128(&self, value: &GUID) -> TokenStream {
        format!(
            "0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}",
            value.0,
            value.1,
            value.2,
            value.3,
            value.4,
            value.5,
            value.6,
            value.7,
            value.8,
            value.9,
            value.10
        )
        .into()
    }

    pub fn field_initializer<'a>(&self, field: Field, input: &'a str) -> (TokenStream, &'a str) {
        let name = to_ident(field.name());

        match field.ty(None) {
            Type::GUID => {
                let (literals, rest) = read_literal_array(input, 11);
                let value = self.write_guid_u128(&GUID(
                    literals[0].parse().unwrap(),
                    literals[1].parse().unwrap(),
                    literals[2].parse().unwrap(),
                    literals[3].parse().unwrap(),
                    literals[4].parse().unwrap(),
                    literals[5].parse().unwrap(),
                    literals[6].parse().unwrap(),
                    literals[7].parse().unwrap(),
                    literals[8].parse().unwrap(),
                    literals[9].parse().unwrap(),
                    literals[10].parse().unwrap(),
                ));
                let crate_name = self.write_core();
                (quote! { #name: #crate_name GUID::from_u128(#value), }, rest)
            }
            Type::ArrayFixed(_, len) => {
                let (literals, rest) = read_literal_array(input, len);
                let literals = literals.iter().map(|literal| TokenStream::from(*literal));
                (quote! { #name: [#(#literals,)*], }, rest)
            }
            Type::PtrMut(_, _) => {
                // The Win32 metadata uses integer values for initializing pointers. This is a workaround
                // to allow most such cases to work.
                let (_, rest) = read_literal(input);
                (quote! { #name: core::ptr::null_mut(), }, rest)
            }
            _ => {
                let (literal, rest) = read_literal(input);
                let literal: TokenStream = literal.into();
                (quote! { #name: #literal, }, rest)
            }
        }
    }
}

fn read_literal_array(input: &str, len: usize) -> (Vec<&str>, &str) {
    let mut input = read_token(input, b'{');
    let mut result = vec![];

    for _ in 0..len {
        let (literal, rest) = read_literal(input);
        result.push(literal);
        input = rest;
    }

    (result, read_token(input, b'}'))
}

fn read_literal(input: &str) -> (&str, &str) {
    let mut start = None;
    let mut end = 0;

    for (pos, c) in input.bytes().enumerate() {
        if start.is_none() {
            if c != b' ' && c != b',' {
                start = Some(pos);
            }
        } else if c == b' ' || c == b',' || c == b'}' {
            break;
        }
        end += 1;
    }

    let Some(start) = start else {
        panic!();
    };

    (&input[start..end], &input[end..])
}

fn read_token(input: &str, token: u8) -> &str {
    for (pos, c) in input.bytes().enumerate() {
        if c == token {
            return &input[pos + 1..];
        } else if c != b' ' && c != b',' {
            break;
        }
    }

    panic!("`{}` expected", token.escape_ascii());
}
