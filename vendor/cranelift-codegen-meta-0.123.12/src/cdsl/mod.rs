//! Cranelift DSL classes.
//!
//! This module defines the classes that are used to define Cranelift
//! instructions and other entities.

pub mod formats;
pub mod instructions;
pub mod isa;
pub mod operands;
pub mod settings;
pub mod types;
pub mod typevar;

/// A macro that converts boolean settings into predicates to look more natural.
#[macro_export]
macro_rules! predicate {
    ($a:ident && $($b:tt)*) => {
        PredicateNode::And(Box::new($a.into()), Box::new(predicate!($($b)*)))
    };
    ($a:ident) => {
        $a.into()
    };
}

/// A macro that joins boolean settings into a list (e.g. `preset!(feature_a && feature_b)`).
#[macro_export]
macro_rules! preset {
    () => {
        vec![]
    };
    ($($x:tt)&&*) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x.into());
            )*
            v
        }
    };
}

/// Convert the string `s` to CamelCase.
pub fn camel_case(s: &str) -> String {
    let mut output_chars = String::with_capacity(s.len());

    let mut capitalize = true;
    for curr_char in s.chars() {
        if curr_char == '_' {
            capitalize = true;
        } else {
            if capitalize {
                output_chars.extend(curr_char.to_uppercase());
            } else {
                output_chars.push(curr_char);
            }
            capitalize = false;
        }
    }

    output_chars
}

#[cfg(test)]
mod tests {
    use super::camel_case;

    #[test]
    fn camel_case_works() {
        assert_eq!(camel_case("x"), "X");
        assert_eq!(camel_case("camel_case"), "CamelCase");
    }
}
