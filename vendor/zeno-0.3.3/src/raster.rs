//! Path rasterizer.

#![allow(clippy::too_many_arguments)]

use super::geometry::{Point, Vector};
use super::path_builder::PathBuilder;
use super::style::Fill;

use crate::lib::Vec;
use core::fmt;

#[inline(always)]
fn coverage(fill: Fill, mut coverage: i32) -> u8 {
    coverage >>= PIXEL_BITS * 2 + 1 - 8;
    if fill == Fill::EvenOdd {
        coverage &= 511;
        if coverage >= 256 {
            coverage = 511i32.wrapping_sub(coverage);
        }
    } else {
        if coverage < 0 {
            coverage = !coverage;
        }
        if coverage >= 256 {
            coverage = 255;
        }
    }
    coverage as u8
}

pub struct Rasterizer<'a, S: RasterStorage> {
    storage: &'a mut S,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    height: i32,
    shift: Vector,
    start: FixedPoint,
    closed: bool,
    current: Point,
    x: i32,
    y: i32,
    px: i32,
    py: i32,
    cover: i32,
    area: i32,
    invalid: bool,
}

impl<'a, S: RasterStorage> Rasterizer<'a, S> {
    pub fn new(storage: &'a mut S) -> Self {
        Self {
            storage,
            xmin: 0,
            xmax: 0,
            ymin: 0,
            ymax: 0,
            height: 0,
            shift: Vector::ZERO,
            start: FixedPoint::default(),
            closed: false,
            current: Point::ZERO,
            x: 0,
            y: 0,
            px: 0,
            py: 0,
            cover: 0,
            area: 0,
            invalid: false,
        }
    }

    pub fn rasterize(
        &mut self,
        shift: Vector,
        width: u32,
        height: u32,
        apply: &mut impl FnMut(&mut Self),
        fill: Fill,
        buffer: &mut [u8],
        pitch: usize,
        y_up: bool,
    ) {
        let w = width as i32;
        let h = height as i32;
        self.storage
            .reset(FixedPoint { x: 0, y: 0 }, FixedPoint { x: w, y: h });
        self.shift = shift;
        self.start = FixedPoint::default();
        self.closed = true;
        self.current = Point::ZERO;
        self.xmin = 0;
        self.ymin = 0;
        self.xmax = w;
        self.ymax = h;
        self.height = h;
        self.x = 0;
        self.y = 0;
        self.px = 0;
        self.py = 0;
        self.invalid = true;
        apply(self);
        if !self.closed {
            self.line_to(self.start);
        }
        if !self.invalid {
            self.storage.set(self.x, self.y, self.area, self.cover);
        }
        let indices = self.storage.indices();
        let cells = self.storage.cells();
        let min = FixedPoint::new(self.xmin, self.ymin);
        let max = FixedPoint::new(self.xmax, self.ymax);
        let height = height as usize;
        for (i, &index) in indices.iter().enumerate() {
            if index != -1 {
                let y = ((i as i32) - min.y) as usize;
                let row_offset = if y_up {
                    pitch * (height - 1 - y)
                } else {
                    pitch * y
                };
                let row = &mut buffer[row_offset..];
                let mut x = min.x;
                let mut cover = 0;
                let mut area;
                let mut index = index;
                loop {
                    let cell = &cells[index as usize];
                    if cover != 0 && cell.x > x {
                        let count = (cell.x - x) as usize;
                        let c = coverage(fill, cover);
                        let xi = x as usize;
                        for b in &mut row[xi..xi + count] {
                            *b = c;
                        }
                    }
                    cover = cover.wrapping_add(cell.cover.wrapping_mul(ONE_PIXEL * 2));
                    area = cover.wrapping_sub(cell.area);
                    if area != 0 && cell.x >= min.x {
                        let count = 1;
                        let c = coverage(fill, area);
                        let xi = cell.x as usize;
                        for b in &mut row[xi..xi + count] {
                            *b = c;
                        }
                    }
                    x = cell.x + 1;
                    index = cell.next;
                    if index == -1 {
                        break;
                    }
                }
                if cover != 0 {
                    let count = (max.x - x) as usize;
                    let c = coverage(fill, cover);
                    let xi = x as usize;
                    for b in &mut row[xi..xi + count] {
                        *b = c;
                    }
                }
            }
        }
    }

