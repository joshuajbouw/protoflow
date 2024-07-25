fn main() {
    println!("cargo:rerun-if-changed=proto/");
    prost_build::compile_protos(&["proto/message.proto"], &["proto/"]).unwrap();
}
