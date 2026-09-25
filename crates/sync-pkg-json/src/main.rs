use std::fs;

fn main() {
    let mut cargo_doc: toml_edit::DocumentMut =
        fs::read_to_string("Cargo.toml").unwrap().parse().unwrap();

    if let Some(next) = std::env::args().nth(1) {
        if next != "--check" {
            cargo_doc["package"]["version"] = toml_edit::value(next);
            fs::write("Cargo.toml", cargo_doc.to_string()).unwrap();
        }
    }

    let version = cargo_doc["package"]["version"].as_str().unwrap();
    let mut pkg: serde_json::Value =
        serde_json::from_str(&fs::read_to_string("npm/package.json").unwrap()).unwrap();

    if std::env::args().any(|arg| arg == "--check") {
        if pkg["version"].as_str() != Some(version) {
            eprintln!("npm/package.json version does not match Cargo.toml");
            std::process::exit(1);
        }
    } else {
        pkg["version"] = serde_json::Value::String(version.to_string());
        let formatted = serde_json::to_string_pretty(&pkg).unwrap();
        fs::write("npm/package.json", format!("{formatted}\n")).unwrap();
    }
}
