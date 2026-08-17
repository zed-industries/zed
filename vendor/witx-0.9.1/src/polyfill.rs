use crate::{
    Document, Id, InterfaceFunc, InterfaceFuncParam, Module, RepEquality, Representable, Type,
    TypeRef,
};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolyfillError {
    #[error("Module not present: {name:?}")]
    ModuleNotPresent { name: Id },
    #[error("Function not present: {name:?}")]
    FuncNotPresent { module: Id, name: Id },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Polyfill {
    pub modules: Vec<ModulePolyfill>,
}

impl Polyfill {
    pub fn new(
        new: &Document,
        old: &Document,
        module_mapping: &HashMap<String, String>, // Will need a more sophisticated mapping - what about function names, argument names?
    ) -> Result<Self, PolyfillError> {
        let mut modules = Vec::new();
        for (newname, oldname) in module_mapping {
            let newname = Id::new(newname);
            let oldname = Id::new(oldname);
            let newmod = new
                .module(&newname)
                .ok_or_else(|| PolyfillError::ModuleNotPresent { name: newname })?;
            let oldmod = old
                .module(&oldname)
                .ok_or_else(|| PolyfillError::ModuleNotPresent { name: oldname })?;
            modules.push(ModulePolyfill::new(newmod, oldmod)?);
        }
        Ok(Polyfill { modules })
    }

    pub fn type_polyfills(&self) -> HashSet<TypePolyfill> {
        self.modules
            .iter()
            .map(|m| m.type_polyfills())
            .flatten()
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePolyfill {
    pub new: Rc<Module>,
    pub old: Rc<Module>,
    pub funcs: Vec<FuncPolyfill>,
}

impl ModulePolyfill {
    pub fn new(new: Rc<Module>, old: Rc<Module>) -> Result<Self, PolyfillError> {
        let mut funcs = Vec::new();
        for oldfunc in old.funcs() {
            let newfunc = new
                .func(&oldfunc.name)
                .ok_or_else(|| PolyfillError::FuncNotPresent {
                    module: new.name.clone(),
                    name: oldfunc.name.clone(),
                })?;
            funcs.push(FuncPolyfill::new(newfunc, oldfunc));
        }
        Ok(ModulePolyfill { new, old, funcs })
    }
    pub fn type_polyfills(&self) -> HashSet<TypePolyfill> {
        self.funcs
            .iter()
            .map(|f| f.type_polyfills())
            .flatten()
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncPolyfill {
    pub new: Rc<InterfaceFunc>,
    pub old: Rc<InterfaceFunc>,
    pub mapped_params: Vec<ParamPolyfill>,
    pub unknown_params: Vec<ParamUnknown>,
    pub mapped_results: Vec<ParamPolyfill>,
    pub unknown_results: Vec<ParamUnknown>,
}

impl FuncPolyfill {
    pub fn new(new: Rc<InterfaceFunc>, old: Rc<InterfaceFunc>) -> FuncPolyfill {
        let mut mapped_params = Vec::new();
        let mut unknown_params = Vec::new();

        // Old function is called. Need to map each of its parameters to the new function:
        for old_param in old.params.iter() {
            if let Some(new_param) = new.params.iter().find(|p| p.name == old_param.name) {
                mapped_params.push(ParamPolyfill::param(new_param.clone(), old_param.clone()))
            } else {
                unknown_params.push(ParamUnknown::Old(old_param.clone()));
            }
        }
        // Are any new params not covered by the old params?
        // This search is O(n^2), but n ought to be small.
        for new_param in new.params.iter() {
            if mapped_params
                .iter()
                .find(|m| m.new.name == new_param.name)
                .is_none()
            {
                unknown_params.push(ParamUnknown::New(new_param.clone()));
            }
        }

        let mut mapped_results = Vec::new();
        let mut unknown_results = Vec::new();

        // New function has returned. Need to map each of its results to the old function:
        for new_result in new.results.iter() {
            if let Some(old_result) = old.results.iter().find(|p| p.name == new_result.name) {
                mapped_results.push(ParamPolyfill::result(
                    new_result.clone(),
                    old_result.clone(),
                ))
            } else {
                unknown_results.push(ParamUnknown::New(new_result.clone()));
            }
        }

        // Are any old results not covered by the new results?
        for old_result in old.results.iter() {
            if mapped_results
                .iter()
                .find(|m| m.old.name == old_result.name)
                .is_none()
            {
                unknown_results.push(ParamUnknown::Old(old_result.clone()));
            }
        }

        FuncPolyfill {
            new,
            old,
            mapped_params,
            unknown_params,
            mapped_results,
            unknown_results,
        }
    }

    pub fn full_compat(&self) -> bool {
        self.new.name == self.old.name
            && self.mapped_params.iter().all(|p| p.full_compat())
            && self.unknown_params.is_empty()
            && self.mapped_results.iter().all(|p| p.full_compat())
            && self.unknown_results.is_empty()
    }

    pub fn type_polyfills(&self) -> HashSet<TypePolyfill> {
        self.mapped_params
            .iter()
            .map(|p| p.type_polyfill.clone())
            .chain(self.mapped_results.iter().map(|p| p.type_polyfill.clone()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamPolyfill {
    pub new: InterfaceFuncParam,
    pub old: InterfaceFuncParam,
    pub type_polyfill: TypePolyfill,
}

impl ParamPolyfill {
    fn common_denominator(a: TypeRef, b: TypeRef) -> (TypeRef, TypeRef) {
        match (&a, &b) {
            (TypeRef::Value(va), TypeRef::Value(vb)) => match (&**va, &**vb) {
                (Type::List(a), Type::List(b)) => (a.clone(), b.clone()),
                (Type::Pointer(a), Type::Pointer(b)) => (a.clone(), b.clone()),
                (Type::ConstPointer(a), Type::ConstPointer(b)) => (a.clone(), b.clone()),
                _ => (a, b),
            },
            _ => (a, b),
        }
    }

    pub fn param(new: InterfaceFuncParam, old: InterfaceFuncParam) -> Self {
        let (told, tnew) = Self::common_denominator(old.tref.clone(), new.tref.clone());
        // Call new param type with old param:
        let type_polyfill = TypePolyfill::OldToNew(told, tnew);
        ParamPolyfill {
            new,
            old,
            type_polyfill,
        }
    }

    pub fn result(new: InterfaceFuncParam, old: InterfaceFuncParam) -> Self {
        let (told, tnew) = Self::common_denominator(old.tref.clone(), new.tref.clone());
        // Return old result type from new result:
        let type_polyfill = TypePolyfill::NewToOld(tnew, told);
        ParamPolyfill {
            new,
            old,
            type_polyfill,
        }
    }

    pub fn full_compat(&self) -> bool {
        self.new.name == self.old.name && self.repeq() == RepEquality::Eq
    }

    pub fn repeq(&self) -> RepEquality {
        self.type_polyfill.repeq()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParamUnknown {
    Old(InterfaceFuncParam),
    New(InterfaceFuncParam),
}

impl ParamUnknown {
    pub fn which(&self) -> &'static str {
        match self {
            ParamUnknown::Old { .. } => "old",
            ParamUnknown::New { .. } => "new",
        }
    }
    pub fn param(&self) -> &InterfaceFuncParam {
        match self {
            ParamUnknown::Old(p) => &p,
            ParamUnknown::New(p) => &p,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypePolyfill {
    NewToOld(TypeRef, TypeRef),
    OldToNew(TypeRef, TypeRef),
}

impl TypePolyfill {
    pub fn repeq(&self) -> RepEquality {
        match self {
            TypePolyfill::NewToOld(new, old) => old.type_().representable(&new.type_()),
            TypePolyfill::OldToNew(old, new) => new.type_().representable(&old.type_()),
        }
    }
}
