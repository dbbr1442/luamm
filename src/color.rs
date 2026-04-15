use std::ops::Deref;

use lli::{GetPrototype, ProtoTable};
use mlua::{UserData, UserDataFields};

use crate::{MqColor, MqFrom};

#[derive(PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
    const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    const RED: Self = Self { r: 255, g: 0, b: 0, a: 255 };
    const GREEN: Self = Self { r: 0, g: 255, b: 0, a: 255 };
    const BLUE: Self = Self { r: 0, g: 0, b: 255, a: 255 };

    fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl UserData for Color {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, this| Ok(this.r));
        fields.add_field_method_get("g", |_, this| Ok(this.g));
        fields.add_field_method_get("b", |_, this| Ok(this.b));

        fields.add_field_method_set("r", |_, this, val| { this.r = val; Ok(()) });
        fields.add_field_method_set("g", |_, this, val| { this.g = val; Ok(()) });
        fields.add_field_method_set("b", |_, this, val| { this.b = val; Ok(()) });
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl GetPrototype for Color {
    fn prototype(proto: &mut ProtoTable) {
        proto.add_val("WHITE", Self::WHITE);
        proto.add_val("BLACK", Self::BLACK);
        proto.add_val("RED", Self::RED);
        proto.add_val("GREEN", Self::GREEN);
        proto.add_val("BLUE", Self::BLUE);

        proto.add_fn("new_rgb", |_, args: (u8, u8, u8)| Ok(Self::new_rgb(args.0, args.1, args.2)));
    }
}

impl<T: Deref<Target = Color>> MqFrom<T> for MqColor {
    fn mq_from(val: T) -> Self {
        Self::from_rgba(val.r, val.g, val.b, val.a)
    }
}
