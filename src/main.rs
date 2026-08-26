use std::{fmt::Display, io::Read};

use luamm::insert_library;
use macroquad::{conf::Conf, time::get_frame_time, window::next_frame};
use mlua::prelude::*;

fn conf() -> Conf {
    Conf {
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let file = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No file path provided. Using ./main.lua");
        "./main.lua".to_string()
    });

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(file);

    let mut file = match file {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    drop(file);

    let lua = Lua::new();
    insert_library(&lua);

    let chunk = lua.load(source);
    chunk.exec().unwrap_or_disp();

    match lua.globals().get::<LuaFunction>("start") {
        Err(_) => (),
        Ok(val) => val.call::<()>(()).disp_err(),
    }

    let process: LuaFunction = lua.globals().get("process").unwrap();
    loop {
        let time = get_frame_time();
        let fps = 1.0/time;
        let time = 60.0/fps;
        //println!("{}, {}", fps, time);

        let result = process.call::<LuaValue>(time).unwrap_or_disp();
        if !result.is_nil() {
            break;
        }

        next_frame().await
    }
}

trait ExtaUnwraps<T> {
    fn unwrap_or_disp(self) -> T;
    fn disp_err(&self);
}

impl<T, E: Display> ExtaUnwraps<T> for Result<T, E> {
    fn unwrap_or_disp(self) -> T {
        match self {
            Self::Ok(val) => val,
            Self::Err(e) => panic!("{}", e),
        }
    }

    fn disp_err(&self) {
        if let Err(e) = self {
            println!("{}", e);
        }
    }
}
