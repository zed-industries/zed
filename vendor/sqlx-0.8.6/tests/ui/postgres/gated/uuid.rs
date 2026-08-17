fn main() {
    let _ = sqlx::query!("select 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'::uuid");
    let _ = sqlx::query!("select $1::uuid", ());
}
