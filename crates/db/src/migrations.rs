use rusqlite_migration::{Migrations, M};

// use crate::items::ITEMS_M_1;
use crate::kvp::KVP_M_1;

// This must be ordered by development time! Only ever add new migrations to the end!!
// Bad things will probably happen if you don't monotonically edit this vec!!!!
// And no re-ordering ever!!!!!!!!!! The results of these migrations are on the user's
// file system and so everything we do here is locked in _f_o_r_e_v_e_r_.
lazy_static::lazy_static! {
    pub static ref MIGRATIONS: Migrations<'static> = Migrations::new(vec![
        M::up(KVP_M_1),
        // M::up(ITEMS_M_1),
    ]);
}
