{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };

      toolchain = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [
          "rust-src"
        ];
      };

      nativeBuildInputs = with pkgs; [
        toolchain
        pkg-config
        libxkbcommon
      ];

      buildInputs = with pkgs; [
        libglvnd
        wayland
        libxkbcommon
        shaderc
      ];

      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      src = craneLib.cleanCargoSource ./.;

      commonArgs = {
        inherit src;
        strictDeps = true;

        inherit buildInputs nativeBuildInputs;

        outputHashes = {
          "git+https://git.snix.dev/snix/snix.git?rev=e33040a3e1a500e73dd8a4c2b9e793d7cb85384f#e33040a3e1a500e73dd8a4c2b9e793d7cb85384f" = "sha256-TpWEIhAgzGIupKARl+a3btrBaV9wQGYyxzN42Cnmu14=";
        };
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      individualCrateArgs =
        commonArgs
        // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml {inherit src;}) version;
          doCheck = false;
        };

      fileSetForCrate = crate:
        pkgs.lib.fileset.toSource {
          root = ./.;
          fileset = pkgs.lib.fileset.unions [
            ./Cargo.toml
            ./Cargo.lock
            (craneLib.fileset.commonCargoSources ./crates/wayland_app)
            (craneLib.fileset.commonCargoSources ./crates/core)
            (craneLib.fileset.commonCargoSources crate)
            ./crates/wayland_app/src/shader.wgsl
          ];
        };

      wayggle-bg = craneLib.buildPackage (
        individualCrateArgs
        // {
          pname = "wayggle-bg";
          cargoExtraArgs = "-p wayggle-bg-app";
          src = fileSetForCrate ./crates/wayland_app;
        }
      );
    in {
      packages.default = wayggle-bg;
      packages.cargoArtifacts = cargoArtifacts;
      packages.craneLib = src;

      devShells.default = pkgs.mkShell {
        packages =
          buildInputs
          ++ nativeBuildInputs
          ++ [
            pkgs.xc
            pkgs.cargo-hakari
          ];
      };

      formatter = pkgs.alejandra;
    });
}
