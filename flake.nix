{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      lib = nixpkgs.lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-tree);

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        rec {
          dprint-plugin-typstyle = pkgs.callPackage ./package.nix { };
          default = dprint-plugin-typstyle;
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              bashInteractive
              findutils # xargs
              diffutils # for E2E test
              nixfmt-rfc-style
              nil
              go-task
              typos
              yq-go

              dprint
              typst
              typstyle
              jsonschema-cli
              rustc
              cargo
              rustfmt
              rust-analyzer
              clippy
            ];

            nativeBuildInputs = with pkgs; [
              rustc-wasm32.llvmPackages.bintools # rust-lld
            ];

            # Needed for avoiding "error: linker `rust-lld` not found".
            # Adding packages like binutils is not enough
            #
            # https://github.com/NixOS/nixpkgs/issues/70238
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
          };
        }
      );
    };
}
