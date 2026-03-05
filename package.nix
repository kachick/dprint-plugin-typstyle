{
  lib,
  craneLib,
  rustc,
  dprint,
  writableTmpDirAsHomeHook,
  jsonschema-cli,
  yq-go,
  gnugrep,
}:

let
  wasmTarget = "wasm32-unknown-unknown";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./src
      ./generate_json_schema
      ./Cargo.toml
      ./Cargo.lock
      ./LICENSE
      ./scripts
      ./tests
    ];
  };

  commonArgs = {
    inherit src;
    strictDeps = true;

    nativeBuildInputs = [
      rustc.llvmPackages.bintools # rust-lld
      yq-go
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
      cargoExtraArgs = "--target ${wasmTarget}";
    }
  );
in
craneLib.buildPackage (
  commonArgs
  // rec {
    inherit cargoArtifacts;

    pname = "dprint-plugin-typstyle";
    version = with builtins; (fromTOML (readFile ./Cargo.toml)).package.version;

    doNotPostBuildInstallCargoBinaries = true;

    # Custom buildPhase to include schema generation
    buildPhase = ''
      runHook preBuild

      bash "$src/scripts/normalize_json_schema.bash" > schema.json
      cargo build --release --target=${wasmTarget}

      runHook postBuild
    '';

    # Custom installPhase
    installPhase = ''
      runHook preInstall

      mkdir -p "$out/lib" "$out/share"
      cp target/${wasmTarget}/release/dprint_plugin_typstyle.wasm "$out/lib/plugin.wasm"
      cp schema.json $out/share/

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

      SCHEMA_PATH="$out/share/schema.json" VERSION='${version}' bash "$src/scripts/test-jsonschema.bash"

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
