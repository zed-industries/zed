use alloc::{
    borrow::Cow,
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    arena::Handle,
    proc::{keyword_set::CaseInsensitiveKeywordSet, KeywordSet},
    FastHashMap,
};

pub type EntryPointIndex = u16;
const SEPARATOR: char = '_';

/// A component of a lowered external texture.
///
/// Whereas the WGSL backend implements [`ImageClass::External`]
/// images directly, most other Naga backends lower them to a
/// collection of ordinary textures that represent individual planes
/// (as received from a video decoder, perhaps), together with a
/// struct of parameters saying how they should be cropped, sampled,
/// and color-converted.
///
/// This lowering means that individual globals and function
/// parameters in Naga IR must be split out by the backends into
/// collections of globals and parameters of simpler types.
///
/// A value of this enum serves as a name key for one specific
/// component in the lowered representation of an external texture.
/// That is, these keys are for variables/parameters that do not exist
/// in the Naga IR, only in its lowered form.
///
/// [`ImageClass::External`]: crate::ir::ImageClass::External
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExternalTextureNameKey {
    Plane(usize),
    Params,
}

impl ExternalTextureNameKey {
    const ALL: &[(&str, ExternalTextureNameKey)] = &[
        ("_plane0", ExternalTextureNameKey::Plane(0)),
        ("_plane1", ExternalTextureNameKey::Plane(1)),
        ("_plane2", ExternalTextureNameKey::Plane(2)),
        ("_params", ExternalTextureNameKey::Params),
    ];
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum NameKey {
    Constant(Handle<crate::Constant>),
    Override(Handle<crate::Override>),
    GlobalVariable(Handle<crate::GlobalVariable>),
    Type(Handle<crate::Type>),
    StructMember(Handle<crate::Type>, u32),
    Function(Handle<crate::Function>),
    FunctionArgument(Handle<crate::Function>, u32),
    FunctionLocal(Handle<crate::Function>, Handle<crate::LocalVariable>),

    /// A local variable used by ReadZeroSkipWrite bounds-check policy
    /// when it needs to produce a pointer-typed result for an OOB access.
    /// These are unique per accessed type, so the second element is a
    /// type handle. See docs for [`crate::back::msl`].
    FunctionOobLocal(Handle<crate::Function>, Handle<crate::Type>),

    EntryPoint(EntryPointIndex),
    EntryPointLocal(EntryPointIndex, Handle<crate::LocalVariable>),
    EntryPointArgument(EntryPointIndex, u32),

    /// Entry point version of `FunctionOobLocal`.
    EntryPointOobLocal(EntryPointIndex, Handle<crate::Type>),

    /// A global variable holding a component of a lowered external texture.
    ///
    /// See [`ExternalTextureNameKey`] for details.
    ExternalTextureGlobalVariable(Handle<crate::GlobalVariable>, ExternalTextureNameKey),

    /// A function argument holding a component of a lowered external
    /// texture.
    ///
    /// See [`ExternalTextureNameKey`] for details.
    ExternalTextureFunctionArgument(Handle<crate::Function>, u32, ExternalTextureNameKey),
}

/// This processor assigns names to all the things in a module
/// that may need identifiers in a textual backend.
#[derive(Default)]
pub struct Namer {
    /// The last numeric suffix used for each base name. Zero means "no suffix".
    unique: FastHashMap<String, u32>,
    keywords: &'static KeywordSet,
    builtin_identifiers: &'static KeywordSet,
    keywords_case_insensitive: &'static CaseInsensitiveKeywordSet,
    reserved_prefixes: Vec<&'static str>,
}

impl Namer {
    /// Return a form of `string` suitable for use as the base of an identifier.
    ///
    /// - Drop leading digits.
    /// - Retain only alphanumeric and `_` characters.
    /// - Avoid prefixes in [`Namer::reserved_prefixes`].
    /// - Replace consecutive `_` characters with a single `_` character.
    ///
    /// The return value is a valid identifier prefix in all of Naga's output languages,
    /// and it never ends with a `SEPARATOR` character.
    /// It is used as a key into the unique table.
    fn sanitize<'s>(&self, string: &'s str) -> Cow<'s, str> {
        let string = string
            .trim_start_matches(|c: char| c.is_numeric())
            .trim_end_matches(SEPARATOR);

