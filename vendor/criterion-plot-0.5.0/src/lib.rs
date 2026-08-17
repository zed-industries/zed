//! [Criterion]'s plotting library.
//!
//! [Criterion]: https://github.com/bheisler/criterion.rs
//!
//! **WARNING** This library is criterion's implementation detail and there no plans to stabilize
//! it. In other words, the API may break at any time without notice.
//!
//! # Examples
//!
//! - Simple "curves" (based on [`simple.dem`](http://gnuplot.sourceforge.net/demo/simple.html))
//!
//! ![Plot](curve.svg)
//!
//! ```
//! # use std::fs;
//! # use std::path::Path;
//! use itertools_num::linspace;
//! use criterion_plot::prelude::*;
//!
//! # if let Err(_) = criterion_plot::version() {
//! #     return;
//! # }
//! let ref xs = linspace::<f64>(-10., 10., 51).collect::<Vec<_>>();
//!
//! # fs::create_dir_all(Path::new("target/doc/criterion_plot")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .set(Font("Helvetica"))
//! #   .set(FontSize(12.))
//! #   .set(Output(Path::new("target/doc/criterion_plot/curve.svg")))
//! #   .set(Size(1280, 720))
//!     .configure(Key, |k| {
//!         k.set(Boxed::Yes)
//!          .set(Position::Inside(Vertical::Top, Horizontal::Left))
//!     })
//!     .plot(LinesPoints {
//!               x: xs,
//!               y: xs.iter().map(|x| x.sin()),
//!           },
//!           |lp| {
//!               lp.set(Color::DarkViolet)
//!                 .set(Label("sin(x)"))
//!                 .set(LineType::Dash)
//!                 .set(PointSize(1.5))
//!                 .set(PointType::Circle)
//!           })
//!     .plot(Steps {
//!               x: xs,
//!               y: xs.iter().map(|x| x.atan()),
//!           },
//!           |s| {
//!               s.set(Color::Rgb(0, 158, 115))
//!                .set(Label("atan(x)"))
//!                .set(LineWidth(2.))
//!           })
//!     .plot(Impulses {
//!               x: xs,
//!               y: xs.iter().map(|x| x.atan().cos()),
//!           },
//!           |i| {
//!               i.set(Color::Rgb(86, 180, 233))
//!                .set(Label("cos(atan(x))"))
//!           })
//!     .draw()  // (rest of the chain has been omitted)
//! #   .ok()
//! #   .and_then(|gnuplot| {
//! #       gnuplot.wait_with_output().ok().and_then(|p| String::from_utf8(p.stderr).ok())
//! #   }));
//! ```
//!
//! - error bars (based on
//! [Julia plotting tutorial](https://plot.ly/julia/error-bars/#Colored-and-Styled-Error-Bars))
//!
//! ![Plot](error_bar.svg)
//!
//! ```
//! # use std::fs;
//! # use std::path::Path;
//! use std::f64::consts::PI;
//!
//! use itertools_num::linspace;
//! use rand::Rng;
//! use criterion_plot::prelude::*;
//!
//! fn sinc(mut x: f64) -> f64 {
//!     if x == 0. {
//!         1.
//!     } else {
//!         x *= PI;
//!         x.sin() / x
//!     }
//! }
//!
//! # if let Err(_) = criterion_plot::version() {
//! #     return;
//! # }
//! let ref xs_ = linspace::<f64>(-4., 4., 101).collect::<Vec<_>>();
//!
//! // Fake some data
//! let ref mut rng = rand::thread_rng();
//! let xs = linspace::<f64>(-4., 4., 13).skip(1).take(11);
//! let ys = xs.map(|x| sinc(x) + 0.05 * rng.gen::<f64>() - 0.025).collect::<Vec<_>>();
//! let y_low = ys.iter().map(|&y| y - 0.025 - 0.075 * rng.gen::<f64>()).collect::<Vec<_>>();
//! let y_high = ys.iter().map(|&y| y + 0.025 + 0.075 * rng.gen::<f64>()).collect::<Vec<_>>();
//! let xs = linspace::<f64>(-4., 4., 13).skip(1).take(11);
//! let xs = xs.map(|x| x + 0.2 * rng.gen::<f64>() - 0.1);
//!
//! # fs::create_dir_all(Path::new("target/doc/criterion_plot")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .set(Font("Helvetica"))
//! #   .set(FontSize(12.))
//! #   .set(Output(Path::new("target/doc/criterion_plot/error_bar.svg")))
//! #   .set(Size(1280, 720))
//!     .configure(Axis::BottomX, |a| {
//!         a.set(TicLabels {
//!             labels: &["-π", "0", "π"],
//!             positions: &[-PI, 0., PI],
//!         })
//!     })
//!     .configure(Key,
//!                |k| k.set(Position::Outside(Vertical::Top, Horizontal::Right)))
//!     .plot(Lines {
//!               x: xs_,
//!               y: xs_.iter().cloned().map(sinc),
//!           },
//!           |l| {
//!               l.set(Color::Rgb(0, 158, 115))
//!                .set(Label("sinc(x)"))
//!                .set(LineWidth(2.))
//!           })
//!     .plot(YErrorBars {
//!               x: xs,
//!               y: &ys,
//!               y_low: &y_low,
//!               y_high: &y_high,
//!           },
//!           |eb| {
//!               eb.set(Color::DarkViolet)
//!                 .set(LineWidth(2.))
//!                 .set(PointType::FilledCircle)
//!                 .set(Label("measured"))
//!           })
//!     .draw()  // (rest of the chain has been omitted)
//! #   .ok()
//! #   .and_then(|gnuplot| {
//! #       gnuplot.wait_with_output().ok().and_then(|p| String::from_utf8(p.stderr).ok())
//! #   }));
//! ```
//!
//! - Candlesticks (based on
//! [`candlesticks.dem`](http://gnuplot.sourceforge.net/demo/candlesticks.html))
//!
//! ![Plot](candlesticks.svg)
//!
//! ```
//! # use std::fs;
//! # use std::path::Path;
//! use criterion_plot::prelude::*;
//! use rand::Rng;
//!
//! # if let Err(_) = criterion_plot::version() {
//! #     return;
//! # }
//! let xs = 1..11;
//!
//! // Fake some data
//! let mut rng = rand::thread_rng();
//! let bh = xs.clone().map(|_| 5f64 + 2.5 * rng.gen::<f64>()).collect::<Vec<_>>();
//! let bm = xs.clone().map(|_| 2.5f64 + 2.5 * rng.gen::<f64>()).collect::<Vec<_>>();
//! let wh = bh.iter().map(|&y| y + (10. - y) * rng.gen::<f64>()).collect::<Vec<_>>();
//! let wm = bm.iter().map(|&y| y * rng.gen::<f64>()).collect::<Vec<_>>();
//! let m = bm.iter().zip(bh.iter()).map(|(&l, &h)| (h - l) * rng.gen::<f64>() + l)
//!     .collect::<Vec<_>>();
//!
//! # fs::create_dir_all(Path::new("target/doc/criterion_plot")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .set(Font("Helvetica"))
//! #   .set(FontSize(12.))
//! #   .set(Output(Path::new("target/doc/criterion_plot/candlesticks.svg")))
//! #   .set(Size(1280, 720))
//!     .set(BoxWidth(0.2))
//!     .configure(Axis::BottomX, |a| a.set(Range::Limits(0., 11.)))
//!     .plot(Candlesticks {
//!               x: xs.clone(),
//!               whisker_min: &wm,
//!               box_min: &bm,
//!               box_high: &bh,
//!               whisker_high: &wh,
//!           },
//!           |cs| {
//!               cs.set(Color::Rgb(86, 180, 233))
//!                 .set(Label("Quartiles"))
//!                 .set(LineWidth(2.))
//!           })
//!     // trick to plot the median
//!     .plot(Candlesticks {
//!               x: xs,
//!               whisker_min: &m,
//!               box_min: &m,
//!               box_high: &m,
//!               whisker_high: &m,
//!           },
//!           |cs| {
//!               cs.set(Color::Black)
//!                 .set(LineWidth(2.))
//!           })
//!     .draw()  // (rest of the chain has been omitted)
//! #   .ok()
//! #   .and_then(|gnuplot| {
//! #       gnuplot.wait_with_output().ok().and_then(|p| String::from_utf8(p.stderr).ok())
//! #   }));
//! ```
//!
//! - Multiaxis (based on [`multiaxis.dem`](http://gnuplot.sourceforge.net/demo/multiaxis.html))
//!
//! ![Plot](multiaxis.svg)
//!
//! ```
//! # use std::fs;
//! # use std::path::Path;
//! use std::f64::consts::PI;
//!
//! use itertools_num::linspace;
//! use num_complex::Complex;
//! use criterion_plot::prelude::*;
//!
//! fn tf(x: f64) -> Complex<f64> {
//!     Complex::new(0., x) / Complex::new(10., x) / Complex::new(1., x / 10_000.)
//! }
//!
//! # if let Err(_) = criterion_plot::version() {
//! #     return;
//! # }
//! let (start, end): (f64, f64) = (1.1, 90_000.);
//! let ref xs = linspace(start.ln(), end.ln(), 101).map(|x| x.exp()).collect::<Vec<_>>();
//! let phase = xs.iter().map(|&x| tf(x).arg() * 180. / PI);
//! let magnitude = xs.iter().map(|&x| tf(x).norm());
//!
//! # fs::create_dir_all(Path::new("target/doc/criterion_plot")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new().
//! #   set(Font("Helvetica")).
//! #   set(FontSize(12.)).
//! #   set(Output(Path::new("target/doc/criterion_plot/multiaxis.svg"))).
//! #   set(Size(1280, 720)).
//!     set(Title("Frequency response")).
//!     configure(Axis::BottomX, |a| a.
//!         configure(Grid::Major, |g| g.
//!             show()).
//!         set(Label("Angular frequency (rad/s)")).
//!         set(Range::Limits(start, end)).
//!         set(Scale::Logarithmic)).
//!     configure(Axis::LeftY, |a| a.
//!         set(Label("Gain")).
//!         set(Scale::Logarithmic)).
//!     configure(Axis::RightY, |a| a.
//!         configure(Grid::Major, |g| g.
//!             show()).
//!         set(Label("Phase shift (°)"))).
//!     configure(Key, |k| k.
//!         set(Position::Inside(Vertical::Top, Horizontal::Center)).
//!         set(Title(" "))).
//!     plot(Lines {
//!         x: xs,
//!         y: magnitude,
//!     }, |l| l.
//!         set(Color::DarkViolet).
//!         set(Label("Magnitude")).
//!         set(LineWidth(2.))).
//!     plot(Lines {
//!         x: xs,
//!         y: phase,
//!     }, |l| l.
//!         set(Axes::BottomXRightY).
//!         set(Color::Rgb(0, 158, 115)).
//!         set(Label("Phase")).
//!         set(LineWidth(2.))).
//!     draw().  // (rest of the chain has been omitted)
//! #   ok().and_then(|gnuplot| {
//! #       gnuplot.wait_with_output().ok().and_then(|p| {
//! #           String::from_utf8(p.stderr).ok()
//! #       })
//! #   }));
//! ```
//! - Filled curves (based on
//! [`transparent.dem`](http://gnuplot.sourceforge.net/demo/transparent.html))
//!
//! ![Plot](filled_curve.svg)
//!
//! ```
//! # use std::fs;
//! # use std::path::Path;
//! use std::f64::consts::PI;
//! use std::iter;
//!
//! use itertools_num::linspace;
//! use criterion_plot::prelude::*;
//!
//! # if let Err(_) = criterion_plot::version() {
//! #     return;
//! # }
//! let (start, end) = (-5., 5.);
//! let ref xs = linspace(start, end, 101).collect::<Vec<_>>();
//! let zeros = iter::repeat(0);
//!
//! fn gaussian(x: f64, mu: f64, sigma: f64) -> f64 {
//!     1. / (((x - mu).powi(2) / 2. / sigma.powi(2)).exp() * sigma * (2. * PI).sqrt())
//! }
//!
//! # fs::create_dir_all(Path::new("target/doc/criterion_plot")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .set(Font("Helvetica"))
//! #   .set(FontSize(12.))
//! #   .set(Output(Path::new("target/doc/criterion_plot/filled_curve.svg")))
//! #   .set(Size(1280, 720))
//!     .set(Title("Transparent filled curve"))
//!     .configure(Axis::BottomX, |a| a.set(Range::Limits(start, end)))
//!     .configure(Axis::LeftY, |a| a.set(Range::Limits(0., 1.)))
//!     .configure(Key, |k| {
//!         k.set(Justification::Left)
//!          .set(Order::SampleText)
//!          .set(Position::Inside(Vertical::Top, Horizontal::Left))
//!          .set(Title("Gaussian Distribution"))
//!     })
//!     .plot(FilledCurve {
//!               x: xs,
//!               y1: xs.iter().map(|&x| gaussian(x, 0.5, 0.5)),
//!               y2: zeros.clone(),
//!           },
//!           |fc| {
//!               fc.set(Color::ForestGreen)
//!                 .set(Label("μ = 0.5 σ = 0.5"))
//!           })
//!     .plot(FilledCurve {
//!               x: xs,
//!               y1: xs.iter().map(|&x| gaussian(x, 2.0, 1.0)),
//!               y2: zeros.clone(),
//!           },
//!           |fc| {
//!               fc.set(Color::Gold)
//!                 .set(Label("μ = 2.0 σ = 1.0"))
//!                 .set(Opacity(0.5))
//!           })
//!     .plot(FilledCurve {
//!               x: xs,
//!               y1: xs.iter().map(|&x| gaussian(x, -1.0, 2.0)),
//!               y2: zeros,
//!           },
//!           |fc| {
//!               fc.set(Color::Red)
//!                 .set(Label("μ = -1.0 σ = 2.0"))
//!                 .set(Opacity(0.5))
//!           })
//!     .draw()
//!     .ok()
//!     .and_then(|gnuplot| {
//!         gnuplot.wait_with_output().ok().and_then(|p| String::from_utf8(p.stderr).ok())
//!     }));
//! ```