    pub fn rasterize_write(
        &mut self,
        shift: Vector,
        width: u32,
        height: u32,
        apply: &mut impl FnMut(&mut Self),
        fill: Fill,
        pitch: usize,
        y_up: bool,
        write: &mut impl FnMut(usize, usize, usize, u8),
    ) {
        let w = width as i32;
        let h = height as i32;
        self.storage
            .reset(FixedPoint { x: 0, y: 0 }, FixedPoint { x: w, y: h });
        self.shift = shift;
        self.start = FixedPoint::default();
        self.closed = true;
        self.current = Point::ZERO;
        self.xmin = 0;
        self.ymin = 0;
        self.xmax = w;
        self.ymax = h;
        self.height = h;
        self.x = 0;
        self.y = 0;
        self.px = 0;
        self.py = 0;
        self.invalid = true;
        apply(self);
        if !self.closed {
            self.line_to(self.start);
        }
        if !self.invalid {
            self.storage.set(self.x, self.y, self.area, self.cover);
        }
        let indices = self.storage.indices();
        let cells = self.storage.cells();
        let min = FixedPoint::new(self.xmin, self.ymin);
        let max = FixedPoint::new(self.xmax, self.ymax);
        let height = height as usize;
        for (i, &index) in indices.iter().enumerate() {
            if index != -1 {
                let y = ((i as i32) - min.y) as usize;
                let row_offset = if y_up {
                    pitch * (height - 1 - y)
                } else {
                    pitch * y
                };
                let mut x = min.x;
                let mut cover = 0;
                let mut area;
                let mut index = index;
                loop {
                    let cell = &cells[index as usize];
                    if cover != 0 && cell.x > x {
                        let count = (cell.x - x) as usize;
                        let c = coverage(fill, cover);
                        let xi = x as usize;
                        write(row_offset, xi, count, c);
                    }
                    cover = cover.wrapping_add(cell.cover.wrapping_mul(ONE_PIXEL * 2));
                    area = cover.wrapping_sub(cell.area);
                    if area != 0 && cell.x >= min.x {
                        let count = 1;
                        let c = coverage(fill, area);
                        let xi = cell.x as usize;
                        write(row_offset, xi, count, c);
                    }
                    x = cell.x + 1;
                    index = cell.next;
                    if index == -1 {
                        break;
                    }
                }
                if cover != 0 {
                    let count = (max.x - x) as usize;
                    let c = coverage(fill, cover);
                    let xi = x as usize;
                    write(row_offset, xi, count, c);
                }
            }
        }
    }

    #[inline(always)]
    fn set_cell(&mut self, x: i32, y: i32) {
        if !self.invalid && (self.area != 0 || self.cover != 0) {
            self.storage.set(self.x, self.y, self.area, self.cover);
        }
        self.area = 0;
        self.cover = 0;
        self.x = if x > (self.xmin - 1) {
            x
        } else {
            self.xmin - 1
        };
        self.y = y;
        self.invalid = y >= self.ymax || y < self.ymin || x >= self.xmax;
    }

    fn move_to(&mut self, to: FixedPoint) {
        self.set_cell(trunc(to.x), trunc(to.y));
        self.px = to.x;
        self.py = to.y;
    }

