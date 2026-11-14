{
  lib,
  rustPlatform,
  pkg-config,
  libgit2,
  git,
}:

let
  cargoPackage = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = cargoPackage.name;
  inherit (cargoPackage) version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.lock
      ./Cargo.toml
      ./src
      ./tests
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  # Link the libgit2 of nixpkgs. The git2 crate then builds no copy of its own.
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ libgit2 ];
  env.LIBGIT2_NO_VENDOR = 1;

  # The test scenarios create their repositories with the git command.
  nativeCheckInputs = [ git ];

  meta = {
    inherit (cargoPackage) description;
    homepage = cargoPackage.repository;
    mainProgram = "git-changed";
  };
}
