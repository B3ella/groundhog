{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage (finalAttrs: {
  pname = "groundhog";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "B3ella";
    repo = "groundhog";
    rev = "v0.1.0";
    hash = "sha256-Xrjpk8zLeLctDruuLzXVL9/m1eEp+fx98GiLuxnq6Qs=";
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

