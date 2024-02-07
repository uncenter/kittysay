{
  description = "kittysay";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    forAllSystems = nixpkgs.lib.genAttrs ["x86_64-linux" "x86_64-darwin" "i686-linux" "aarch64-linux" "aarch64-darwin"];
    pkgsForEach = nixpkgs.legacyPackages;
    version = self.shortRev or "dirty";
  in {
    packages = forAllSystems (system: rec {
      default = pkgsForEach.${system}.callPackage ./default.nix {inherit version;};
      kittysay = default;
    });

    devShells = forAllSystems (system: {
      default = pkgsForEach.${system}.callPackage ./shell.nix {};
    });
  };
}
