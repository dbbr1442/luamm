use std::marker::PhantomData;

use mlua::{AnyUserData, FromLua, IntoLua, Lua, UserData, UserDataRef, UserDataRefMut, Value};

pub struct TypedUserdata<T> {
    inner: AnyUserData,
    _phantom: PhantomData<T>,
}

impl<T: 'static + UserData> TypedUserdata<T> {
    pub fn from_struct(lua: &Lua, data: T) -> mlua::Result<Self> {
        let data = lua.create_userdata(data)?;
        Ok(Self { inner: data, _phantom: PhantomData })
    }
}

impl<T: 'static> TypedUserdata<T> {
    pub fn from_any(userdata: AnyUserData) -> mlua::Result<Self> {
        if userdata.is::<T>() {
            Ok(Self { inner: userdata, _phantom: PhantomData})
        } else {
            Err(mlua::Error::UserDataTypeMismatch)
        }
    }

    pub fn get_ref(&self) -> mlua::Result<UserDataRef<T>> {
        self.inner.borrow::<T>()
    }

    pub fn get_ref_mut(&self) -> mlua::Result<UserDataRefMut<T>> {
        self.inner.borrow_mut::<T>()
    }

    pub fn clone_shallow(&self) -> Self {
        let new = Self::from_any(self.inner.clone()).unwrap();
        new
    }
}

pub trait ShallowClone {
    fn shallow_clone(&self, lua: &Lua) -> Self;
}

impl<T: 'static> ShallowClone for TypedUserdata<T> {
    fn shallow_clone(&self, _lua: &Lua) -> Self {
        let new = Self::from_any(self.inner.clone()).unwrap();
        new
    }
}

pub trait TryDeepClone where Self: Sized {
    type Error;
    fn try_deep_clone(&self, lua: &Lua) -> Result<Self, Self::Error>;
}

impl<T: 'static + Clone + UserData> TryDeepClone for TypedUserdata<T> {
    type Error = mlua::Error;
    fn try_deep_clone(&self, lua: &Lua) -> Result<Self, Self::Error> {
        let userdata_ref = self.get_ref()?; 
        let new = userdata_ref.clone();
        let new = Self::from_struct(lua, new)?;

        Ok(new)
    }
}



impl<T: 'static> FromLua for TypedUserdata<T> {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {    
        if let Some(val) = value.as_userdata() {
            if val.is::<T>() {
                let typed = Self::from_any(val.clone()).unwrap();
                return Ok(typed);
            } else {
                return Err(mlua::Error::UserDataTypeMismatch);
            }
        } else {
            return Err(mlua::Error::UserDataTypeMismatch);
        }
    }
}

impl<T> IntoLua for TypedUserdata<T> {
    fn into_lua(self, _lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(Value::UserData(self.inner))
    }
}

impl<T> IntoLua for &TypedUserdata<T> {
    fn into_lua(self, _lua: &Lua) -> mlua::Result<Value> {
        Ok(Value::UserData(self.inner.clone()))
    }
}

//impl<T: Clone> Clone for TypedUserdata<T> {
//    fn clone(&self) -> Self {
//        self.ge
//    }
//}
