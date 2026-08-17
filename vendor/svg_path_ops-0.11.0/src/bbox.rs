#[derive(Debug, Clone, Default)]
pub struct BBox {
    pub min_x: Option<f64>,
    pub min_y: Option<f64>,
    pub max_x: Option<f64>,
    pub max_y: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum ScaleType {
    Fit,   // Scale the box (aspect ratio is not preserved) to fit
    Meet,  // Scale the box (preserve ratio) as much as possible
    Slice, // Scale the box (preserve ratio) as less as possible
    Move,  // Translate only (no scale)
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Min,
    Mid,
    Max,
}

#[derive(Debug, Clone, Copy)]
pub struct BoxAlignment {
    pub x: Alignment,
    pub y: Alignment,
}

#[derive(Debug, Clone)]
pub struct InboxParameters {
    pub destination: BBox,
    pub scale_type: ScaleType,
    pub alignment: BoxAlignment,
}

impl Default for InboxParameters {
    fn default() -> Self {
        Self {
            destination: BBox::default(),
            scale_type: ScaleType::Meet,
            alignment: BoxAlignment { x: Alignment::Mid, y: Alignment::Mid },
        }
    }
}

impl BBox {
    pub fn new() -> Self {
        BBox { min_x: None, min_y: None, max_x: None, max_y: None }
    }
    pub fn from(s: &str) -> Self {
        let mut box_ = BBox::default();

        // Parse the string and update the box
        let numbers: Vec<f64> = s
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() >= 4 {
            let x = numbers[0];
            let y = numbers[1];
            let width = numbers[2];
            let height = numbers[3];

            box_.add_x(x);
            box_.add_x(x + width);
            box_.add_y(y);
            box_.add_y(y + height);
        }

        box_
    }

    pub fn copy(&self) -> Self {
        BBox {
            min_x: self.min_x,
            min_y: self.min_y,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }

    pub fn width(&self) -> f64 {
        match (self.min_x, self.max_x) {
            (Some(min_x), Some(max_x)) => max_x - min_x,
            _ => 0.0,
        }
    }

    pub fn height(&self) -> f64 {
        match (self.min_y, self.max_y) {
            (Some(min_y), Some(max_y)) => max_y - min_y,
            _ => 0.0,
        }
    }

    // isUndefined method
    fn is_undefined(&self) -> bool {
        self.min_x.is_none() || self.min_y.is_none()
    }

    pub fn add_x(&mut self, x: f64) -> &mut Self {
        match self.min_x {
            None => {
                // If undefined, set both min and max to x
                self.min_x = Some(x);
                self.max_x = Some(x);
            }
            Some(min_x) => {
                // If defined, update min and max accordingly
                self.min_x = Some(min_x.min(x));
                self.max_x = Some(self.max_x.unwrap().max(x));
            }
        }
        self
    }

    pub fn add_y(&mut self, y: f64) -> &mut Self {
        match self.min_y {
            None => {
                self.min_y = Some(y);
                self.max_y = Some(y);
            }
            Some(min_y) => {
                self.min_y = Some(min_y.min(y));
                self.max_y = Some(self.max_y.unwrap().max(y));
            }
        }
        self
    }

    fn add_point(&mut self, x: f64, y: f64) -> &mut Self {
        self.add_x(x).add_y(y)
    }

    /// Add new quadratic curve to X coordinate
    pub fn add_x_q(&mut self, a: &[f64]) -> &mut Self {
        let minmax = minmax_q(a);
        self.add_x(minmax[0]).add_x(minmax[1])
    }

    /// Add new quadratic curve to Y coordinate
    pub fn add_y_q(&mut self, a: &[f64]) -> &mut Self {
        let minmax = minmax_q(a);
        self.add_y(minmax[0]).add_y(minmax[1])
    }

    /// Add new cubic curve to X coordinate
    ///
    /// # Arguments
    /// * `a` - Slice containing four coefficients of the cubic curve
    ///
    /// # Returns
    /// * `Result<&mut Self, &'static str>` - Returns self for method chaining or an error
    pub fn add_x_c(&mut self, a: &[f64]) -> &mut Self {
        let minmax = minmax_c(a);
        self.add_x(minmax[0]).add_x(minmax[1])
    }

    pub fn add_y_c(&mut self, a: &[f64]) -> &mut Self {
        let minmax = minmax_c(a);
        self.add_y(minmax[0]).add_y(minmax[1])
    }

    /// Convert box to array representation
    ///
    /// # Returns
    /// * `Result<[f64; 4], &'static str>` - Array of coordinates or error if undefined
    pub fn to_array(&self) -> Result<[f64; 4], &'static str> {
        if self.is_undefined() {
            Ok([0.0, 0.0, 0.0, 0.0])
            // Or alternatively, if you want to handle undefined as an error:
            // Err("Box is undefined")
        } else {
            Ok([
                self.min_x.unwrap(),
                self.min_y.unwrap(),
                self.max_x.unwrap(),
                self.max_y.unwrap(),
            ])
        }
    }

