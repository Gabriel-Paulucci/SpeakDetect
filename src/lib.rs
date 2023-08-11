mod commands;
mod data;
mod discord;
mod event;
mod receiver;

use std::ffi::CStr;

use discord::startup_bot;

type Notify = unsafe extern "C" fn(user_id: u64, speak: bool);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn startup(token: *const i8, channel_id: u64, notify: Notify) -> bool {
    let token = unsafe { CStr::from_ptr(token) };

    let token = token.to_str();

    let token = if let Ok(token) = token {
        token
    } else {
        return false;
    };

    let _ = startup_bot(token, channel_id, notify);

    true
}
