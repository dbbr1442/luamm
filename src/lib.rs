use std::{ops::Deref, sync::atomic::AtomicBool, time::Duration};

use lli::Library;
use macroquad::{input::{KeyCode, get_keys_down}, miniquad::window::screen_size, shapes::draw_rectangle, text::{draw_text, get_text_center}, window::clear_background};
use mlua::prelude::*;

use crate::{key::MqKey, rect::{Edge, Rect}};

use crate::{color::Color, key::Key, vec2::Vec2};

//mod uuid;
mod vec2;
mod color;
mod key;
mod rect;
mod userdata;

type MqColor = macroquad::color::Color;
type LRes = mlua::Result<()>;

pub trait MqFrom<T> {
    fn mq_from(val: T) -> Self;
}

pub trait IntoMq<T> {
    fn into_mq(self) -> T;
}

impl<T, I> IntoMq<I> for T 
where 
    I: MqFrom<T>
{
    fn into_mq(self) -> I {
        I::mq_from(self)
    }
}

pub trait TryMqFrom<T, E = ()> 
where 
    Self: Sized
{
    fn try_mq_from(val: T) -> Result<Self, E>;
}

pub trait TryIntoMq<T, E = ()> {
    fn try_into_mq(self) -> Result<T, E>;
}

impl<T, E, I> TryIntoMq<I, E> for T
where 
    I: TryMqFrom<T, E>,
{
    fn try_into_mq(self) -> Result<I, E> {
        I::try_mq_from(self)
    }
}


fn sleep(_lua: &Lua, secs: f64) -> LRes {
    std::thread::sleep(Duration::from_secs_f64(secs));
    Ok(())
}

fn clear_screen(_lua: &Lua, color: LuaUserDataRef<Color>) -> LRes {
    let color = MqColor::from_rgba(color.r, color.g, color.b, color.a);
    clear_background(color);
    Ok(())
}


fn rect(_lua: &Lua, args: (LuaUserDataRef<Vec2>, LuaUserDataRef<Vec2>, LuaUserDataRef<Color>)) -> LRes {
    let color = args.2;
    let color = MqColor::from_rgba(color.r, color.g, color.b, color.a);
    let position = args.0;
    let size = args.1;
    draw_rectangle(position.x as f32, position.y as f32, size.x as f32, size.y as f32, color);
    Ok(())
}

fn is_key_down(_lua: &Lua, key: LuaUserDataRef<Key>) -> LuaResult<bool> {
    let mq_key: KeyCode = key.deref().into_mq();
    for key in get_keys_down() {
        if mq_key == key {
            return Ok(true);
        }
    }

    Ok(false)
}

fn get_wasd_as_vec(_lua: &Lua, _args: ()) -> LuaResult<Vec2> {
    let mut vec = Vec2::ZERO;
    
    for key in get_keys_down() {
        match key {
            MqKey::W => vec += Vec2::UP,
            MqKey::A => vec += Vec2::LEFT,
            MqKey::S => vec += Vec2::DOWN,
            MqKey::D => vec += Vec2::RIGHT,
            _ => (),
        };
    }

    Ok(vec)
}

fn get_arrow_as_vec(_lua: &Lua, _args: ()) -> LuaResult<Vec2> {
    let mut vec = Vec2::ZERO;
    
    for key in get_keys_down() {
        match key {
            MqKey::Up => vec += Vec2::UP,
            MqKey::Down => vec += Vec2::DOWN,
            MqKey::Left => vec += Vec2::LEFT,
            MqKey::Right => vec += Vec2::RIGHT,
            _ => (),
        };
    }

    Ok(vec)
}

fn get_screen(_lua: &Lua, _args: ()) -> LuaResult<Vec2> {
    let vec = screen_size();
    Ok(Vec2 { x: vec.0 as f64, y: vec.1 as f64 })
}

