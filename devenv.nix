{ pkgs, ... }:

{
  packages = [
    pkgs.cargo-outdated
    pkgs.pkg-config
    pkgs.openssl
    pkgs.zlib
  ];

  languages.rust = {
    enable = true;
    channel = "nixpkgs";
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy = {
      enable = true;
      settings.denyWarnings = true;
    };
    editorconfig-checker.enable = true;
    statix.enable = true;
    deadnix.enable = true;
    nixfmt.enable = true;
    check-toml.enable = true;
    check-yaml.enable = true;
    check-merge-conflicts.enable = true;
    typos.enable = true;
    ripsecrets.enable = true;
    shellcheck.enable = true;
    shfmt.enable = true;
  };

  enterTest = ''
    cargo test
  '';
}