#![deny(missing_docs)]
#![deny(warnings)]
#![deny(bare_trait_objects)]
// This lint has lots of false positives ATM, see
// https://github.com/Manishearth/rust-clippy/issues/761
#![cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
// False positives with images
#![cfg_attr(feature = "cargo-clippy", allow(clippy::doc_markdown))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::many_single_char_names))]

extern crate cast;
#[macro_use]
extern crate itertools;

use std::borrow::Cow;
use std::fmt;
use std::fs::File;
use std::io;
use std::num::ParseIntError;
use std::path::Path;
use std::process::{Child, Command};
use std::str;

use crate::data::Matrix;
use crate::traits::{Configure, Set};

mod data;
mod display;
mod map;

pub mod axis;
pub mod candlestick;
pub mod curve;
pub mod errorbar;
pub mod filledcurve;
pub mod grid;
pub mod key;
pub mod prelude;
pub mod proxy;
pub mod traits;

/// Plot container
#[derive(Clone)]
pub struct Figure {
    alpha: Option<f64>,
    axes: map::axis::Map<axis::Properties>,
    box_width: Option<f64>,
    font: Option<Cow<'static, str>>,
    font_size: Option<f64>,
    key: Option<key::Properties>,
    output: Cow<'static, Path>,
    plots: Vec<Plot>,
    size: Option<(usize, usize)>,
    terminal: Terminal,
    tics: map::axis::Map<String>,
    title: Option<Cow<'static, str>>,
}

