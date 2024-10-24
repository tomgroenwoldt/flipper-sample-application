//! Demonstrates use of the Flipper Zero GUI.
//!
//! This app writes "Hello, Rust!" to the display.
//!
//! Currently uses unsafe `sys` bindings as there is no high level GUI API yet.

#![no_main]
#![no_std]

// Required for panic handler
extern crate alloc;
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::{c_void, CStr};
use core::mem::{self, MaybeUninit};
use core::ptr::addr_of;

use flipperzero_rt::{entry, manifest};
use flipperzero_sys::{
    self as sys, furi::UnsafeRecord, Icon, IconRotation, IconRotation_IconRotation0,
    IconRotation_IconRotation180, IconRotation_IconRotation270, IconRotation_IconRotation90,
};

static mut FORKLIFT_ICON: Icon = Icon {
    width: 10,
    height: 10,
    frame_count: 1,
    frame_rate: 0,
    frames: unsafe { FORKLIFT_FRAMES.as_ptr() },
};
static mut FORKLIFT_FRAMES: [*const u8; 1] = [include_bytes!("icons/forklift.icon").as_ptr()];

static mut DEAD_MANAGER_ICON: Icon = Icon {
    width: 10,
    height: 10,
    frame_count: 1,
    frame_rate: 0,
    frames: unsafe { DEAD_MANAGER_FRAMES.as_ptr() },
};
static mut DEAD_MANAGER_FRAMES: [*const u8; 1] =
    [include_bytes!("icons/dead_manager.icon").as_ptr()];

manifest!(name = "Rust GUI example");
entry!(main);

/// View draw handler.
pub unsafe extern "C" fn draw_callback(canvas: *mut sys::Canvas, context: *mut c_void) {
    let game_state = context as *mut GameState;
    unsafe {
        sys::canvas_draw_rframe(canvas, 0, 0, 128, 64, 0);

        for x in 0..12 {
            for y in 0..6 {
                sys::canvas_draw_box(canvas, x * 10 + 3, y * 10 + 1, 10, 10);
            }
        }

        for manager in (*game_state).managers.iter() {
            draw_manager(canvas, manager);
        }

        draw_forklift(canvas, context);
    }
}

extern "C" fn input_callback(input_event: *mut sys::InputEvent, ctx: *mut c_void) {
    unsafe {
        let event_queue = ctx as *mut sys::FuriMessageQueue;
        sys::furi_message_queue_put(event_queue, input_event as *mut c_void, 0);
    }
}

pub unsafe extern "C" fn draw_forklift(canvas: *mut sys::Canvas, context: *mut c_void) {
    let game_state = context as *mut GameState;
    unsafe {
        sys::canvas_draw_icon_ex(
            canvas,
            (*game_state).x * 10 + 3,
            (*game_state).y * 10 + 1,
            addr_of!(FORKLIFT_ICON) as *const Icon as *const c_void as *const sys::Icon,
            (&(*game_state).direction).into(),
        );
    }
}

pub unsafe extern "C" fn draw_manager(canvas: *mut sys::Canvas, manager: &Manager) {
    unsafe {
        sys::canvas_draw_icon(
            canvas,
            manager.x * 10 + 3,
            manager.y * 10 + 1,
            addr_of!(DEAD_MANAGER_ICON) as *const Icon as *const c_void as *const sys::Icon,
        );
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Implement rotation for direction
impl From<&Direction> for IconRotation {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => IconRotation_IconRotation0,
            Direction::Down => IconRotation_IconRotation180,
            Direction::Left => IconRotation_IconRotation270,
            Direction::Right => IconRotation_IconRotation90,
        }
    }
}

struct Manager {
    pub x: i32,
    pub y: i32,
    pub alive: bool,
}

struct GameState {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub managers: Vec<Manager>,
}

fn main(_args: Option<&CStr>) -> i32 {
    // TODO: Randomize game state.
    let mut managers = Vec::new();
    managers.push(Manager {
        x: 1,
        y: 1,
        alive: false,
    });
    let mut game_state = Box::new(GameState {
        x: 0,
        y: 0,
        direction: Direction::Right,
        managers,
    });
    unsafe {
        let view_port = sys::view_port_alloc();
        sys::view_port_draw_callback_set(
            view_port,
            Some(draw_callback),
            &*game_state as *const GameState as *mut c_void,
        );

        let event_queue = sys::furi_message_queue_alloc(8, mem::size_of::<sys::InputEvent>() as u32)
            as *mut sys::FuriMessageQueue;

        sys::view_port_input_callback_set(
            view_port,
            Some(input_callback),
            event_queue as *mut c_void,
        );

        let gui = UnsafeRecord::open(c"gui".as_ptr());
        sys::gui_add_view_port(gui.as_ptr(), view_port, sys::GuiLayer_GuiLayerFullscreen);

        let mut event: MaybeUninit<sys::InputEvent> = MaybeUninit::uninit();

        let mut running = true;
        while running {
            if sys::furi_message_queue_get(event_queue, event.as_mut_ptr() as *mut c_void, 100)
                == sys::FuriStatus_FuriStatusOk
            {
                let event = event.assume_init();
                if event.type_ == sys::InputType_InputTypePress
                    || event.type_ == sys::InputType_InputTypeRepeat
                {
                    match event.key {
                        sys::InputKey_InputKeyLeft => {
                            game_state.direction = Direction::Left;
                            if game_state.x > 0 {
                                game_state.x -= 1;
                            }
                        }
                        sys::InputKey_InputKeyRight => {
                            game_state.direction = Direction::Right;
                            if game_state.x < 11 {
                                game_state.x += 1;
                            }
                        }
                        sys::InputKey_InputKeyUp => {
                            game_state.direction = Direction::Up;
                            if game_state.y > 0 {
                                game_state.y -= 1;
                            }
                        }
                        sys::InputKey_InputKeyDown => {
                            game_state.direction = Direction::Down;
                            if game_state.y < 5 {
                                game_state.y += 1;
                            }
                        }
                        _ => running = false,
                    }
                }
            }
            sys::view_port_update(view_port);
        }

        sys::view_port_enabled_set(view_port, false);
        sys::gui_remove_view_port(gui.as_ptr(), view_port);
        sys::view_port_free(view_port);
        sys::furi_message_queue_free(event_queue);
    }

    0
}
