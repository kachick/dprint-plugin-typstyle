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
                yq-go
              ])
              ++ (with unstables; [
                dprint
                typst
                typstyle
                jsonschema-cli
                rustc
                cargo
                rustfmt
                rust-analyzer
                clippy
              ])
              ++ [ selfup.packages.${system}.default ];

            nativeBuildInputs = with unstables; [
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
