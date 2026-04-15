use mlua::{UserData, UserDataRef};

pub struct UUID(u128);

impl UUID {
    pub fn new(num: u128) -> Self {
        UUID(num)
    }

    pub fn get(&self) -> u128 {
        self.0
    }

    fn get_string(&self) -> String {
        self.0.to_string()
    }
}

impl UserData for UUID {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function("__eq", |_, args: (UserDataRef<UUID>, UserDataRef<UUID>)| {
            Ok(args.0.0 == args.1.0)
        });
    }
}
