{
  callPackage,
  rust-analyzer,
  rustfmt,
  clippy,
}: let
  mainPkg = callPackage ./default.nix {};
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
