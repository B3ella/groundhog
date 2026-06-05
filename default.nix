{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage {
  pname = "groundhog";
  version = "0.1.3";

  src = pkgs.fetchFromGitHub {
    owner = "B3ella";
    repo = "groundhog";
    rev = "v0.1.3";
    hash = "sha256-06lTyg+rSBZL2loRsXBKYTIq7hmM7l7nTeQiZ4Ozuso=";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = {
    description = "Templater for daily notes";
    homepage = "https://github.com/B3ella/groundhog";
    license = pkgs.lib.licenses.unlicense;
    maintainers = [ ];
  };
}
