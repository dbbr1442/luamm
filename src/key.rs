use std::ops::Deref;

use lli::GetPrototype;
use mlua::{UserData, UserDataMethods, UserDataRef};

use crate::MqFrom;
pub type MqKey = macroquad::input::KeyCode;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Key {
    #[default]
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

impl Key {
    pub fn as_string(&self) -> String {
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

    pub fn from_string(string: &str) -> Option<Self> {
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

impl<T: Deref<Target = Key>> MqFrom<T> for MqKey {
    fn mq_from(val: T) -> Self {
        match *val {
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

impl TryFrom<&MqKey> for Key {
    type Error = ();
    fn try_from(value: &MqKey) -> Result<Self, Self::Error> {
        let res = match value {
            MqKey::W      => Key::W,
            MqKey::A      => Key::A,
            MqKey::S      => Key::S,
            MqKey::D      => Key::D,
            MqKey::Up     => Key::Up,
            MqKey::Left   => Key::Left,
            MqKey::Right  => Key::Right,
            MqKey::Down   => Key::Down,
            MqKey::Q      => Key::Q,
            MqKey::E      => Key::E,
            MqKey::Escape => Key::Esc,
            MqKey::Enter  => Key::Enter,
            _ => return Err(()),
        };

        Ok(res)
    }
}

impl UserData for Key {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function("__eq", |_, args: (UserDataRef<Self>, UserDataRef<Self>)| {
            let val = *args.0 == *args.1;
            Ok(val)
        });
    }
}

impl GetPrototype for Key {
    fn prototype(proto: &mut lli::ProtoTable) {
        proto.add_val("W"     ,   Key::W);         
        proto.add_val("A"     ,   Key::A);
        proto.add_val("S"     ,   Key::S);
        proto.add_val("D"     ,   Key::D);
        proto.add_val("UP"    ,   Key::Up);
        proto.add_val("LEFT"  ,   Key::Left);
        proto.add_val("RIGHT" ,   Key::Right);
        proto.add_val("DOWN"  ,   Key::Down);
        proto.add_val("Q"     ,   Key::Q);
        proto.add_val("E"     ,   Key::E);
        proto.add_val("ESCAPE",   Key::Esc);
        proto.add_val("ENTER" ,   Key::Enter);     
    }
}
