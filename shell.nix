{ pkgs ? import <nixpkgs> {} }:

with pkgs;

stdenv.mkDerivation {
  name = "rsget";
  buildInputs = [
    pkg-config
    openssl
  ];
}