fn get_keys_down_luamm(lua: &Lua, _args: ()) -> LuaResult<LuaTable> {
    let keys = lua.create_table()?;

    for key in get_keys_down() {
        if let Ok(key) = Key::try_from(&key) {
            let _ = keys.push(key);
        }
    }

    Ok(keys)
}

fn get_screen_as_rect(lua: &Lua, _args: ()) -> LuaResult<Rect> {
    let screen = screen_size();
    let rect = Rect::new(lua, &Vec2::new(0.0, 0.0), &Vec2::new(screen.0 as f64, screen.1 as f64))?;
    Ok(rect)
}

pub trait TryDraw {
    fn try_draw(&self, color: impl Deref<Target = Color>) -> LuaResult<()>;
}

impl TryDraw for LuaAnyUserData {
    fn try_draw(&self, color: impl Deref<Target = Color>) -> LRes {
        if let Ok(val) = self.borrow::<Rect>() {
            let mq_color = MqColor::from_rgba(color.r, color.g, color.b, color.a);
            let point = val.point.get_ref()?;
            let size = val.size.get_ref()?;
            draw_rectangle(point.x as f32, point.y as f32, size.x as f32, size.y as f32, mq_color);
        } else {
            return Err(LuaError::RuntimeError("Type is not drawable".to_string()));
        }

        Ok(())
    }
}

fn draw(_lua: &Lua, args: (LuaAnyUserData, LuaUserDataRef<Color>)) -> LRes {
    args.0.try_draw(args.1)?; 
    Ok(())
}

fn draw_text_internal(_lua: &Lua, args: (LuaUserDataRef<Vec2>, u16, String, LuaUserDataRef<Color>)) -> LRes {
    let loc = args.0;
    let size = args.1;
    let text = args.2;
    let color = args.3;

    draw_text(text.as_str(), loc.x as f32, loc.y as f32, size as f32, color.into_mq());
    Ok(())
}

fn draw_text_center(_lua: &Lua, args: (LuaUserDataRef<Vec2>, u16, String, LuaUserDataRef<Color>)) -> LRes {
    let mut loc = *args.0;
    let size = args.1;
    let text = args.2;
    let color = args.3;

    let center = get_text_center(text.as_str(), None, size, 1.0, 0.0);
    let center = Vec2::new(center.x as f64, center.y as f64);

    loc -= center;

    draw_text(text.as_str(), loc.x as f32, loc.y as f32, size as f32, color.into_mq());
    Ok(())
}

macro_rules! unwrap_all {
    ( $( $x:expr; )+ ) => {
        {
            $(
                $x.unwrap();
            )*
        }
    };
}

pub fn insert_library(lua: &Lua) {
    let math = lua.globals().get::<LuaTable>("math").unwrap();
    
    let clamp = lua.create_function(|_, args: (f64, f64, f64)| Ok(LuaNumber::clamp(args.0, args.1, args.2))).unwrap();
    math.set("clamp", clamp).unwrap();

    let lib = Library::new(lua).expect("Failed creating library table");

    unwrap_all! {
        lib.register_function("sleep", sleep);
        lib.register_function("clear_screen", clear_screen);
        lib.register_function("draw_rect", rect);
        lib.register_function("is_key_down", is_key_down);
        lib.register_function("get_wasd", get_wasd_as_vec);
        lib.register_function("get_screen", get_screen);
        lib.register_function("get_keys_down", get_keys_down_luamm);
        lib.register_function("draw", draw);
        lib.register_function("get_arrows", get_arrow_as_vec);
        lib.register_function("get_screen_rect", get_screen_as_rect);
        lib.register_function("draw_text", draw_text_internal);
        lib.register_function("draw_text_center", draw_text_center);

        lib.register_class::<Vec2>("Vec2");
        lib.register_class::<Key>("Key");
        lib.register_class::<Rect>("Rect");
        lib.register_class::<Edge>("Edge");
        lib.register_class::<Color>("Color"); 

        lib.inject_as_global("luamm");
    }
} 
