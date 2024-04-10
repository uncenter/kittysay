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
    description = "cowsay, but with a cute kitty :3";
    homepage = "https://github.com/uncenter/kittysay";
    license = licenses.mit;
    maintainers = [maintainers.uncenter];
  };
}
