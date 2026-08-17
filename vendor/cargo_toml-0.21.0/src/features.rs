//! Helper for parsing the microsyntax of the `[features]` section and computing implied features from optional dependencies.

use crate::{Dependency, DepsSet, Manifest, Product, TargetDepsSet};
use std::borrow::Cow;
use std::collections::hash_map::{Entry, RandomState};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::hash::BuildHasher;
use std::marker::PhantomData;

/// Maximum number of features and dependencies, to protect against DoS
/// crates.io limit is 300.
const MAX_ITEMS: usize = 2048;

/// Call [`features::Resolver::new()`](Resolver::new) to get started.
///
/// The extra `Hasher` arg is for optionally using [`ahash`](https://lib.rs/ahash).
pub struct Resolver<'config, Hasher = RandomState> {
    always_keep: Option<&'config dyn Fn(&str) -> bool>,
    _hasher: PhantomData<fn() -> Hasher>,
}

/// Parse result
///
/// It's a temporary struct that borrows from the manifest. Copy things out of this if lifetimes get in the way.
///
/// The extra `Hasher` arg is for optionally using [`ahash`](https://lib.rs/ahash).
#[derive(Debug)]
#[non_exhaustive]
pub struct Features<'manifest, 'deps, Hasher = RandomState> {
    /// All features, resolved and normalized
    ///
    /// Default features are literally under the "default" key.
    pub features: HashMap<&'manifest str, Feature<'manifest>, Hasher>,
    /// FYI, dependencies referenced by the features, keyed by dependencies' name/identifier in TOML (that's not always the crate name)
    ///
    /// This doesn't include *all* dependencies. Dependencies unaffected by any features selection are skipped.
    pub dependencies: HashMap<&'deps str, FeatureDependency<'deps>, Hasher>,
    /// True if there were features with names staring with `_` and were inlined and merged into other features
    ///
    /// See arg of [`new_with_hasher_and_filter`](Resolver::new_with_hasher_and_filter) to disable removal.
    pub removed_hidden_features: bool,

    /// A redirect from removed feature to its replacements
    pub hidden_features: HashMap<&'manifest str, BTreeSet<&'manifest str>, Hasher>,
}

/// How an enabled feature affects the dependency
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct DepAction<'a> {
    /// Uses `?` syntax, so it doesn't enable the depenency
    pub is_conditional: bool,
    /// Uses `dep:` or `?` syntax, so it doesn't imply a feature name
    pub is_dep_only: bool,
    /// Features of the dependency to enable (the text after slash, possibly aggregated from multiple items)
    pub dep_features: BTreeSet<Cow<'a, str>>,
}

/// A feature from `[features]` with all the details
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Feature<'a> {
    /// Name of the feature
    pub key: &'a str,
    /// Deps this enables or modifies, by their manifest key (the key isn't always the same as the crate name)
    ///
    /// This set is shallow (this feature may also be enabling other features that enable more deps), see [`Feature::enables_recursive`].
    pub enables_deps: BTreeMap<&'a str, DepAction<'a>>,
    /// Enables these explicitly named features
    ///
    /// This set is shallow, and the features listed here may be enabling more features, see [`Feature::enables_recursive`].
    /// Note that Cargo permits infinite loops (A enables B, B enables A).
    pub enables_features: BTreeSet<&'a str>,

    /// Keys of other features that directly enable this feature (this is shallow, not recursive)
    pub enabled_by: BTreeSet<&'a str>,

    /// Names of binaries that mention this feature in their `required-features = [this feature]`
    ///
    /// Name of the default unnamed binary is set to the package name, and not normalized.
    pub required_by_bins: Vec<&'a str>,

    /// If true, it's from `[features]`. If false, it's from `[dependencies]`.
    ///
    /// If it's not explicit, and `is_referenced() == true`, it's probably a mistake and wasn't supposed to be a feature.
    /// See `is_user_facing`.
    pub explicit: bool,
}

