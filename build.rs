extern crate capnpc;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/call.capnp")
        .file("schema/world.capnp")
        .run()
        .expect("compiling schema");
    // Temp fix for capnproto paths
    fix_capnp_paths("call_capnp.rs").unwrap();
    fix_capnp_paths("world_capnp.rs").unwrap();
}

fn fix_capnp_paths(file: &str) -> io::Result<()> {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join(file);
    let file_contents = fs::read_to_string(&dst)
        .unwrap()
        .replace("::call_capnp::presence", "self");
    fs::write(dst, file_contents)
}
