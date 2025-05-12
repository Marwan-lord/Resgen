{
  description = "Resgen flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {

        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "resgen";
          version = "2.1";

          buildInputs = with pkgs; [
            fontconfig
            pkg-config
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

        
          src = pkgs.lib.cleanSource ./.;
          
        };
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config
              fontconfig
              eza
              fd
              rust-bin.beta.latest.default
            ];

            shellHook = ''
              alias ls=eza
              alias find=fd
            '';
          };
      }
    );
}
