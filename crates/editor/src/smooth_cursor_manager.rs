use collections::HashMap;
use gpui::Pixels;

const DELTA_PERCENT_PER_FRAME: f32 = 0.01;

pub struct Cursor {
    current_position: gpui::Point<Pixels>,
    target_position: gpui::Point<Pixels>,
}

pub enum SmoothCursorManager {
    Inactive,
    Active { cursors: HashMap<usize, Cursor> },
}

impl SmoothCursorManager {
    pub fn update(
        &mut self,
        source_positions: HashMap<usize, Option<gpui::Point<Pixels>>>,
        target_positions: HashMap<usize, Option<gpui::Point<Pixels>>>,
    ) {
        if source_positions.len() == 1 && target_positions.len() == 1 {
            let old_id = source_positions.keys().next().unwrap();
            let new_id = target_positions.keys().next().unwrap();
            if old_id != new_id {
                if let (Some(Some(old_pos)), Some(Some(new_pos))) = (
                    source_positions.values().next(),
                    target_positions.values().next(),
                ) {
                    *self = Self::Active {
                        cursors: HashMap::from_iter([(
                            *new_id,
                            Cursor {
                                current_position: *old_pos,
                                target_position: *new_pos,
                            },
                        )]),
                    };
                    return;
                }
            }
        }
        match self {
            Self::Inactive => {
                let mut cursors = HashMap::default();
                for (id, target_position) in target_positions.iter() {
                    let Some(target_position) = target_position else {
                        continue;
                    };
                    let Some(Some(source_position)) = source_positions.get(id) else {
                        continue;
                    };
                    if source_position == target_position {
                        continue;
                    }
                    cursors.insert(
                        *id,
                        Cursor {
                            current_position: *source_position,
                            target_position: *target_position,
                        },
                    );
                }
                if !cursors.is_empty() {
                    *self = Self::Active { cursors };
                }
            }
            Self::Active { cursors } => {
                for (id, target_position) in target_positions.iter() {
                    let Some(target_position) = target_position else {
                        continue;
                    };
                    if let Some(cursor) = cursors.get_mut(id) {
                        cursor.target_position = *target_position;
                    }
                }
            }
        }
    }

    pub fn animate(&mut self) -> HashMap<usize, gpui::Point<Pixels>> {
        match self {
            Self::Inactive => HashMap::default(),
            Self::Active { cursors } => {
                let mut new_positions = HashMap::default();
                let mut completed = Vec::new();

                for (id, cursor) in cursors.iter_mut() {
                    let dx = cursor.target_position.x - cursor.current_position.x;
                    let dy = cursor.target_position.y - cursor.current_position.y;

                    let distance = (dx.0.powi(2) + dy.0.powi(2)).sqrt();
                    if distance < 0.2 {
                        new_positions.insert(*id, cursor.target_position);
                        completed.push(*id);
                    } else {
                        cursor.current_position.x =
                            Pixels(cursor.current_position.x.0 + dx.0 * DELTA_PERCENT_PER_FRAME);
                        cursor.current_position.y =
                            Pixels(cursor.current_position.y.0 + dy.0 * DELTA_PERCENT_PER_FRAME);
                        new_positions.insert(*id, cursor.current_position);
                    }
                }

                for id in completed {
                    cursors.remove(&id);
                }

                if cursors.is_empty() {
                    *self = Self::Inactive;
                }

                new_positions
            }
        }
    }
}