impl Figure {
    /// Creates an empty figure
    pub fn new() -> Figure {
        Figure {
            alpha: None,
            axes: map::axis::Map::new(),
            box_width: None,
            font: None,
            font_size: None,
            key: None,
            output: Cow::Borrowed(Path::new("output.plot")),
            plots: Vec::new(),
            size: None,
            terminal: Terminal::Svg,
            tics: map::axis::Map::new(),
            title: None,
        }
    }

    // Allow clippy::format_push_string even with older versions of rust (<1.62) which
    // don't have it defined.
    #[allow(clippy::all)]
    fn script(&self) -> Vec<u8> {
        let mut s = String::new();

        s.push_str(&format!(
            "set output '{}'\n",
            self.output.display().to_string().replace('\'', "''")
        ));

        if let Some(width) = self.box_width {
            s.push_str(&format!("set boxwidth {}\n", width))
        }

        if let Some(ref title) = self.title {
            s.push_str(&format!("set title '{}'\n", title))
        }

        for axis in self.axes.iter() {
            s.push_str(&axis.script());
        }

        for (_, script) in self.tics.iter() {
            s.push_str(script);
        }

        if let Some(ref key) = self.key {
            s.push_str(&key.script())
        }

        if let Some(alpha) = self.alpha {
            s.push_str(&format!("set style fill transparent solid {}\n", alpha))
        }

        s.push_str(&format!("set terminal {} dashed", self.terminal.display()));

        if let Some((width, height)) = self.size {
            s.push_str(&format!(" size {}, {}", width, height))
        }

        if let Some(ref name) = self.font {
            if let Some(size) = self.font_size {
                s.push_str(&format!(" font '{},{}'", name, size))
            } else {
                s.push_str(&format!(" font '{}'", name))
            }
        }

        // TODO This removes the crossbars from the ends of error bars, but should be configurable
        s.push_str("\nunset bars\n");

        let mut is_first_plot = true;
        for plot in &self.plots {
            let data = plot.data();

            if data.bytes().is_empty() {
                continue;
            }

            if is_first_plot {
                s.push_str("plot ");
                is_first_plot = false;
            } else {
                s.push_str(", ");
            }

            s.push_str(&format!(
                "'-' binary endian=little record={} format='%float64' using ",
                data.nrows()
            ));

            let mut is_first_col = true;
            for col in 0..data.ncols() {
                if is_first_col {
                    is_first_col = false;
                } else {
                    s.push(':');
                }
                s.push_str(&(col + 1).to_string());
            }
            s.push(' ');

            s.push_str(plot.script());
        }

        let mut buffer = s.into_bytes();
        let mut is_first = true;
        for plot in &self.plots {
            if is_first {
                is_first = false;
                buffer.push(b'\n');
            }
            buffer.extend_from_slice(plot.data().bytes());
        }

        buffer
    }

