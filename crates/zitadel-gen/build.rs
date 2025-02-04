use std::{fs, path::PathBuf};
use tonic_build;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    //classic
    println!("cargo:rerun-if-changed=proto");


    // Put generated code in $OUT_DIR (a build-time env var):
    //let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| "api".to_string());
    let out_path = PathBuf::from("src/api");
    println!("OUT_DIR: {:?}", out_path);
    //create dir if not exists
    fs::create_dir_all(&out_path)?;
    // Collect .proto files :
    let mut proto_files = Vec::new();
    let proto_dir = PathBuf::from("proto/zitadel");
    find_proto_files(&proto_dir, &mut proto_files)?;
    // for import paths....:
    let proto_include_dirs = vec!["proto"];

    println!("proto_files: {:?}", proto_files);

    //build
    tonic_build::configure()
        .build_server(false) // skip server stubs
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .out_dir(&out_path)
        .compile_protos(&proto_files, &proto_include_dirs)?;

    Ok(())
}

fn find_proto_files(dir: &std::path::Path, acc: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            find_proto_files(&path, acc)?;
        } else if let Some(ext) = path.extension() {
            if ext == "proto" {
                acc.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}