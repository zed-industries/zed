use smallvec::SmallVec;

pub fn rgb(hex: u32) -> Rgba {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    Rgba { r, g, b, a: 1.0 }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Hsla> for Rgba {
    fn from(color: Hsla) -> Self {
        let h = color.h;
        let s = color.s;
        let l = color.l;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        let cm = c + m;
        let xm = x + m;

        let (r, g, b) = match (h * 6.0).floor() as i32 {
            0 | 6 => (cm, xm, m),
            1 => (xm, cm, m),
            2 => (m, cm, xm),
            3 => (m, xm, cm),
            4 => (xm, m, cm),
            _ => (cm, m, xm),
        };

        Rgba {
            r,
            g,
            b,
            a: color.a,
        }
    }
}

impl Into<gpui::color::Color> for Rgba {
    fn into(self) -> gpui::color::Color {
        gpui::color::rgba(self.r, self.g, self.b, self.a)
    }
}

#[derive(Copy, Clone)]
pub struct Hsla {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
}

impl From<Rgba> for Hsla {
    fn from(color: Rgba) -> Self {
        let r = color.r;
        let g = color.g;
        let b = color.b;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let l = (max + min) / 2.0;
        let s = match l {
            0.0 | 1.0 => 0.0,
            l if l < 0.5 => delta / (2.0 * l),
            l => delta / (2.0 - 2.0 * l),
        };

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            ((g - b) / delta).rem_euclid(6.0) / 6.0
        } else if max == g {
            ((b - r) / delta + 2.0) / 6.0
        } else {
            ((r - g) / delta + 4.0) / 6.0
        };

        Hsla {
            h,
            s,
            l,
            a: color.a,
        }
    }
}

impl Hsla {
    /// Increases the saturation of the color by a certain amount, with a max
    /// value of 1.0.
    pub fn saturate(mut self, amount: f32) -> Self {
        self.s += amount;
        self.s = self.s.clamp(0.0, 1.0);
        self
    }

    /// Decreases the saturation of the color by a certain amount, with a min
    /// value of 0.0.
    pub fn desaturate(mut self, amount: f32) -> Self {
        self.s -= amount;
        self.s = self.s.max(0.0);
        if self.s < 0.0 {
            self.s = 0.0;
        }
        self
    }

    /// Brightens the color by increasing the lightness by a certain amount,
    /// with a max value of 1.0.
    pub fn brighten(mut self, amount: f32) -> Self {
        self.l += amount;
        self.l = self.l.clamp(0.0, 1.0);
        self
    }

    /// Darkens the color by decreasing the lightness by a certain amount,
    /// with a max value of 0.0.
    pub fn darken(mut self, amount: f32) -> Self {
        self.l -= amount;
        self.l = self.l.clamp(0.0, 1.0);
        self
    }
}

pub struct ColorScale {
    colors: SmallVec<[Hsla; 2]>,
    positions: SmallVec<[f32; 2]>,
}

pub fn scale<I, C>(colors: I) -> ColorScale
where
    I: IntoIterator<Item = C>,
    C: Into<Hsla>,
{
    let mut scale = ColorScale {
        colors: colors.into_iter().map(Into::into).collect(),
        positions: SmallVec::new(),
    };
    let num_colors: f32 = scale.colors.len() as f32 - 1.0;
    scale.positions = (0..scale.colors.len())
        .map(|i| i as f32 / num_colors)
        .collect();
    scale
}

impl ColorScale {
    fn at(&self, t: f32) -> Hsla {
        // Ensure that the input is within [0.0, 1.0]
        debug_assert!(
            0.0 <= t && t <= 1.0,
            "t value {} is out of range. Expected value in range 0.0 to 1.0",
            t
        );

        let position = match self
            .positions
            .binary_search_by(|a| a.partial_cmp(&t).unwrap())
        {
            Ok(index) | Err(index) => index,
        };
        let lower_bound = position.saturating_sub(1);
        let upper_bound = position.min(self.colors.len() - 1);
        let lower_color = &self.colors[lower_bound];
        let upper_color = &self.colors[upper_bound];

        match upper_bound.checked_sub(lower_bound) {
            Some(0) | None => *lower_color,
            Some(_) => {
                let interval_t = (t - self.positions[lower_bound])
                    / (self.positions[upper_bound] - self.positions[lower_bound]);
                let h = lower_color.h + interval_t * (upper_color.h - lower_color.h);
                let s = lower_color.s + interval_t * (upper_color.s - lower_color.s);
                let l = lower_color.l + interval_t * (upper_color.l - lower_color.l);
                let a = lower_color.a + interval_t * (upper_color.a - lower_color.a);
                Hsla { h, s, l, a }
            }
        }
    }
}
