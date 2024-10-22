{
  description = "rsget";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            openssl
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
          ];
        };
      });
}
