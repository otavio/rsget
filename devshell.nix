{ pkgs, ... }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    cargo-audit
    cargo-release
    rustc
    rustfmt
    clippy
    rust-analyzer
  ];
}