    /// Spawns a drawing child process
    ///
    /// NOTE: stderr, stdin, and stdout are piped
    pub fn draw(&mut self) -> io::Result<Child> {
        use std::process::Stdio;

        let mut gnuplot = Command::new("gnuplot")
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        self.dump(gnuplot.stdin.as_mut().unwrap())?;
        Ok(gnuplot)
    }

    /// Dumps the script required to produce the figure into `sink`
    pub fn dump<W>(&mut self, sink: &mut W) -> io::Result<&mut Figure>
    where
        W: io::Write,
    {
        sink.write_all(&self.script())?;
        Ok(self)
    }

    /// Saves the script required to produce the figure to `path`
    pub fn save(&self, path: &Path) -> io::Result<&Figure> {
        use std::io::Write;

        File::create(path)?.write_all(&self.script())?;
        Ok(self)
    }
}

impl Configure<Axis> for Figure {
    type Properties = axis::Properties;

    /// Configures an axis
    fn configure<F>(&mut self, axis: Axis, configure: F) -> &mut Figure
    where
        F: FnOnce(&mut axis::Properties) -> &mut axis::Properties,
    {
        if self.axes.contains_key(axis) {
            configure(self.axes.get_mut(axis).unwrap());
        } else {
            let mut properties = Default::default();
            configure(&mut properties);
            self.axes.insert(axis, properties);
        }
        self
    }
}

