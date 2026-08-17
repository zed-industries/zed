// This test asserts that a boxed error trait object can be used as a source.

use snafu::prelude::*;

mod trait_object {
    pub type Error = Box<dyn std::error::Error + 'static>;

    pub fn function() -> Result<i32, Error> {
        Ok(42)
    }
}

mod trait_object_send {
    pub type Error = Box<dyn std::error::Error + Send + 'static>;

    pub fn function() -> Result<i32, Error> {
        Ok(42)
    }
}

mod trait_object_sync {
    pub type Error = Box<dyn std::error::Error + Sync + 'static>;

    pub fn function() -> Result<i32, Error> {
        Ok(42)
    }
}

mod trait_object_send_sync {
    pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    pub fn function() -> Result<i32, Error> {
        Ok(42)
    }
}

#[derive(Debug, Snafu)]
enum Error {
    TraitObject {
        user_id: i32,
        source: trait_object::Error,
    },

    TraitObjectSend {
        user_id: i32,
        source: trait_object_send::Error,
    },

    TraitObjectSync {
        user_id: i32,
        source: trait_object_sync::Error,
    },

    TraitObjectSendSync {
        user_id: i32,
        source: trait_object_send_sync::Error,
    },
}

fn example() -> Result<(), Error> {
    trait_object::function().context(TraitObjectSnafu { user_id: 42 })?;
    trait_object_send::function().context(TraitObjectSendSnafu { user_id: 42 })?;
    trait_object_sync::function().context(TraitObjectSyncSnafu { user_id: 42 })?;
    trait_object_send_sync::function().context(TraitObjectSendSyncSnafu { user_id: 42 })?;

    Ok(())
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
    example().unwrap();
}
