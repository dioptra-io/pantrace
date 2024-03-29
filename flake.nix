{
  description = "Convert between traceroute formats";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages = {
          pantrace = pkgs.rustPlatform.buildRustPackage {
            pname = "pantrace";
            version = "0.6.2";
            src = self;
            doCheck = false;
            cargoLock = { lockFile = ./Cargo.lock; };
          };
        };
        defaultPackage = self.packages.${system}.pantrace;
      }
    );
}
