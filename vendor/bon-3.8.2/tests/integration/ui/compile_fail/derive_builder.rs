use bon::Builder;

#[derive(Builder)]
struct TupleStruct(u32, u32);

#[derive(Builder)]
struct TupleStructsAreUnsupported(u32, u32);

#[derive(Builder)]
enum EnumsAreUnsupportedWithDerive {}

fn main() {}
