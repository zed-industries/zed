rm -rf sea-orm-macros/src/strum/helpers
rm -rf sea-orm-macros/src/strum/enum_iter.rs

cp -r ../strum/strum_macros/src/helpers sea-orm-macros/src/strum/helpers
cp -r ../strum/strum_macros/src/macros/enum_iter.rs sea-orm-macros/src/strum/enum_iter.rs

sed -i 's/crate::helpers::{*/super::helpers::{/' sea-orm-macros/src/strum/enum_iter.rs
sed -i 's/parse_quote!(::strum)*/parse_quote!(sea_orm::strum)/' sea-orm-macros/src/strum/helpers/type_props.rs