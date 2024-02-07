{
  callPackage,
  rust-analyzer,
  rustfmt,
  clippy,
}: let
  mainPkg = callPackage ./default.nix {version = "debug";};
in
  mainPkg.overrideAttrs (oa: {
    nativeBuildInputs =
      [
        rust-analyzer
        rustfmt
        clippy
      ]
      ++ (oa.nativeBuildInputs or []);
  })