    /// Returns a string representation of the Box in format "minX minY width height"
    /// If pr (precision) is provided, formats numbers with specified decimal places
    pub fn to_string(&self, pr: Option<usize>) -> String {
        // If box is undefined/empty
        if self.is_undefined() {
            return String::from("0 0 0 0");
        }

        match pr {
            // No precision specified - use regular formatting
            None => {
                format!(
                    "{} {} {} {}",
                    self.min_x.unwrap(),
                    self.min_y.unwrap(),
                    self.width(),
                    self.height()
                )
            }
            // Precision specified - format with fixed decimal places
            Some(precision) => {
                format!(
                    "{:.*} {:.*} {:.*} {:.*}",
                    precision,
                    self.min_x.unwrap(),
                    precision,
                    self.min_y.unwrap(),
                    precision,
                    self.width(),
                    precision,
                    self.height()
                )
            }
        }
    }

    pub fn inbox_matrix(&self, params: &InboxParameters) -> [f64; 6] {
        // Calculate scale factors based on action
        let (rx, ry) = self.calculate_scale_factors(&params.destination, params.scale_type);

        // Calculate origins for source and destination boxes
        let src_origin = self.calculate_origin(params.alignment);
        let dst_origin = params.destination.calculate_origin(params.alignment);

        // Return transformation matrix
        [
            rx,
            0.0,
            0.0,
            ry,
            dst_origin.x - rx * src_origin.x,
            dst_origin.y - ry * src_origin.y,
        ]
    }

    fn calculate_scale_factors(&self, dst: &BBox, scale_type: ScaleType) -> (f64, f64) {
        match scale_type {
            ScaleType::Fit => (
                if self.width() != 0.0 {
                    dst.width() / self.width()
                } else {
                    1.0
                },
                if self.height() != 0.0 {
                    dst.height() / self.height()
                } else {
                    1.0
                },
            ),

            ScaleType::Slice => {
                if self.width() != 0.0 && self.height() != 0.0 {
                    let scale = f64::max(dst.width() / self.width(), dst.height() / self.height());
                    (scale, scale)
                } else {
                    self.calculate_preserved_ratio_scale(dst)
                }
            }

            ScaleType::Meet => self.calculate_preserved_ratio_scale(dst),

            ScaleType::Move => (1.0, 1.0),
        }
    }

    fn calculate_preserved_ratio_scale(&self, dst: &BBox) -> (f64, f64) {
        let scale = if self.width() == 0.0 && self.height() == 0.0 {
            1.0
        } else {
            f64::min(dst.width() / self.width(), dst.height() / self.height())
        };
        (scale, scale)
    }

