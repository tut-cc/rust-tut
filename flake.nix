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
              key: value: if value ? ${system} then { ${key} = value.${system}; } else { }
            ) self.outputs
          )
        );
    in
    {
      devShells = perSystem (
        { pkgs, ... }:
        {
          default =
            let
              guiLibs = with pkgs; [
                # Wayland
                libxkbcommon
                wayland

                # X11
                libx11
                libxcursor
                libxi
              ];
            in
            pkgs.mkShell {
              strictDeps = true;
              nativeBuildInputs = with pkgs; [
                rustc
                cargo
                rustfmt
                clippy
                rust-analyzer

                rust-script
              ];

              buildInputs = guiLibs;

              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath guiLibs;

              # Certain Rust tools won't work without this
              RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
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

          nl = pkgs.callPackage ./l1_basics/nl.nix { };

          mandelbrot = pkgs.callPackage ./l2_concurrency/mandelbrot.nix { };
        }
      );

      formatter = perSystem ({ pkgs, ... }: pkgs.nixfmt-tree);
    };
}
