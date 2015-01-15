// See LICENSE file for copyright and license details.

#![allow(unstable)] // TODO: remove this

#[cfg(target_os = "android")]
#[macro_use]
extern crate android_glue;

extern crate libc;
extern crate cgmath;
extern crate "zoc_gl" as gl;
extern crate serialize;
extern crate glutin;
extern crate image;

use visualizer::{Visualizer};

mod core;
mod visualizer; // TODO: reexport Visualizer

pub fn main() {
    let mut visualizer = Visualizer::new();
    while visualizer.is_running() {
        visualizer.tick();
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub fn rust_android_main(app: *mut()) {
    android_glue::android_main2(app, move|| main());
}

// vim: set tabstop=5 shiftwidth=4 softtabstop=4 expandtab:
