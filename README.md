# wayggle-bg
[![demonstration of wayggle-bg](http://img.youtube.com/vi/eBu3p4VQqkQ/0.jpg)](https://www.youtube.com/watch?v=eBu3p4VQqkQ)
## Installation
### via Nix
wayggle-bg is available through the Nix package manager and it's distributed as a Nix flake.
```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    wayggle-bg = {
      url = "github:comavius/wayggle-bg";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, wayggle-bg }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      packages.default = wayggle-bg.packages.${system}.default;
    };
}
```
If you have `nix` installed but do not intend to configure flake.nix, you can simply `nix run` or `nix build` the package:
```bash
nix run github:comavius/wayggle-bg # for running once
nix build github:comavius/wayggle-bg # for building the app
```

### via Cargo
WIP

## Tasks

[![xc compatible](https://xcfile.dev/badge.svg)](https://xcfile.dev)

### dev
If you don't have `nix` installed, prepare `pkg-config`, `libglvnd`, `wayland` and rust toolchain.
```bash
nix develop
```

### build:nix
```bash
nix build
```

### build:host
```bash
cargo build --release
```

### format:rust
```bash
cargo fmt
```

### format:nix
```bash
nix fmt *
```

### format:all
```bash
cargo fmt
nix fmt *
```