    fn line_to(&mut self, to: FixedPoint) {
        let to_x = to.x;
        let to_y = to.y;
        let mut ey1 = trunc(self.py);
        let ey2 = trunc(to_y);
        if (ey1 >= self.ymax && ey2 >= self.ymax) || (ey1 < self.ymin && ey2 < self.ymin) {
            self.px = to_x;
            self.py = to_y;
            return;
        }
        let mut ex1 = trunc(self.px);
        let ex2 = trunc(to_x);
        let mut fx1 = fract(self.px);
        let mut fy1 = fract(self.py);
        let dx = to_x - self.px;
        let dy = to_y - self.py;
        if ex1 == ex2 && ey1 == ey2 {
            // empty
        } else if dy == 0 {
            self.set_cell(ex2, ey2);
            self.px = to_x;
            self.py = to_y;
            return;
        } else if dx == 0 {
            if dy > 0 {
                loop {
                    let fy2 = ONE_PIXEL;
                    self.cover += fy2 - fy1;
                    self.area += (fy2 - fy1) * fx1 * 2;
                    fy1 = 0;
                    ey1 += 1;
                    self.set_cell(ex1, ey1);
                    if ey1 == ey2 {
                        break;
                    }
                }
            } else {
                loop {
                    let fy2 = 0;
                    self.cover += fy2 - fy1;
                    self.area += (fy2 - fy1) * fx1 * 2;
                    fy1 = ONE_PIXEL;
                    ey1 -= 1;
                    self.set_cell(ex1, ey1);
                    if ey1 == ey2 {
                        break;
                    }
                }
            }
        } else {
            let mut prod = dx * fy1 - dy * fx1;
            let dx_r = if ex1 != ex2 { (0x00FFFFFF) / dx } else { 0 };
            let dy_r = if ey1 != ey2 { (0x00FFFFFF) / dy } else { 0 };
            fn udiv(a: i32, b: i32) -> i32 {
                ((a as u64 * b as u64) >> (4 * 8 - PIXEL_BITS)) as i32
            }
            loop {
                if prod <= 0 && prod - dx * ONE_PIXEL > 0 {
                    let fx2 = 0;
                    let fy2 = udiv(-prod, -dx_r);
                    prod -= dy * ONE_PIXEL;
                    self.cover += fy2 - fy1;
                    self.area += (fy2 - fy1) * (fx1 + fx2);
                    fx1 = ONE_PIXEL;
                    fy1 = fy2;
                    ex1 -= 1;
                } else if prod - dx * ONE_PIXEL <= 0 && prod - dx * ONE_PIXEL + dy * ONE_PIXEL > 0 {
                    prod -= dx * ONE_PIXEL;
                    let fx2 = udiv(-prod, dy_r);
                    let fy2 = ONE_PIXEL;
                    self.cover += fy2 - fy1;
                    self.area += (fy2 - fy1) * (fx1 + fx2);
                    fx1 = fx2;
                    fy1 = 0;
                    ey1 += 1;
                } else if prod - dx * ONE_PIXEL + dy * ONE_PIXEL <= 0 && prod + dy * ONE_PIXEL >= 0
                {
                    prod += dy * ONE_PIXEL;
                    let fx2 = ONE_PIXEL;
                    let fy2 = udiv(prod, dx_r);
                    self.cover += fy2 - fy1;
                    self.area += (fy2 - fy1) * (fx1 + fx2);
                    fx1 = 0;
                    fy1 = fy2;
                    ex1 += 1;
                } else {
                    let fx2 = udiv(prod, -dy_r);
                    let fy2 = 0;
                    prod += dx * ONE_PIXEL;
                    self.cover += fy2 - fy1;
                    self.area += (fy2 - fy1) * (fx1 + fx2);
                    fx1 = fx2;
                    fy1 = ONE_PIXEL;
                    ey1 -= 1;
                }
                self.set_cell(ex1, ey1);
                if ex1 == ex2 && ey1 == ey2 {
                    break;
                }
            }
        }
        let fx2 = fract(to_x);
        let fy2 = fract(to_y);
        self.cover += fy2 - fy1;
        self.area += (fy2 - fy1) * (fx1 + fx2);
        self.px = to_x;
        self.py = to_y;
    }

