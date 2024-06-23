{
  description = "commit-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        packageName = "commit-rs";
        packageVersion = "0.1.0";

        rustVersion = "1.79.0";
        rustToolchain = pkgs.rust-bin.stable.${rustVersion}.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname = packageName;
          version = packageVersion;
          buildInputs = [pkgs.openssl pkgs.libgit2 pkgs.darwin.apple_sdk.frameworks.Security];
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };

      in {
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustToolchain.override { extensions = [ "rust-src" ]; }) ];
        };
      });
}
