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
use flipperzero::furi::time::{Duration, Instant};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys::{
    furi::UnsafeRecord, furi_message_queue_alloc, furi_message_queue_free, furi_message_queue_get,
    gui_add_view_port, gui_remove_view_port, view_port_alloc, view_port_draw_callback_set,
    view_port_enabled_set, view_port_free, view_port_input_callback_set, view_port_update,
    FuriStatus_FuriStatusOk, GuiLayer_GuiLayerFullscreen, InputEvent, InputKey_InputKeyDown,
    InputKey_InputKeyLeft, InputKey_InputKeyRight, InputKey_InputKeyUp, InputType_InputTypePress,
    InputType_InputTypeRepeat,
};

use draw::draw_callback;
use input::input_callback;
use schema::{Direction, Forklift, GameState, Manager, Movement, Position};

mod assets;
mod constants;
mod draw;
mod input;
mod schema;

manifest!(name = "Forklifts vs. Managers");
entry!(main);

fn main(_args: Option<&CStr>) -> i32 {
    let mut managers = Vec::new();
    for i in 0..12 {
        for j in 0..6 {
            let position = Position { x: i, y: j };
            managers.push(Manager {
                position,
                direction: Direction::Right,
                time_of_death: None,
            });
        }
    }
    let mut game_state = Box::new(GameState {
        forklift: Forklift::default(),
        managers,
    });
    unsafe {
        let view_port = view_port_alloc();
        view_port_draw_callback_set(
            view_port,
            Some(draw_callback),
            &*game_state as *const GameState as *mut c_void,
        );

        let event_queue = furi_message_queue_alloc(8, mem::size_of::<InputEvent>() as u32);

        view_port_input_callback_set(view_port, Some(input_callback), event_queue as *mut c_void);

        let gui = UnsafeRecord::open(c"gui".as_ptr());
        gui_add_view_port(gui.as_ptr(), view_port, GuiLayer_GuiLayerFullscreen);

        let mut event: MaybeUninit<InputEvent> = MaybeUninit::uninit();

        let mut running = true;
        let mut manager_move_tick = Instant::now();
        while running {
            if manager_move_tick.elapsed() > Duration::from_millis(500) {
                manager_move_tick = Instant::now();
                for manager in game_state.managers.iter_mut() {
                    if let Some(_time_of_death) = manager.time_of_death {
                    } else {
                        manager.hunt(game_state.forklift.position);
                    }
                    if manager.position == game_state.forklift.position {
                        manager.time_of_death = Some(Instant::now());
                    }
                }
            }
            if furi_message_queue_get(event_queue, event.as_mut_ptr() as *mut c_void, 100)
                == FuriStatus_FuriStatusOk
            {
                let event = event.assume_init();
                if event.type_ == InputType_InputTypePress
                    || event.type_ == InputType_InputTypeRepeat
                {
                    #[allow(non_upper_case_globals)]
                    match event.key {
                        InputKey_InputKeyLeft => {
                            game_state.forklift.step(Direction::Left);
                        }
                        InputKey_InputKeyRight => {
                            game_state.forklift.step(Direction::Right);
                        }
                        InputKey_InputKeyUp => {
                            game_state.forklift.step(Direction::Up);
                        }
                        InputKey_InputKeyDown => {
                            game_state.forklift.step(Direction::Down);
                        }
                        _ => running = false,
                    }
                    for manager in game_state
                        .managers
                        .iter_mut()
                        .filter(|manager| manager.time_of_death.is_none())
                    {
                        if manager.position == game_state.forklift.position {
                            manager.time_of_death = Some(Instant::now());
                        }
                    }
                }
            }
            view_port_update(view_port);
        }

        view_port_enabled_set(view_port, false);
        gui_remove_view_port(gui.as_ptr(), view_port);
        view_port_free(view_port);
        furi_message_queue_free(event_queue);
    }
    0
}
