{
  description = "Development environment with fontconfig";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ pkg-config fontconfig ];
          buildInputs = with pkgs; [ fontconfig ];
          
          shellHook = ''
            echo "Development environment with fontconfig is ready!"
          '';
        };
      }
    );
}
