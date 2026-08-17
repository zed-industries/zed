use cxx::CxxVector;

#[test]
fn test_cxx_vector_new() {
    let vector = CxxVector::<i32>::new();
    assert!(vector.is_empty());
}
