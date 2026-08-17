//! Key (or legend)

use std::borrow::Cow;

use crate::traits::Set;
use crate::{Default, Display, Script, Title};

/// Properties of the key
#[derive(Clone)]
pub struct Properties {
    boxed: bool,
    hidden: bool,
    justification: Option<Justification>,
    order: Option<Order>,
    position: Option<Position>,
    stacked: Option<Stacked>,
    title: Option<Cow<'static, str>>,
}

impl Default for Properties {
    fn default() -> Properties {
        Properties {
            boxed: false,
            hidden: false,
            justification: None,
            order: None,
            position: None,
            stacked: None,
            title: None,
        }
    }
}

impl Properties {
    /// Hides the key
    pub fn hide(&mut self) -> &mut Properties {
        self.hidden = true;
        self
    }

    /// Shows the key
    ///
    /// **Note** The key is shown by default
    pub fn show(&mut self) -> &mut Properties {
        self.hidden = false;
        self
    }
}

impl Script for Properties {
    // Allow clippy::format_push_string even with older versions of rust (<1.62) which
    // don't have it defined.
    #[allow(clippy::all)]
    fn script(&self) -> String {
        let mut script = if self.hidden {
            return String::from("set key off\n");
        } else {
            String::from("set key on ")
        };

        match self.position {
            None => {}
            Some(Position::Inside(v, h)) => {
                script.push_str(&format!("inside {} {} ", v.display(), h.display()))
            }
            Some(Position::Outside(v, h)) => {
                script.push_str(&format!("outside {} {} ", v.display(), h.display()))
            }
        }

        if let Some(stacked) = self.stacked {
            script.push_str(stacked.display());
            script.push(' ');
        }

        if let Some(justification) = self.justification {
            script.push_str(justification.display());
            script.push(' ');
        }

        if let Some(order) = self.order {
            script.push_str(order.display());
            script.push(' ');
        }

        if let Some(ref title) = self.title {
            script.push_str(&format!("title '{}' ", title))
        }

        if self.boxed {
            script.push_str("box ")
        }

        script.push('\n');
        script
    }
}

impl Set<Boxed> for Properties {
    /// Select if the key will be surrounded with a box or not
    ///
    /// **Note** The key is not boxed by default
    fn set(&mut self, boxed: Boxed) -> &mut Properties {
        match boxed {
            Boxed::No => self.boxed = false,
            Boxed::Yes => self.boxed = true,
        }

        self
    }
}

impl Set<Justification> for Properties {
    /// Changes the justification of the text of each entry
    ///
    /// **Note** The text is `RightJustified` by default
    fn set(&mut self, justification: Justification) -> &mut Properties {
        self.justification = Some(justification);
        self
    }
}

impl Set<Order> for Properties {
    /// How to order each entry
    ///
    /// **Note** The default order is `TextSample`
    fn set(&mut self, order: Order) -> &mut Properties {
        self.order = Some(order);
        self
    }
}

impl Set<Position> for Properties {
    /// Selects where to place the key
    ///
    /// **Note** By default, the key is placed `Inside(Vertical::Top, Horizontal::Right)`
    fn set(&mut self, position: Position) -> &mut Properties {
        self.position = Some(position);
        self
    }
}

impl Set<Stacked> for Properties {
    /// Changes how the entries of the key are stacked
    fn set(&mut self, stacked: Stacked) -> &mut Properties {
        self.stacked = Some(stacked);
        self
    }
}

impl Set<Title> for Properties {
    fn set(&mut self, title: Title) -> &mut Properties {
        self.title = Some(title.0);
        self
    }
}

/// Whether the key is surrounded by a box or not
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Boxed {
    No,
    Yes,
}

/// Horizontal position of the key
#[derive(Clone, Copy)]
pub enum Horizontal {
    /// Center of the figure
    Center,
    /// Left border of the figure
    Left,
    /// Right border of the figure
    Right,
}

/// Text justification of the key
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Justification {
    Left,
    Right,
}

/// Order of the elements of the key
#[derive(Clone, Copy)]
pub enum Order {
    /// Sample first, then text
    SampleText,
    /// Text first, then sample
    TextSample,
}

/// Position of the key
// TODO XY position
#[derive(Clone, Copy)]
pub enum Position {
    /// Inside the area surrounded by the four (BottomX, TopX, LeftY and RightY) axes
    Inside(Vertical, Horizontal),
    /// Outside of that area
    Outside(Vertical, Horizontal),
}

/// How the entries of the key are stacked
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Stacked {
    Horizontally,
    Vertically,
}

/// Vertical position of the key
#[derive(Clone, Copy)]
pub enum Vertical {
    /// Bottom border of the figure
    Bottom,
    /// Center of the figure
    Center,
    /// Top border of the figure
    Top,
}