impl Configure<Key> for Figure {
    type Properties = key::Properties;

    /// Configures the key (legend)
    fn configure<F>(&mut self, _: Key, configure: F) -> &mut Figure
    where
        F: FnOnce(&mut key::Properties) -> &mut key::Properties,
    {
        if self.key.is_some() {
            configure(self.key.as_mut().unwrap());
        } else {
            let mut key = Default::default();
            configure(&mut key);
            self.key = Some(key);
        }
        self
    }
}

impl Set<BoxWidth> for Figure {
    /// Changes the box width of all the box related plots (bars, candlesticks, etc)
    ///
    /// **Note** The default value is 0
    ///
    /// # Panics
    ///
    /// Panics if `width` is a negative value
    fn set(&mut self, width: BoxWidth) -> &mut Figure {
        let width = width.0;

        assert!(width >= 0.);

        self.box_width = Some(width);
        self
    }
}

impl Set<Font> for Figure {
    /// Changes the font
    fn set(&mut self, font: Font) -> &mut Figure {
        self.font = Some(font.0);
        self
    }
}

impl Set<FontSize> for Figure {
    /// Changes the size of the font
    ///
    /// # Panics
    ///
    /// Panics if `size` is a non-positive value
    fn set(&mut self, size: FontSize) -> &mut Figure {
        let size = size.0;

        assert!(size >= 0.);

        self.font_size = Some(size);
        self
    }
}

