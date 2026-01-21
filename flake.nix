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
            # How to use `inputsFrom`: https://github.com/NixOS/nixpkgs/issues/58624#issuecomment-1576860784
            inputsFrom = [ self.packages.${system}.dprint-plugin-typstyle ];

            buildInputs = with pkgs; [
              bashInteractive
              findutils # xargs
              diffutils # for E2E test
              nixfmt
              nixfmt-tree
              nixd
              go-task
              typos

              typst
              typstyle
              wasm-tools # How to use: https://github.com/NixOS/nixpkgs/pull/451399#pullrequestreview-3402766846

              # buildRustPackage does not enable these
              rust-analyzer
              clippy
            ];

            nativeBuildInputs = with pkgs; [
              rustc.llvmPackages.bintools # rust-lld
            ];

            env = {
              CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

              # Needed for avoiding "error: linker `rust-lld` not found".
              # Adding packages like binutils is not enough
              #
              # https://github.com/NixOS/nixpkgs/issues/70238
              CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";

              # Workaround for rust-analyzer error
              RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
            };
          };
        }
      );
    };
}
