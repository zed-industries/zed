# Design

We are heavily inspired by ActiveRecord, Eloquent and TypeORM.

1. Intuitive and ergonomic

API should state the intention clearly. Provide syntax sugar for common things.

2. Fast(er) compilation

Balance between compile-time checking and compilation speed.

3. Avoid 'symbol soup'

Avoid macros with DSL, use derive macros where appropriate. Be friendly with IDE tools.

## Test Time

After some bitterness we realized it is not possible to capture everything at compile time. But we don't 
want to encounter problems at run time either. The solution is to perform checking at 'test time' to
uncover problems. These checks will be removed at production so there will be no run time penalty.

## API style

### Turbofish and inference

Consider the following method:
```rust
fn left_join<E>(self) -> Self
where
    E: EntityTrait,
{
    // ...
}
```
which has to be invoked like:
```rust
.left_join::<fruit::Entity>()
```

If we instead do:
```rust
fn left_join<E>(self, _: E) -> Self
where
    E: EntityTrait,
{
    // ...
}
```
then the Turbofish can be omitted:
```rust
.left_join(fruit::Entity)
```
provided that `fruit::Entity` is a unit struct.

### Builder pattern

Instead of:
```rust
fn has_many(entity: Entity, from: Column, to: Column);

has_many(cake::Entity, cake::Column::Id, fruit::Column::CakeId)
```

we'd prefer having a builder and stating the params explicitly:
```rust
has_many(cake::Entity).from(cake::Column::Id).to(fruit::Column::CakeId)
```

### Method overloading

Consider the following two methods, which accept the same parameter but in different forms:

```rust
fn method_with_model(m: Model) { ... }
fn method_with_active_model(a: ActiveModel) { ... }
```

We would define a trait

```rust
pub trait IntoActiveModel {
    fn into_active_model(self) -> ActiveModel;
}
```

Such that `Model` and `ActiveModel` both impl this trait.

In this way, we can overload the two methods:

```rust
pub fn method<A>(a: A)
where
    A: IntoActiveModel,
{
    let a: ActiveModel = a.into_active_model();
    ...
}
```
