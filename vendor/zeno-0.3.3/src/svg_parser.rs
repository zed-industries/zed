//! SVG path data parser.

use super::command::Command;
use super::geometry::Vector;
use super::path_builder::{Arc, ArcSize, ArcSweep};

#[derive(Copy, Clone)]
enum State {
    Initial,
    Next,
    Continue(u8),
}

#[derive(Clone)]
pub struct SvgCommands<'a> {
    buf: &'a [u8],
    cur: u8,
    pub pos: usize,
    cmd_pos: usize,
    pub error: bool,
    pub done: bool,
    start_point: Vector,
    cur_point: Vector,
    last_control: Vector,
    last_cmd: u8,
    state: State,
    arc: Arc,
}

impl Iterator for SvgCommands<'_> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

impl<'a> SvgCommands<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            buf: source.as_bytes(),
            cur: 0,
            pos: 0,
            cmd_pos: 0,
            error: false,
            done: false,
            start_point: Vector::ZERO,
            cur_point: Vector::ZERO,
            last_control: Vector::ZERO,
            last_cmd: 0,
            state: State::Initial,
            arc: Arc::default(),
        }
    }

    fn parse(&mut self) -> Option<Command> {
        use Command::*;
        let mut cmd = self.cur;
        loop {
            if let Some(cmd) = self.arc.next() {
                return Some(cmd);
            }
            self.last_cmd = cmd;
            match self.state {
                State::Initial => {
                    self.advance();
                    self.skip_whitespace();
                    self.state = State::Next;
                    continue;
                }
                State::Next => {
                    self.skip_whitespace();
                    self.cmd_pos = self.pos;
                    cmd = self.cur;
                    self.advance();
                    self.skip_whitespace();
                    self.state = State::Continue(cmd);
                    match cmd {
                        b'z' | b'Z' => {
                            self.state = State::Next;
                            self.cur_point = self.start_point;
                            return Some(Close);
                        }
                        b'M' => {
                            let to = self.point_to()?;
                            self.start_point = to;
                            self.skip_comma_whitespace();
                            return Some(MoveTo(to));
                        }
                        b'm' => {
                            let to = self.rel_point_to()?;
                            self.start_point = to;
                            self.skip_comma_whitespace();
                            return Some(MoveTo(to));
                        }
                        b'L' => {
                            let to = self.point_to()?;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        }
                        b'l' => {
                            let to = self.rel_point_to()?;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        }
                        b'H' => {
                            let x = self.coord()?;
                            let to = Vector::new(x, self.cur_point.y);
                            self.cur_point = to;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        }
                        b'h' => {
                            let x = self.coord()?;
                            let to = Vector::new(self.cur_point.x + x, self.cur_point.y);
                            self.cur_point = to;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        }
                        b'V' => {
                            let y = self.coord()?;
                            let to = Vector::new(self.cur_point.x, y);
                            self.cur_point = to;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        }
                        b'v' => {
                            let y = self.coord()?;
                            let to = Vector::new(self.cur_point.x, self.cur_point.y + y);
                            self.cur_point = to;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        }
                        b'C' => {
                            let (c1, c2, to) = self.three_points_to()?;
                            self.last_control = c2;
                            self.skip_comma_whitespace();
                            return Some(CurveTo(c1, c2, to));
                        }
                        b'c' => {
                            let (c1, c2, to) = self.rel_three_points_to()?;
                            self.last_control = c2;
                            self.skip_comma_whitespace();
                            return Some(CurveTo(c1, c2, to));
                        }
                        b'S' => {
                            let (c2, to) = self.two_points()?;
                            let c1 = self.reflected_control(cmd);
                            self.cur_point = to;
                            self.last_control = c2;
                            self.skip_comma_whitespace();
                            return Some(CurveTo(c1, c2, to));
                        }
                        b's' => {
                            let (c2, to) = self.rel_two_points()?;
                            let c1 = self.reflected_control(cmd);
                            self.cur_point = to;
                            self.last_control = c2;
                            self.skip_comma_whitespace();
                            return Some(CurveTo(c1, c2, to));
                        }
                        b'Q' => {
                            let (c, to) = self.two_points_to()?;
                            self.last_control = c;
                            self.skip_comma_whitespace();
                            return Some(QuadTo(c, to));
                        }
                        b'q' => {
                            let (c, to) = self.rel_two_points_to()?;
                            self.last_control = c;
                            self.skip_comma_whitespace();
                            return Some(QuadTo(c, to));
                        }
                        b'T' => {
                            let to = self.point()?;
                            let c = self.reflected_control(cmd);
                            self.cur_point = to;
                            self.last_control = c;
                            self.skip_comma_whitespace();
                            return Some(QuadTo(c, to));
                        }
                        b't' => {
                            let to = self.rel_point()?;
                            let c = self.reflected_control(cmd);
                            self.cur_point = to;
                            self.last_control = c;
                            self.skip_comma_whitespace();
                            return Some(QuadTo(c, to));
                        }
                        b'A' => {
                            let from = self.cur_point;
                            let (rx, ry, a, size, sweep, to) = self.arc_arguments(false)?;
                            self.arc = Arc::new(from, rx, ry, a.to_radians(), size, sweep, to);
                            self.skip_comma_whitespace();
                            continue;
                        }
                        b'a' => {
                            let from = self.cur_point;
                            let (rx, ry, a, size, sweep, to) = self.arc_arguments(true)?;
                            self.arc = Arc::new(from, rx, ry, a.to_radians(), size, sweep, to);
                            self.skip_comma_whitespace();
                            continue;
                        }
                        _ => {
                            if !self.done || cmd != 0 {
                                self.error = true;
                                self.pos = self.cmd_pos;
                            }
                            return None;
                        }
                    }
                }
                State::Continue(cmd) => match cmd {
                    b'M' => {
                        if let Some(to) = self.point_to() {
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'm' => {
                        if let Some(to) = self.rel_point_to() {
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'L' => {
                        if let Some(to) = self.point_to() {
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'l' => {
                        if let Some(to) = self.rel_point_to() {
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'H' => {
                        if let Some(x) = self.coord() {
                            let to = Vector::new(x, self.cur_point.y);
                            self.cur_point = to;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'h' => {
                        if let Some(x) = self.coord() {
                            let to = Vector::new(self.cur_point.x + x, self.cur_point.y);
                            self.cur_point = to;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'V' => {
                        if let Some(y) = self.coord() {
                            let to = Vector::new(self.cur_point.x, y);
                            self.cur_point = to;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'v' => {
                        if let Some(y) = self.coord() {
                            let to = Vector::new(self.cur_point.x, self.cur_point.y + y);
                            self.cur_point = to;
                            self.skip_comma_whitespace();
                            return Some(LineTo(to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'C' => {
                        if let Some(c1) = self.point() {
                            self.skip_comma_whitespace();
                            let (c2, to) = self.two_points_to()?;
                            self.last_control = c2;
                            self.skip_comma_whitespace();
                            return Some(CurveTo(c1, c2, to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'c' => {
                        if let Some(c1) = self.rel_point() {
                            self.skip_comma_whitespace();
                            let (c2, to) = self.rel_two_points_to()?;
                            self.last_control = c2;
                            self.skip_comma_whitespace();
                            return Some(CurveTo(c1, c2, to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'S' => {
                        if let Some(c2) = self.point() {
                            self.skip_comma_whitespace();
                            let to = self.point()?;
                            let c1 = self.reflected_control(cmd);
                            self.cur_point = to;
                            self.last_control = c2;
                            self.skip_comma_whitespace();
                            return Some(CurveTo(c1, c2, to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b's' => {
                        if let Some(c2) = self.rel_point() {
                            self.skip_comma_whitespace();
                            let to = self.rel_point()?;
                            let c1 = self.reflected_control(cmd);
                            self.cur_point = to;
                            self.last_control = c2;
                            self.skip_comma_whitespace();
                            return Some(CurveTo(c1, c2, to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'Q' => {
                        if let Some(c) = self.point() {
                            self.last_control = c;
                            self.skip_comma_whitespace();
                            let to = self.point_to()?;
                            self.skip_comma_whitespace();
                            return Some(QuadTo(c, to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'q' => {
                        if let Some(c) = self.rel_point() {
                            self.last_control = c;
                            self.skip_comma_whitespace();
                            let to = self.rel_point_to()?;
                            self.skip_comma_whitespace();
                            return Some(QuadTo(c, to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'T' => {
                        if let Some(to) = self.point() {
                            let c = self.reflected_control(cmd);
                            self.cur_point = to;
                            self.last_control = c;
                            self.skip_comma_whitespace();
                            return Some(QuadTo(c, to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b't' => {
                        if let Some(to) = self.rel_point() {
                            let c = self.reflected_control(cmd);
                            self.cur_point = to;
                            self.last_control = c;
                            self.skip_comma_whitespace();
                            return Some(QuadTo(c, to));
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'A' => {
                        if let Some(rx) = self.coord() {
                            let from = self.cur_point;
                            let (ry, a, size, sweep, to) = self.arc_rest_arguments(false)?;
                            self.arc = Arc::new(from, rx, ry, a.to_radians(), size, sweep, to);
                            self.skip_comma_whitespace();
                        } else {
                            self.state = State::Next;
                        }
                    }
                    b'a' => {
                        if let Some(rx) = self.coord() {
                            let from = self.cur_point;
                            let (ry, a, size, sweep, to) = self.arc_rest_arguments(true)?;
                            self.arc = Arc::new(from, rx, ry, a.to_radians(), size, sweep, to);
                            self.skip_comma_whitespace();
                        } else {
                            self.state = State::Next;
                        }
                    }
                    _ => {
                        if !self.done || cmd != 0 {
                            self.error = true;
                            self.pos = self.cmd_pos;
                        }
                        return None;
                    }
                },
            }
        }
    }

    fn reflected_control(&self, cmd: u8) -> Vector {
        let cur = self.cur_point;
        let old = self.last_control;
        if cmd == b'S' || cmd == b's' {
            match self.last_cmd {
                b'C' | b'c' | b'S' | b's' => (2. * cur.x - old.x, 2. * cur.y - old.y).into(),
                _ => self.cur_point,
            }
        } else {
            match self.last_cmd {
                b'Q' | b'q' | b'T' | b't' => (2. * cur.x - old.x, 2. * cur.y - old.y).into(),
                _ => self.cur_point,
            }
        }
    }

    fn arc_arguments(&mut self, rel: bool) -> Option<(f32, f32, f32, ArcSize, ArcSweep, Vector)> {
        let rx = self.coord()?;
        self.skip_comma_whitespace();
        let ry = self.coord()?;
        self.skip_comma_whitespace();
        let a = self.coord()?;
        self.skip_comma_whitespace();
        let large_arc = self.boolean()?;
        self.skip_comma_whitespace();
        let sweep = self.boolean()?;
        self.skip_comma_whitespace();
        let to = if rel {
            self.rel_point_to()?
        } else {
            self.point_to()?
        };
        let size = if large_arc {
            ArcSize::Large
        } else {
            ArcSize::Small
        };
        let sweep = if sweep {
            ArcSweep::Positive
        } else {
            ArcSweep::Negative
        };
        Some((rx, ry, a, size, sweep, to))
    }

    fn arc_rest_arguments(&mut self, rel: bool) -> Option<(f32, f32, ArcSize, ArcSweep, Vector)> {
        let ry = self.coord()?;
        self.skip_comma_whitespace();
        let a = self.coord()?;
        self.skip_comma_whitespace();
        let large_arc = self.boolean()?;
        self.skip_comma_whitespace();
        let sweep = self.boolean()?;
        self.skip_comma_whitespace();
        let to = if rel {
            self.rel_point_to()?
        } else {
            self.point_to()?
        };
        let size = if large_arc {
            ArcSize::Large
        } else {
            ArcSize::Small
        };
        let sweep = if sweep {
            ArcSweep::Positive
        } else {
            ArcSweep::Negative
        };
        Some((ry, a, size, sweep, to))
    }

    fn point(&mut self) -> Option<Vector> {
        let a = self.coord()?;
        self.skip_comma_whitespace();
        let b = self.coord()?;
        Some((a, b).into())
    }

    fn point_to(&mut self) -> Option<Vector> {
        let p = self.point()?;
        self.cur_point = p;
        Some(p)
    }

    fn rel_point(&mut self) -> Option<Vector> {
        let p = self.point()?;
        Some(p + self.cur_point)
    }

    fn rel_point_to(&mut self) -> Option<Vector> {
        let p = self.rel_point()?;
        self.cur_point = p;
        Some(p)
    }

    fn two_points_to(&mut self) -> Option<(Vector, Vector)> {
        let a = self.point()?;
        self.skip_comma_whitespace();
        let b = self.point_to()?;
        Some((a, b))
    }

    fn two_points(&mut self) -> Option<(Vector, Vector)> {
        let a = self.point()?;
        self.skip_comma_whitespace();
        let b = self.point()?;
        Some((a, b))
    }

    fn rel_two_points_to(&mut self) -> Option<(Vector, Vector)> {
        let a = self.rel_point()?;
        self.skip_comma_whitespace();
        let b = self.rel_point_to()?;
        Some((a, b))
    }

    fn rel_two_points(&mut self) -> Option<(Vector, Vector)> {
        let a = self.rel_point()?;
        self.skip_comma_whitespace();
        let b = self.rel_point()?;
        Some((a, b))
    }

    fn three_points_to(&mut self) -> Option<(Vector, Vector, Vector)> {
        let a = self.point()?;
        self.skip_comma_whitespace();
        let b = self.point()?;
        self.skip_comma_whitespace();
        let c = self.point_to()?;
        Some((a, b, c))
    }

    fn rel_three_points_to(&mut self) -> Option<(Vector, Vector, Vector)> {
        let a = self.rel_point()?;
        self.skip_comma_whitespace();
        let b = self.rel_point()?;
        self.skip_comma_whitespace();
        let c = self.rel_point_to()?;
        Some((a, b, c))
    }

    fn coord(&mut self) -> Option<f32> {
        match self.cur {
            b'+' => {
                self.advance();
                self.number()
            }
            b'-' => {
                self.advance();
                Some(-self.number()?)
            }
            _ => self.number(),
        }
    }

    fn number(&mut self) -> Option<f32> {
        let mut buf = [0u8; 32];
        let mut pos = 0;
        let mut has_decimal = false;
        loop {
            match self.cur {
                b'.' => {
                    if has_decimal {
                        break;
                    } else {
                        *buf.get_mut(pos)? = self.cur;
                        pos += 1;
                        has_decimal = true;
                    }
                }
                b'0'..=b'9' => {
                    *buf.get_mut(pos)? = self.cur;
                    pos += 1;
                }
                _ => break,
            }
            self.advance();
        }
        let s = core::str::from_utf8(&buf[..pos]).ok()?;
        s.parse::<f32>().ok()
    }

    fn boolean(&mut self) -> Option<bool> {
        match self.cur {
            b'0' => {
                self.advance();
                Some(false)
            }
            b'1' => {
                self.advance();
                Some(true)
            }
            _ => None,
        }
    }

    fn skip_comma_whitespace(&mut self) {
        self.skip_whitespace();
        if self.accept(b',') {
            self.skip_whitespace();
        }
    }

    #[allow(clippy::match_like_matches_macro)]
    fn skip_whitespace(&mut self) {
        while self.accept_by(|b| match b {
            0x9 | 0x20 | 0xA | 0xC | 0xD => true,
            _ => false,
        }) {}
    }

    fn accept(&mut self, b: u8) -> bool {
        if self.cur == b {
            self.advance();
            return true;
        }
        false
    }

    fn accept_by(&mut self, f: impl Fn(u8) -> bool) -> bool {
        if f(self.cur) {
            self.advance();
            return true;
        }
        false
    }

    fn advance(&mut self) {
        if self.pos == self.buf.len() {
            self.done = true;
            self.cur = 0;
            return;
        }
        self.cur = self.buf[self.pos];
        self.pos += 1;
    }
}

/// Returns an error indicating the first position of invalid SVG path data.
pub fn validate_svg(svg: &str) -> Result<(), usize> {
    let cmds = &mut SvgCommands::new(svg);
    cmds.count();
    let pos = cmds.pos;
    if cmds.error || pos != svg.len() {
        Err(pos.saturating_sub(1))
    } else {
        Ok(())
    }
}