/// Outer key is the dependency key/name, the `Vec` contains feature names
pub type DependenciesEnabledByFeatures<'a, S> = HashMap<&'a str, Vec<(&'a str, &'a DepAction<'a>)>, S>;

impl<'a> Feature<'a> {
    /// Heuristic whether this feature should be shown to users
    ///
    /// Skips underscore-prefixed features, and possibly unintended features implied by optional dependencies
    #[inline]
    #[must_use]
    pub fn is_user_facing(&self) -> bool {
        (self.explicit || !self.is_referenced()) && !self.key.starts_with('_')
    }

    /// Just `enabled_by` except the "default" feature
    #[inline]
    pub fn non_default_enabled_by(&self) -> impl Iterator<Item = &str> {
        self.enabled_by.iter().copied().filter(|&e| e != "default")
    }

    /// Is any other feature using this one?
    #[inline]
    #[must_use]
    pub fn is_referenced(&self) -> bool {
        !self.enabled_by.is_empty()
    }

    /// Finds all features and dependencies that this feature enables, recursively and exhaustively
    ///
    /// The first `HashMap` is features by their key, the second is dependencies by their key. It includes only dependencies changed by the features, not all crate dependencies.
    #[must_use]
    pub fn enables_recursive<S: BuildHasher + Default>(&'a self, features: &'a HashMap<&'a str, Feature<'a>, S>) -> (HashMap<&'a str, &'a Feature<'a>, S>, DependenciesEnabledByFeatures<'a, S>) {
        let mut features_set = HashMap::with_capacity_and_hasher(self.enabled_by.len() + self.enabled_by.len()/2, S::default());
        let mut deps_set = HashMap::with_capacity_and_hasher(self.enables_deps.len() + self.enables_deps.len()/2, S::default());
        self.add_to_set(features, &mut features_set, &mut deps_set);
        (features_set, deps_set)
    }

    #[inline(never)]
    fn add_to_set<S: BuildHasher>(&'a self, features: &'a HashMap<&'a str, Feature<'a>, S>, features_set: &mut HashMap<&'a str, &'a Feature<'a>, S>, deps_set: &mut DependenciesEnabledByFeatures<'a, S>) {
        if features_set.insert(self.key, self).is_none() {
            for (&dep_key, action) in &self.enables_deps {
                if !action.is_conditional {
                    deps_set.entry(dep_key).or_default().push((self.key, action));
                }
            }
            for &key in &self.enables_features {
                if let Some(feature) = features.get(key) {
                    feature.add_to_set(features, features_set, deps_set);
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetKey<'a> {
    pub kind: Kind,
    /// cfg. None for all targets.
    pub target: Option<&'a str>,
}

/// A dependency referenced by a feature
#[derive(Debug)]
#[non_exhaustive]
pub struct FeatureDependency<'dep> {
    /// Actual crate of this dependency. Note that multiple dependencies can be the same crate, in different versions.
    pub crate_name: &'dep str,

    /// By kind and target
    pub targets: BTreeMap<TargetKey<'dep>, &'dep Dependency>,
}

impl<'dep> FeatureDependency<'dep> {
    #[inline]
    #[must_use]
    pub fn dep(&self) -> &'dep Dependency {
        self.detail().0
    }

    /// Extra metadata for the most common usage (normal > build > dev) of this dependency
    #[inline]
    #[must_use]
    pub fn detail(&self) -> (&'dep Dependency, Kind) {
        let (k, dep) = self.targets.iter().next().unwrap();
        (dep, k.kind)
    }
}

impl Resolver<'static, RandomState> {
    /// Next step: [`.parse(manifest)`](Resolver::parse).
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            always_keep: None,
            _hasher: PhantomData,
        }
    }
}

