{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    nix-github-actions = {
      url = "github:nix-community/nix-github-actions";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    let
      inherit (inputs.nixpkgs) lib;
      inherit (inputs) self;
    in
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      systems = lib.systems.flakeExposed;

      flake.githubActions = inputs.nix-github-actions.lib.mkGithubMatrix {
        checks = lib.getAttrs [ "x86_64-linux" ] self.checks;
      };

      perSystem = { self', config, pkgs, ... }: {
        packages.default = self'.packages.rsget;

        rust-project.crates.rsget.crane.args = {
          buildInputs = with pkgs; [
            pkg-config
            openssl
          ];
        };


        devShells.default = pkgs.mkShell {
          inputsFrom = [
            self'.packages.rsget
          ];

          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
          ];
        };
      };
    };
}
