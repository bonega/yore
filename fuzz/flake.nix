{
  description = "Fuzzing environment for Yore (requires nightly Rust)";

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

        # Nightly Rust toolchain (required for fuzzing with sanitizers)
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            cargo-fuzz
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "🔍 Yore fuzzing environment (nightly)"
            echo "Rust: $(rustc --version)"
            echo ""
            echo "  cargo fuzz list            - List fuzz targets"
            echo "  cargo fuzz run <target>    - Run fuzzer"
          '';
        };
      }
    );
}