impl<'manifest, 'config, RandomState: BuildHasher + Default> Resolver<'config, RandomState> {
    /// Use turbofish to configure `RandomState` of a hasher you want this to use.
    ///
    /// `should_keep_hidden_feature` is a reference to a closure that will receive feature names starting with `_`, and return whether to keep or hide them.
    #[must_use]
    pub fn new_with_hasher_and_filter(should_keep_hidden_feature: &'config dyn Fn(&str) -> bool) -> Self {
        Self {
            always_keep: Some(should_keep_hidden_feature),
            _hasher: PhantomData,
        }
    }

    /// Parse features from a Cargo.toml manifest
    pub fn parse<M>(&self, manifest: &'manifest Manifest<M>) -> Features<'manifest, 'manifest, RandomState> {
        let mut features = Self::parse_features(
            manifest.features.iter().take(MAX_ITEMS),
            manifest.features.contains_key("default"),
        );

        let dependencies = Self::add_dependencies(
            &mut features,
            &manifest.dependencies,
            &manifest.build_dependencies,
            &manifest.dev_dependencies,
            &manifest.target,
        );

        Self::set_required_by_bins(&mut features, &manifest.bin, manifest.package().name());

        Self::remove_redundant_dep_action_features(&mut features, &dependencies);
        Self::set_enabled_by(&mut features);
        let hidden_features = self.remove_hidden_features(&mut features);

        Features {
            features,
            dependencies,
            removed_hidden_features: !hidden_features.is_empty(),
            hidden_features,
        }
    }

    /// Instead of processing a `Cargo.toml` manifest, take bits of information directly instead
    ///
    /// It won't fill in `required_by_bins` fields.
    pub fn parse_custom<'deps, S: BuildHasher>(&self, manifest_features: &'manifest HashMap<String, Vec<String>, S>, deps: impl Iterator<Item=ParseDependency<'manifest, 'deps>>) -> Features<'manifest, 'deps, RandomState> where 'manifest: 'deps {
        let mut features: HashMap<&'manifest str, Feature<'manifest>, _> = Self::parse_features(
            manifest_features.iter().take(MAX_ITEMS),
            manifest_features.contains_key("default"),
        );

        let named_using_dep_syntax = Self::named_using_dep_syntax(&features);

        // First one wins, so order is important
        let mut dependencies = HashMap::<&'deps str, FeatureDependency<'deps>, RandomState>::default();
        for dep in deps {
            Self::add_dependency(&mut features, &mut dependencies, named_using_dep_syntax.get(dep.key).copied(), dep.kind, dep.target, dep.key, dep.dep);
        }

        Self::remove_redundant_dep_action_features(&mut features, &dependencies);
        Self::set_enabled_by(&mut features);
        let hidden_features = self.remove_hidden_features(&mut features);

        Features {
            features,
            dependencies,
            removed_hidden_features: !hidden_features.is_empty(),
            hidden_features,
        }
    }
}

/// For parsing in `parse_custom`. Can be constructed from the crates.io index, instead of `Cargo.toml`.
///
/// Note about lifetimes: it's not possible to make `&Dependency` on the fly.
/// You will have to collect *owned* `Dependency` objects to a `Vec` or `HashMap` first.
#[derive(Debug, Clone)]
pub struct ParseDependency<'a, 'tmp> {
    /// Name/id of the dependency, not always the crate name
    pub key: &'a str,
    pub kind: Kind,
    /// Name `[target."from here".dependencies]`
    pub target: Option<&'a str>,
    /// Possibly more detail
    pub dep: &'tmp Dependency,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum Kind {
    #[default]
    Normal,
    Build,
    Dev,
}

