{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage {
  pname = "groundhog";
  version = "0.1.2";

  src = pkgs.fetchFromGitHub {
    owner = "B3ella";
    repo = "groundhog";
    rev = "v0.1.2";
    hash = "sha256-6VdLoMs4eZP+xwxGjdqoYzf5ktl38FFY0yDKrz4Cd84=";
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
