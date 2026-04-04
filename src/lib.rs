use std::{ops::Deref, sync::atomic::AtomicBool, time::Duration};

use lli::Library;
use macroquad::{input::{get_keys_down, KeyCode}, miniquad::window::screen_size, shapes::draw_rectangle, window::clear_background};
use mlua::prelude::*;

type MqColor = macroquad::color::Color;

#[derive(Clone)]
struct Vec2 {
    pub x: f32,
    pub y: f32,
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

        methods.add_meta_function_mut("__add", |_, args: (LuaUserDataRef<Vec2>, LuaUserDataRef<Vec2>)| {
            let mut vec = args.0.clone();
            vec.x += args.1.x;
            vec.y += args.1.y;
            Ok(vec)
        });

        methods.add_meta_function_mut("__mul", |_, args: (LuaUserDataRef<Vec2>, LuaNumber)| {
            let num = args.1 as f32; 

            let mut vec = args.0.clone();
            vec.x *= num;
            vec.y *= num;
            Ok(vec)
        });
    }
}

struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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

type LRes = mlua::Result<()>;

fn sleep(_lua: &Lua, secs: f64) -> LRes {
    std::thread::sleep(Duration::from_secs_f64(secs));
    Ok(())
}

fn clear_screen(_lua: &Lua, color: LuaUserDataRef<Color>) -> LRes {
    let color = MqColor::from_rgba(color.r, color.g, color.b, color.a);
    clear_background(color);
    Ok(())
}

pub enum Key {
    W,
    A,
    S,
    D,

    Up,
    Left,
    Down,
    Right,

    Q,
    E,
    Esc,

    Enter,
}

impl Default for Key {
    fn default() -> Self {
        Self::W
    }
}

impl Key {
    fn as_string(&self) -> String {
        match self {
            Self::W => "w",
            Self::A => "a",
            Self::S => "s",
            Self::D => "d",
            Self::Up => "up",
            Self::Left => "left",
            Self::Right => "right",
            Self::Down => "down",
            Self::Q => "q",
            Self::E => "e",
            Self::Esc => "esc",
            Self::Enter => "enter",
        }.to_string()
    }

    fn from_string(string: &str) -> Option<Self> {
        let res = match string {
            "w"     => Self::W,
            "a"     => Self::A,
            "s"     => Self::S,
            "d"     => Self::D,
            "up"    => Self::Up,
            "left"  => Self::Left,
            "right" => Self::Right,
            "down"  => Self::Down,
            "q"     => Self::Q,
            "e"     => Self::E,
            "esc"   => Self::Esc,
            "enter" => Self::Enter,
            _ => return None,
        };

        Some(res)
    }
}


type MqKey = macroquad::input::KeyCode;

impl Into<MqKey> for &Key {
    fn into(self) -> MqKey {
        match self {
            Key::W     => MqKey::W, 
            Key::A     => MqKey::C, 
            Key::S     => MqKey::S, 
            Key::D     => MqKey::D, 
            Key::Up    => MqKey::Up, 
            Key::Left  => MqKey::Left, 
            Key::Right => MqKey::Right, 
            Key::Down  => MqKey::Down, 
            Key::Q     => MqKey::Q, 
            Key::E     => MqKey::E, 
            Key::Esc   => MqKey::Escape, 
            Key::Enter => MqKey::Enter, 
        }
    }
}

impl LuaUserData for Key {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("key", |_, this| Ok(this.as_string()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, key: String| {
            match Key::from_string(&key) {
                Some(val) => Ok(val),
                None => Err(LuaError::RuntimeError("Key did not match a known key".to_string())),
            }
        });
    }
}

fn rect(_lua: &Lua, args: (LuaUserDataRef<Vec2>, LuaUserDataRef<Vec2>, LuaUserDataRef<Color>)) -> LRes {
    let color = args.2;
    let color = MqColor::from_rgba(color.r, color.g, color.b, color.a);
    let position = args.0;
    let size = args.1;
    draw_rectangle(position.x, position.y, size.x, size.y, color);
    Ok(())
}

fn is_key_down(_lua: &Lua, key: LuaUserDataRef<Key>) -> LuaResult<bool> {
    let mq_key: KeyCode = key.deref().into();
    for key in get_keys_down() {
        if mq_key == key {
            return Ok(true);
        }
    }

    Ok(false)
}

fn get_wasd_as_vec(_lua: &Lua, _args: ()) -> LuaResult<Vec2> {
    let mut vec = Vec2 { x: 0.0, y: 0.0 };
    
    for key in get_keys_down() {
        match key {
            MqKey::W => vec.y -= 1.0,
            MqKey::A => vec.x -= 1.0,
            MqKey::S => vec.y += 1.0,
            MqKey::D => vec.x += 1.0,
            _ => (),
        };
    }

    Ok(vec)
}

fn get_screen(_lua: &Lua, _args: ()) -> LuaResult<Vec2> {
    let vec = screen_size();
    Ok(Vec2 { x: vec.0, y: vec.1 })
}

pub struct Signals {
    close: bool,
}

pub static PROGRAM_SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

fn close(_lua: &Lua, _args: ()) -> LRes {
    PROGRAM_SHOULD_EXIT.store(true, std::sync::atomic::Ordering::Release);
    Ok(()) 
} 

pub fn insert_library(lua: &Lua) {
    let lib = Library::new(lua);
    lib.register_function("sleep", sleep);
    lib.register_function("clear_screen", clear_screen);
    lib.register_class::<Color>("Color"); 
    lib.register_function("draw_rect", rect);
    lib.register_function("is_key_down", is_key_down);
    lib.register_function("get_wasd", get_wasd_as_vec);
    lib.register_function("get_screen", get_screen);
    lib.register_function("close", close);
    lib.register_class::<Vec2>("Vec2");
    lib.register_class::<Key>("Key");
    lib.inject("@luamm");
} 
