use std::{env, path::PathBuf};

fn main () -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/time_service.proto")?;
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("time_service_descriptor.bin"))
        .compile(&["../proto/time_service.proto"], &["../proto"])
        .unwrap();
    Ok(())
  }