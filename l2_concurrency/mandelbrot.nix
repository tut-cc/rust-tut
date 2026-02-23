{
  lib,
  libxkbcommon,
  libx11,
  libxcursor,
  libxi,
  makeWrapper,
  pkg-config,
  rustPlatform,
  wayland,
}:

rustPlatform.buildRustPackage (
  finalAttrs:
  let
    manifest = (lib.importTOML "${finalAttrs.src}/Cargo.toml").package;
  in
  {
    pname = manifest.name;
    inherit (manifest) version;

    src = ./.;

    cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";

    nativeBuildInputs = [
      makeWrapper
      pkg-config
    ];
    buildInputs = [
      libxkbcommon
      wayland

      libx11
      libxcursor
      libxi
    ];
    postInstall = ''
      wrapProgram $out/bin/mandelbrot \
        --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath finalAttrs.buildInputs}"
    '';

    meta = {
      inherit (manifest) description;
      homepage = "https://github.com/tut-cc/rust-tut/tree/main/l2_concurrency";
      license = lib.licenses.gpl2Only;
      platforms = with lib.platforms; linux ++ darwin;
      # maintainers = [  ];
      mainProgram = "mandelbrot";
    };
  }
)
