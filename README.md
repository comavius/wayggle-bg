# wayggle-bg

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
