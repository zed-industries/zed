use std::iter::{Extend, IntoIterator};

#[derive(Debug, Default)]
pub struct AnyQueryResult {
    #[doc(hidden)]
    pub rows_affected: u64,
    #[doc(hidden)]
    pub last_insert_id: Option<i64>,
}

impl AnyQueryResult {
    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }

    pub fn last_insert_id(&self) -> Option<i64> {
        self.last_insert_id
    }
}

impl Extend<AnyQueryResult> for AnyQueryResult {
    fn extend<T: IntoIterator<Item = AnyQueryResult>>(&mut self, iter: T) {
        for elem in iter {
            self.rows_affected += elem.rows_affected;
            self.last_insert_id = elem.last_insert_id;
        }
    }
}
