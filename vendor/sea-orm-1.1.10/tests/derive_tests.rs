use sea_orm::{FromQueryResult, TryGetable};

#[derive(FromQueryResult)]
struct SimpleTest {
    _foo: i32,
    _bar: String,
}

#[derive(FromQueryResult)]
struct GenericTest<T: TryGetable> {
    _foo: i32,
    _bar: T,
}

#[derive(FromQueryResult)]
struct DoubleGenericTest<T: TryGetable, F: TryGetable> {
    _foo: T,
    _bar: F,
}

#[derive(FromQueryResult)]
struct BoundsGenericTest<T: TryGetable + Copy + Clone + 'static> {
    _foo: T,
}

#[derive(FromQueryResult)]
struct WhereGenericTest<T>
where
    T: TryGetable + Copy + Clone + 'static,
{
    _foo: T,
}

#[derive(FromQueryResult)]
struct AlreadySpecifiedBoundsGenericTest<T: TryGetable> {
    _foo: T,
}

#[derive(FromQueryResult)]
struct MixedGenericTest<T: TryGetable + Clone, F>
where
    F: TryGetable + Copy + Clone + 'static,
{
    _foo: T,
    _bar: F,
}

trait MyTrait {
    type Item: TryGetable;
}

#[derive(FromQueryResult)]
struct TraitAssociateTypeTest<T>
where
    T: MyTrait,
{
    _foo: T::Item,
}

#[derive(FromQueryResult)]
struct FromQueryAttributeTests {
    #[sea_orm(skip)]
    _foo: i32,
    _bar: String,
}

#[derive(FromQueryResult)]
struct FromQueryResultNested {
    #[sea_orm(nested)]
    _test: SimpleTest,
}
