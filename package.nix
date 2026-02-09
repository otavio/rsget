{ pkgs, inputs, flake, ... }:

let
  craneLib = inputs.crane.mkLib pkgs;
  src = craneLib.cleanCargoSource flake;
  commonArgs = {
    inherit src;
    buildInputs = [ pkgs.openssl ];
    nativeBuildInputs = [ pkgs.pkg-config ];
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (commonArgs // {
  inherit cargoArtifacts;
})
