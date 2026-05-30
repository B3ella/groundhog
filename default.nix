{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage (finalAttrs: {
  pname = "groundhog";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "B3ella";
    repo = "groundhog";
    rev = "f7e899ebb75f174682cd2f4583424fbf1deab9e0";
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

