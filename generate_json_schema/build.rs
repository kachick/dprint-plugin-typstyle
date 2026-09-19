fn main() {
    let schema = dprint_plugin_typstyle::configuration::generate_json_schema();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("schema.json");
    std::fs::write(&dest_path, schema).unwrap();
    println!("cargo:rerun-if-changed=../src");
}
