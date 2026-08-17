#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct SqlxValues(pub sea_query::Values);
