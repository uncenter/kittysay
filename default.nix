{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "kittysay";
  version = "0.3.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  doCheck = false;
}
