{
  lib,
  rustPlatform,
  rustc,
  dprint,
  writableTmpDirAsHomeHook,
}:

let
  wasmTarget = "wasm32-unknown-unknown";
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "dprint-plugin-typstyle";
  version = with builtins; (fromTOML (readFile ./Cargo.toml)).package.version;

  __structuredAttrs = true;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./src
      ./generate_json_schema
      ./Cargo.toml
      ./Cargo.lock
      ./LICENSE
      ./tests
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    rustc.llvmPackages.bintools # rust-lld
  ];

  cargoBuildFlags = [
    "--target"
    wasmTarget
    "--package"
    "dprint-plugin-typstyle"
    "--package"
    "generate_json_schema"
  ];

  installPhase = ''
    runHook preInstall

    mkdir -p "$out/lib" "$out/share"
    cp target/${wasmTarget}/release/dprint_plugin_typstyle.wasm "$out/lib/plugin.wasm"
    cp target/${wasmTarget}/release/build/generate_json_schema-*/out/schema.json "$out/share/schema.json"

    runHook postInstall
  '';

  doInstallCheck = true;

  nativeInstallCheckInputs = [
    dprint
    writableTmpDirAsHomeHook
  ];

  installCheckPhase = ''
    runHook preInstallCheck
    cd "$(mktemp --directory)"
    dprint check --allow-no-files --config-discovery=false --plugins "$out/lib/plugin.wasm"
    runHook postInstallCheck
  '';

  meta = {
    description = "Dprint Wasm plugin for Typst";
    homepage = "https://github.com/kachick/dprint-plugin-typstyle";
    license = lib.licenses.asl20;
  };
})