impl Set<Output> for Figure {
    /// Changes the output file
    ///
    /// **Note** The default output file is `output.plot`
    fn set(&mut self, output: Output) -> &mut Figure {
        self.output = output.0;
        self
    }
}

impl Set<Size> for Figure {
    /// Changes the figure size
    fn set(&mut self, size: Size) -> &mut Figure {
        self.size = Some((size.0, size.1));
        self
    }
}

impl Set<Terminal> for Figure {
    /// Changes the output terminal
    ///
    /// **Note** By default, the terminal is set to `Svg`
    fn set(&mut self, terminal: Terminal) -> &mut Figure {
        self.terminal = terminal;
        self
    }
}

impl Set<Title> for Figure {
    /// Sets the title
    fn set(&mut self, title: Title) -> &mut Figure {
        self.title = Some(title.0);
        self
    }
}

impl Default for Figure {
    fn default() -> Self {
        Self::new()
    }
}

/// Box width for box-related plots: bars, candlesticks, etc
#[derive(Clone, Copy)]
pub struct BoxWidth(pub f64);

/// A font name
pub struct Font(Cow<'static, str>);

/// The size of a font
#[derive(Clone, Copy)]
pub struct FontSize(pub f64);

/// The key or legend
#[derive(Clone, Copy)]
pub struct Key;

/// Plot label
pub struct Label(Cow<'static, str>);

/// Width of the lines
#[derive(Clone, Copy)]
pub struct LineWidth(pub f64);

/// Fill color opacity
#[derive(Clone, Copy)]
pub struct Opacity(pub f64);

/// Output file path
pub struct Output(Cow<'static, Path>);

/// Size of the points
#[derive(Clone, Copy)]
pub struct PointSize(pub f64);

/// Axis range
#[derive(Clone, Copy)]
pub enum Range {
    /// Autoscale the axis
    Auto,
    /// Set the limits of the axis
    Limits(f64, f64),
}

/// Figure size
#[derive(Clone, Copy)]
pub struct Size(pub usize, pub usize);

/// Labels attached to the tics of an axis
pub struct TicLabels<P, L> {
    /// Labels to attach to the tics
    pub labels: L,
    /// Position of the tics on the axis
    pub positions: P,
}

/// Figure title
pub struct Title(Cow<'static, str>);

/// A pair of axes that define a coordinate system
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Axes {
    BottomXLeftY,
    BottomXRightY,
    TopXLeftY,
    TopXRightY,
}

/// A coordinate axis
#[derive(Clone, Copy)]
pub enum Axis {
    /// X axis on the bottom side of the figure
    BottomX,
    /// Y axis on the left side of the figure
    LeftY,
    /// Y axis on the right side of the figure
    RightY,
    /// X axis on the top side of the figure
    TopX,
}

impl Axis {
    fn next(self) -> Option<Axis> {
        use crate::Axis::*;

        match self {
            BottomX => Some(LeftY),
            LeftY => Some(RightY),
            RightY => Some(TopX),
            TopX => None,
        }
    }
}

/// Color
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    DarkViolet,
    ForestGreen,
    Gold,
    Gray,
    Green,
    Magenta,
    Red,
    /// Custom RGB color
    Rgb(u8, u8, u8),
    White,
    Yellow,
}

/// Grid line
#[derive(Clone, Copy)]
pub enum Grid {
    /// Major gridlines
    Major,
    /// Minor gridlines
    Minor,
}

impl Grid {
    fn next(self) -> Option<Grid> {
        use crate::Grid::*;

        match self {
            Major => Some(Minor),
            Minor => None,
        }
    }
}

/// Line type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum LineType {
    Dash,
    Dot,
    DotDash,
    DotDotDash,
    /// Line made of minimally sized dots
    SmallDot,
    Solid,
}

/// Point type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum PointType {
    Circle,
    FilledCircle,
    FilledSquare,
    FilledTriangle,
    Plus,
    Square,
    Star,
    Triangle,
    X,
}

/// Axis scale
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Scale {
    Linear,
    Logarithmic,
}

/// Axis scale factor
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub struct ScaleFactor(pub f64);

/// Output terminal
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Terminal {
    Svg,
}

/// Not public version of `std::default::Default`, used to not leak default constructors into the
/// public API
trait Default {
    /// Creates `Properties` with default configuration
    fn default() -> Self;
}

/// Enums that can produce gnuplot code
trait Display<S> {
    /// Translates the enum in gnuplot code
    fn display(&self) -> S;
}

/// Curve variant of Default
trait CurveDefault<S> {
    /// Creates `curve::Properties` with default configuration
    fn default(s: S) -> Self;
}

/// Error bar variant of Default
trait ErrorBarDefault<S> {
    /// Creates `errorbar::Properties` with default configuration
    fn default(s: S) -> Self;
}

/// Structs that can produce gnuplot code
trait Script {
    /// Translates some configuration struct into gnuplot code
    fn script(&self) -> String;
}

#[derive(Clone)]
struct Plot {
    data: Matrix,
    script: String,
}

impl Plot {
    fn new<S>(data: Matrix, script: &S) -> Plot
    where
        S: Script,
    {
        Plot {
            data,
            script: script.script(),
        }
    }

    fn data(&self) -> &Matrix {
        &self.data
    }

    fn script(&self) -> &str {
        &self.script
    }
}

/// Possible errors when parsing gnuplot's version string
#[derive(Debug)]
pub enum VersionError {
    /// The `gnuplot` command couldn't be executed
    Exec(io::Error),
    /// The `gnuplot` command returned an error message
    Error(String),
    /// The `gnuplot` command returned invalid utf-8
    OutputError,
    /// The `gnuplot` command returned an unparseable string
    ParseError(String),
}
impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionError::Exec(err) => write!(f, "`gnuplot --version` failed: {}", err),
            VersionError::Error(msg) => {
                write!(f, "`gnuplot --version` failed with error message:\n{}", msg)
            }
            VersionError::OutputError => write!(f, "`gnuplot --version` returned invalid utf-8"),
            VersionError::ParseError(msg) => write!(
                f,
                "`gnuplot --version` returned an unparseable version string: {}",
                msg
            ),
        }
    }
}
impl ::std::error::Error for VersionError {
    fn description(&self) -> &str {
        match self {
            VersionError::Exec(_) => "Execution Error",
            VersionError::Error(_) => "Other Error",
            VersionError::OutputError => "Output Error",
            VersionError::ParseError(_) => "Parse Error",
        }
    }

