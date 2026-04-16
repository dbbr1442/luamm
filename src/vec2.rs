use std::ops::Deref;

use lli::{GetPrototype, ProtoTable};
use mlua::{Number, UserData, UserDataFields, UserDataMethods, UserDataRef};

use crate::MqFrom;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl UserData for Vec2 {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));

        fields.add_field_method_set("x", |_, this, val| { this.x = val; Ok(()) });
        fields.add_field_method_set("y", |_, this, val| { this.y = val; Ok(()) });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function_mut("__add", |_, args: (UserDataRef<Vec2>, UserDataRef<Vec2>)| {
            let mut vec = *args.0;
            vec.x += args.1.x;
            vec.y += args.1.y;
            Ok(vec)
        });

        methods.add_meta_function_mut("__mul", |_, args: (UserDataRef<Vec2>, Number)| {
            let num = args.1; 

            let mut vec = *args.0;
            vec.x *= num;
            vec.y *= num;
            Ok(vec)
        });
    }
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const UP: Self = Self { x: 0.0, y: -1.0 };
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };
    pub const RIGHT: Self = Self { x: -1.0, y: 0.0 };
    pub const LEFT: Self = Self { x: 1.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl std::ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl GetPrototype for Vec2 {
    fn prototype(proto: &mut ProtoTable) {
        proto.add_fn("new", |_, args: (f64, f64)| Ok(Self::new(args.0, args.1)));
        proto.add_val("UP", Self::UP);
        proto.add_val("DOWN", Self::DOWN);
        proto.add_val("LEFT", Self::LEFT);
        proto.add_val("RIGHT", Self::RIGHT);
        proto.add_val("ZERO", Self::ZERO);
    }
}

impl<T: Deref<Target = Vec2>> MqFrom<T> for Vec2 {
    fn mq_from(val: T) -> Self {
        Self::new(val.x, val.y)
    }
}
