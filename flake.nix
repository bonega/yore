{
  description = "Development environment for Yore - Rust OEM code page library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system; inherit overlays; };

        # Stable Rust toolchain for regular development.
        # The bare-metal `thumbv7em-none-eabi` target (no std) is included so
        # `no_std` builds can be verified in CI.
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "thumbv7em-none-eabi" ];
        };

        # Nightly toolchain with Miri, used to validate the bulk decoder's
        # hand-written `unsafe` (the `Utf8Writer`). `selectLatestNightlyWith`
        # picks the most recent nightly on which `miri` actually built, so CI
        # never trips over a nightly that shipped without the component.
        # `rust-src` is required for Miri to build its sysroot (incl. the
        # big-endian cross target).
        miriToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = [ "miri" "rust-src" ];
          });
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            cargo-edit
            critcmp
            nixpkgs-fmt
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "🦀 Yore development environment"
            echo "Rust: $(rustc --version)"
          '';
        };

        # `nix develop .#miri --command cargo miri test ...`
        devShells.miri = pkgs.mkShell {
          packages = [ miriToolchain ];

          RUST_SRC_PATH = "${miriToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";
        };
      }
    );
}
