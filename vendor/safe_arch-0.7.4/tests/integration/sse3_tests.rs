use super::*;

#[test]
fn test_addsub_m128d() {
  let a = m128d::from_array([10.0, 50.0]);
  let b = m128d::from_array([100.0, 500.0]);
  let c = addsub_m128d(a, b).to_array();
  assert_eq!(c, [-90.0, 550.0]);
}

#[test]
fn test_addsub_m128() {
  let a = m128::from_array([10.0, 20.0, 30.0, 40.0]);
  let b = m128::from_array([100.0, 200.0, 300.0, 400.0]);
  let c = addsub_m128(a, b).to_array();
  assert_eq!(c, [-90.0, 220.0, -270.0, 440.0]);
}

#[test]
fn test_add_horizontal_m128d() {
  let a = m128d::from_array([10.0, 50.0]);
  let b = m128d::from_array([100.0, 500.0]);
  let c = add_horizontal_m128d(a, b).to_array();
  assert_eq!(c, [60.0, 600.0]);
}

#[test]
fn test_add_horizontal_m128() {
  let a = m128::from_array([10.0, 20.0, 30.0, 40.0]);
  let b = m128::from_array([100.0, 200.0, 300.0, 400.0]);
  let c = add_horizontal_m128(a, b).to_array();
  assert_eq!(c, [30.0, 70.0, 300.0, 700.0]);
}

#[test]
fn test_sub_horizontal_m128d() {
  let a = m128d::from_array([10.0, 50.0]);
  let b = m128d::from_array([100.0, 500.0]);
  let c = sub_horizontal_m128d(a, b).to_array();
  assert_eq!(c, [-40.0, -400.0]);
}

#[test]
fn test_sub_horizontal_m128() {
  let a = m128::from_array([10.0, 20.0, 30.0, 45.0]);
  let b = m128::from_array([100.0, 200.0, 300.0, 450.0]);
  let c = sub_horizontal_m128(a, b).to_array();
  assert_eq!(c, [-10.0, -15.0, -100.0, -150.0]);
}

#[test]
fn test_duplicate_low_lane_m128d_s() {
  let a = m128d::from_array([1.0, 2.0]);
  let b = duplicate_low_lane_m128d_s(a);
  assert_eq!(b.to_array(), [1.0, 1.0]);
}

#[test]
fn test_duplicate_odd_lanes_m128() {
  let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
  let b = duplicate_odd_lanes_m128(a);
  assert_eq!(b.to_array(), [1.0, 1.0, 3.0, 3.0]);
}

#[test]
fn test_duplicate_even_lanes_m128() {
  let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
  let b = duplicate_even_lanes_m128(a);
  assert_eq!(b.to_array(), [0.0, 0.0, 2.0, 2.0]);
}
