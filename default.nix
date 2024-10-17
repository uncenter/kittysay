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
  cargoBuildFlags = "-p kittysay";

  meta = {
    description = "cowsay, but with a cute kitty :3";
    homepage = "https://github.com/uncenter/kittysay";
    license = lib.licenses.gpl3Only;
    maintainers = with lib.maintainers; [ uncenter ];
    mainProgram = "kittysay";
  };
}
