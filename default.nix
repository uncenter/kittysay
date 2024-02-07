{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "kittysay";
  version = "0.2.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  doCheck = false;
}