        let base = if !string.is_empty()
            && !string.contains("__")
            && string
                .chars()
                .all(|c: char| c.is_ascii_alphanumeric() || c == '_')
        {
            Cow::Borrowed(string)
        } else {
            let mut filtered = string.chars().fold(String::new(), |mut s, c| {
                let c = match c {
                    // Make several common characters in C++-ish types become snake case
                    // separators.
                    ':' | '<' | '>' | ',' => '_',
                    c => c,
                };
                let had_underscore_at_end = s.ends_with('_');
                if had_underscore_at_end && c == '_' {
                    return s;
                }
                if c.is_ascii_alphanumeric() || c == '_' {
                    s.push(c);
                } else {
                    use core::fmt::Write as _;
                    if !s.is_empty() && !had_underscore_at_end {
                        s.push('_');
                    }
                    write!(s, "u{:04x}_", c as u32).unwrap();
                }
                s
            });
            let stripped_len = filtered.trim_end_matches(SEPARATOR).len();
            filtered.truncate(stripped_len);
            if filtered.is_empty() {
                filtered.push_str("unnamed");
            } else if filtered.starts_with(|c: char| c.is_ascii_digit()) {
                unreachable!(
                    "internal error: invalid identifier starting with ASCII digit {:?}",
                    filtered.chars().nth(0)
                )
            }
            Cow::Owned(filtered)
        };

        for prefix in &self.reserved_prefixes {
            if base.starts_with(prefix) {
                return format!("gen_{base}").into();
            }
        }

