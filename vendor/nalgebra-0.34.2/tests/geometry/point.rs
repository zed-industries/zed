use na::{Point3, Vector3, Vector4};
use num::Zero;

#[test]
fn point_clone() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let p2 = p.clone();
    assert_eq!(p, p2);
}

#[test]
fn point_ops() {
    let a = Point3::new(1.0, 2.0, 3.0);
    let b = Point3::new(1.0, 2.0, 3.0);
    let c = Vector3::new(1.0, 2.0, 3.0);

    assert_eq!(a - b, Vector3::zero());
    assert_eq!(&a - &b, Vector3::zero());
    assert_eq!(a - &b, Vector3::zero());
    assert_eq!(&a - b, Vector3::zero());

    assert_eq!(b - c, Point3::origin());
    assert_eq!(&b - &c, Point3::origin());
    assert_eq!(b - &c, Point3::origin());
    assert_eq!(&b - c, Point3::origin());

    assert_eq!(b + c, 2.0 * a);
    assert_eq!(&b + &c, 2.0 * a);
    assert_eq!(b + &c, 2.0 * a);
    assert_eq!(&b + c, 2.0 * a);

    let mut a1 = a;
    let mut a2 = a;
    let mut a3 = a;
    let mut a4 = a;

    a1 -= c;
    a2 -= &c;
    a3 += c;
    a4 += &c;

    assert_eq!(a1, Point3::origin());
    assert_eq!(a2, Point3::origin());
    assert_eq!(a3, 2.0 * a);
    assert_eq!(a4, 2.0 * a);
}

#[test]
fn point_coordinates() {
    let mut pt = Point3::origin();

    assert_eq!(pt.x, 0);
    assert_eq!(pt.y, 0);
    assert_eq!(pt.z, 0);

    pt.x = 1;
    pt.y = 2;
    pt.z = 3;

    assert_eq!(pt.x, 1);
    assert_eq!(pt.y, 2);
    assert_eq!(pt.z, 3);
}

#[test]
fn point_scale() {
    let pt = Point3::new(1, 2, 3);
    let expected = Point3::new(10, 20, 30);

    assert_eq!(pt * 10, expected);
    assert_eq!(&pt * 10, expected);
    assert_eq!(10 * pt, expected);
    assert_eq!(10 * &pt, expected);
}

#[test]
fn point_vector_sum() {
    let pt = Point3::new(1, 2, 3);
    let vec = Vector3::new(10, 20, 30);
    let expected = Point3::new(11, 22, 33);

    assert_eq!(&pt + &vec, expected);
    assert_eq!(pt + &vec, expected);
    assert_eq!(&pt + vec, expected);
    assert_eq!(pt + vec, expected);
}

#[test]
fn to_homogeneous() {
    let a = Point3::new(1.0, 2.0, 3.0);
    let expected = Vector4::new(1.0, 2.0, 3.0, 1.0);

    assert_eq!(a.to_homogeneous(), expected);
}

#[test]
fn display_fmt_respects_modifiers() {
    let p = Point3::new(1.23, 3.45, 5.67);
    assert_eq!(&format!("{p}"), "{1.23, 3.45, 5.67}");
    assert_eq!(&format!("{p:.1}"), "{1.2, 3.5, 5.7}");
    assert_eq!(&format!("{p:.0}"), "{1, 3, 6}");
}
