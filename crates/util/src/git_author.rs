/// Represents the common denominator of most git hosting authors
#[derive(Debug)]
pub struct GitAuthor {
    pub id: u64,
    pub email: String,
    pub avatar_url: String,
}
