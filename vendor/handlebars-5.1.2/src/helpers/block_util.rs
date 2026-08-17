use crate::block::BlockContext;
use crate::json::value::PathAndJson;

pub(crate) fn create_block<'rc>(param: &PathAndJson<'rc>) -> BlockContext<'rc> {
    let mut block = BlockContext::new();

    if let Some(new_path) = param.context_path() {
        block.base_path_mut().clone_from(new_path)
    } else {
        // use clone for now
        block.set_base_value(param.value().clone());
    }

    block
}
