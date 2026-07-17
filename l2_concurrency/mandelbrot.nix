{
  lib,
  stdenv,
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
    linuxBuildInputs = [
      libxkbcommon
      wayland

      libx11
      libxcursor
      libxi
    ];
  in
  {
    pname = manifest.name;
    inherit (manifest) version;

    src = ./.;

    cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";

    nativeBuildInputs = lib.optionals stdenv.hostPlatform.isLinux [
      makeWrapper
      pkg-config
    ];

    buildInputs = lib.optionals stdenv.hostPlatform.isLinux linuxBuildInputs;

    postInstall = lib.optionalString stdenv.hostPlatform.isLinux ''
      wrapProgram $out/bin/mandelbrot \
        --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath linuxBuildInputs}"
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