    #[allow(clippy::uninit_assumed_init, invalid_value)]
    fn quad_to(&mut self, control: FixedPoint, to: FixedPoint) {
        let mut arc = [FixedPoint::default(); 16 * 2 + 1];
        arc[0].x = to.x;
        arc[0].y = to.y;
        arc[1].x = control.x;
        arc[1].y = control.y;
        arc[2].x = self.px;
        arc[2].y = self.py;
        if (trunc(arc[0].y) >= self.ymax
            && trunc(arc[1].y) >= self.ymax
            && trunc(arc[2].y) >= self.ymax)
            || (trunc(arc[0].y) < self.ymin
                && trunc(arc[1].y) < self.ymin
                && trunc(arc[2].y) < self.ymin)
        {
            self.px = arc[0].x;
            self.py = arc[0].y;
            return;
        }
        let mut dx = (arc[2].x + arc[0].x - 2 * arc[1].x).abs();
        let dy = (arc[2].y + arc[0].y - 2 * arc[1].y).abs();
        if dx < dy {
            dx = dy;
        }
        let mut draw = 1;
        while dx > ONE_PIXEL / 4 {
            dx >>= 2;
            draw <<= 1;
        }
        let mut a = 0;
        loop {
            let mut split = draw & (-draw);
            loop {
                split >>= 1;
                if split == 0 {
                    break;
                }
                split_quad(&mut arc[a..]);
                a += 2;
            }
            let p = arc[a];
            self.line_to(p);
            draw -= 1;
            if draw == 0 {
                break;
            }
            a -= 2;
        }
    }

    #[allow(clippy::uninit_assumed_init, invalid_value)]
    fn curve_to(&mut self, control1: FixedPoint, control2: FixedPoint, to: FixedPoint) {
        let mut arc = [FixedPoint::default(); 16 * 8 + 1];
        arc[0].x = to.x;
        arc[0].y = to.y;
        arc[1].x = control2.x;
        arc[1].y = control2.y;
        arc[2].x = control1.x;
        arc[2].y = control1.y;
        arc[3].x = self.px;
        arc[3].y = self.py;
        if (trunc(arc[0].y) >= self.ymax
            && trunc(arc[1].y) >= self.ymax
            && trunc(arc[2].y) >= self.ymax
            && trunc(arc[3].y) >= self.ymax)
            || (trunc(arc[0].y) < self.ymin
                && trunc(arc[1].y) < self.ymin
                && trunc(arc[2].y) < self.ymin
                && trunc(arc[3].y) < self.ymin)
        {
            self.px = arc[0].x;
            self.py = arc[0].y;
            return;
        }
        let mut a = 0;
        loop {
            if (2 * arc[a].x - 3 * arc[a + 1].x + arc[a + 3].x).abs() > ONE_PIXEL / 2
                || (2 * arc[a].y - 3 * arc[a + 1].y + arc[a + 3].y).abs() > ONE_PIXEL / 2
                || (arc[a].x - 3 * arc[a + 2].x + 2 * arc[a + 3].x).abs() > ONE_PIXEL / 2
                || (arc[a].y - 3 * arc[a + 2].y + 2 * arc[a + 3].y).abs() > ONE_PIXEL / 2
            {
                let buf = &mut arc[a..];
                // if buf.len() < 7 {
                //     return;
                // }
                if buf.len() >= 7 {
                    split_cubic(buf);
                    a += 3;
                    continue;
                } else {
                    self.line_to(to);
                    return;
                }
            }
            let p = arc[a];
            self.line_to(p);
            if a == 0 {
                return;
            }
            a -= 3;
        }
    }
}

impl<S: RasterStorage> PathBuilder for Rasterizer<'_, S> {
    fn current_point(&self) -> Point {
        self.current + self.shift
    }

    #[inline(always)]
    fn move_to(&mut self, to: impl Into<Point>) -> &mut Self {
        if !self.closed {
            self.line_to(self.start);
        }
        let to = to.into();
        let p = FixedPoint::from_point(to + self.shift);
        self.move_to(p);
        self.closed = false;
        self.start = p;
        self.current = to;
        self
    }

    #[inline(always)]
    fn line_to(&mut self, to: impl Into<Point>) -> &mut Self {
        let to = to.into();
        self.current = to;
        self.closed = false;
        self.line_to(FixedPoint::from_point(to + self.shift));
        self
    }

    #[inline(always)]
    fn quad_to(&mut self, control: impl Into<Point>, to: impl Into<Point>) -> &mut Self {
        let to = to.into();
        self.current = to;
        self.closed = false;
        self.quad_to(
            FixedPoint::from_point(control.into() + self.shift),
            FixedPoint::from_point(to + self.shift),
        );
        self
    }

    #[inline(always)]
    fn curve_to(
        &mut self,
        control1: impl Into<Point>,
        control2: impl Into<Point>,
        to: impl Into<Point>,
    ) -> &mut Self {
        let to = to.into();
        self.current = to;
        self.closed = false;
        self.curve_to(
            FixedPoint::from_point(control1.into() + self.shift),
            FixedPoint::from_point(control2.into() + self.shift),
            FixedPoint::from_point(to + self.shift),
        );
        self
    }

    #[inline(always)]
    fn close(&mut self) -> &mut Self {
        self.line_to(self.start);
        self.closed = true;
        self
    }
}

