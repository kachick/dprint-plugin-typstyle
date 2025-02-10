{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    unstable-nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    selfup = {
      url = "github:kachick/selfup/v1.1.9";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      unstable-nixpkgs,
      selfup,
    }:
    let
      lib = nixpkgs.lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          unstables = unstable-nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            buildInputs =
              (with pkgs; [
                bashInteractive
                findutils # xargs
                diffutils # for E2E test
                nixfmt-rfc-style
                nil
                go-task

                typos

                rustc
                cargo
                rustfmt
                rust-analyzer
                clippy
              ])
              ++ (with unstables; [
                dprint
                typst
                typstyle
              ])
              ++ [ selfup.packages.${system}.default ];

            nativeBuildInputs = with pkgs; [
              rustc-wasm32.llvmPackages.bintools # rust-lld
            ];

            # Needed for avoiding "error: linker `rust-lld` not found".
            # Adding packages like binutils is not enough
            #
            # https://github.com/NixOS/nixpkgs/issues/70238
            env.CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
          };
        }
      );
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          # Don't use buildRustPackage for now. Using it for WASM build looks not easy way.
          # e.g https://discourse.nixos.org/t/building-a-rust-package-derivation-using-buildrustpackage-for-wasm32-unknown-unknown/59925/2
          default = pkgs.stdenv.mkDerivation rec {
            name = "dprint-plugin-typstyle";
            pname = "dprint-plugin-typstyle";
            src = pkgs.lib.cleanSource self;
            version = "0.2.7";

            nativeBuildInputs = with pkgs; [
              rustc
              rustc-wasm32.llvmPackages.bintools # rust-lld
              cargo
              rustPlatform.cargoSetupHook
            ];

            # Needed for avoiding "error: linker `rust-lld` not found".
            # Adding packages like binutils is not enough
            #
            # https://github.com/NixOS/nixpkgs/issues/70238
            env.CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";

            cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
              inherit src;
              name = "${pname}-${version}";
              hash = "sha256-KEaxkkmGtAH0ZZqCmyyDm7A6l7ar9HRmThFzNJ274Ko=";
            };

            buildPhase = ''
              runHook preBuild
              cargo build --release --target=wasm32-unknown-unknown
              runHook postBuild
            '';
          };
        }
      );
    };
}
