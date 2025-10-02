use crate::connection::Connection;

pub trait Domain: 'static {
    const NAME: &str;
    const MIGRATIONS: &[&str];

    fn should_allow_migration_change(_index: usize, _old: &str, _new: &str) -> bool {
        false
    }
}

pub trait Migrator: 'static {
    fn migrate(connection: &Connection) -> anyhow::Result<()>;
}

impl Migrator for () {
    fn migrate(_connection: &Connection) -> anyhow::Result<()> {
        Ok(()) // Do nothing
    }
}

impl<D: Domain> Migrator for D {
    fn migrate(connection: &Connection) -> anyhow::Result<()> {
        connection.migrate(
            Self::NAME,
            Self::MIGRATIONS,
            Self::should_allow_migration_change,
        )
    }
}

impl<D1: Domain, D2: Domain> Migrator for (D1, D2) {
    fn migrate(connection: &Connection) -> anyhow::Result<()> {
        D1::migrate(connection)?;
        D2::migrate(connection)
    }
}

impl<D1: Domain, D2: Domain, D3: Domain> Migrator for (D1, D2, D3) {
    fn migrate(connection: &Connection) -> anyhow::Result<()> {
        D1::migrate(connection)?;
        D2::migrate(connection)?;
        D3::migrate(connection)
    }
}

impl<D1: Domain, D2: Domain, D3: Domain, D4: Domain> Migrator for (D1, D2, D3, D4) {
    fn migrate(connection: &Connection) -> anyhow::Result<()> {
        D1::migrate(connection)?;
        D2::migrate(connection)?;
        D3::migrate(connection)?;
        D4::migrate(connection)
    }
}

impl<D1: Domain, D2: Domain, D3: Domain, D4: Domain, D5: Domain> Migrator for (D1, D2, D3, D4, D5) {
    fn migrate(connection: &Connection) -> anyhow::Result<()> {
        D1::migrate(connection)?;
        D2::migrate(connection)?;
        D3::migrate(connection)?;
        D4::migrate(connection)?;
        D5::migrate(connection)
    }
}
