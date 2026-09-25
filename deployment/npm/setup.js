// @ts-check
const fs = require("fs");
const path = require("path");
const args = process.argv.slice(2);

function getWasmPath() {
  if (process.env.PLUGIN_PATH && fs.existsSync(process.env.PLUGIN_PATH)) {
    return process.env.PLUGIN_PATH;
  }
  const candidates = [
    path.join(__dirname, "../../dist/lib/plugin.wasm"),
    path.join(
      __dirname,
      "../../target/wasm32-unknown-unknown/release/dprint_plugin_typstyle.wasm",
    ),
    path.join(
      __dirname,
      "../../target/wasm32-unknown-unknown/debug/dprint_plugin_typstyle.wasm",
    ),
    path.join(__dirname, "../../result/lib/plugin.wasm"),
  ];
  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }
  throw new Error(
    `Could not find Wasm file. Looked in: ${candidates.join(", ")}`,
  );
}

const wasmPath = getWasmPath();
fs.copyFileSync(wasmPath, path.join(__dirname, "plugin.wasm"));

if (args.length > 0) {
  const packageJsonPath = path.join(__dirname, "package.json");
  const packageJsonText = fs.readFileSync(packageJsonPath, "utf8");
  const packageJson = JSON.parse(packageJsonText);
  if (args[0] === "sync-version") {
    const cargoTomlPath = path.join(__dirname, "../../Cargo.toml");
    const cargoTomlText = fs.readFileSync(cargoTomlPath, "utf8");
    const versionMatch = cargoTomlText.match(/^version\s*=\s*"([^"]+)"/m);
    if (!versionMatch) {
      throw new Error("Could not find version in Cargo.toml");
    }
    packageJson.version = versionMatch[1];
  } else {
    packageJson.version = args[0];
  }
  fs.writeFileSync(
    packageJsonPath,
    JSON.stringify(packageJson, undefined, 2) + "\n",
  );
}
