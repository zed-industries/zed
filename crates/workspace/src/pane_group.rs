use anyhow::{anyhow, Result};
use gpui::{elements::*, Axis, ViewHandle};
use theme::Theme;

use crate::Pane;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaneGroup {
    root: Member,
}

impl PaneGroup {
    pub fn new(pane: ViewHandle<Pane>) -> Self {
        Self {
            root: Member::Pane(pane),
        }
    }

    pub fn split(
        &mut self,
        old_pane: &ViewHandle<Pane>,
        new_pane: &ViewHandle<Pane>,
        direction: SplitDirection,
    ) -> Result<()> {
        match &mut self.root {
            Member::Pane(pane) => {
                if pane == old_pane {
                    self.root = Member::new_axis(old_pane.clone(), new_pane.clone(), direction);
                    Ok(())
                } else {
                    Err(anyhow!("Pane not found"))
                }
            }
            Member::Axis(axis) => axis.split(old_pane, new_pane, direction),
        }
    }

    pub fn find_adjacent(
        &self,
        focus_pane: &ViewHandle<Pane>,
        direction: SplitDirection,
    ) -> Option<ViewHandle<Pane>> {
        match &self.root {
            Member::Pane(_) => None,
            Member::Axis(axis) => {
                use SplitDirection::*;
                axis.find_adjacent(
                    focus_pane,
                    direction,
                    FindResult {
                        last: match direction {
                            Left | Up => axis.members.first().and_then(|m| m.first()),
                            Right | Down => axis.members.last().and_then(|m| m.last()),
                        },
                        success: false,
                    },
                )
                .into()
            }
        }
    }

    pub fn remove(&mut self, pane: &ViewHandle<Pane>) -> Result<bool> {
        match &mut self.root {
            Member::Pane(_) => Ok(false),
            Member::Axis(axis) => {
                if let Some(last_pane) = axis.remove(pane)? {
                    self.root = last_pane;
                }
                Ok(true)
            }
        }
    }

    pub fn render<'a>(&self, theme: &Theme) -> ElementBox {
        self.root.render(theme)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Member {
    Axis(PaneAxis),
    Pane(ViewHandle<Pane>),
}

struct FindResult {
    last: Option<ViewHandle<Pane>>,
    success: bool,
}

impl From<FindResult> for Option<ViewHandle<Pane>> {
    fn from(result: FindResult) -> Self {
        if result.success {
            result.last
        } else {
            None
        }
    }
}

impl Member {
    fn new_axis(
        old_pane: ViewHandle<Pane>,
        new_pane: ViewHandle<Pane>,
        direction: SplitDirection,
    ) -> Self {
        use Axis::*;
        use SplitDirection::*;

        let axis = match direction {
            Up | Down => Vertical,
            Left | Right => Horizontal,
        };

        let members = match direction {
            Up | Left => vec![Member::Pane(new_pane), Member::Pane(old_pane)],
            Down | Right => vec![Member::Pane(old_pane), Member::Pane(new_pane)],
        };

        Member::Axis(PaneAxis { axis, members })
    }

    fn find_adjacent(
        &self,
        focus_pane: &ViewHandle<Pane>,
        direction: SplitDirection,
        in_parallel_axis: bool,
        mut current_result: FindResult,
    ) -> FindResult {
        match self {
            Member::Axis(axis) => {
                current_result = axis.find_adjacent(focus_pane, direction, current_result);
            }
            Member::Pane(pane) => {
                if pane == focus_pane {
                    current_result.success = true;
                } else {
                    if in_parallel_axis {
                        current_result.last = Some(pane.clone());
                    }
                }
            }
        }
        current_result
    }

    fn first(&self) -> Option<ViewHandle<Pane>> {
        match self {
            Member::Axis(axis) => axis.members.first().and_then(|m| m.first()),
            Member::Pane(pane) => Some(pane.clone()),
        }
    }

    fn last(&self) -> Option<ViewHandle<Pane>> {
        match self {
            Member::Axis(axis) => axis.members.last().and_then(|m| m.last()),
            Member::Pane(pane) => Some(pane.clone()),
        }
    }

