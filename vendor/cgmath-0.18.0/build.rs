use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::string::String;

/// Generate the name of the swizzle function and what it returns.
/// NOTE: This function assumes that variables are in ASCII format
#[cfg(feature = "swizzle")]
fn gen_swizzle_nth<'a>(variables: &'a str, mut i: usize, upto: usize) -> Option<(String, String)> {
    debug_assert!(i > 0); // zeroth permutation is empty
    let mut swizzle_impl = String::new();
    let mut swizzle = String::new();
    let n = variables.len() + 1;
    for _ in 0..upto {
        if i == 0 {
            break;
        }
        if i % n == 0 {
            return None;
        }
        let c = variables.as_bytes()[i % n - 1] as char;
        swizzle.push(c);
        swizzle_impl.push_str(&format!("self.{}, ", c));
        i = i / n;
    }
    Some((swizzle, swizzle_impl))
}

/// A function that generates swizzle functions as a string.
/// `variables`: swizzle variables (e.g. "xyz")
/// `upto`: largest output vector size (e.g. for `variables = "xy"` and `upto = 4`, `xyxy()` is a
/// valid swizzle operator.
/// NOTE: This function assumes that variables are in ASCII format
#[cfg(feature = "swizzle")]
fn gen_swizzle_functions(variables: &'static str, upto: usize) -> String {
    let mut result = String::new();
    let nn = (variables.len() + 1).pow(upto as u32);
    for i in 1..nn {
        if let Some((swizzle_name, swizzle_impl)) = gen_swizzle_nth(variables, i, upto) {
            let dim = format!("{}", swizzle_name.len());
            result.push_str(&format!(
                "
        /// Swizzle operator that creates a new type with dimension {2} from variables `{0}`.
        #[inline] pub fn {0}(&self) -> $vector_type{2}<$S> {{ $vector_type{2}::new({1}) }}\n",
                swizzle_name, swizzle_impl, dim
            ));
        }
    }
    result
}

#[cfg(not(feature = "swizzle"))]
fn gen_swizzle_functions(_: &'static str, _: usize) -> String {
    String::new()
}

/// This script generates the macro for building swizzle operators for multidimensional
/// vectors and points. This macro is included in macros.rs
fn main() {
    // save the file to output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let swizzle_file_path = Path::new(&out_dir).join("swizzle_operator_macro.rs");

    // This is the string representing the generated macro
    let data = format!(
"/// Generate glm/glsl style swizzle operators
macro_rules! impl_swizzle_functions {{
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $S:ident, x) => {{
{x3}
    }};
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $S:ident, xy) => {{
{xy3}
    }};
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $S:ident, xyz) => {{
{xyz3}
    }};
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, x) => {{
{x4}
    }};
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, xy) => {{
{xy4}
    }};
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, xyz) => {{
{xyz4}
    }};
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, xyzw) => {{
{xyzw4}
    }};
}}", x3 = gen_swizzle_functions("x", 3),
     xy3 = gen_swizzle_functions("xy", 3),
     xyz3 = gen_swizzle_functions("xyz", 3),
     x4 = gen_swizzle_functions("x", 4),
     xy4 = gen_swizzle_functions("xy", 4),
     xyz4 = gen_swizzle_functions("xyz", 4),
     xyzw4 = gen_swizzle_functions("xyzw", 4));
    let mut f = File::create(swizzle_file_path)
        .expect("Unable to create file that defines the swizzle operator macro.");
    f.write_all(data.as_bytes())
        .expect("Unable to write swizzle operator macro.");
}
