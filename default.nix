{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage (finalAttrs: {
  pname = "groundhog";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "B3ella";
    repo = "groundhog";
    rev = "2efae7734a1be4d22a3eb937c8f8b7d8514a1523";
    hash = "sha256-T6sVqO43ASk2rDB9Lhk4jzexU2TzukkdtB2hvqj33PA=";
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
})

