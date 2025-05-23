use gpui_macros::reflect_methods;

#[reflect_methods]
trait Transform: Clone {
    fn double(self) -> Self;
    fn triple(self) -> Self;
    fn increment(self) -> Self {
        // Default implementation
        self.add_one()
    }
    fn quadruple(self) -> Self {
        // Default implementation with mut self
        self.double().double()
    }
    // These methods will be filtered out:
    #[allow(dead_code)]
    fn add(&self, other: &Self) -> Self;
    #[allow(dead_code)]
    fn set_value(&mut self, value: i32);
    #[allow(dead_code)]
    fn get_value(&self) -> i32;
    fn add_one(self) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
struct Number(i32);

impl Transform for Number {
    fn double(self) -> Self {
        Number(self.0 * 2)
    }

    fn triple(self) -> Self {
        Number(self.0 * 3)
    }

    fn add(&self, other: &Self) -> Self {
        Number(self.0 + other.0)
    }

    fn set_value(&mut self, value: i32) {
        self.0 = value;
    }

    fn get_value(&self) -> i32 {
        self.0
    }

    fn add_one(self) -> Self {
        Number(self.0 + 1)
    }
}

#[test]
fn test_reflect_methods() {
    use transform_reflection::*;

    // Get all methods that match the pattern fn(self) -> Self or fn(mut self) -> Self
    let methods = methods::<Number>();

    // Should have 5 methods: double, triple, increment, quadruple, add_one
    assert_eq!(methods.len(), 5);

    let method_names: Vec<_> = methods.iter().map(|m| &*m.name).collect();
    assert!(method_names.contains(&"double"));
    assert!(method_names.contains(&"triple"));
    assert!(method_names.contains(&"increment"));
    assert!(method_names.contains(&"quadruple"));
    assert!(method_names.contains(&"add_one"));

    // Invoke methods by name
    let num = Number(5);

    let doubled = invoke_method("double", num.clone()).unwrap();
    assert_eq!(doubled, Number(10));

    let tripled = invoke_method("triple", num.clone()).unwrap();
    assert_eq!(tripled, Number(15));

    let incremented = invoke_method("increment", num.clone()).unwrap();
    assert_eq!(incremented, Number(6));

    let quadrupled = invoke_method("quadruple", num.clone()).unwrap();
    assert_eq!(quadrupled, Number(20));

    // Try to invoke a non-existent method
    let result = invoke_method::<Number>("nonexistent", num.clone());
    assert!(result.is_none());

    // Chain operations
    let num = Number(10);
    let result = invoke_method("double", num)
        .and_then(|n| invoke_method("increment", n))
        .and_then(|n| invoke_method("triple", n));

    assert_eq!(result, Some(Number(63))); // (10 * 2 + 1) * 3 = 63
}