impl<'a, 'c, S: BuildHasher + Default> Resolver<'c, S> {
    fn parse_features(features: impl Iterator<Item = (&'a String, &'a Vec<String>)>, has_explicit_default: bool) -> HashMap<&'a str, Feature<'a>, S> {
        features
            .map(|(key, f)| (key.as_str(), f.as_slice()))
            .chain(if has_explicit_default { None } else { Some(("default", &[][..])) }) // there must always be the default key
            .map(|(feature_key, actions)| Self::parse_feature(feature_key, actions))
            .collect()
    }

    #[inline(never)]
    fn parse_feature(feature_key: &'a str, actions: &'a [String]) -> (&'a str, Feature<'a>) {
        // coalesce dep_feature
        let mut enables_deps = BTreeMap::new();
        let mut enables_features = BTreeSet::new();
        actions.iter().take(MAX_ITEMS).for_each(|action| {
            let mut parts = action.splitn(2, '/');
            let mut atarget = parts.next().unwrap_or_default();
            let dep_feature = parts.next();

            let is_dep_prefix = if let Some(k) = atarget.strip_prefix("dep:") { atarget = k; true } else { false };
            let is_conditional = if let Some(k) = atarget.strip_suffix('?') { atarget = k; true } else { false };

            // dep/feature is old, doesn't actually mean it's dep:only!
            let is_dep_only = is_dep_prefix;
            if is_dep_only || dep_feature.is_some() {
                let action = enables_deps.entry(atarget).or_insert(DepAction {
                    is_conditional,
                    is_dep_only,
                    dep_features: BTreeSet::default(),
                });
                if !is_conditional { action.is_conditional = false; }
                if is_dep_only { action.is_dep_only = true; }
                if let Some(df) = dep_feature { action.dep_features.insert(Cow::Borrowed(df)); }
            } else {
                // dep/foo can be both, and missing enables_deps is added later after checking all for dep:
                enables_features.insert(atarget);
            }
        });

        (feature_key, Feature {
            key: feature_key,
            enables_features,
            required_by_bins: vec![],
            enables_deps,
            explicit: true,
            enabled_by: BTreeSet::new(), // fixed later
        })
    }

    fn add_implied_optional_deps(features: &mut HashMap<&'a str, Feature<'a>, S>, deps_for_features: &mut HashMap<&'a str, FeatureDependency<'a>, S>, crate_deps: &'a BTreeMap<String, Dependency>, named_using_dep_syntax: &HashMap<&str, bool, S>, dep_kind: Kind, only_for_target: Option<&'a str>) {
        for (key, dep) in crate_deps.iter().take(MAX_ITEMS) {
            let key = key.as_str();
            Self::add_dependency(features, deps_for_features, named_using_dep_syntax.get(key).copied(), dep_kind, only_for_target, key, dep);
        }
    }

    #[inline(never)]
    fn add_dependency<'d>(features: &mut HashMap<&'a str, Feature<'a>, S>, deps_for_features: &mut HashMap<&'d str, FeatureDependency<'d>, S>, named_using_dep_syntax: Option<bool>, dep_kind: Kind, only_for_target: Option<&'d str>, key: &'a str, dep: &'d Dependency) where 'a: 'd {
        let is_optional = dep.optional();
        let entry = deps_for_features.entry(key);
        if !is_optional && named_using_dep_syntax.is_none() && matches!(entry, Entry::Vacant(_)) {
            return;
        }

        let entry = entry.or_insert_with(move || FeatureDependency {
            crate_name: dep.package().unwrap_or(key),
            targets: BTreeMap::new(),
        });

        entry.targets.entry(TargetKey { kind: dep_kind, target: only_for_target }).or_insert(dep);

        // explicit features never overlap with deps, unless using "dep:" syntax.

        // We need to know about all deps referenced by features, or all optional ones.
        // We won't see optional build or dev dep feature, if there's normal non-optional dep. Not a big deal?
        // if we added one for normal, then keep adding build and dev.
        if is_optional && named_using_dep_syntax != Some(true) {
            features.entry(key).or_insert_with(move || Feature {
                key,
                enables_features: BTreeSet::default(),
                enables_deps: BTreeMap::from_iter([(key, DepAction {
                    is_dep_only: false,
                    is_conditional: false,
                    dep_features: BTreeSet::default(),
                })]),
                explicit: false,
                enabled_by: BTreeSet::new(), // will do later
                required_by_bins: vec![],
            });
        }
    }

    /// find which names are affected by use of the `dep:` syntax and supress implicit features
    #[inline(never)]
    fn named_using_dep_syntax(features: &HashMap<&'a str, Feature<'a>, S>) -> HashMap<&'a str, bool, S> {
        // explicit features exist, even if their name clashes with a `dep:name`.
        let mut named_using_dep_syntax: HashMap::<_, _, S> = features.keys().map(|&k| (k, false)).collect();

        for f in features.values() {
            f.enables_deps.iter().for_each(|(&dep_key, a)| {
                named_using_dep_syntax.entry(dep_key)
                    .and_modify(|v| *v |= a.is_dep_only)
                    .or_insert(a.is_dep_only);
            });
        }

        named_using_dep_syntax
    }

    fn add_dependencies(features: &mut HashMap<&'a str, Feature<'a>, S>, dependencies: &'a DepsSet, build_dependencies: &'a DepsSet, dev_dependencies: &'a DepsSet, target: &'a TargetDepsSet) -> HashMap<&'a str, FeatureDependency<'a>, S> {
        let named_using_dep_syntax = Self::named_using_dep_syntax(features);

        // First one wins, so order is important
        let mut all_deps = HashMap::<_, _, S>::default();
        Self::add_implied_optional_deps(features, &mut all_deps, dependencies, &named_using_dep_syntax, Kind::Normal, None);
        Self::add_implied_optional_deps(features, &mut all_deps, build_dependencies, &named_using_dep_syntax, Kind::Build, None);
        for (target_cfg, target_deps) in target {
            Self::add_implied_optional_deps(features, &mut all_deps, &target_deps.dependencies, &named_using_dep_syntax, Kind::Normal, Some(target_cfg));
            Self::add_implied_optional_deps(features, &mut all_deps, &target_deps.build_dependencies, &named_using_dep_syntax, Kind::Build, Some(target_cfg));
        }
        Self::add_implied_optional_deps(features, &mut all_deps, dev_dependencies, &named_using_dep_syntax, Kind::Dev, None);
        for (target_cfg, target_deps) in target {
            Self::add_implied_optional_deps(features, &mut all_deps, &target_deps.dev_dependencies, &named_using_dep_syntax, Kind::Dev, Some(target_cfg));
        }
        all_deps
    }

    #[inline(never)]
    fn set_required_by_bins(features: &mut HashMap<&'a str, Feature<'a>, S>, bin: &'a [Product], package_name: &'a str) {
        for bin in bin {
            for f in &bin.required_features {
                let bin_name = bin.name.as_deref().unwrap_or(package_name);
                if let Some(f) = features.get_mut(f.as_str()) {
                    // fallback to package name is not quite accurate, because Cargo normalizes exe names slightly
                    f.required_by_bins.push(bin_name);
                }
            }
        }
    }

    #[inline(never)]
    fn remove_redundant_dep_action_features(features: &mut HashMap<&str, Feature<'_>, S>, dependencies: &HashMap<&str, FeatureDependency<'_>, S>) {
        features.values_mut()
            .flat_map(|f| &mut f.enables_deps)
            .filter(|(_, action)| !action.dep_features.is_empty())
            .for_each(|(dep_key, action)| {
                if let Some(dep) = dependencies.get(dep_key).and_then(|d| d.dep().detail()) {
                    action.dep_features.retain(move |dep_f| {
                        let dep_f = &**dep_f;
                        (!dep.default_features || dep_f != "default") &&
                        !dep.features.iter().any(|k| k == dep_f)
                    });
                }
            });
    }

    #[inline(never)]
    fn set_enabled_by(features: &mut HashMap<&'a str, Feature<'a>, S>) {
        let mut all_enabled_by = HashMap::<_, _, S>::default();
        for (&feature_key, f) in features.iter() {
            f.enables_features.iter().copied().for_each(|action_key| if action_key != feature_key {
                all_enabled_by.entry(action_key).or_insert_with(BTreeSet::new).insert(feature_key);
            });
            f.enables_deps.iter().for_each(|(&action_key, action)| if !action.is_conditional && !action.is_dep_only && action_key != feature_key {
                all_enabled_by.entry(action_key).or_insert_with(BTreeSet::new).insert(feature_key);
            });
        }

        for (key, enabled_by) in all_enabled_by {
            if let Some(f) = features.get_mut(key) {
                f.enabled_by = enabled_by;
            }
        }
    }

    /// find `__features` and inline them
    #[inline(never)]
    fn remove_hidden_features(&self, features: &mut HashMap<&'a str, Feature<'a>, S>) -> HashMap<&'a str, BTreeSet<&'a str>, S> {
        let features_to_remove: BTreeSet<_> = features.keys().copied().filter(|&k| {
            k.starts_with('_') && !self.always_keep.is_some_and(|cb| (cb)(k)) // if user thinks that is useful info
        }).collect();

        let mut removed_mapping = HashMap::<_, _, S>::default();

        if features_to_remove.is_empty() {
            return removed_mapping;
        }

        features_to_remove.into_iter().for_each(|key| {
            let Some(mut janky) = features.remove(key) else { return };

            janky.enabled_by.iter().for_each(|&parent_key| if let Some(parent) = features.get_mut(parent_key) {
                parent.enabled_by.remove(janky.key); // just in case it's circular

                // the filter tries to avoid adding new redundant enables_features, but it's order-dependent
                parent.enables_features.extend(&janky.enables_features);
                parent.enables_features.remove(janky.key);

                janky.enables_deps.iter().for_each(|(&k, ja)| {
                    parent.enables_deps.entry(k)
                        .and_modify(|old| {
                            if !ja.is_conditional { old.is_conditional = false; }
                            if ja.is_dep_only { old.is_dep_only = true; }
                            old.dep_features.extend(ja.dep_features.iter().cloned());
                        })
                        .or_insert_with(|| ja.clone());
                });
            });

            janky.enables_features.iter().for_each(|&f| {
                if let Some(child) = features.get_mut(f) {
                    // this list is sometimes a bit redundant,
                    // but the hidden feature cleanup is not recursive, so it needs to contain all possible places
                    child.enabled_by.extend(&janky.enabled_by);
                    child.enabled_by.remove(janky.key);

                    janky.required_by_bins.iter().take(10).for_each(|&bin| {
                        if !child.required_by_bins.contains(&bin) {
                            child.required_by_bins.push(bin);
                        }
                    });
                }
            });

            janky.enables_deps.iter().filter(|&(k, a)| !a.is_dep_only && !janky.enables_features.contains(k)).for_each(|(&d, _)| {
                if let Some(d) = features.get_mut(d) {
                    d.enabled_by.extend(&janky.enabled_by);
                    d.enabled_by.remove(janky.key);
                }
            });

            removed_mapping.entry(janky.key).or_default().append(&mut janky.enables_features);
        });
        removed_mapping
    }
}

