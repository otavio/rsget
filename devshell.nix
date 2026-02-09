{ pkgs, ... }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    cargo-audit
    rustc
    rustfmt
    clippy
    rust-analyzer
  ];
}
