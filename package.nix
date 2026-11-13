{
  lib,
  rustPlatform,
  pkg-config,
  libgit2,
  git,
}:

rustPlatform.buildRustPackage {
  pname = "git-changed";
  version = "0.0.1";

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
    description = "Lists files modified since branching off";
    homepage = "https://github.com/function-artisans/git-changed";
    mainProgram = "git-changed";
  };
}
