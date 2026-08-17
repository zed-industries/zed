// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate approx;
extern crate cgmath;

use cgmath::{Angle, Deg, Rad};

#[test]
fn test_normalize() {
    let angle: Rad<f64> = Rad::full_turn().normalize();
    assert_ulps_eq!(&angle, &Rad(0f64));

    let angle: Rad<f64> = (Rad::full_turn() + Rad::turn_div_4()).normalize();
    assert_ulps_eq!(&angle, &Rad::turn_div_4());

    let angle: Rad<f64> = (-Rad::turn_div_4()).normalize();
    assert_ulps_eq!(&angle, &(Rad::full_turn() - Rad::turn_div_4()));
}

#[test]
fn test_normalize_signed() {
    let angle: Rad<f64> = Rad::full_turn().normalize_signed();
    assert_ulps_eq!(&angle, &Rad(0f64));

    let angle: Rad<f64> = (Rad::full_turn() + Rad::turn_div_4()).normalize_signed();
    assert_ulps_eq!(&angle, &Rad::turn_div_4());

    let angle: Rad<f64> = (Rad::full_turn() - Rad::turn_div_4()).normalize_signed();
    assert_ulps_eq!(&angle, &-Rad::turn_div_4());

    let angle: Rad<f64> = Rad::turn_div_2().normalize_signed();
    assert_ulps_eq!(&angle, &Rad::turn_div_2());

    let angle: Rad<f64> = (-Rad::turn_div_2()).normalize_signed();
    assert_ulps_eq!(&angle, &Rad::turn_div_2());
}

#[test]
fn test_conv() {
    let angle: Rad<_> = Deg(-5.0f64).into();
    let angle: Deg<_> = angle.into();
    assert_ulps_eq!(&angle, &Deg(-5.0f64));

    let angle: Rad<_> = Deg(30.0f64).into();
    let angle: Deg<_> = angle.into();
    assert_ulps_eq!(&angle, &Deg(30.0f64));

    let angle: Deg<_> = Rad(-5.0f64).into();
    let angle: Rad<_> = angle.into();
    assert_ulps_eq!(&angle, &Rad(-5.0f64));

    let angle: Deg<_> = Rad(30.0f64).into();
    let angle: Rad<_> = angle.into();
    assert_ulps_eq!(&angle, &Rad(30.0f64));
}

mod rad {
    use cgmath::Rad;

    #[test]
    fn test_iter_sum() {
        assert_eq!(
            Rad(2.0) + Rad(3.0) + Rad(4.0),
            [Rad(2.0), Rad(3.0), Rad(4.0)].iter().sum()
        );
        assert_eq!(
            Rad(2.0) + Rad(3.0) + Rad(4.0),
            [Rad(2.0), Rad(3.0), Rad(4.0)].iter().cloned().sum()
        );
    }
}

mod deg {
    use cgmath::Deg;

    #[test]
    fn test_iter_sum() {
        assert_eq!(
            Deg(2.0) + Deg(3.0) + Deg(4.0),
            [Deg(2.0), Deg(3.0), Deg(4.0)].iter().sum()
        );
        assert_eq!(
            Deg(2.0) + Deg(3.0) + Deg(4.0),
            [Deg(2.0), Deg(3.0), Deg(4.0)].iter().cloned().sum()
        );
    }
}
