use std::collections::{hash_map::IntoIter as HashMapIter, HashMap};
use std::marker::PhantomData;
use std::ops::AddAssign;

use crate::chart::ChartContext;
use crate::coord::cartesian::Cartesian2d;
use crate::coord::ranged1d::{DiscreteRanged, Ranged};
use crate::element::Rectangle;
use crate::style::{Color, ShapeStyle, GREEN};
use plotters_backend::DrawingBackend;

pub trait HistogramType {}
pub struct Vertical;
pub struct Horizontal;

impl HistogramType for Vertical {}
impl HistogramType for Horizontal {}

/**
Presents data in a histogram. Input data can be raw or aggregated.

# Examples

```
use plotters::prelude::*;
let data = [1, 1, 2, 2, 1, 3, 3, 2, 2, 1, 1, 2, 2, 2, 3, 3, 1, 2, 3];
let drawing_area = SVGBackend::new("histogram_vertical.svg", (300, 200)).into_drawing_area();
drawing_area.fill(&WHITE).unwrap();
let mut chart_builder = ChartBuilder::on(&drawing_area);
chart_builder.margin(5).set_left_and_bottom_label_area_size(20);
let mut chart_context = chart_builder.build_cartesian_2d((1..3).into_segmented(), 0..9).unwrap();
chart_context.configure_mesh().draw().unwrap();
chart_context.draw_series(Histogram::vertical(&chart_context).style(BLUE.filled()).margin(10)
    .data(data.map(|x| (x, 1)))).unwrap();
```

The result is a histogram counting the occurrences of 1, 2, and 3 in `data`:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@a617d37/apidoc/histogram_vertical.svg)

Here is a variation with [`Histogram::horizontal()`], replacing `(1..3).into_segmented(), 0..9` with
`0..9, (1..3).into_segmented()`:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@a617d37/apidoc/histogram_horizontal.svg)

The spacing between histogram bars is adjusted with [`Histogram::margin()`].
Here is a version of the figure where `.margin(10)` has been replaced by `.margin(20)`;
the resulting bars are narrow and more spaced:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@a617d37/apidoc/histogram_margin20.svg)

[`crate::coord::ranged1d::IntoSegmentedCoord::into_segmented()`] is useful for discrete data; it makes sure the histogram bars
are centered on each data value. Here is another variation with `(1..3).into_segmented()`
replaced by `1..4`:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@a617d37/apidoc/histogram_not_segmented.svg)

[`Histogram::style()`] sets the style of the bars. Here is a histogram without `.filled()`:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@a617d37/apidoc/histogram_hollow.svg)

The following version uses [`Histogram::style_func()`] for finer control. Let's replace `.style(BLUE.filled())` with
`.style_func(|x, _bar_height| if let SegmentValue::Exact(v) = x {[BLACK, RED, GREEN, BLUE][*v as usize].filled()} else {BLACK.filled()})`.
The resulting bars come in different colors:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@a617d37/apidoc/histogram_style_func.svg)

[`Histogram::baseline()`] adjusts the base of the bars. The following figure adds `.baseline(1)`
to the right of `.margin(10)`. The lower portion of the bars are removed:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@a617d37/apidoc/histogram_baseline.svg)

The following figure uses [`Histogram::baseline_func()`] for finer control. Let's add
`.baseline_func(|x| if let SegmentValue::Exact(v) = x {*v as i32} else {0})`
to the right of `.margin(10)`. The lower portion of the bars are removed; the removed portion is taller
to the right:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@a617d37/apidoc/histogram_baseline_func.svg)
*/
pub struct Histogram<'a, BR, A, Tag = Vertical>
where
    BR: DiscreteRanged,
    A: AddAssign<A> + Default,
    Tag: HistogramType,
{
    style: Box<dyn Fn(&BR::ValueType, &A) -> ShapeStyle + 'a>,
    margin: u32,
    iter: HashMapIter<usize, A>,
    baseline: Box<dyn Fn(&BR::ValueType) -> A + 'a>,
    br: BR,
    _p: PhantomData<Tag>,
}

