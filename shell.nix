with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "env";
  nativeBuildInputs = [ pkg-config  fontconfig ];
  buildInputs = [
    fontconfig
  ];
}
