use crate::{
    join_tbl_on_condition, unpack_table_ref, EntityTrait, QuerySelect, RelationDef, Select,
};
use sea_query::{Alias, Condition, IntoIden, JoinType, SeaRc};

/// Same as [RelationDef]
pub type LinkDef = RelationDef;

/// A Trait for links between Entities
pub trait Linked {
    #[allow(missing_docs)]
    type FromEntity: EntityTrait;

    #[allow(missing_docs)]
    type ToEntity: EntityTrait;

    /// Link for an Entity
    fn link(&self) -> Vec<LinkDef>;

    /// Find all the Entities that are linked to the Entity
    fn find_linked(&self) -> Select<Self::ToEntity> {
        let mut select = Select::new();
        for (i, mut rel) in self.link().into_iter().rev().enumerate() {
            let from_tbl = Alias::new(format!("r{i}")).into_iden();
            let to_tbl = if i > 0 {
                Alias::new(format!("r{}", i - 1)).into_iden()
            } else {
                unpack_table_ref(&rel.to_tbl)
            };
            let table_ref = rel.from_tbl;

            let mut condition = Condition::all().add(join_tbl_on_condition(
                SeaRc::clone(&from_tbl),
                SeaRc::clone(&to_tbl),
                rel.from_col,
                rel.to_col,
            ));
            if let Some(f) = rel.on_condition.take() {
                condition = condition.add(f(SeaRc::clone(&from_tbl), SeaRc::clone(&to_tbl)));
            }

            select
                .query()
                .join_as(JoinType::InnerJoin, table_ref, from_tbl, condition);
        }
        select
    }
}
