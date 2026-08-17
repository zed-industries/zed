use itertools::assert_equal;
use any_vec::any_value::AnyValueWrapper;
use any_vec::AnyVec;

#[test]
fn remove_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    let mut vec = any_vec.downcast_mut::<String>().unwrap();
    vec.push(String::from("0"));
    vec.push(String::from("1"));
    vec.push(String::from("2"));
    vec.push(String::from("3"));
    vec.push(String::from("4"));

    // type erased remove
    vec.remove(2);
    assert_equal(vec.as_slice(), &[
        String::from("0"),
        String::from("1"),
        String::from("3"),
        String::from("4"),
    ]);

    // remove last
    vec.remove(3);
    assert_equal(vec.as_slice(), &[
        String::from("0"),
        String::from("1"),
        String::from("3"),
    ]);

    // remove first
    vec.remove(0);
    assert_equal(vec.as_slice(), &[
        String::from("1"),
        String::from("3"),
    ]);
}

#[test]
fn swap_remove_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    let mut vec = any_vec.downcast_mut::<String>().unwrap();

    vec.push(String::from("0"));
    vec.push(String::from("1"));
    vec.push(String::from("2"));
    vec.push(String::from("3"));
    vec.push(String::from("4"));

    let e: String = vec.swap_remove(1);
    assert_eq!(e, String::from("1"));
    assert_equal(vec.as_slice(), &[
        String::from("0"),
        String::from("4"),
        String::from("2"),
        String::from("3"),
    ]);

    vec.swap_remove(2);
    assert_equal(vec.as_slice(), &[
        String::from("0"),
        String::from("4"),
        String::from("3"),
    ]);

    vec.swap_remove(2);
    assert_equal(vec.as_slice(), &[
        String::from("0"),
        String::from("4"),
    ]);
}

#[test]
fn drain_all_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    let mut vec = any_vec.downcast_mut::<String>().unwrap();
    vec.push(String::from("0"));
    vec.push(String::from("1"));
    vec.push(String::from("2"));
    vec.push(String::from("3"));

    let mut vec2 = Vec::new();
    for e in vec.drain(..){
        vec2.push(e);
    }
    assert_eq!(any_vec.len(), 0);
    assert_equal(vec2, [
        String::from("0"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
    ]);
}

#[test]
fn any_vec_drain_in_the_middle_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    let mut vec = any_vec.downcast_mut::<String>().unwrap();
    vec.push(String::from("0"));
    vec.push(String::from("1"));
    vec.push(String::from("2"));
    vec.push(String::from("3"));
    vec.push(String::from("4"));

    let mut vec2 = Vec::new();
    for e in vec.drain(1..3){
        vec2.push(e);
    }
    assert_equal(vec2, [
        String::from("1"),
        String::from("2"),
    ]);
    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("3"),
        String::from("4"),
    ]);
}

#[test]
fn any_vec_splice_lifetime_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    any_vec.push(AnyValueWrapper::new(String::from("0")));
    any_vec.push(AnyValueWrapper::new(String::from("1")));
    any_vec.push(AnyValueWrapper::new(String::from("2")));
    any_vec.push(AnyValueWrapper::new(String::from("3")));
    any_vec.push(AnyValueWrapper::new(String::from("4")));

    // lifetime check ?
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        let replace = [
            String::from("100"),
            String::from("200")
        ];
        let drained = vec.splice(1..4, replace.iter().cloned());
        drained.count();
        drop(replace);
        //drained.count();
    }
}

#[test]
pub fn downcast_mut_test(){
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    let mut vec = any_vec.downcast_mut::<String>().unwrap();
    vec.push("Hello".into());
    assert_eq!(vec.len(), 1);
}

#[test]
pub fn downcast_ref_test(){
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    any_vec.clear();
    let vec1 = any_vec.downcast_ref::<String>().unwrap();
    let vec2 = any_vec.downcast_ref::<String>().unwrap();
    let vec3 = vec2.clone();
    assert_eq!(vec1.len(), 0);
    assert_eq!(vec2.len(), 0);
    assert_eq!(vec3.len(), 0);
}

#[test]
fn any_vec_into_iter_test() {
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    let mut vec = any_vec.downcast_mut::<usize>().unwrap();
    vec.push(1);
    vec.push(10);
    vec.push(100);

    let mut sum = 0;
    for e in vec{
        sum += *e;
    }
    assert_eq!(sum, 111);
}

#[test]
fn any_vec_debug() {
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    let mut vec = any_vec.downcast_mut::<usize>().unwrap();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut control_vec = Vec::new();
    control_vec.push(1);
    control_vec.push(2);
    control_vec.push(3);

    assert_eq!(format!("{vec:?}"), format!("{control_vec:?}"));
    drop(vec);

    let vec = any_vec.downcast_ref::<usize>().unwrap();
    assert_eq!(format!("{vec:?}"), format!("{control_vec:?}"));
}

/*
#[test]
fn any_vec_index_test() {
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    let mut vec = any_vec.downcast_mut::<usize>().unwrap();
    vec.push(1);
    vec.push(10);
    vec.push(100);

    assert_eq!(vec[1], 10);
    assert_eq!(vec[..], [1, 10, 100]);
    assert_eq!(vec[1..3], [10, 100]);
}
 */