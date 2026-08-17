use crate::Iden;

/// PostgreSQL `ltree` extension type.
///
/// `ltree` stores a raber path which in this struct is represented as the
/// tuple's first value.
///
/// # PostcreSQL Reference
///
/// The following set of SQL statements can be used to create a table with
/// a `ltree` column. Here the `ltree` column is called `path`.
///
/// The `path` column is then populated to generate the tree.
///
/// ```ignore
/// CREATE TABLE test (path ltree);
/// INSERT INTO test VALUES ('Top');
/// INSERT INTO test VALUES ('Top.Science');
/// INSERT INTO test VALUES ('Top.Science.Astronomy');
/// INSERT INTO test VALUES ('Top.Science.Astronomy.Astrophysics');
/// INSERT INTO test VALUES ('Top.Science.Astronomy.Cosmology');
/// INSERT INTO test VALUES ('Top.Hobbies');
/// INSERT INTO test VALUES ('Top.Hobbies.Amateurs_Astronomy');
/// INSERT INTO test VALUES ('Top.Collections');
/// INSERT INTO test VALUES ('Top.Collections.Pictures');
/// INSERT INTO test VALUES ('Top.Collections.Pictures.Astronomy');
/// INSERT INTO test VALUES ('Top.Collections.Pictures.Astronomy.Stars');
/// INSERT INTO test VALUES ('Top.Collections.Pictures.Astronomy.Galaxies');
/// INSERT INTO test VALUES ('Top.Collections.Pictures.Astronomy.Astronauts');
/// CREATE INDEX path_gist_idx ON test USING GIST (path);
/// CREATE INDEX path_idx ON test USING BTREE (path);
/// ```
///
/// The set of queries above will generate the following tree:
///
/// ```ignore
///                        Top
///                     /   |  \
///              Science Hobbies Collections
///                /       |              \
///       Astronomy   Amateurs_Astronomy Pictures
///            /  \                            |
/// Astrophysics  Cosmology                Astronomy
///                                       /    |    \
///                                Galaxies  Stars  Astronauts
/// ```
/// [Source][1]
///
/// [1]: https://www.postgresql.org/docs/current/ltree.html
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PgLTree;

impl Iden for PgLTree {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "ltree").unwrap();
    }
}

impl From<PgLTree> for String {
    fn from(l: PgLTree) -> Self {
        l.to_string()
    }
}
