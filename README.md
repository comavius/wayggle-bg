# wayggle-bg

If you want to try it out, you can run the following command:
```bash
nix run github:comavius/wayggle-bg -- default --name box
```

---

wayggle-bg is a wallpaper application for Wayland compositors that supports GLSL shaders, including ShaderToy and The Book of Shaders formats. It allows you to set animated wallpapers using custom shaders.

https://www.youtube.com/watch?v=eBu3p4VQqkQ

Demonstration video above.


## Example usage
### Default shader
```bash
wayggle-bg default --name box
```

### ShaderToy shader
In this mode, this application passes uniform variables in shader toy format, like `iTime`.
```bash
wayggle-bg shadertoy --fragment-shader <path-to-shadertoy-fragment.glsl>
```

### The Book of Shaders
In this mode, this application passes uniform variables in The Book of Shaders format, like `u_time`.
```bash
wayggle-bg book-of-shaders --fragment-shader <path-to-book-of-shaders-fragment.glsl>
# You can also specify a custom vertex shader
wayggle-bg book-of-shaders --fragment-shader <path-to-book-of-shaders-fragment.glsl> --vertex-shader <path-to-book-of-shaders-vertex.glsl>
```
### Cursor support
Hyprland users can enable cursor support by passing `--enable-cursor-support` flag.

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
