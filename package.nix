{
  pkgs,
  lib,
  rustPlatform,
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
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = with pkgs; [
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

    mkdir -p $out/share
    cp target/${wasmTarget}/release/dprint_plugin_typstyle.wasm $out/share/plugin.wasm
    cp deployment/schema.json $out/share/

    runHook postInstall
  '';

  meta = {
    description = "Typst formatter plugin for dprint";
    homepage = "https://github.com/kachick/dprint-plugin-typstyle";
    license = lib.licenses.asl20;
  };
})
