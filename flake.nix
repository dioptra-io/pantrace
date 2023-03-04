{
  description = "Convert between traceroute formats";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, flake-utils, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.nightly."2022-12-16".default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
      in
      {
        packages = {
          pantrace = rustPlatform.buildRustPackage {
            pname = "pantrace";
            version = "0.5.1";
            src = self;
            doCheck = false;
            cargoLock = { lockFile = ./Cargo.lock; };
          };
        };
        defaultPackage = self.packages.${system}.pantrace;
      }
    );
}
