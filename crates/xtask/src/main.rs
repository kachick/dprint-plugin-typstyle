use std::env;
use std::process::{Command, ExitStatus};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let task = args.first().map(|s| s.as_str()).unwrap_or("default");
    let rest_args = if args.is_empty() { &[][..] } else { &args[1..] };

    if let Err(err) = dispatch(task, rest_args) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn dispatch(task: &str, args: &[String]) -> Result<(), String> {
    match task {
        "default" => {
            task_check()?;
            task_test(args)?;
        }
        "build" => task_build()?,
        "check" => task_check()?,
        "test" => task_test(args)?,
        "test-unit" => task_test_unit()?,
        "test-e2e" => {
            let filter = args.first().map(|s| s.as_str());
            task_test_e2e(filter)?;
        }
        "bump-fixtures" => {
            let filter = args.first().map(|s| s.as_str());
            task_bump_fixtures(filter)?;
        }
        "fmt" => task_fmt()?,
        "lint" => task_lint()?,
        "sync-package-json" => task_sync_package_json()?,
        "bump" => {
            let version = args.first().ok_or_else(|| {
                "bump requires a version argument, e.g. `cargo x bump 0.5.2`".to_string()
            })?;
            task_bump(version)?;
        }
        "schema" => task_schema()?,
        "selfup" => task_selfup(false)?,
        "--help" | "-h" | "help" => print_help(),
        other => {
            print_help();
            return Err(format!("unknown task: `{other}`"));
        }
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"Usage: cargo x <task> [args...]

Available tasks:
    default             Run check and test
    build               Build wasm target (cargo build --target wasm32-unknown-unknown)
    check               Run test and lint
    test [test_name]    Run unit tests and e2e tests
    test-unit           Run cargo test --workspace
    test-e2e [name]     Run e2e tests (builds wasm first unless SKIP_BUILD=true)
    bump-fixtures [name] Update e2e test fixtures
    fmt                 Format Rust code and other files (cargo fmt + dprint fmt)
    lint                Run clippy, fmt --check, dprint, typos, zizmor, selfup
    sync-package-json   Sync Cargo version into npm/package.json
    bump <version>      Bump version, update Cargo.lock, and create git commit
    schema              Generate schema.json
    selfup              Run selfup across git-tracked files
"#
    );
}

fn should_skip_build() -> bool {
    env::var("SKIP_BUILD").map(|v| v == "true").unwrap_or(false)
}

fn run_cmd(program: &str, args: &[&str]) -> Result<(), String> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    let status = cmd
        .status()
        .map_err(|e| format!("failed to spawn `{program}`: {e}"))?;
    check_status(program, status)
}

fn check_status(program: &str, status: ExitStatus) -> Result<(), String> {
    if status.success() {
        Ok(())
    } else {
        Err(format!("`{program}` failed with exit status: {status}"))
    }
}

fn task_build() -> Result<(), String> {
    if should_skip_build() {
        println!("SKIP_BUILD=true: skipping build");
        return Ok(());
    }
    run_cmd(
        "cargo",
        &["build", "--locked", "--target", "wasm32-unknown-unknown"],
    )
}

fn task_test_unit() -> Result<(), String> {
    if should_skip_build() {
        println!("SKIP_BUILD=true: skipping unit tests");
        return Ok(());
    }
    run_cmd("cargo", &["test", "--locked", "--workspace"])
}

fn task_test_e2e(filter: Option<&str>) -> Result<(), String> {
    task_build()?;
    let mut args = vec!["run", "--locked", "--package", "e2e", "--", "check"];
    if let Some(f) = filter {
        args.push(f);
    }
    run_cmd("cargo", &args)
}

fn task_bump_fixtures(filter: Option<&str>) -> Result<(), String> {
    task_build()?;
    let mut args = vec!["run", "--locked", "--package", "e2e", "--", "bump"];
    if let Some(f) = filter {
        args.push(f);
    }
    run_cmd("cargo", &args)
}

fn task_test(args: &[String]) -> Result<(), String> {
    task_test_unit()?;
    let filter = args.first().map(|s| s.as_str());
    task_test_e2e(filter)
}

fn task_fmt() -> Result<(), String> {
    run_cmd("cargo", &["fmt"])?;
    run_cmd("dprint", &["fmt"])
}

fn task_lint() -> Result<(), String> {
    run_cmd("cargo", &["clippy", "--locked", "--", "--deny", "warnings"])?;
    run_cmd("cargo", &["fmt", "--check"])?;
    run_cmd(
        "cargo",
        &[
            "run",
            "--locked",
            "--package",
            "sync-pkg-json",
            "--",
            "--check",
        ],
    )?;
    run_cmd("dprint", &["check"])?;
    run_cmd("typos", &[".", ".github", ".vscode"])?;
    run_cmd("zizmor", &["."])?;
    task_selfup(true)
}

fn task_check() -> Result<(), String> {
    task_test(&[])?;
    task_lint()
}

fn task_sync_package_json() -> Result<(), String> {
    run_cmd("cargo", &["run", "--locked", "--package", "sync-pkg-json"])
}

fn task_bump(version: &str) -> Result<(), String> {
    run_cmd(
        "cargo",
        &[
            "run",
            "--locked",
            "--package",
            "sync-pkg-json",
            "--",
            version,
        ],
    )?;
    // Do not pass --locked here because this step intentionally updates Cargo.lock with the new version
    run_cmd("cargo", &["check", "--workspace"])?;
    run_cmd(
        "git",
        &["add", "Cargo.toml", "npm/package.json", "Cargo.lock"],
    )?;
    let commit_msg = format!("Bump version to {version}");
    run_cmd("git", &["commit", "-m", &commit_msg])
}

fn task_schema() -> Result<(), String> {
    run_cmd("cargo", &["run", "--locked", "--package", "schemagen"])
}

fn task_selfup(check: bool) -> Result<(), String> {
    let output = Command::new("git")
        .args(["ls-files"])
        .output()
        .map_err(|e| format!("failed to run git ls-files: {e}"))?;

    if !output.status.success() {
        return Err("git ls-files failed".to_string());
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("invalid utf8 from git ls-files: {e}"))?;
    let files: Vec<&str> = stdout.lines().collect();

    let mut args = vec!["run", "github:kachick/selfup/v1.3.1", "--"];
    if check {
        args.extend(&["list", "-check"]);
    } else {
        args.push("run");
    }
    args.extend(files);

    run_cmd("nix", &args)
}
