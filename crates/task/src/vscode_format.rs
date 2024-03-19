use serde::Deserialize;

use crate::static_source::DefinitionProvider;

#[derive(Deserialize)]
/// TODO: docs for this
pub struct VsCodeTaskFile {}
impl TryFrom<VsCodeTaskFile> for DefinitionProvider {
    type Error = anyhow::Error;

    fn try_from(value: VsCodeTaskFile) -> Result<Self, Self::Error> {
        dbg!("Hello, I'm trying to parse this");
        Ok(Default::default())
    }
}