#[test]
fn features_test() {
    let m = crate::Manifest::from_str(r#"
[package]
name = "foo"

[[bin]]
name = "thebin"
required-features = ["__hidden2", "loop3"]

[dependencies]
not_optional = { path = "..", package = "actual_pkg", features = ["f1", "f2"] }
depend = { version = "1.0.0", package = "actual_pkg", optional = true, default-features = false }
implied_standalone = { version = "1.0.0", optional = true }
implied_referenced = { version = "1.0.0", optional = true }
not_relevant = "2"
feature_for_hidden = { version = "2", optional = true }
__hidden_dep = { version = "2", optional = true }

[build-dependencies]
a_dep = { version = "1.0.0", optional = true }

[features]
default = ["x"]
a = []
b = ["a", "implied_referenced/with_feature", "depend/default", "depend/default"]
c = ["__hidden"]
x = ["__hidden", "c", "not_optional/f2", "not_optional/f3", "not_optional/default"]
__hidden = ["__hidden2"]
__hidden2 = ["dep:depend", "depend/with_x", "depend?/with_y", "__hidden0"]
__hidden0 = ["a"]
enables_hidden = ["__hidden0"]
__feature_for_hidden = ["feature_for_hidden"]

loop1 = ["loop2"]
loop2 = ["loop3", "a_dep?/maybe", "a_dep?/maybe_too", "depend/with_loop"]
loop3 = ["loop1", "implied_referenced/from_loop_3"]

    "#).unwrap();
    let r = Resolver::new().parse(&m);
    let f = r.features;
    let d = r.dependencies;

    assert!(r.removed_hidden_features);

    // __hidden completely removed
    assert!(!f.iter().any(|(&k, f)| {
        k.starts_with('_') ||
            f.enables_features.iter().any(|&k| k.starts_with('_')) ||
            f.enables_deps.iter().any(|(&k, _)| k.starts_with('_')) ||
            f.enabled_by.iter().any(|&k| k.starts_with('_'))
    }));

    assert!(!d.keys().any(|&k| k.starts_with('_') && k != "__hidden_dep"));
    assert!(!f.contains_key("__hidden_dep"));

    assert_eq!(f.len(), 13);
    assert!(!f.contains_key("depend"));

    assert_eq!(d.len(), 7);
    assert!(!d.contains_key("not_relevant"));
    assert!(!f.contains_key("not_relevant"));

    assert!(!f.contains_key("not_optional"));
    assert_eq!(d["not_optional"].crate_name, "actual_pkg");

    assert_eq!(d["implied_standalone"].crate_name, "implied_standalone");
    assert!(d["implied_standalone"].detail().0.optional());
    assert!(d["a_dep"].targets.keys().all(|t| t.kind == Kind::Build));
    assert!(!f["implied_standalone"].explicit);
    assert!(!f["implied_referenced"].explicit);
    assert!(!f["a_dep"].explicit);
    assert!(!f["feature_for_hidden"].is_referenced());

    assert_eq!(f["x"].enables_deps.keys().copied().collect::<Vec<_>>(), &["depend", "not_optional"]);
    assert_eq!(f["x"].enables_features.iter().copied().collect::<Vec<_>>(), &["a", "c"]);

    assert_eq!(f["a"].enabled_by.iter().copied().collect::<Vec<_>>(), &["b", "c", "enables_hidden", "x"]);
    assert!(f["a"].enables_deps.is_empty());
    assert!(f["a"].enables_features.is_empty());

    assert_eq!(f["loop1"].enabled_by.iter().copied().collect::<Vec<_>>(), &["loop3"]);
    assert_eq!(f["loop2"].enabled_by.iter().copied().collect::<Vec<_>>(), &["loop1"]);
    assert_eq!(f["loop3"].enabled_by.iter().copied().collect::<Vec<_>>(), &["loop2"]);

    assert!(f["loop1"].enables_deps.is_empty());
    assert_eq!(f["loop2"].enables_deps.keys().copied().collect::<Vec<_>>(), &["a_dep", "depend"]);
    assert!(f["loop2"].enables_deps["a_dep"].is_conditional);
    assert!(!f["loop2"].enables_deps["depend"].is_conditional);
    assert_eq!(f["loop3"].enables_deps.keys().copied().collect::<Vec<_>>(), &["implied_referenced"]);

    assert_eq!(f["loop1"].enables_features.iter().copied().collect::<Vec<_>>(), &["loop2"]);
    assert_eq!(f["loop2"].enables_features.iter().copied().collect::<Vec<_>>(), &["loop3"]);
    assert_eq!(f["loop3"].enables_features.iter().copied().collect::<Vec<_>>(), &["loop1"]);

    let (rf, rd) = f["loop1"].enables_recursive(&f);
    assert_eq!(rf["loop1"].key, "loop1");
    assert_eq!(rf["loop2"].key, "loop2");
    assert_eq!(rf["loop3"].key, "loop3");
    assert_eq!(rd["implied_referenced"][0].0, "loop3");
    assert_eq!(rd["depend"][0].0, "loop2");
    assert!(!rd.contains_key("a_dep"));
}

