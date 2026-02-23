{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    # https://github.com/nix-systems/nix-systems
    systems.url = "github:nix-systems/default";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
    }:
    let
      perSystem =
        cb:
        nixpkgs.lib.genAttrs (import systems) (
          system:
          cb (
            {
              inherit system;
              pkgs = nixpkgs.legacyPackages.${system};
            }
            // nixpkgs.lib.attrsets.concatMapAttrs (
              name: value:
              if builtins.isAttrs value && value ? ${system} then { ${name} = value.${system}; } else { }
            ) self.outputs
          )
        );
    in
    {
      devShells = perSystem (
        { pkgs, ... }:
        {
          default = pkgs.mkShell {
            strictDeps = true;
            nativeBuildInputs = with pkgs; [
              rustc
              cargo
              rustfmt
              clippy
              rust-analyzer

              python3 # required for running l1_basics/generate_rust_analyzer.py
            ];

            # Certain Rust tools won't work without this
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

            shellHook = ''
              export PROJECT_ROOT=$(git rev-parse --show-toplevel)
              ${self}/l1_basics/generate_rust_analyzer.py "$(rustc --print sysroot)" "$RUST_SRC_PATH" "$PROJECT_ROOT/l1_basics" >"$PROJECT_ROOT/l1_basics/rust-project.json"
            '';
          };
        }
      );

      packages = perSystem (
        { pkgs, devShells, ... }:
        {
          # buildNixShellImage
          # nix build .#container && podman load <result
          # streamNixShellImage
          # nix run .#container | podman image load
          # podman -it localhost/rust-tut:latest
          container = pkgs.dockerTools.streamNixShellImage {
            name = "rust-tut";
            tag = "latest";
            drv = devShells.default;
          };
          # default = pkgs.callPackage ./bang-search.nix { };
        }
      );

      formatter = perSystem ({ pkgs, ... }: pkgs.nixfmt-tree);
    };
}
