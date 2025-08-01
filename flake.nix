{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };

      toolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
        ];
      };

      rustPlatform = pkgs.makeRustPlatform {
        cargo = toolchain;
        rustc = toolchain;
      };

      app-version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

      nativeBuildInputs = with pkgs; [
        toolchain
        pkg-config
      ];

      buildInputs = with pkgs; [
        libglvnd
        wayland
      ];
    in {
      packages.default = rustPlatform.buildRustPackage {
        pname = "wayggle-bg";
        version = app-version;
        src = self;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        inherit nativeBuildInputs buildInputs;
      };

      devShells.default = pkgs.mkShell {
        packages =
          buildInputs
          ++ nativeBuildInputs
          ++ [
            pkgs.xc
          ];
      };

      formatter = pkgs.alejandra;
    });
}
