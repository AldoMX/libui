//! Rust bindings to `libui`.
//!
//! Main C source repository: https://github.com/andlabs/libui
//!
//! Copyright © 2016 Mozilla Foundation

#[macro_use]
extern crate bitflags;
extern crate libc;

pub use controls::{Area, BoxControl, Button, Checkbox, ColorButton, Combobox, Control};
pub use controls::{DateTimePicker, Entry, FontButton, Group, Label, MultilineEntry, ProgressBar};
pub use controls::{RadioButtons, Separator, Slider, Spinbox, Tab};
pub use ffi_utils::Text;
pub use menus::{Menu, MenuItem};
pub use ui::{InitError, InitOptions, init, main, msg_box, msg_box_error, on_should_quit};
pub use ui::{open_file, queue_main, quit, save_file, uninit};
pub use windows::Window;

mod controls;
pub mod draw;
pub mod ffi;
pub mod ffi_utils;
mod menus;
mod ui;
mod windows;

pub mod platform {
    #[cfg(target_os = "macos")]
    pub mod macos;
    #[cfg(all(not(target_os = "macos"), target_family = "unix"))]
    pub mod unix;
    #[cfg(target_os = "windows")]
    pub mod windows;
}

