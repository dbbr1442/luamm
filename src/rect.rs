use std::ops::Deref;

use lli::GetPrototype;
use mlua::{Lua, Result, UserData, UserDataFields, UserDataMethods, UserDataRef};

use crate::{userdata::TypedUserdata, vec2::Vec2};

#[derive(PartialEq, Clone, Copy)]
pub enum Edge {
    Right,
    Top,
    Left,
    Bottom,
}

impl UserData for Edge {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| {
            Ok(match this {
                Edge::Top => "top".to_string(),
                Edge::Bottom => "bottom".to_string(),
                Edge::Left => "left".to_string(),
                Edge::Right => "right".to_string(),
            })
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function("__eq", |_, args: (UserDataRef<Self>, UserDataRef<Self>)| Ok(*args.0 == *args.1));
    }
}

impl GetPrototype for Edge {
    fn prototype(proto: &mut lli::ProtoTable) {
        proto.add_val("RIGHT", Self::Right);
        proto.add_val("TOP", Self::Top);
        proto.add_val("LEFT", Self::Left);
            //let ptr = &raw const this.point as *mut c_void;
            //Ok(Value::LightUserData(LightUserData(ptr)))
            //let user_data = lua.create_userdata(data);
        proto.add_val("BOTTOM", Self::Bottom);
    }
}

pub struct Rect {
    //pub point: Vec2,
    //pub size: Vec2,
    pub point: TypedUserdata<Vec2>,
    pub size: TypedUserdata<Vec2>,
}

impl Rect {
    pub fn new(lua: &Lua, point: impl Deref<Target = Vec2>, size: impl Deref<Target = Vec2>) -> Result<Self> {
        let point = TypedUserdata::from_struct(lua, *point)?;
        let size = TypedUserdata::from_struct(lua, *size)?;
        Ok(Self { point: point, size: size, })
    }

    fn get_edge(&self, edge: impl Deref<Target = Edge>) -> Result<f64> {
        let point = self.point.get_ref()?;
        let size = self.size.get_ref()?;
        Ok(match *edge {
            Edge::Bottom => point.y+size.y,
            Edge::Top => point.y,
            Edge::Left => point.x,
            Edge::Right => point.x+size.x,
        })
    }

    fn overlaps(&self, other: impl Deref<Target = Rect>) -> Result<bool> {
        let res = {
            self.get_edge(&Edge::Bottom)? < other.get_edge(&Edge::Top)? ||
            self.get_edge(&Edge::Top)? > other.get_edge(&Edge::Bottom)? ||
            self.get_edge(&Edge::Right)? < other.get_edge(&Edge::Left)? ||
            self.get_edge(&Edge::Left)? > other.get_edge(&Edge::Right)?
        };

        Ok(!res)
    }

    fn contains(&self, contained: impl Deref<Target = Rect>) -> Result<bool> {
        let res = self.get_edge(&Edge::Top)? <= contained.get_edge(&Edge::Top)? &&
        self.get_edge(&Edge::Bottom)? >= contained.get_edge(&Edge::Top)? &&
        self.get_edge(&Edge::Right)? <= contained.get_edge(&Edge::Right)? &&
        self.get_edge(&Edge::Left)? >= contained.get_edge(&Edge::Right)?;

        Ok(res)
    }
}

impl UserData for Rect {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_set("point", |_, this, val: UserDataRef<Vec2>| {
            let mut point = this.point.get_ref_mut()?;
            point.x = val.x;
            point.y = val.y;

            Ok(())
        });

        fields.add_field_method_set("size", |_, this, val: UserDataRef<Vec2>| {
            let mut size = this.size.get_ref_mut()?;

            size.x = val.x;
            size.y = val.y;

            Ok(())
        });

        fields.add_field_method_get("point", |_, this| {
            Ok(this.point.clone_shallow())
        });

        fields.add_field_method_get("size", |_, this| Ok(this.size.clone_shallow()));
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |lua, args: (UserDataRef<Vec2>, UserDataRef<Vec2>)| {
            let new = Self::new(lua, args.0, args.1)?;
            Ok(new)
        });

        methods.add_method("overlaps", |_, this, other: UserDataRef<Rect>| Ok(this.overlaps(other)));
        methods.add_method("get_edge", |_, this, edge: UserDataRef<Edge>| Ok(this.get_edge(edge)));
        methods.add_method("contains", |_, this, contained: UserDataRef<Rect>| Ok(this.contains(contained)));
    }
}

impl GetPrototype for Rect {
    fn prototype(proto: &mut lli::ProtoTable) {
        proto.add_fn("new", |lua, args: (UserDataRef<Vec2>, UserDataRef<Vec2>)| {
            let new = Self::new(lua, args.0, args.1)?;
            Ok(new)
        });
    }
}
