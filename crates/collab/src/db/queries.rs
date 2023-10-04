use super::*;

pub mod access_tokens;
pub mod buffers;
pub mod channels;
pub mod contacts;
pub mod messages;
pub mod projects;
pub mod rooms;
pub mod servers;
pub mod users;

fn max_assign<T: Ord>(max: &mut Option<T>, val: T) {
    if let Some(max_val) = max {
        if val > *max_val {
            *max = Some(val);
        }
    } else {
        *max = Some(val);
    }
}
