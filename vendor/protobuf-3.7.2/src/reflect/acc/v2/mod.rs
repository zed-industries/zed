use crate::reflect::acc::v2::map::MapFieldAccessorHolder;
use crate::reflect::acc::v2::repeated::RepeatedFieldAccessorHolder;
use crate::reflect::acc::v2::singular::SingularFieldAccessorHolder;

pub(crate) mod map;
pub(crate) mod repeated;
pub(crate) mod singular;

#[derive(Debug)]
pub(crate) enum AccessorV2 {
    Singular(SingularFieldAccessorHolder),
    Repeated(RepeatedFieldAccessorHolder),
    Map(MapFieldAccessorHolder),
}
