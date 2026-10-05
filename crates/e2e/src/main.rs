use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --package e2e -- <check|bump> [test_name]");
        std::process::exit(1);
    }

    let action = &args[1];
    let test_name = args.get(2).map(|s| s.as_str());

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("Failed to find repo root")
        .to_path_buf();

    let plugin_path = std::env::var("PLUGIN_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            repo_root.join("target/wasm32-unknown-unknown/debug/dprint_plugin_typstyle.wasm")
        });

    if !plugin_path.exists() {
        eprintln!(
            "Plugin wasm not found at {}. Run `cargo x build` first.",
            plugin_path.display()
        );
        std::process::exit(1);
    }

    match action.as_str() {
        "check" => run_check(&repo_root, &plugin_path, test_name),
        "bump" => run_bump(&repo_root, &plugin_path, test_name),
        other => {
            eprintln!("Unknown action: {other}");
            std::process::exit(1);
        }
    }
}

fn get_test_dirs(repo_root: &Path, test_name: Option<&str>) -> Vec<PathBuf> {
    let tests_root = repo_root.join("tests");
    if let Some(name) = test_name {
        let dir = tests_root.join(name);
        if !dir.is_dir() {
            eprintln!("Test directory not found: {}", dir.display());
            std::process::exit(1);
        }
        vec![dir]
    } else {
        let mut dirs = Vec::new();
        for entry in fs::read_dir(&tests_root).expect("Failed to read tests directory") {
            let entry = entry.expect("Failed to read test entry");
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            }
        }
        dirs.sort();
        dirs
    }
}

fn find_raw_path(repo_root: &Path, dir: &Path) -> PathBuf {
    let local_raw = dir.join("raw.typ");
    if local_raw.is_file() {
        local_raw
    } else {
        repo_root.join("tests/raw.typ")
    }
}

fn run_check(repo_root: &Path, plugin_path: &Path, test_name: Option<&str>) {
    let test_dirs = get_test_dirs(repo_root, test_name);

    for dir in test_dirs {
        let expected_path = dir.join("expected.typ");
        if !expected_path.is_file() {
            eprintln!("expected.typ not found in {}", dir.display());
            std::process::exit(1);
        }

        // 1. `dprint check --plugins=<PLUGIN_PATH> expected.typ`
        let status = Command::new("dprint")
            .current_dir(&dir)
            .arg("check")
            .arg(format!("--plugins={}", plugin_path.display()))
            .arg("expected.typ")
            .status()
            .expect("Failed to run dprint check");

        if !status.success() {
            eprintln!("dprint check failed for expected.typ in {}", dir.display());
            std::process::exit(1);
        }

        // 2. Format raw file and compare with expected
        let raw_path = find_raw_path(repo_root, &dir);
        let raw_content = fs::read(&raw_path).expect("Failed to read raw file");
        let expected_content =
            fs::read_to_string(&expected_path).expect("Failed to read expected file");

        let mut child = Command::new("dprint")
            .current_dir(&dir)
            .arg("fmt")
            .arg("--stdin")
            .arg("raw.typ")
            .arg(format!("--plugins={}", plugin_path.display()))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn dprint fmt");

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&raw_content)
                .expect("Failed to write to stdin");
        }

        let output = child
            .wait_with_output()
            .expect("Failed to read dprint output");
        if !output.status.success() {
            eprintln!("dprint fmt failed for {}", raw_path.display());
            std::process::exit(1);
        }

        let actual_content = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
        if actual_content != expected_content {
            eprintln!(
                "Difference detected between formatted output and {}",
                expected_path.display()
            );
            print_diff(&expected_path, &actual_content);
            std::process::exit(1);
        }
    }
}

fn run_bump(repo_root: &Path, plugin_path: &Path, test_name: Option<&str>) {
    let test_dirs = get_test_dirs(repo_root, test_name);

    for dir in test_dirs {
        let raw_path = find_raw_path(repo_root, &dir);
        let expected_path = dir.join("expected.typ");

        let raw_content = fs::read(&raw_path).expect("Failed to read raw file");

        let mut child = Command::new("dprint")
            .current_dir(&dir)
            .arg("fmt")
            .arg("--stdin")
            .arg("raw.typ")
            .arg(format!("--plugins={}", plugin_path.display()))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn dprint fmt");

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&raw_content)
                .expect("Failed to write to stdin");
        }

        let output = child
            .wait_with_output()
            .expect("Failed to read dprint output");
        if !output.status.success() {
            eprintln!("dprint fmt failed for {}", raw_path.display());
            std::process::exit(1);
        }

        fs::write(&expected_path, output.stdout).expect("Failed to write expected file");
        println!("Updated {}", expected_path.display());
    }
}

fn print_diff(expected_path: &Path, actual_content: &str) {
    if let Ok(mut child) = Command::new("diff")
        .arg("-u")
        .arg(expected_path)
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(actual_content.as_bytes());
        }
        let _ = child.wait();
    } else {
        eprintln!("(diff command not available)");
    }
}
