{
  lib,
  rustPlatform,
  rustc-wasm32,
  dprint,
  writableTmpDirAsHomeHook,
  jsonschema-cli,
  yq-go,
  gnugrep,
}:

let
  wasmTarget = "wasm32-unknown-unknown";
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "dprint-plugin-typstyle";
  version = with builtins; (fromTOML (readFile ./Cargo.toml)).package.version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./src
      ./generate_json_schema
      ./Cargo.toml
      ./Cargo.lock
      ./LICENSE
      ./scripts
      ./deployment
      ./tests
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    rustc-wasm32.llvmPackages.bintools # rust-lld
    yq-go
  ];

  # https://gburghoorn.com/posts/just-nix-rust-wasm/
  buildPhase = ''
    runHook preBuild

    bash "$src/scripts/normalize_json_schema.bash"
    cargo build --release --target=${wasmTarget}

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/lib
    cp target/${wasmTarget}/release/dprint_plugin_typstyle.wasm $out/lib/plugin.wasm
    cp deployment/schema.json $out/lib/

    runHook postInstall
  '';

  doInstallCheck = true;

  nativeInstallCheckInputs = [
    dprint
    writableTmpDirAsHomeHook
    jsonschema-cli
    yq-go
    gnugrep
  ];

  installCheckPhase = ''
    runHook preInstallCheck

    export SCHEMA_PATH="$out/lib/schema.json"
    bash "$src/scripts/test-jsonschema.bash"
    grep --quiet --fixed-strings '${finalAttrs.version}' "$SCHEMA_PATH"

    cd "$(mktemp --directory)"
    dprint check --allow-no-files --plugins "$out/lib/plugin.wasm"

    runHook postInstallCheck
  '';

  meta = {
    description = "Dprint Wasm plugin for Typst";
    homepage = "https://github.com/kachick/dprint-plugin-typstyle";
    license = lib.licenses.asl20;
  };
})
