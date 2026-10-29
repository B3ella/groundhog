{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        groundhog = pkgs.rustPlatform.buildRustPackage {
          pname = "groundhog";
          version = "0.1.3";

          src = pkgs.fetchFromGitHub {
            owner = "B3ella";
            repo = "groundhog";
            rev = "v0.1.3";
            hash = "sha256-06lTyg+rSBZL2loRsXBKYTIq7hmM7l7nTeQiZ4Ozuso=";
          };

          cargoLock.lockFile = ./Cargo.lock;

          meta = with pkgs.lib; {
            description = "Templater for daily notes";
            homepage = "https://github.com/B3ella/groundhog";
            license = licenses.unlicense;
            maintainers = [ ];
          };
        };
      in {
        packages.default = groundhog;

        apps.default = {
          type = "app";
          program = "${groundhog}/bin/groundhog";
        };
      });
}