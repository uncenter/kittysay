{
  lib,
  rustPlatform,
  version ? "latest",
  ...
}:
rustPlatform.buildRustPackage {
  pname = "kittysay";
  inherit version;

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "cowsay, but with a cute kitty :3";
    homepage = "https://github.com/uncenter/kittysay";
    license = lib.licenses.gpl3;
    maintainers = with lib.maintainers; [uncenter];
    mainProgram = "kittysay";
  };
}