    pub fn render(&self, theme: &Theme) -> ElementBox {
        match self {
            Member::Pane(pane) => ChildView::new(pane).boxed(),
            Member::Axis(axis) => axis.render(theme),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PaneAxis {
    axis: Axis,
    members: Vec<Member>,
}

impl PaneAxis {
    fn split(
        &mut self,
        old_pane: &ViewHandle<Pane>,
        new_pane: &ViewHandle<Pane>,
        direction: SplitDirection,
    ) -> Result<()> {
        use SplitDirection::*;

        for (idx, member) in self.members.iter_mut().enumerate() {
            match member {
                Member::Axis(axis) => {
                    if axis.split(old_pane, new_pane, direction).is_ok() {
                        return Ok(());
                    }
                }
                Member::Pane(pane) => {
                    if pane == old_pane {
                        if direction.matches_axis(self.axis) {
                            match direction {
                                Up | Left => {
                                    self.members.insert(idx, Member::Pane(new_pane.clone()));
                                }
                                Down | Right => {
                                    self.members.insert(idx + 1, Member::Pane(new_pane.clone()));
                                }
                            }
                        } else {
                            *member =
                                Member::new_axis(old_pane.clone(), new_pane.clone(), direction);
                        }
                        return Ok(());
                    }
                }
            }
        }
        Err(anyhow!("Pane not found"))
    }

    fn find_adjacent(
        &self,
        focus_pane: &ViewHandle<Pane>,
        direction: SplitDirection,
        mut current_result: FindResult,
    ) -> FindResult {
        use SplitDirection::*;
        let in_matching_axis = direction.matches_axis(self.axis);
        match direction {
            Right | Down => {
                for member in self.members.iter().rev() {
                    current_result = member.find_adjacent(
                        focus_pane,
                        direction,
                        in_matching_axis,
                        current_result,
                    );
                    if current_result.success {
                        return current_result;
                    }
                }
                if !in_matching_axis {
                    current_result.last = self.members.first().and_then(|m| m.first());
                }
            }
            Left | Up => {
                for member in self.members.iter() {
                    current_result = member.find_adjacent(
                        focus_pane,
                        direction,
                        in_matching_axis,
                        current_result,
                    );
                    if current_result.success {
                        return current_result;
                    }
                }
                if !in_matching_axis {
                    current_result.last = self.members.last().and_then(|m| m.last());
                }
            }
        }
        current_result
    }

    fn remove(&mut self, pane_to_remove: &ViewHandle<Pane>) -> Result<Option<Member>> {
        let mut found_pane = false;
        let mut remove_member = None;
        for (idx, member) in self.members.iter_mut().enumerate() {
            match member {
                Member::Axis(axis) => {
                    if let Ok(last_pane) = axis.remove(pane_to_remove) {
                        if let Some(last_pane) = last_pane {
                            *member = last_pane;
                        }
                        found_pane = true;
                        break;
                    }
                }
                Member::Pane(pane) => {
                    if pane == pane_to_remove {
                        found_pane = true;
                        remove_member = Some(idx);
                        break;
                    }
                }
            }
        }

        if found_pane {
            if let Some(idx) = remove_member {
                self.members.remove(idx);
            }

            if self.members.len() == 1 {
                Ok(self.members.pop())
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow!("Pane not found"))
        }
    }

    fn render<'a>(&self, theme: &Theme) -> ElementBox {
        let last_member_ix = self.members.len() - 1;
        Flex::new(self.axis)
            .with_children(self.members.iter().enumerate().map(|(ix, member)| {
                let mut member = member.render(theme);
                if ix < last_member_ix {
                    let mut border = theme.workspace.pane_divider;
                    border.left = false;
                    border.right = false;
                    border.top = false;
                    border.bottom = false;
                    match self.axis {
                        Axis::Vertical => border.bottom = true,
                        Axis::Horizontal => border.right = true,
                    }
                    member = Container::new(member).with_border(border).boxed();
                }

                Flexible::new(1.0, true, member).boxed()
            }))
            .boxed()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SplitDirection {
    fn matches_axis(self, orientation: Axis) -> bool {
        use Axis::*;
        use SplitDirection::*;

        match self {
            Up | Down => match orientation {
                Vertical => true,
                Horizontal => false,
            },
            Left | Right => match orientation {
                Vertical => false,
                Horizontal => true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use serde_json::json;

    // #[test]
    // fn test_split_and_remove() -> Result<()> {
    //     let mut group = PaneGroup::new(1);
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "pane",
    //             "paneId": 1,
    //         })
    //     );

    //     group.split(1, 2, SplitDirection::Right)?;
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "axis",
    //             "orientation": "horizontal",
    //             "members": [
    //                 {"type": "pane", "paneId": 1},
    //                 {"type": "pane", "paneId": 2},
    //             ]
    //         })
    //     );

    //     group.split(2, 3, SplitDirection::Up)?;
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "axis",
    //             "orientation": "horizontal",
    //             "members": [
    //                 {"type": "pane", "paneId": 1},
    //                 {
    //                     "type": "axis",
    //                     "orientation": "vertical",
    //                     "members": [
    //                         {"type": "pane", "paneId": 3},
    //                         {"type": "pane", "paneId": 2},
    //                     ]
    //                 },
    //             ]
    //         })
    //     );

    //     group.split(1, 4, SplitDirection::Right)?;
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "axis",
    //             "orientation": "horizontal",
    //             "members": [
    //                 {"type": "pane", "paneId": 1},
    //                 {"type": "pane", "paneId": 4},
    //                 {
    //                     "type": "axis",
    //                     "orientation": "vertical",
    //                     "members": [
    //                         {"type": "pane", "paneId": 3},
    //                         {"type": "pane", "paneId": 2},
    //                     ]
    //                 },
    //             ]
    //         })
    //     );

    //     group.split(2, 5, SplitDirection::Up)?;
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "axis",
    //             "orientation": "horizontal",
    //             "members": [
    //                 {"type": "pane", "paneId": 1},
    //                 {"type": "pane", "paneId": 4},
    //                 {
    //                     "type": "axis",
    //                     "orientation": "vertical",
    //                     "members": [
    //                         {"type": "pane", "paneId": 3},
    //                         {"type": "pane", "paneId": 5},
    //                         {"type": "pane", "paneId": 2},
    //                     ]
    //                 },
    //             ]
    //         })
    //     );

    //     assert_eq!(true, group.remove(5)?);
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "axis",
    //             "orientation": "horizontal",
    //             "members": [
    //                 {"type": "pane", "paneId": 1},
    //                 {"type": "pane", "paneId": 4},
    //                 {
    //                     "type": "axis",
    //                     "orientation": "vertical",
    //                     "members": [
    //                         {"type": "pane", "paneId": 3},
    //                         {"type": "pane", "paneId": 2},
    //                     ]
    //                 },
    //             ]
    //         })
    //     );

    //     assert_eq!(true, group.remove(4)?);
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "axis",
    //             "orientation": "horizontal",
    //             "members": [
    //                 {"type": "pane", "paneId": 1},
    //                 {
    //                     "type": "axis",
    //                     "orientation": "vertical",
    //                     "members": [
    //                         {"type": "pane", "paneId": 3},
    //                         {"type": "pane", "paneId": 2},
    //                     ]
    //                 },
    //             ]
    //         })
    //     );

    //     assert_eq!(true, group.remove(3)?);
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "axis",
    //             "orientation": "horizontal",
    //             "members": [
    //                 {"type": "pane", "paneId": 1},
    //                 {"type": "pane", "paneId": 2},
    //             ]
    //         })
    //     );

    //     assert_eq!(true, group.remove(2)?);
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "pane",
    //             "paneId": 1,
    //         })
    //     );

    //     assert_eq!(false, group.remove(1)?);
    //     assert_eq!(
    //         serde_json::to_value(&group)?,
    //         json!({
    //             "type": "pane",
    //             "paneId": 1,
    //         })
    //     );

    //     Ok(())
    // }
}
