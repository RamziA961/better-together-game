use core::panic;
use fs_extra::dir::CopyOptions;
use std::fs;

const PROTO_PATH_SRC: &str = "../proto";
const PROTO_PATH_DST: &str = ".";

fn main() {
    println!("Compiling Proto Definitions");
    println!("OUT_DIR {}", std::env::var("OUT_DIR").unwrap());

    if let Err(_) = fs::read_dir(PROTO_PATH_DST) {
        fs::create_dir_all(PROTO_PATH_DST).unwrap();
    }

    fs_extra::dir::copy(
        PROTO_PATH_SRC,
        PROTO_PATH_DST,
        &CopyOptions {
            overwrite: true,
            ..Default::default()
        },
    )
    .unwrap();

    tonic_build::compile_protos(format!("{PROTO_PATH_SRC}/updates.proto")).unwrap_or_else(|e| {
        eprintln!("Failed to compile protos:\n{e:?}");
        panic!("Failed to compile protos:\n{e:?}")
    });
}