    fn calculate_origin(&self, alignment: BoxAlignment) -> Point {
        Point {
            x: match alignment.x {
                Alignment::Min => self.min_x.unwrap_or(0.0),
                Alignment::Mid => (self.min_x.unwrap_or(0.0) + self.max_x.unwrap_or(0.0)) / 2.0,
                Alignment::Max => self.max_x.unwrap_or(0.0),
            },
            y: match alignment.y {
                Alignment::Min => self.min_y.unwrap_or(0.0),
                Alignment::Mid => (self.min_y.unwrap_or(0.0) + self.max_y.unwrap_or(0.0)) / 2.0,
                Alignment::Max => self.max_y.unwrap_or(0.0),
            },
        }
    }
}

const EPSILON: f64 = 1e-10; // Define epsilon as needed

pub fn minmax_c(a: &[f64]) -> [f64; 2] {
    // if the polynomial is (almost) quadratic and not cubic
    let k = a[0] - 3.0 * a[1] + 3.0 * a[2] - a[3];
    if k.abs() < EPSILON {
        return minmax_q(&[
            a[0],
            -0.5 * a[0] + 1.5 * a[1],
            a[0] - 3.0 * a[1] + 3.0 * a[2],
        ]);
    }

    // the reduced discriminant of the derivative
    let t = -a[0] * a[2] + a[0] * a[3] - a[1] * a[2] - a[1] * a[3] + a[1] * a[1] + a[2] * a[2];

    // if the polynomial is monotone in [0,1]
    if t <= 0.0 {
        return [a[0].min(a[3]), a[0].max(a[3])];
    }

    let s = t.sqrt();

    // potential extrema
    let mut max = a[0].max(a[3]);
    let mut min = a[0].min(a[3]);

    let l = a[0] - 2.0 * a[1] + a[2];

    // check both local extrema
    let roots = [(l + s) / k, (l - s) / k];

    for &r in &roots {
        if r > 0.0 && r < 1.0 {
            // if the extrema is for r in [0,1]
            let one_minus_r = 1.0 - r;
            let q = a[0] * one_minus_r * one_minus_r * one_minus_r
                + a[1] * 3.0 * one_minus_r * one_minus_r * r
                + a[2] * 3.0 * one_minus_r * r * r
                + a[3] * r * r * r;

            min = min.min(q);
            max = max.max(q);
        }
    }

    [min, max]
}

pub fn minmax_q(a: &[f64]) -> [f64; 2] {
    if a.len() < 3 {
        panic!("Array must have at least 3 elements");
    }

    let min = a[0].min(a[2]);
    let max = a[0].max(a[2]);

    // if no extremum in ]0,1[
    if (a[1] > a[0] && a[2] >= a[1]) || (a[1] <= a[0] && a[2] <= a[1]) {
        return [min, max];
    }

    // check if the extremum E is min or max
    let e = (a[0] * a[2] - a[1] * a[1]) / (a[0] - 2.0 * a[1] + a[2]);

    if e < min {
        [e, max]
    } else {
        [min, e]
    }
}

#[cfg(test)]
mod test {
    macro_rules! assert_approx_eq {
        ($left:expr, $right:expr, epsilon = $epsilon:expr) => {
            assert!(
                ($left - $right).abs() < $epsilon,
                "assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n epsilon: `{}`",
                $left,
                $right,
                $epsilon
            );
        };
    }

    use super::*;

    #[test]
    fn default_box_is_undefined_with_zero_size() {
        let b = BBox::default();
        assert!(b.is_undefined());
        assert_eq!(b.width(), 0.0);
        assert_eq!(b.height(), 0.0);
    }

    #[test]
    fn parse_from_string() {
        let b = BBox::from("-1 2 4 5");

        assert_eq!(b.min_x, Some(-1.0));
        assert_eq!(b.max_x, Some(3.0));
        assert_eq!(b.min_y, Some(2.0));
        assert_eq!(b.max_y, Some(7.0));
    }

    #[test]
    fn test_copy() {
        let b = BBox::from("-1 2 4 5").copy();

        assert_eq!(b.min_x, Some(-1.0));
        assert_eq!(b.max_x, Some(3.0));
        assert_eq!(b.min_y, Some(2.0));
        assert_eq!(b.max_y, Some(7.0));
    }

    #[test]
    fn test_width_and_height() {
        let b = BBox::from("-1 2 4 5");

        assert_eq!(b.width(), 4.0);
        assert_eq!(b.height(), 5.0);
    }

    #[test]
    fn test_add_point_and_coordinates() {
        let mut b = BBox::new();

        // Add initial point (1,1)
        b.add_point(1.0, 1.0);
        assert_eq!(b.min_x, Some(1.0));
        assert_eq!(b.max_x, Some(1.0));
        assert_eq!(b.min_y, Some(1.0));
        assert_eq!(b.max_y, Some(1.0));

        // Add x coordinate 2
        b.add_x(2.0);
        assert_eq!(b.min_x, Some(1.0));
        assert_eq!(b.max_x, Some(2.0));
        assert_eq!(b.min_y, Some(1.0));
        assert_eq!(b.max_y, Some(1.0));

        // Add y coordinate 3
        b.add_y(3.0);
        assert_eq!(b.min_x, Some(1.0));
        assert_eq!(b.max_x, Some(2.0));
        assert_eq!(b.min_y, Some(1.0));
        assert_eq!(b.max_y, Some(3.0));

        // Add point (4,-5)
        b.add_point(4.0, -5.0);
        assert_eq!(b.min_x, Some(1.0));
        assert_eq!(b.max_x, Some(4.0));
        assert_eq!(b.min_y, Some(-5.0));
        assert_eq!(b.max_y, Some(3.0));
    }

    #[test]
    fn test_add_quadratic_curve() {
        // Test X quadratic curve
        let mut b = BBox::new();
        b.add_x_q(&[0.0, 3.0, 1.0]);

        assert_eq!(b.min_x, Some(0.0));
        assert_approx_eq!(b.max_x.unwrap(), 1.8, epsilon = 0.1);

        // Test Y quadratic curve
        let mut b = BBox::new();
        b.add_y_q(&[0.0, -2.0, 1.0]);

        assert_approx_eq!(b.min_y.unwrap(), -0.8, epsilon = 0.1);
        assert_eq!(b.max_y, Some(1.0));
    }

