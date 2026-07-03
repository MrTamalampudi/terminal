use std::io::Result;

fn main() -> Result<()> {
    // Tells Cargo to recompile if the proto file changes
    println!("cargo:rerun-if-changed=proto/msg.proto");

    // Compile the protobuf file
    prost_build::compile_protos(&["proto/msg.proto"], &["proto/"])?;
    Ok(())
}
