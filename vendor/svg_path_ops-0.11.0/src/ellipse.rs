use std::f64::consts::PI;

use cgmath::{Angle, Deg, Rad};

pub struct Ellipse {
    pub rx: f64,
    pub ry: f64,
    pub ax: Deg<f64>,
}

const EPSILON: f64 = 1e-10;

impl Ellipse {
    // constructor
    // an ellipse centred at 0 with radii rx,ry and x - axis - angle ax.
    pub fn new(rx: f64, ry: f64, ax: f64) -> Self {
        Ellipse { rx, ry, ax: Deg(ax) }
    }

    pub fn transform(&mut self, matrix: &[f64; 4]) -> &mut Self {
        let c = Rad::from(self.ax).cos();
        let s = Rad::from(self.ax).sin();
        let ma = [
            self.rx * (matrix[0] * c + matrix[2] * s),
            self.rx * (matrix[1] * c + matrix[3] * s),
            self.ry * (-matrix[0] * s + matrix[2] * c),
            self.ry * (-matrix[1] * s + matrix[3] * c),
        ];

        let j = ma[0] * ma[0] + ma[2] * ma[2];
        let k = ma[1] * ma[1] + ma[3] * ma[3];

        // the discriminant of the characteristic polynomial of ma * transpose(ma)
        let d = ((ma[0] - ma[3]) * (ma[0] - ma[3]) + (ma[2] + ma[1]) * (ma[2] + ma[1]))
            * ((ma[0] + ma[3]) * (ma[0] + ma[3]) + (ma[2] - ma[1]) * (ma[2] - ma[1]));

        // the "mean eigenvalue"
        let jk = (j + k) / 2.0;

        // check if the image is (almost) a circle
        if d < EPSILON * jk {
            // if it is
            self.rx = jk.sqrt();
            self.ry = jk.sqrt();
            self.ax = Deg(0.0);
            self
        } else {
            // if it is not a circle
            let l = ma[0] * ma[1] + ma[2] * ma[3];

            let d = d.sqrt();

            // {l1,l2} = the two eigen values of ma * transpose(ma)
            let l1 = jk + d / 2.0;
            let l2 = jk - d / 2.0;
            // the x - axis - rotation angle is the argument of the l1 - eigenvector
            self.ax = Deg(if l.abs() < EPSILON && (l1 - k).abs() < EPSILON {
                90.0
            } else {
                if l.abs() > (l1 - k).abs() {
                    (l1 - j) / l
                } else {
                    l / (l1 - k)
                }
                .atan()
                    * 180.0
                    / PI
            });

            // if ax > 0 => rx = sqrt(l1), ry = sqrt(l2), else exchange axes and ax += 90
            if self.ax >= Deg(0.0) {
                // if ax in [0,90]
                self.rx = l1.sqrt();
                self.ry = l2.sqrt();
            } else {
                // if ax in ]-90,0[ => exchange axes
                self.ax += Deg(90.0);
                self.rx = l2.sqrt();
                self.ry = l1.sqrt();
            }
            self
        }
    }

    pub fn is_degenrate(&self) -> bool {
        self.rx < EPSILON * self.ry || self.ry < EPSILON * self.rx
    }
}

#[cfg(test)]
mod test {}
