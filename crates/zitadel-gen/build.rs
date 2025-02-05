use std::{fs, path::PathBuf};
use std::collections::HashMap;
use std::fs::DirEntry;
use tonic_build;

#[derive(Default)]
struct Node {
    children: HashMap<String, Node>,
    rs_file: Option<String>,
}

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

    //step 1 build
    tonic_build::configure()
        .build_server(false) // skip server stubs
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .out_dir(&out_path)
        .compile_protos(&proto_files, &proto_include_dirs)?;

    // step 2: generate mod.rs
    generate_mod_rs(&out_path.to_string_lossy())?;

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

fn insert_path(root: &mut Node, path_parts: &[&str]) {
    // The final node is the one that holds .rs_file = Some(...)
    let (parents, leaf) = path_parts.split_at(path_parts.len().saturating_sub(1));
    // Descend into parents
    let mut node = root;
    for &segment in parents {
        node = node.children.entry(segment.to_owned()).or_default();
    }
    // The leaf node is where .rs_file actually lives
    let leaf_name = leaf.get(0).unwrap_or(&"");
    let child = node.children.entry(leaf_name.to_string()).or_default();
    child.rs_file = Some(path_parts.join(".") + ".rs");
}

fn build_feature_name(rs_file: &str) -> String {
    let without_rs = rs_file.strip_suffix(".rs").unwrap_or(rs_file);
    // e.g., "zitadel.admin.v1" → ["zitadel", "admin", "v1"]
    let parts: Vec<_> = without_rs.split('.').collect();
    parts.join("-")
}

fn generate_mod_code(node: &Node, indent: usize) -> Vec<String> {
    let mut lines = Vec::new();


    // Sort child nodes by name for stable output  
    for (segment, child_node) in node.children.iter().collect::<Vec<_>>() {
        if let Some(ref rs_file) = child_node.rs_file {
            // Leaf node → actual file  
            let feature_name = build_feature_name(rs_file);
            lines.push(format!("{:indent$}#[cfg(feature = \"{feature_name}\")]", "", indent = indent));
            lines.push(format!("{:indent$}pub mod {segment} {{", "", indent = indent));
            lines.push(format!("{:spaces$}include!(\"{rs_file}\");", "", spaces = indent + 4, rs_file = rs_file));
            lines.push(format!("{:indent$}}}", "", indent = indent));
        } else {
            // Intermediate node (directory-like)  
            lines.push(format!("{:indent$}pub mod {segment} {{", "", indent = indent));
            let sub_lines = generate_mod_code(child_node, indent + 4);
            lines.extend(sub_lines);
            lines.push(format!("{:indent$}}}", "", indent = indent));
        }
    }

    lines
}

fn generate_mod_rs(generated_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut root = Node::default();
    for entry in fs::read_dir(generated_dir)? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        if !path.is_file() {
            continue;
        }
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Looking for files like "zitadel.*.rs"
        if file_name.starts_with("zitadel.") && file_name.ends_with(".rs") {
            // e.g. "zitadel.admin.v1.rs" → ["zitadel","admin","v1"]
            let without_rs = &file_name[..file_name.len().saturating_sub(3)]; // drop ".rs"
            let parts: Vec<&str> = without_rs.split('.').collect();
            insert_path(&mut root, &parts);
        }
    }

    // 3) Build the final “mod.rs” content. We want everything under “pub mod zitadel { ... }”.
    let mut lines = Vec::new();
    lines.push("pub mod zitadel {".to_string());
    lines.extend(generate_mod_code(&root, 4));
    lines.push("}".to_string());

    let final_code = lines.join("\n") + "\n";

    // 4) Write the final code to “mod.rs” inside the same directory or wherever you prefer.
    let mod_file_path = format!("{}/mod.rs", generated_dir);
    fs::write(&mod_file_path, final_code)?;

    Ok(())
}