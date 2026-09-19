{
  lib,
  craneLib,
  rustc,
  dprint,
  writableTmpDirAsHomeHook,
}:

let
  wasmTarget = "wasm32-unknown-unknown";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./src
      ./crates/schemagen
      ./Cargo.toml
      ./Cargo.lock
      ./LICENSE
      ./tests
    ];
  };

  commonArgs = {
    inherit src;
    strictDeps = true;

    nativeBuildInputs = [
      rustc.llvmPackages.bintools # rust-lld
    ];

    # Needed for avoiding "error: linker `rust-lld` not found".
    # Adding packages like binutils is not enough
    #
    # https://github.com/NixOS/nixpkgs/issues/70238
    CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
  };

  cargoArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      cargoExtraArgs = "--target ${wasmTarget} --package dprint-plugin-typstyle --package schemagen";
    }
  );
in
craneLib.buildPackage (
  commonArgs
  // rec {
    inherit cargoArtifacts;

    pname = "dprint-plugin-typstyle";
    version = with builtins; (fromTOML (readFile ./Cargo.toml)).package.version;

    cargoExtraArgs = "--target ${wasmTarget} --package dprint-plugin-typstyle --package schemagen";

    # Wasm targets cannot run standard cargo test without a runner
    doCheck = false;

    doNotPostBuildInstallCargoBinaries = true;

    installPhase = ''
      runHook preInstall

      mkdir -p "$out/lib" "$out/share"
      cp target/${wasmTarget}/release/dprint_plugin_typstyle.wasm "$out/lib/plugin.wasm"
      cp target/${wasmTarget}/release/build/schemagen-*/out/schema.json "$out/share/schema.json"

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
  }
)
