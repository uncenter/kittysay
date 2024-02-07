{
  pkgs ? import <nixpkgs> {},
  lib ? pkgs.lib,
  ...
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "kittysay";
  version = "0.3.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  doCheck = false;

  meta = with lib; {
    description = "The cutest successor of cowsay";
    homepage = "https://github.com/uncenter/kittysay";
    license = licenses.mit;
    maintainers = [maintainers.uncenter];
  };
}
