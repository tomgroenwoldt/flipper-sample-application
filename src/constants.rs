use flipperzero::{
    notification::{messages, sounds::C4, NotificationSequence},
    notification_sequence,
};

pub static X_AXIS_OFFSET: i32 = 4;
pub static Y_AXIS_OFFSET: i32 = 2;

pub static CELL_WIDTH: i32 = 10;
pub static CELL_HEIGHT: i32 = 10;

pub static X_CELL_COUNT: i32 = 12;
pub static Y_CELL_COUNT: i32 = 6;

pub static MANAGER_DESPAWN_SECONDS: u64 = 3;

pub const SINGLE_VIBRO: NotificationSequence =
    notification_sequence![messages::VIBRO_ON, messages::DELAY_50, messages::VIBRO_OFF,];
pub const BLINK_START_RED: NotificationSequence =
    notification_sequence![messages::BLINK_START_10, messages::BLINK_SET_COLOR_RED,];
pub const KILL_SOUND: NotificationSequence = notification_sequence![C4, messages::DELAY_50];
