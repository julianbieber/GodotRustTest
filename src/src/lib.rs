mod globals;
mod player;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<globals::Globals>();
    handle.add_class::<player::Player>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