    fn cause(&self) -> Option<&dyn ::std::error::Error> {
        match self {
            VersionError::Exec(err) => Some(err),
            _ => None,
        }
    }
}

/// Structure representing a gnuplot version number.
pub struct Version {
    /// The major version number
    pub major: usize,
    /// The minor version number
    pub minor: usize,
    /// The patch level
    pub patch: String,
}

/// Returns `gnuplot` version
pub fn version() -> Result<Version, VersionError> {
    let command_output = Command::new("gnuplot")
        .arg("--version")
        .output()
        .map_err(VersionError::Exec)?;
    if !command_output.status.success() {
        let error =
            String::from_utf8(command_output.stderr).map_err(|_| VersionError::OutputError)?;
        return Err(VersionError::Error(error));
    }

    let output = String::from_utf8(command_output.stdout).map_err(|_| VersionError::OutputError)?;

    parse_version(&output).map_err(|_| VersionError::ParseError(output.clone()))
}

fn parse_version(version_str: &str) -> Result<Version, Option<ParseIntError>> {
    let mut words = version_str.split_whitespace().skip(1);
    let mut version = words.next().ok_or(None)?.split('.');
    let major = version.next().ok_or(None)?.parse()?;
    let minor = version.next().ok_or(None)?.parse()?;
    let patchlevel = words.nth(1).ok_or(None)?.to_owned();

    Ok(Version {
        major,
        minor,
        patch: patchlevel,
    })
}