impl<'a, BR, A, Tag> Histogram<'a, BR, A, Tag>
where
    BR: DiscreteRanged + Clone,
    A: AddAssign<A> + Default + 'a,
    Tag: HistogramType,
{
    fn empty(br: &BR) -> Self {
        Self {
            style: Box::new(|_, _| GREEN.filled()),
            margin: 5,
            iter: HashMap::new().into_iter(),
            baseline: Box::new(|_| A::default()),
            br: br.clone(),
            _p: PhantomData,
        }
    }
    /**
    Sets the style of the histogram bars.

    See [`Histogram`] for more information and examples.
    */
    pub fn style<S: Into<ShapeStyle>>(mut self, style: S) -> Self {
        let style = style.into();
        self.style = Box::new(move |_, _| style);
        self
    }

    /**
    Sets the style of histogram using a closure.

    The closure takes the position of the bar in guest coordinates as argument.
    The argument may need some processing if the data range has been transformed by
    [`crate::coord::ranged1d::IntoSegmentedCoord::into_segmented()`] as shown in the [`Histogram`] example.
    */
    pub fn style_func(
        mut self,
        style_func: impl Fn(&BR::ValueType, &A) -> ShapeStyle + 'a,
    ) -> Self {
        self.style = Box::new(style_func);
        self
    }

    /**
    Sets the baseline of the histogram.

    See [`Histogram`] for more information and examples.
    */
    pub fn baseline(mut self, baseline: A) -> Self
    where
        A: Clone,
    {
        self.baseline = Box::new(move |_| baseline.clone());
        self
    }

    /**
    Sets the histogram bar baselines using a closure.

    The closure takes the bar position and height as argument.
    The argument may need some processing if the data range has been transformed by
    [`crate::coord::ranged1d::IntoSegmentedCoord::into_segmented()`] as shown in the [`Histogram`] example.
    */
    pub fn baseline_func(mut self, func: impl Fn(&BR::ValueType) -> A + 'a) -> Self {
        self.baseline = Box::new(func);
        self
    }

    /**
    Sets the margin for each bar, in backend pixels.

    See [`Histogram`] for more information and examples.
    */
    pub fn margin(mut self, value: u32) -> Self {
        self.margin = value;
        self
    }

    /**
    Specifies the input data for the histogram through an appropriate data iterator.

    See [`Histogram`] for more information and examples.
    */
    pub fn data<TB: Into<BR::ValueType>, I: IntoIterator<Item = (TB, A)>>(
        mut self,
        iter: I,
    ) -> Self {
        let mut buffer = HashMap::<usize, A>::new();
        for (x, y) in iter.into_iter() {
            if let Some(x) = self.br.index_of(&x.into()) {
                *buffer.entry(x).or_default() += y;
            }
        }
        self.iter = buffer.into_iter();
        self
    }
}

impl<'a, BR, A> Histogram<'a, BR, A, Vertical>
where
    BR: DiscreteRanged + Clone,
    A: AddAssign<A> + Default + 'a,
{
    /**
    Creates a vertical histogram.

    See [`Histogram`] for more information and examples.
    */
    pub fn vertical<ACoord, DB: DrawingBackend + 'a>(
        parent: &ChartContext<DB, Cartesian2d<BR, ACoord>>,
    ) -> Self
    where
        ACoord: Ranged<ValueType = A>,
    {
        let dp = parent.as_coord_spec().x_spec();

        Self::empty(dp)
    }
}

impl<'a, BR, A> Histogram<'a, BR, A, Horizontal>
where
    BR: DiscreteRanged + Clone,
    A: AddAssign<A> + Default + 'a,
{
    /**
    Creates a horizontal histogram.

    See [`Histogram`] for more information and examples.
    */
    pub fn horizontal<ACoord, DB: DrawingBackend>(
        parent: &ChartContext<DB, Cartesian2d<ACoord, BR>>,
    ) -> Self
    where
        ACoord: Ranged<ValueType = A>,
    {
        let dp = parent.as_coord_spec().y_spec();
        Self::empty(dp)
    }
}

impl<'a, BR, A> Iterator for Histogram<'a, BR, A, Vertical>
where
    BR: DiscreteRanged,
    A: AddAssign<A> + Default,
{
    type Item = Rectangle<(BR::ValueType, A)>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((x, y)) = self.iter.next() {
            if let Some((x, Some(nx))) = self
                .br
                .from_index(x)
                .map(|v| (v, self.br.from_index(x + 1)))
            {
                let base = (self.baseline)(&x);
                let style = (self.style)(&x, &y);
                let mut rect = Rectangle::new([(x, y), (nx, base)], style);
                rect.set_margin(0, 0, self.margin, self.margin);
                return Some(rect);
            }
        }
        None
    }
}

impl<'a, BR, A> Iterator for Histogram<'a, BR, A, Horizontal>
where
    BR: DiscreteRanged,
    A: AddAssign<A> + Default,
{
    type Item = Rectangle<(A, BR::ValueType)>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((y, x)) = self.iter.next() {
            if let Some((y, Some(ny))) = self
                .br
                .from_index(y)
                .map(|v| (v, self.br.from_index(y + 1)))
            {
                let base = (self.baseline)(&y);
                let style = (self.style)(&y, &x);
                let mut rect = Rectangle::new([(x, y), (base, ny)], style);
                rect.set_margin(self.margin, self.margin, 0, 0);
                return Some(rect);
            }
        }
        None
    }
}
