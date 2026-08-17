use std::collections::{BTreeMap, HashMap};

use crate::context::Context;
use crate::error::RenderError;
use crate::helpers::HelperDef;
use crate::json::value::{PathAndJson, ScopedJson};
use crate::registry::Registry;
use crate::render::{Helper, RenderContext};

use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{Dynamic, Engine, Scope, AST};

use serde_json::value::Value as Json;

pub(crate) struct ScriptHelper {
    pub(crate) script: AST,
}

#[inline]
fn call_script_helper<'reg: 'rc, 'rc>(
    params: &[PathAndJson<'reg, 'rc>],
    hash: &BTreeMap<&'reg str, PathAndJson<'reg, 'rc>>,
    engine: &Engine,
    script: &AST,
) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
    let params: Dynamic = to_dynamic(params.iter().map(|p| p.value()).collect::<Vec<&Json>>())?;

    let hash: Dynamic = to_dynamic(
        hash.iter()
            .map(|(k, v)| ((*k).to_owned(), v.value()))
            .collect::<HashMap<String, &Json>>(),
    )?;

    let mut scope = Scope::new();
    scope.push_dynamic("params", params);
    scope.push_dynamic("hash", hash);

    let result = engine
        .eval_ast_with_scope::<Dynamic>(&mut scope, script)
        .map_err(RenderError::from)?;

    let result_json: Json = from_dynamic(&result)?;

    Ok(ScopedJson::Derived(result_json))
}

impl HelperDef for ScriptHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        reg: &'reg Registry<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        call_script_helper(h.params(), h.hash(), &reg.engine, &self.script)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::json::value::{PathAndJson, ScopedJson};
    use rhai::Engine;

    #[test]
    fn test_dynamic_convert() {
        let j0 = json! {
            [{"name": "tomcat"}, {"name": "jetty"}]
        };

        let d0 = to_dynamic(&j0).unwrap();
        assert_eq!("array", d0.type_name());

        let j1 = json!({
            "name": "tomcat",
            "value": 4000,
        });

        let d1 = to_dynamic(&j1).unwrap();
        assert_eq!("map", d1.type_name());
    }

    #[test]
    fn test_to_json() {
        let d0 = Dynamic::from("tomcat".to_owned());

        assert_eq!(
            Json::String("tomcat".to_owned()),
            from_dynamic::<Json>(&d0).unwrap()
        );
    }

    #[test]
    fn test_script_helper_value_access() {
        let engine = Engine::new();

        let script = "let plen = len(params); let p0 = params[0]; let hlen = len(hash); let hme = hash[\"me\"]; plen + \",\" + p0 + \",\" + hlen + \",\" + hme";
        let ast = engine.compile(&script).unwrap();

        let params = vec![PathAndJson::new(None, ScopedJson::Derived(json!(true)))];

        let mut hash = BTreeMap::new();
        hash.insert(
            "me",
            PathAndJson::new(None, ScopedJson::Derived(json!("no"))),
        );
        hash.insert(
            "you",
            PathAndJson::new(None, ScopedJson::Derived(json!("yes"))),
        );

        let result = call_script_helper(&params, &hash, &engine, &ast)
            .unwrap()
            .render();
        assert_eq!("1,true,2,no", &result);
    }
}
