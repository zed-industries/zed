#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableLookupKind {
    Variable,
    Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableScope {
    Local,
    Global,
}

#[derive(Debug, Clone)]
pub struct InlineValueLocation {
    pub variable_name: String,
    pub scope: VariableScope,
    pub lookup: VariableLookupKind,
    pub row: usize,
    pub column: usize,
}
