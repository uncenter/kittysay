{
  pkgs ? import <nixpkgs> {},
  lib ? pkgs.lib,
  version ? "latest",
  ...
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "kittysay";
  inherit version;

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  doCheck = false;

  meta = with lib; {
    description = "The cutest successor of cowsay.";
    homepage = "https://github.com/uncenter/kittysay";
    license = licenses.mit;
    maintainers = [maintainers.uncenter];
  };
}
