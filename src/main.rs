use std::{f64, io::Read, time::Duration};

use lli::Library;
use macroquad::{conf::Conf, shapes::draw_rectangle, window::{clear_background, next_frame}};
use mlua::prelude::*;

type MqColor = macroquad::color::Color;

#[derive(FromLua, Clone)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl LuaUserData for Vec2 {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));

        fields.add_field_method_set("x", |_, this, val| { this.x = val; Ok(()) });
        fields.add_field_method_set("y", |_, this, val| { this.y = val; Ok(()) });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, (x, y)| Ok(Self { x, y }));
    }
}

#[derive(FromLua, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl LuaUserData for Color {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, this| Ok(this.r));
        fields.add_field_method_get("g", |_, this| Ok(this.g));
        fields.add_field_method_get("b", |_, this| Ok(this.b));

        fields.add_field_method_set("r", |_, this, val| { this.r = val; Ok(()) });
        fields.add_field_method_set("g", |_, this, val| { this.g = val; Ok(()) });
        fields.add_field_method_set("b", |_, this, val| { this.b = val; Ok(()) });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, (r, g, b)| Ok(Self { r, g, b, a: 255 }));
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl Into<MqColor> for &Color {
    fn into(self) -> MqColor {
        MqColor::from_rgba(self.r, self.g, self.b, self.a)
    }
}

impl<T: FromLua> FromLuaMulti for T {
    fn from_lua_multi(values: LuaMultiValue, lua: &Lua) -> LuaResult<Self> {
        
    }
}

type LRes = mlua::Result<()>;

fn sleep(_lua: &Lua, secs: f64) -> LRes {
    std::thread::sleep(Duration::from_secs_f64(secs));
    Ok(())
}

fn clear_screen(_lua: &Lua, color: &Color) -> LRes {
    let color = color.into();
    clear_background(color);
    Ok(())
}

fn draw_rect(_lua: &Lua, args: (&Vec2, &Vec2, &Color)) -> LRes {
    let color = args.2.into();
    let position = args.0;
    let size = args.1;
    draw_rectangle(position.x, position.y, size.x, size.y, color);
    Ok(())
}

fn insert_library(lua: &Lua) {
    let lib = Library::new(lua);
    lib.register_function("sleep", sleep);
    lib.register_function("clear_screen", clear_screen);
    lib.register_class::<Color>("Color"); 
    lib.register_function("draw_rect", draw_rect);
    lib.register_class::<Vec2>("Vec2");
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
