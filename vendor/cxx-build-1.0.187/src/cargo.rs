use crate::gen::{CfgEvaluator, CfgResult};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BTreeMap as Map, BTreeSet as Set};
use std::env;
use std::sync::OnceLock;

static ENV: OnceLock<CargoEnv> = OnceLock::new();

struct CargoEnv {
    features: Set<Name>,
    cfgs: Map<Name, String>,
}

pub(super) struct CargoEnvCfgEvaluator;

impl CfgEvaluator for CargoEnvCfgEvaluator {
    fn eval(&self, name: &str, query_value: Option<&str>) -> CfgResult {
        let env = ENV.get_or_init(CargoEnv::load);
        if name == "feature" {
            return if let Some(query_value) = query_value {
                CfgResult::from(env.features.contains(Lookup::new(query_value)))
            } else {
                let msg = "expected `feature = \"...\"`".to_owned();
                CfgResult::Undetermined { msg }
            };
        }
        if name == "test" && query_value.is_none() {
            let msg = "cfg(test) is not supported because Cargo runs your build script only once across the lib and test build of the same crate".to_owned();
            return CfgResult::Undetermined { msg };
        }
        if let Some(cargo_value) = env.cfgs.get(Lookup::new(name)) {
            return if let Some(query_value) = query_value {
                CfgResult::from(cargo_value.split(',').any(|value| value == query_value))
            } else {
                CfgResult::True
            };
        }
        if name == "debug_assertions" && query_value.is_none() {
            return CfgResult::from(cfg!(debug_assertions));
        }
        CfgResult::False
    }
}

impl CargoEnv {
    fn load() -> Self {
        const CARGO_FEATURE_PREFIX: &str = "CARGO_FEATURE_";
        const CARGO_CFG_PREFIX: &str = "CARGO_CFG_";

        let mut features = Set::new();
        let mut cfgs = Map::new();
        for (k, v) in env::vars_os() {
            let Some(k) = k.to_str() else {
                continue;
            };
            let Ok(v) = v.into_string() else {
                continue;
            };
            if let Some(feature_name) = k.strip_prefix(CARGO_FEATURE_PREFIX) {
                let feature_name = Name(feature_name.to_owned());
                features.insert(feature_name);
            } else if let Some(cfg_name) = k.strip_prefix(CARGO_CFG_PREFIX) {
                let cfg_name = Name(cfg_name.to_owned());
                cfgs.insert(cfg_name, v);
            }
        }
        CargoEnv { features, cfgs }
    }
}

struct Name(String);

impl Ord for Name {
    fn cmp(&self, rhs: &Self) -> Ordering {
        Lookup::new(&self.0).cmp(Lookup::new(&rhs.0))
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Eq for Name {}

impl PartialEq for Name {
    fn eq(&self, rhs: &Self) -> bool {
        Lookup::new(&self.0).eq(Lookup::new(&rhs.0))
    }
}

#[repr(transparent)]
struct Lookup(str);

impl Lookup {
    fn new(name: &str) -> &Self {
        unsafe { &*(name as *const str as *const Self) }
    }
}

impl Borrow<Lookup> for Name {
    fn borrow(&self) -> &Lookup {
        Lookup::new(&self.0)
    }
}

impl Ord for Lookup {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0
            .bytes()
            .map(CaseAgnosticByte)
            .cmp(rhs.0.bytes().map(CaseAgnosticByte))
    }
}

impl PartialOrd for Lookup {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Eq for Lookup {}

impl PartialEq for Lookup {
    fn eq(&self, rhs: &Self) -> bool {
        self.0
            .bytes()
            .map(CaseAgnosticByte)
            .eq(rhs.0.bytes().map(CaseAgnosticByte))
    }
}

struct CaseAgnosticByte(u8);

impl Ord for CaseAgnosticByte {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.to_ascii_lowercase().cmp(&rhs.0.to_ascii_lowercase())
    }
}

impl PartialOrd for CaseAgnosticByte {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Eq for CaseAgnosticByte {}

impl PartialEq for CaseAgnosticByte {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}