    #[test]
    fn test_add_cubic_curve() {
        // Test X cubic curve
        let mut b = BBox::new();
        b.add_x_c(&[0.0, -70.0, 210.0, 100.0]);

        assert_approx_eq!(b.min_x.unwrap(), -11.0, epsilon = 1.0);
        assert_approx_eq!(b.max_x.unwrap(), 126.0, epsilon = 1.0);

        // Test Y cubic curve
        let mut b = BBox::new();
        b.add_y_c(&[0.0, 1.0, 2.0, 3.0]);

        assert_eq!(b.min_y, Some(0.0));
        assert_eq!(b.max_y, Some(3.0));
    }

    #[test]
    fn test_view_box() {
        // Test cubic curves and toString
        let mut b = BBox::new();
        b.add_x_c(&[0.0, -70.0, 210.0, 100.0])
            .add_y_c(&[0.0, -30.0, 70.0, 40.0]);

        assert_eq!(b.to_string(Some(0)), "-11 -6 137 51");

        // Test box creation from string
        let b = BBox::from("-10 20 30 50");

        assert_eq!(b.min_x, Some(-10.0));
        assert_eq!(b.max_x, Some(20.0)); // -10 + 30 = 20
        assert_eq!(b.min_y, Some(20.0));
        assert_eq!(b.max_y, Some(70.0)); // 20 + 50 = 70
    }

    #[test]
    fn test_matrix_to_put_in_box() {
        let b = BBox::from("-10 0 40 50");

        // Test default (meet xMidYMid)
        let params = InboxParameters {
            destination: BBox::from("0 0 100 200"),
            scale_type: ScaleType::Meet,
            alignment: BoxAlignment { x: Alignment::Mid, y: Alignment::Mid },
        };
        let m = b.inbox_matrix(&params);
        assert_approx_eq!(m[0], 2.5, epsilon = 0.1);
        assert_approx_eq!(m[1], 0.0, epsilon = 0.1);
        assert_approx_eq!(m[2], 0.0, epsilon = 0.1);
        assert_approx_eq!(m[3], 2.5, epsilon = 0.1);
        assert_approx_eq!(m[4], 25.0, epsilon = 0.1);
        assert_approx_eq!(m[5], 37.5, epsilon = 0.1);

        // Test slice xMinYMax
        let params = InboxParameters {
            destination: BBox::from("0 0 100 200"),
            scale_type: ScaleType::Slice,
            alignment: BoxAlignment { x: Alignment::Min, y: Alignment::Max },
        };
        let m = b.inbox_matrix(&params);
        assert_approx_eq!(m[0], 4.0, epsilon = 0.1);
        assert_approx_eq!(m[1], 0.0, epsilon = 0.1);
        assert_approx_eq!(m[2], 0.0, epsilon = 0.1);
        assert_approx_eq!(m[3], 4.0, epsilon = 0.1);
        assert_approx_eq!(m[4], 40.0, epsilon = 0.1);
        assert_approx_eq!(m[5], 0.0, epsilon = 0.1);

        // Test fit
        let params = InboxParameters {
            destination: BBox::from("0 0 100 200"),
            scale_type: ScaleType::Fit,
            alignment: BoxAlignment { x: Alignment::Mid, y: Alignment::Mid },
        };
        let m = b.inbox_matrix(&params);
        assert_approx_eq!(m[0], 2.5, epsilon = 0.1);
        assert_approx_eq!(m[1], 0.0, epsilon = 0.1);
        assert_approx_eq!(m[2], 0.0, epsilon = 0.1);
        assert_approx_eq!(m[3], 4.0, epsilon = 0.1);
        assert_approx_eq!(m[4], 25.0, epsilon = 0.1);
        assert_approx_eq!(m[5], 0.0, epsilon = 0.1);

        // Test move xMinYmid
        let params = InboxParameters {
            destination: BBox::from("0 0 100 200"),
            scale_type: ScaleType::Move,
            alignment: BoxAlignment { x: Alignment::Min, y: Alignment::Mid },
        };
        let m = b.inbox_matrix(&params);
        assert_approx_eq!(m[0], 1.0, epsilon = 0.1);
        assert_approx_eq!(m[1], 0.0, epsilon = 0.1);
        assert_approx_eq!(m[2], 0.0, epsilon = 0.1);
        assert_approx_eq!(m[3], 1.0, epsilon = 0.1);
        assert_approx_eq!(m[4], 10.0, epsilon = 0.1);
        assert_approx_eq!(m[5], 75.0, epsilon = 0.1);
    }
}
