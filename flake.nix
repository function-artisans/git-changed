{
  description = "Lists files modified since branching off";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      forEachSystem = nixpkgs.lib.genAttrs systems;
    in
    {
      overlays.default = final: _prev: {
        git-changed = final.callPackage ./package.nix { };
      };

      packages = forEachSystem (system: {
        git-changed = nixpkgs.legacyPackages.${system}.callPackage ./package.nix { };
        default = self.packages.${system}.git-changed;
      });

      apps = forEachSystem (system: rec {
        git-changed = {
          type = "app";
          program = nixpkgs.lib.getExe self.packages.${system}.git-changed;
          meta.description = "List the files that changed since the branch point";
        };
        default = git-changed;
      });

      checks = forEachSystem (system: {
        inherit (self.packages.${system}) git-changed;
      });
    };
}
