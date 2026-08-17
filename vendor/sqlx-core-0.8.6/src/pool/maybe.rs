use crate::database::Database;
use crate::pool::PoolConnection;
use std::ops::{Deref, DerefMut};

pub enum MaybePoolConnection<'c, DB: Database> {
    #[allow(dead_code)]
    Connection(&'c mut DB::Connection),
    PoolConnection(PoolConnection<DB>),
}

impl<'c, DB: Database> Deref for MaybePoolConnection<'c, DB> {
    type Target = DB::Connection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            MaybePoolConnection::Connection(v) => v,
            MaybePoolConnection::PoolConnection(v) => v,
        }
    }
}

impl<'c, DB: Database> DerefMut for MaybePoolConnection<'c, DB> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MaybePoolConnection::Connection(v) => v,
            MaybePoolConnection::PoolConnection(v) => v,
        }
    }
}

impl<'c, DB: Database> From<PoolConnection<DB>> for MaybePoolConnection<'c, DB> {
    fn from(v: PoolConnection<DB>) -> Self {
        MaybePoolConnection::PoolConnection(v)
    }
}

impl<'c, DB: Database> From<&'c mut DB::Connection> for MaybePoolConnection<'c, DB> {
    fn from(v: &'c mut DB::Connection) -> Self {
        MaybePoolConnection::Connection(v)
    }
}
