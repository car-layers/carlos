extern crate capnp;

#[allow(dead_code)]
mod call_capnp {
    use std::fmt;

    include!(concat!(env!("OUT_DIR"), "/call_capnp.rs"));

    impl fmt::Debug for presence::Role {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let role = match self {
                presence::Role::Vehicle => "Role::Vehicle",
                presence::Role::Driver => "Role::Driver",
            };
            write!(f, "{}", role)
        }
    }
}

pub mod presence {
    use crate::call_capnp::presence;
    use crate::capnp::{
        message::{Builder, HeapAllocator, Reader, ReaderOptions, TypedReader},
        serialize_packed, Result,
    };
    pub use presence::{Owned, Role};
    use std::io;

    pub fn who(role: Role, name: &str) -> TypedReader<Builder<HeapAllocator>, Owned> {
        let mut message = Builder::new_default();
        let mut p = message.init_root::<presence::Builder>();
        p.set_role(role);
        p.set_name(name);
        message.into()
    }

    pub fn pack(msg: Reader<Builder<HeapAllocator>>) -> io::Result<Vec<u8>> {
        let mut data = Vec::new();
        serialize_packed::write_message(&mut data, &msg.into_segments())?;
        Ok(data)
    }

    pub fn from(msg: Vec<u8>) -> Result<Role> {
        let mut data = io::Cursor::new(msg);
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let msg = reader.get_root::<presence::Reader>()?;
        Ok(msg.get_role()?)
    }
}
