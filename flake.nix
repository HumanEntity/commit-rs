{
  description = "A very basic flake";

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
        rustVersion = pkgs.rust-bin.stable."1.79.0".default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname =
            "commit-rs";
          version = "0.1.0";
          buildInputs = [pkgs.openssl pkgs.libgit2 pkgs.darwin.apple_sdk.frameworks.Security];
          src = ./.; # the folder with the cargo.toml

          cargoLock.lockFile = ./Cargo.lock;
        };

      in {
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
        };
      });
}
