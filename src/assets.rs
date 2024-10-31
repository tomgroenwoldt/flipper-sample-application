use flipperzero_sys::Icon;

pub static mut FORKLIFT_ICON: Icon = Icon {
    width: 10,
    height: 10,
    frame_count: 1,
    frame_rate: 0,
    frames: unsafe { FORKLIFT_FRAMES.as_ptr() },
};
static mut FORKLIFT_FRAMES: [*const u8; 1] = [include_bytes!("icons/forklift.icon").as_ptr()];

pub static mut ALIVE_MANAGER_ICON: Icon = Icon {
    width: 10,
    height: 10,
    frame_count: 1,
    frame_rate: 0,
    frames: unsafe { ALIVE_MANAGER_FRAMES.as_ptr() },
};
static mut ALIVE_MANAGER_FRAMES: [*const u8; 1] =
    [include_bytes!("icons/alive_manager.icon").as_ptr()];

pub static mut DEAD_MANAGER_ICON: Icon = Icon {
    width: 10,
    height: 10,
    frame_count: 1,
    frame_rate: 0,
    frames: unsafe { DEAD_MANAGER_FRAMES.as_ptr() },
};
static mut DEAD_MANAGER_FRAMES: [*const u8; 1] =
    [include_bytes!("icons/dead_manager.icon").as_ptr()];