#[derive(Copy, Clone, Default)]
pub struct Cell {
    x: i32,
    cover: i32,
    area: i32,
    next: i32,
}

pub trait RasterStorage {
    fn reset(&mut self, min: FixedPoint, max: FixedPoint);
    fn cells(&self) -> &[Cell];
    fn indices(&self) -> &[i32];
    fn set(&mut self, x: i32, y: i32, area: i32, cover: i32);
}

#[derive(Default)]
pub struct HeapStorage {
    min: FixedPoint,
    max: FixedPoint,
    cells: Vec<Cell>,
    indices: Vec<i32>,
}

impl RasterStorage for HeapStorage {
    fn reset(&mut self, min: FixedPoint, max: FixedPoint) {
        self.min = min;
        self.max = max;
        self.cells.clear();
        self.indices.clear();
        self.indices.resize((max.y - min.y) as usize, -1);
    }

    fn cells(&self) -> &[Cell] {
        &self.cells
    }

    fn indices(&self) -> &[i32] {
        &self.indices
    }

    #[inline(always)]
    #[allow(clippy::comparison_chain)]
    fn set(&mut self, x: i32, y: i32, area: i32, cover: i32) {
        let yindex = (y - self.min.y) as usize;
        let mut cell_index = self.indices[yindex];
        let mut last_index = -1;
        while cell_index != -1 {
            let cell = &mut self.cells[cell_index as usize];
            if cell.x > x {
                break;
            } else if cell.x == x {
                cell.area = cell.area.wrapping_add(area);
                cell.cover = cell.cover.wrapping_add(cover);
                return;
            }
            last_index = cell_index;
            cell_index = cell.next;
        }
        let new_index = self.cells.len();
        let cell = Cell {
            x,
            area,
            cover,
            next: cell_index,
        };
        if last_index != -1 {
            self.cells[last_index as usize].next = new_index as i32;
        } else {
            self.indices[yindex] = new_index as i32;
        }
        self.cells.push(cell);
    }
}

const MAX_CELLS: usize = 1024;
const MAX_BAND: usize = 512;

pub struct AdaptiveStorage {
    min: FixedPoint,
    max: FixedPoint,
    height: usize,
    cell_count: usize,
    cells: [Cell; MAX_CELLS],
    heap_cells: Vec<Cell>,
    indices: [i32; MAX_BAND],
    heap_indices: Vec<i32>,
}

impl AdaptiveStorage {
    #[allow(clippy::uninit_assumed_init, invalid_value)]
    pub fn new() -> Self {
        Self {
            min: FixedPoint::default(),
            max: FixedPoint::default(),
            height: 0,
            cell_count: 0,
            cells: [Default::default(); MAX_CELLS],
            heap_cells: Vec::new(),
            indices: [Default::default(); MAX_BAND],
            heap_indices: Vec::new(),
        }
    }
}

impl RasterStorage for AdaptiveStorage {
    fn reset(&mut self, min: FixedPoint, max: FixedPoint) {
        self.min = min;
        self.max = max;
        self.height = (max.y - min.y) as usize;
        self.cell_count = 0;
        self.heap_cells.clear();
        self.heap_indices.clear();
        if self.height > MAX_BAND {
            self.heap_indices.resize((max.y - min.y) as usize, -1);
        } else {
            for i in 0..self.height {
                self.indices[i] = -1;
            }
        }
    }

