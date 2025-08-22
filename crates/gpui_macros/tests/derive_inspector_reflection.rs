//! This code was generated using Zed Agent with Claude Opus 4.

use gpui_macros::derive_inspector_reflection;

#[derive_inspector_reflection]
trait Transform: Clone {
    /// Doubles the value
    fn double(self) -> Self;

    /// Triples the value
    fn triple(self) -> Self;

    /// Increments the value by one
    ///
    /// This method has a default implementation
    fn increment(self) -> Self {
        // Default implementation
        self.add_one()
    }

    /// Quadruples the value by doubling twice
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

    /// Adds one to the value
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
fn test_derive_inspector_reflection() {
    use transform_reflection::*;

    // Get all methods that match the pattern fn(self) -> Self or fn(mut self) -> Self
    let methods = methods::<Number>();

    assert_eq!(methods.len(), 5);
    let method_names: Vec<_> = methods.iter().map(|m| m.name).collect();
    assert!(method_names.contains(&"double"));
    assert!(method_names.contains(&"triple"));
    assert!(method_names.contains(&"increment"));
    assert!(method_names.contains(&"quadruple"));
    assert!(method_names.contains(&"add_one"));

    // Invoke methods by name
    let num = Number(5);

    let doubled = find_method::<Number>("double").unwrap().invoke(num.clone());
    assert_eq!(doubled, Number(10));

    let tripled = find_method::<Number>("triple").unwrap().invoke(num.clone());
    assert_eq!(tripled, Number(15));

    let incremented = find_method::<Number>("increment")
        .unwrap()
        .invoke(num.clone());
    assert_eq!(incremented, Number(6));

    let quadrupled = find_method::<Number>("quadruple").unwrap().invoke(num);
    assert_eq!(quadrupled, Number(20));

    // Try to invoke a non-existent method
    let result = find_method::<Number>("nonexistent");
    assert!(result.is_none());

    // Chain operations
    let num = Number(10);
    let result = find_method::<Number>("double")
        .map(|m| m.invoke(num))
        .and_then(|n| find_method::<Number>("increment").map(|m| m.invoke(n)))
        .and_then(|n| find_method::<Number>("triple").map(|m| m.invoke(n)));

    assert_eq!(result, Some(Number(63))); // (10 * 2 + 1) * 3 = 63

    // Test documentationumentation capture
    let double_method = find_method::<Number>("double").unwrap();
    assert_eq!(double_method.documentation, Some("Doubles the value"));

    let triple_method = find_method::<Number>("triple").unwrap();
    assert_eq!(triple_method.documentation, Some("Triples the value"));

    let increment_method = find_method::<Number>("increment").unwrap();
    assert_eq!(
        increment_method.documentation,
        Some("Increments the value by one\n\nThis method has a default implementation")
    );

    let quadruple_method = find_method::<Number>("quadruple").unwrap();
    assert_eq!(
        quadruple_method.documentation,
        Some("Quadruples the value by doubling twice")
    );

    let add_one_method = find_method::<Number>("add_one").unwrap();
    assert_eq!(add_one_method.documentation, Some("Adds one to the value"));
}