        base
    }

    /// Return a new identifier based on `label_raw`.
    ///
    /// The result:
    /// - is a valid identifier even if `label_raw` is not
    /// - conflicts with no keywords listed in `Namer::keywords`, and
    /// - is different from any identifier previously constructed by this
    ///   `Namer`.
    ///
    /// Guarantee uniqueness by applying a numeric suffix when necessary. If `label_raw`
    /// itself ends with digits, separate them from the suffix with an underscore.
    pub fn call(&mut self, label_raw: &str) -> String {
        use core::fmt::Write as _; // for write!-ing to Strings

        let base = self.sanitize(label_raw);
        debug_assert!(!base.is_empty() && !base.ends_with(SEPARATOR));

        // This would seem to be a natural place to use `HashMap::entry`. However, `entry`
        // requires an owned key, and we'd like to avoid heap-allocating strings we're
        // just going to throw away. The approach below double-hashes only when we create
        // a new entry, in which case the heap allocation of the owned key was more
        // expensive anyway.
        match self.unique.get_mut(base.as_ref()) {
            Some(count) => {
                *count += 1;
                // Add the suffix. This may fit in base's existing allocation.
                let mut suffixed = base.into_owned();
                write!(suffixed, "{}{}", SEPARATOR, *count).unwrap();
                suffixed
            }
            None => {
                let mut suffixed = base.to_string();
                if base.ends_with(char::is_numeric)
                    || self.keywords.contains(base.as_ref())
                    || self.keywords_case_insensitive.contains(base.as_ref())
                    || self.builtin_identifiers.contains(base.as_ref())
                {
                    suffixed.push(SEPARATOR);
                }
                debug_assert!(!self.keywords.contains(&suffixed));
                // `self.unique` wants to own its keys. This allocates only if we haven't
                // already done so earlier.
                self.unique.insert(base.into_owned(), 0);
                suffixed
            }
        }
    }

    pub fn call_or(&mut self, label: &Option<String>, fallback: &str) -> String {
        self.call(match *label {
            Some(ref name) => name,
            None => fallback,
        })
    }

    /// Enter a local namespace for things like structs.
    ///
    /// Struct member names only need to be unique amongst themselves, not
    /// globally. This function temporarily establishes a fresh, empty naming
    /// context for the duration of the call to `body`.
    fn namespace(&mut self, capacity: usize, body: impl FnOnce(&mut Self)) {
        let empty_unique = FastHashMap::with_capacity_and_hasher(capacity, Default::default());
        let saved_unique = core::mem::replace(&mut self.unique, empty_unique);
        let saved_builtin_identifiers = core::mem::take(&mut self.builtin_identifiers);
        body(self);
        self.unique = saved_unique;
        self.builtin_identifiers = saved_builtin_identifiers;
    }

    pub fn reset(
        &mut self,
        module: &crate::Module,
        reserved_keywords: &'static KeywordSet,
        builtin_identifiers: &'static KeywordSet,
        reserved_keywords_case_insensitive: &'static CaseInsensitiveKeywordSet,
        reserved_prefixes: &[&'static str],
        output: &mut FastHashMap<NameKey, String>,
    ) {
        self.reserved_prefixes.clear();
        self.reserved_prefixes.extend(reserved_prefixes.iter());

        self.unique.clear();
        self.keywords = reserved_keywords;
        self.builtin_identifiers = builtin_identifiers;
        self.keywords_case_insensitive = reserved_keywords_case_insensitive;

        // Choose fallback names for anonymous entry point return types.
        let mut entrypoint_type_fallbacks = FastHashMap::default();
        for ep in &module.entry_points {
            if let Some(ref result) = ep.function.result {
                if let crate::Type {
                    name: None,
                    inner: crate::TypeInner::Struct { .. },
                } = module.types[result.ty]
                {
                    let label = match ep.stage {
                        crate::ShaderStage::Vertex => "VertexOutput",
                        crate::ShaderStage::Fragment => "FragmentOutput",
                        crate::ShaderStage::Compute => "ComputeOutput",
                        crate::ShaderStage::Task
                        | crate::ShaderStage::Mesh
                        | crate::ShaderStage::RayGeneration
                        | crate::ShaderStage::ClosestHit
                        | crate::ShaderStage::AnyHit
                        | crate::ShaderStage::Miss => unreachable!(),
                    };
                    entrypoint_type_fallbacks.insert(result.ty, label);
                }
            }
        }

        let mut temp = String::new();

        for (ty_handle, ty) in module.types.iter() {
            // If the type is anonymous, check `entrypoint_types` for
            // something better than just `"type"`.
            let raw_label = match ty.name {
                Some(ref given_name) => given_name.as_str(),
                None => entrypoint_type_fallbacks
                    .get(&ty_handle)
                    .cloned()
                    .unwrap_or("type"),
            };
            let ty_name = self.call(raw_label);
            output.insert(NameKey::Type(ty_handle), ty_name);

            if let crate::TypeInner::Struct { ref members, .. } = ty.inner {
                // struct members have their own namespace, because access is always prefixed
                self.namespace(members.len(), |namer| {
                    for (index, member) in members.iter().enumerate() {
                        let name = namer.call_or(&member.name, "member");
                        output.insert(NameKey::StructMember(ty_handle, index as u32), name);
                    }
                })
            }
        }

        for (ep_index, ep) in module.entry_points.iter().enumerate() {
            let ep_name = self.call(&ep.name);
            output.insert(NameKey::EntryPoint(ep_index as _), ep_name);
            for (index, arg) in ep.function.arguments.iter().enumerate() {
                let name = self.call_or(&arg.name, "param");
                output.insert(
                    NameKey::EntryPointArgument(ep_index as _, index as u32),
                    name,
                );
            }
            for (handle, var) in ep.function.local_variables.iter() {
                let name = self.call_or(&var.name, "local");
                output.insert(NameKey::EntryPointLocal(ep_index as _, handle), name);
            }
        }

        for (fun_handle, fun) in module.functions.iter() {
            let fun_name = self.call_or(&fun.name, "function");
            output.insert(NameKey::Function(fun_handle), fun_name);
            for (index, arg) in fun.arguments.iter().enumerate() {
                let name = self.call_or(&arg.name, "param");
                output.insert(NameKey::FunctionArgument(fun_handle, index as u32), name);

                if matches!(
                    module.types[arg.ty].inner,
                    crate::TypeInner::Image {
                        class: crate::ImageClass::External,
                        ..
                    }
                ) {
                    let base = arg.name.as_deref().unwrap_or("param");
                    for &(suffix, ext_key) in ExternalTextureNameKey::ALL {
                        let name = self.call(&format!("{base}_{suffix}"));
                        output.insert(
                            NameKey::ExternalTextureFunctionArgument(
                                fun_handle,
                                index as u32,
                                ext_key,
                            ),
                            name,
                        );
                    }
                }
            }
            for (handle, var) in fun.local_variables.iter() {
                let name = self.call_or(&var.name, "local");
                output.insert(NameKey::FunctionLocal(fun_handle, handle), name);
            }
        }

        for (handle, var) in module.global_variables.iter() {
            let name = self.call_or(&var.name, "global");
            output.insert(NameKey::GlobalVariable(handle), name);

            if matches!(
                module.types[var.ty].inner,
                crate::TypeInner::Image {
                    class: crate::ImageClass::External,
                    ..
                }
            ) {
                let base = var.name.as_deref().unwrap_or("global");
                for &(suffix, ext_key) in ExternalTextureNameKey::ALL {
                    let name = self.call(&format!("{base}_{suffix}"));
                    output.insert(
                        NameKey::ExternalTextureGlobalVariable(handle, ext_key),
                        name,
                    );
                }
            }
        }

        for (handle, constant) in module.constants.iter() {
            let label = match constant.name {
                Some(ref name) => name,
                None => {
                    use core::fmt::Write;
                    // Try to be more descriptive about the constant values
                    temp.clear();
                    write!(temp, "const_{}", output[&NameKey::Type(constant.ty)]).unwrap();
                    &temp
                }
            };
            let name = self.call(label);
            output.insert(NameKey::Constant(handle), name);
        }

        for (handle, override_) in module.overrides.iter() {
            let label = match override_.name {
                Some(ref name) => name,
                None => {
                    use core::fmt::Write;
                    // Try to be more descriptive about the override values
                    temp.clear();
                    write!(temp, "override_{}", output[&NameKey::Type(override_.ty)]).unwrap();
                    &temp
                }
            };
            let name = self.call(label);
            output.insert(NameKey::Override(handle), name);
        }
    }
}

#[test]
fn test() {
    let mut namer = Namer::default();
    assert_eq!(namer.call("x"), "x");
    assert_eq!(namer.call("x"), "x_1");
    assert_eq!(namer.call("x1"), "x1_");
    assert_eq!(namer.call("__x"), "_x");
    assert_eq!(namer.call("1___x"), "_x_1");
}
