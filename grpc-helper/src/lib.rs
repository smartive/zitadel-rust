extern crate proc_macro;

use proc_macro::{TokenStream};
use std::collections::HashMap;
use litrs::StringLit;

#[derive(Debug)]
struct Module {
    name: String,
    children: HashMap<String, Module>,
}

impl Module {
    fn to_code(&self, code: &mut String, parent: &str) {
        code.push_str(&format!("pub mod {} {{", &self.name));

        let next_parent = match parent {
            "root" => self.name.to_string(),
            _ => format!("{}.{}", parent, &self.name),
        };

        for (_, package) in &self.children {
            package.to_code(code, &next_parent);
        }

        if self.children.is_empty() {
            // this is a leaf
            code.push_str(&format!(r#"tonic::include_proto!("{}");"#, &next_parent));
        }

        code.push_str("}");
    }
}

#[proc_macro]
pub fn generate_grpc_modules(input: TokenStream) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();
    let mut packages = Vec::new();

    for token in &input {
        match StringLit::try_from(token) {
            Err(_) => continue,
            Ok(literal) => packages.push(literal.value().to_string()),
        }
    }

    let mut root_module = Module {
        name: "root".to_string(),
        children: HashMap::new(),
    };

    for package in packages {
        let mut current_module = &mut root_module;
        for group in package.split('.') {
            current_module = current_module.children.entry(group.to_string()).or_insert(Module {
                name: group.to_string(),
                children: HashMap::new(),
            });
        }
    }

    let mut result = String::new();
    for (_, module) in root_module.children {
        module.to_code(&mut result, "root");
    }

    result.parse().unwrap()
}
