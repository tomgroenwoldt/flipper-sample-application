use core::ffi::c_void;

use flipperzero_sys::{furi_message_queue_put, FuriMessageQueue, InputEvent};

pub unsafe extern "C" fn input_callback(input_event: *mut InputEvent, ctx: *mut c_void) {
    let event_queue = ctx as *mut FuriMessageQueue;
    furi_message_queue_put(event_queue, input_event as *mut c_void, 0);
}
