{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage {
  pname = "groundhog";
  version = "0.1.2";

  src = pkgs.fetchFromGitHub {
    owner = "B3ella";
    repo = "groundhog";
    rev = "v0.1.2";
    hash = "sha256-by7Dp5taF4nvypzdmrax7y3mo2FL3qjrDzxEPLSb0Sg=";
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
