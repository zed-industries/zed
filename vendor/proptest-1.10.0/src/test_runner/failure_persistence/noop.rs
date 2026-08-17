//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{fmt, Box, Vec};
use core::any::Any;

use crate::test_runner::failure_persistence::{
    FailurePersistence, PersistedSeed,
};

/// Failure persistence option that loads and saves nothing at all.
#[derive(Debug, Default, PartialEq)]
#[allow(dead_code)]
struct NoopFailurePersistence;

impl FailurePersistence for NoopFailurePersistence {
    fn load_persisted_failures2(
        &self,
        _source_file: Option<&'static str>,
    ) -> Vec<PersistedSeed> {
        Vec::new()
    }

    fn save_persisted_failure2(
        &mut self,
        _source_file: Option<&'static str>,
        _seed: PersistedSeed,
        _shrunken_value: &dyn fmt::Debug,
    ) {
    }

    fn box_clone(&self) -> Box<dyn FailurePersistence> {
        Box::new(NoopFailurePersistence)
    }

    fn eq(&self, other: &dyn FailurePersistence) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_runner::failure_persistence::tests::*;

    #[test]
    fn default_load_is_empty() {
        assert!(NoopFailurePersistence::default()
            .load_persisted_failures2(None)
            .is_empty());
        assert!(NoopFailurePersistence::default()
            .load_persisted_failures2(HI_PATH)
            .is_empty());
    }

    #[test]
    fn seeds_not_recoverable() {
        let mut p = NoopFailurePersistence::default();
        p.save_persisted_failure2(HI_PATH, INC_SEED, &"");
        assert!(p.load_persisted_failures2(HI_PATH).is_empty());
        assert!(p.load_persisted_failures2(None).is_empty());
        assert!(p.load_persisted_failures2(UNREL_PATH).is_empty());
    }
}
