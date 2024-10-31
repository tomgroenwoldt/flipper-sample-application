use core::{ffi::c_void, ptr::addr_of};

use flipperzero_sys::{canvas_draw_icon, canvas_draw_rframe, Canvas};

use crate::{
    assets::{ALIVE_MANAGER_ICON, DEAD_MANAGER_ICON, FORKLIFT_ICON},
    constants::{
        CELL_HEIGHT, CELL_WIDTH, X_AXIS_OFFSET, X_CELL_COUNT, Y_AXIS_OFFSET, Y_CELL_COUNT,
    },
    Direction, GameState, Manager,
};

/// View draw handler.
pub unsafe extern "C" fn draw_callback(canvas: *mut Canvas, context: *mut c_void) {
    let game_state = context as *mut GameState;
    canvas_draw_rframe(
        canvas,
        X_AXIS_OFFSET - 1,
        Y_AXIS_OFFSET - 1,
        (X_CELL_COUNT * CELL_WIDTH + 2) as usize,
        (Y_CELL_COUNT * CELL_WIDTH + 2) as usize,
        0,
    );

    for manager in (*game_state).managers.iter() {
        draw_manager(canvas, manager);
    }

    draw_forklift(canvas, context);
}

unsafe fn draw_forklift(canvas: *mut flipperzero_sys::Canvas, context: *mut c_void) {
    let game_state = context as *mut GameState;

    let forklift = &(*game_state).forklift;
    let direction = &forklift.direction;
    let (mut x, y) = (
        forklift.position.x * CELL_WIDTH + X_AXIS_OFFSET,
        forklift.position.y * CELL_HEIGHT + Y_AXIS_OFFSET,
    );

    // If the forklift drives to the right, we have to shift the icon two
    // pixels to the left in order to be centered. This is probably a bug in
    // the `IconRotation`.
    if let Direction::Right = direction {
        x -= 2;
    }

    flipperzero_sys::canvas_draw_icon_ex(canvas, x, y, addr_of!(FORKLIFT_ICON), direction.into());
}

unsafe fn draw_manager(canvas: *mut flipperzero_sys::Canvas, manager: &Manager) {
    let icon = if manager.time_of_death.is_none() {
        addr_of!(ALIVE_MANAGER_ICON)
    } else {
        addr_of!(DEAD_MANAGER_ICON)
    };
    canvas_draw_icon(
        canvas,
        manager.position.x * 10 + 4,
        manager.position.y * 10 + 2,
        icon,
    );
}
