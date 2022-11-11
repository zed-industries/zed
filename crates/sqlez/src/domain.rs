use crate::connection::Connection;

pub trait Domain {
    fn migrate(conn: &Connection) -> anyhow::Result<()>;
}

impl<D1: Domain, D2: Domain> Domain for (D1, D2) {
    fn migrate(conn: &Connection) -> anyhow::Result<()> {
        D1::migrate(conn)?;
        D2::migrate(conn)
    }
}

impl<D1: Domain, D2: Domain, D3: Domain> Domain for (D1, D2, D3) {
    fn migrate(conn: &Connection) -> anyhow::Result<()> {
        D1::migrate(conn)?;
        D2::migrate(conn)?;
        D3::migrate(conn)
    }
}

impl<D1: Domain, D2: Domain, D3: Domain, D4: Domain> Domain for (D1, D2, D3, D4) {
    fn migrate(conn: &Connection) -> anyhow::Result<()> {
        D1::migrate(conn)?;
        D2::migrate(conn)?;
        D3::migrate(conn)?;
        D4::migrate(conn)
    }
}

impl<D1: Domain, D2: Domain, D3: Domain, D4: Domain, D5: Domain> Domain for (D1, D2, D3, D4, D5) {
    fn migrate(conn: &Connection) -> anyhow::Result<()> {
        D1::migrate(conn)?;
        D2::migrate(conn)?;
        D3::migrate(conn)?;
        D4::migrate(conn)?;
        D5::migrate(conn)
    }
}
