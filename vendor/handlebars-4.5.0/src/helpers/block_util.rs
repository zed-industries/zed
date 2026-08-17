use crate::block::BlockContext;
use crate::json::value::PathAndJson;

pub(crate) fn create_block<'reg: 'rc, 'rc>(
    param: &'rc PathAndJson<'reg, 'rc>,
) -> BlockContext<'reg> {
    let mut block = BlockContext::new();

    if let Some(new_path) = param.context_path() {
        *block.base_path_mut() = new_path.clone();
    } else {
        // use clone for now
        block.set_base_value(param.value().clone());
    }

    block
}
