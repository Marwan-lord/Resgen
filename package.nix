{
  pkgs ? import <nixpkgs> { },
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "resgen";
  version = "0.2.1";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  nativeBuildInputs = with pkgs;[ fontconfig pkg-config ];
  buildInputs = with pkgs; [fontconfig pkg-config ];
}