fn scale_factor(map: &map::axis::Map<axis::Properties>, axes: Axes) -> (f64, f64) {
    use crate::Axes::*;
    use crate::Axis::*;

    match axes {
        BottomXLeftY => (
            map.get(BottomX).map_or(1., ScaleFactorTrait::scale_factor),
            map.get(LeftY).map_or(1., ScaleFactorTrait::scale_factor),
        ),
        BottomXRightY => (
            map.get(BottomX).map_or(1., ScaleFactorTrait::scale_factor),
            map.get(RightY).map_or(1., ScaleFactorTrait::scale_factor),
        ),
        TopXLeftY => (
            map.get(TopX).map_or(1., ScaleFactorTrait::scale_factor),
            map.get(LeftY).map_or(1., ScaleFactorTrait::scale_factor),
        ),
        TopXRightY => (
            map.get(TopX).map_or(1., ScaleFactorTrait::scale_factor),
            map.get(RightY).map_or(1., ScaleFactorTrait::scale_factor),
        ),
    }
}

// XXX :-1: to intra-crate privacy rules
/// Private
trait ScaleFactorTrait {
    /// Private
    fn scale_factor(&self) -> f64;
}

#[cfg(test)]
mod test {
    #[test]
    fn version() {
        if let Ok(version) = super::version() {
            assert!(version.major >= 4);
        } else {
            println!("Gnuplot not installed.");
        }
    }

    #[test]
    fn test_parse_version_on_valid_string() {
        let string = "gnuplot 5.0 patchlevel 7";
        let version = super::parse_version(&string).unwrap();
        assert_eq!(5, version.major);
        assert_eq!(0, version.minor);
        assert_eq!("7", &version.patch);
    }

    #[test]
    fn test_parse_gentoo_version() {
        let string = "gnuplot 5.2 patchlevel 5a (Gentoo revision r0)";
        let version = super::parse_version(&string).unwrap();
        assert_eq!(5, version.major);
        assert_eq!(2, version.minor);
        assert_eq!("5a", &version.patch);
    }

    #[test]
    fn test_parse_version_returns_error_on_invalid_strings() {
        let strings = [
            "",
            "foobar",
            "gnuplot 50 patchlevel 7",
            "gnuplot 5.0 patchlevel",
            "gnuplot foo.bar patchlevel 7",
        ];
        for string in &strings {
            assert!(super::parse_version(string).is_err());
        }
    }
}