    fn cells(&self) -> &[Cell] {
        if self.cell_count > MAX_CELLS {
            &self.heap_cells
        } else {
            &self.cells
        }
    }

    fn indices(&self) -> &[i32] {
        if self.height > MAX_BAND {
            &self.heap_indices
        } else {
            &self.indices[..self.height]
        }
    }

    #[inline(always)]
    #[allow(clippy::comparison_chain)]
    fn set(&mut self, x: i32, y: i32, area: i32, cover: i32) {
        let yindex = (y - self.min.y) as usize;
        let indices = if self.height > MAX_BAND {
            &mut self.heap_indices[..]
        } else {
            &mut self.indices[..]
        };
        let cells = if !self.heap_cells.is_empty() {
            &mut self.heap_cells[..]
        } else {
            &mut self.cells[..]
        };
        let mut cell_index = indices[yindex];
        let mut last_index = -1;
        while cell_index != -1 {
            let cell = &mut cells[cell_index as usize];
            if cell.x > x {
                break;
            } else if cell.x == x {
                cell.area = cell.area.wrapping_add(area);
                cell.cover = cell.cover.wrapping_add(cover);
                return;
            }
            last_index = cell_index;
            cell_index = cell.next;
        }
        let new_index = self.cell_count;
        self.cell_count += 1;
        let cell = Cell {
            x,
            area,
            cover,
            next: cell_index,
        };
        if last_index != -1 {
            cells[last_index as usize].next = new_index as i32;
        } else {
            indices[yindex] = new_index as i32;
        }
        if new_index < MAX_CELLS {
            cells[new_index] = cell;
        } else {
            if self.heap_cells.is_empty() {
                self.heap_cells.extend_from_slice(&self.cells);
            }
            self.heap_cells.push(cell);
        }
    }
}

const _MAX_DIM: u32 = i16::MAX as u32;

fn split_quad(base: &mut [FixedPoint]) {
    let mut a;
    let mut b;
    base[4].x = base[2].x;
    a = base[0].x + base[1].x;
    b = base[1].x + base[2].x;
    base[3].x = b >> 1;
    base[2].x = (a + b) >> 2;
    base[1].x = a >> 1;
    base[4].y = base[2].y;
    a = base[0].y + base[1].y;
    b = base[1].y + base[2].y;
    base[3].y = b >> 1;
    base[2].y = (a + b) >> 2;
    base[1].y = a >> 1;
}

fn split_cubic(base: &mut [FixedPoint]) {
    let mut a;
    let mut b;
    let mut c;
    base[6].x = base[3].x;
    a = base[0].x + base[1].x;
    b = base[1].x + base[2].x;
    c = base[2].x + base[3].x;
    base[5].x = c >> 1;
    c += b;
    base[4].x = c >> 2;
    base[1].x = a >> 1;
    a += b;
    base[2].x = a >> 2;
    base[3].x = (a + c) >> 3;
    base[6].y = base[3].y;
    a = base[0].y + base[1].y;
    b = base[1].y + base[2].y;
    c = base[2].y + base[3].y;
    base[5].y = c >> 1;
    c += b;
    base[4].y = c >> 2;
    base[1].y = a >> 1;
    a += b;
    base[2].y = a >> 2;
    base[3].y = (a + c) >> 3;
}

#[derive(Copy, Clone, Default)]
pub struct FixedPoint {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FixedPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn from_point(p: Point) -> Self {
        Self {
            x: to_fixed(p.x),
            y: to_fixed(p.y),
        }
    }
}

#[inline(always)]
fn to_fixed(v: f32) -> i32 {
    unsafe { (v * 256.).to_int_unchecked() }
}

const PIXEL_BITS: i32 = 8;
const ONE_PIXEL: i32 = 1 << PIXEL_BITS;

#[inline(always)]
fn trunc(x: i32) -> i32 {
    x >> PIXEL_BITS
}

#[inline(always)]
fn fract(x: i32) -> i32 {
    x & (ONE_PIXEL - 1)
}
