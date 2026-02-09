{ pkgs, ... }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer
  ];
}
