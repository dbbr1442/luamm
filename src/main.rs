use std::{f64, io::Read, time::Duration};

use lli::Library;
use macroquad::{color::Color, conf::Conf, window::{clear_background, next_frame}};
use mlua::prelude::*;

type LRes = mlua::Result<()>;

fn sleep(_lua: &Lua, secs: f64) -> LRes {
    std::thread::sleep(Duration::from_secs_f64(secs));
    Ok(())
}



fn clear_screen(_lua: &Lua, color: LuaVector) -> LRes {
    let (r, g, b) = (color.x(), color.y(), color.z());
    let (r, g, b) = (r*255.0, g*255.0, b*255.0);
    let color = Color::new(r, g, b, 1.0);
    clear_background(color);
    Ok(())
}

fn insert_library(lua: &Lua) {
    let lib = Library::new(lua);
    lib.register_function("sleep", sleep);
    lib.register_function("clear_screen", clear_screen);
    lib.inject("@luamm");
} 

fn conf() -> Conf {
    Conf {
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open("./main.lua")
        .unwrap();

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    drop(file);

    let lua = Lua::new();

    insert_library(&lua);

    let chunk = lua.load(source);
    chunk.exec().unwrap();

    let process: LuaFunction = lua.globals().get("process").unwrap();
    let start: LuaFunction = lua.globals().get("start").unwrap();

    start.call::<()>(()).unwrap();

    loop {
        process.call::<()>(()).unwrap();

        next_frame().await
    }
}
