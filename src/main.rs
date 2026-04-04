use std::{fmt::Display, io::Read};

use luamm::insert_library;
use macroquad::{conf::Conf, window::next_frame};
use mlua::prelude::*;

fn conf() -> Conf {
    Conf {
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open("./main.luau")
        .unwrap();

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
        process.call::<()>(()).disp_err();
        if luamm::PROGRAM_SHOULD_EXIT.load(std::sync::atomic::Ordering::Relaxed) {
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
