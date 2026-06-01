{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage (finalAttrs: {
  pname = "groundhog";
  version = "0.1.2";

  src = pkgs.fetchFromGitHub {
    owner = "B3ella";
    repo = "groundhog";
    rev = "v0.1.2";
    hash = "sha256-2s2vdeI1jsgYSsVK9N6EQunwGg0q2TcpwGtPgyIKOuk=";
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

