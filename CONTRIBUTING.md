# Contributing to Yore

## Development Setup

### Using Nix (Recommended)

This project provides a Nix flake for a complete development environment.

#### Prerequisites

1. Install Nix with flakes enabled:
   ```bash
   sh <(curl -L https://nixos.org/nix/install) --daemon
   echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
   ```

2. (Optional) Install direnv for automatic environment activation:
   ```bash
   nix profile install nixpkgs#direnv nixpkgs#nix-direnv

   # Add to ~/.bashrc or ~/.zshrc
   eval "$(direnv hook bash)"  # or 'zsh'
   ```

#### Quick Start

**With direnv** (recommended):
```bash
direnv allow  # Allow the environment
# Environment loads automatically when entering directory
cargo build
cargo test
```

**Without direnv**:
```bash
nix develop  # Enter development shell
cargo build
cargo test
```

#### IDE Setup

For VSCode, install the [direnv extension](https://marketplace.visualstudio.com/items?itemName=mkhl.direnv) to load the Nix environment (including rust-analyzer).

### Traditional Setup

If you prefer not to use Nix, ensure you have:
- Rust 1.71.0 or later (stable)
- Rust nightly (for fuzzing)

```bash
cargo build
cargo test
```

## Benchmarking

Run benchmarks with Criterion:

```bash
cargo bench
```

To compare benchmarks before and after a change:

```bash
# Save baseline
cargo bench -- --save-baseline before

# Make changes, then compare
cargo bench -- --save-baseline after
critcmp before after
```

## Fuzzing

Fuzzing requires nightly Rust. The `fuzz/` directory has its own Nix flake:

```bash
cd fuzz
nix develop        # Enter nightly Rust environment
cargo fuzz list    # List available fuzz targets
cargo fuzz run validate
```

Without Nix:

```bash
cd fuzz
rustup run nightly cargo fuzz run validate
```
