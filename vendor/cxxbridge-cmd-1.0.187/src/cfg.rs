use crate::gen::{CfgEvaluator, CfgResult};
use std::collections::{BTreeMap as Map, BTreeSet as Set};
use std::fmt::{self, Debug};
use syn::parse::ParseStream;
use syn::{Ident, LitBool, LitStr, Token};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum CfgValue {
    Bool(bool),
    Str(String),
}

impl CfgValue {
    const FALSE: Self = CfgValue::Bool(false);
    const TRUE: Self = CfgValue::Bool(true);
}

pub(crate) struct FlagsCfgEvaluator {
    map: Map<String, Set<CfgValue>>,
}

impl FlagsCfgEvaluator {
    pub(crate) fn new(map: Map<String, Set<CfgValue>>) -> Self {
        FlagsCfgEvaluator { map }
    }
}

impl CfgEvaluator for FlagsCfgEvaluator {
    fn eval(&self, name: &str, value: Option<&str>) -> CfgResult {
        let set = self.map.get(name);
        if let Some(value) = value {
            if let Some(set) = set {
                CfgResult::from(set.contains(&CfgValue::Str(value.to_owned())))
            } else if name == "feature" {
                CfgResult::False
            } else {
                let msg = format!(
                    "pass `--cfg {}=\"...\"` to be able to use this attribute",
                    name,
                );
                CfgResult::Undetermined { msg }
            }
        } else {
            let (mut is_false, mut is_true) = (false, false);
            if let Some(set) = set {
                is_false = set.contains(&CfgValue::FALSE);
                is_true = set.contains(&CfgValue::TRUE);
            }
            if is_false && is_true {
                let msg = format!("the cxxbridge flags say both {0}=false and {0}=true", name);
                CfgResult::Undetermined { msg }
            } else if is_false {
                CfgResult::False
            } else if is_true {
                CfgResult::True
            } else {
                let msg = format!(
                    "pass either `--cfg {0}=true` or `--cfg {0}=false` to be able to use this cfg attribute",
                    name,
                );
                CfgResult::Undetermined { msg }
            }
        }
    }
}

impl Debug for CfgValue {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CfgValue::Bool(value) => Debug::fmt(value, formatter),
            CfgValue::Str(value) => Debug::fmt(value, formatter),
        }
    }
}

pub(crate) fn parse(input: ParseStream) -> syn::Result<(String, CfgValue)> {
    let ident: Ident = input.parse()?;
    let name = ident.to_string();
    if input.is_empty() {
        return Ok((name, CfgValue::TRUE));
    }
    input.parse::<Token![=]>()?;
    let lookahead = input.lookahead1();
    if lookahead.peek(LitBool) {
        let lit: LitBool = input.parse()?;
        Ok((name, CfgValue::Bool(lit.value)))
    } else if lookahead.peek(LitStr) {
        let lit: LitStr = input.parse()?;
        Ok((name, CfgValue::Str(lit.value())))
    } else {
        Err(lookahead.error())
    }
}
