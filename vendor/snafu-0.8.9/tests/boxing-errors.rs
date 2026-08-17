use snafu::prelude::*;

trait Database: Sized {
    type Error: snafu::Error;

    fn connect() -> Result<Self, Self::Error>;
}

#[derive(Debug)]
struct ConcreteDatabase;

#[derive(Debug, Snafu)]
struct ConcreteError;

impl Database for ConcreteDatabase {
    type Error = ConcreteError;

    fn connect() -> Result<Self, Self::Error> {
        Err(ConcreteError)
    }
}

fn is_error(_: impl snafu::Error) -> bool {
    true
}

#[test]
fn make_send_sync_static() {
    #[derive(Debug, Snafu)]
    struct Error {
        source: Box<dyn snafu::Error + Send + Sync>,
    }

    fn connect_to_db<Db>() -> Result<Db, Error>
    where
        Db: Database,
        Db::Error: Send + Sync + 'static,
    {
        Db::connect().boxed().context(Snafu)
    }

    let r = connect_to_db::<ConcreteDatabase>();
    assert!(is_error(r.unwrap_err()));
}

#[test]
fn make_send_sync_non_static() {
    // Can't create a SNAFU error with non-static source (issue #99)

    fn connect_to_db<'a, Db>() -> Result<Db, Box<dyn snafu::Error + Send + Sync + 'a>>
    where
        Db: Database,
        Db::Error: Send + Sync + 'a,
    {
        Db::connect().boxed()
    }

    let r = connect_to_db::<ConcreteDatabase>();
    assert!(r.is_err());
}

#[test]
fn make_static() {
    #[derive(Debug, Snafu)]
    struct Error {
        source: Box<dyn snafu::Error + 'static>,
    }

    fn connect_to_db<Db>() -> Result<Db, Error>
    where
        Db: Database,
        Db::Error: 'static,
    {
        Db::connect().boxed_local().context(Snafu)
    }

    let r = connect_to_db::<ConcreteDatabase>();
    assert!(is_error(r.unwrap_err()));
}

#[test]
fn make_non_static() {
    // Can't create a SNAFU error with non-static source (issue #99)

    fn connect_to_db<'a, Db>() -> Result<Db, Box<dyn snafu::Error + 'a>>
    where
        Db: Database,
        Db::Error: 'a,
    {
        Db::connect().boxed_local()
    }

    let r = connect_to_db::<ConcreteDatabase>();
    assert!(r.is_err());
}
