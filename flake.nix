{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      mkGroundhogPackage = pkgs:
        pkgs.rustPlatform.buildRustPackage {
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

      nixosModule = { config, lib, pkgs, ... }:
        let
          cfg = config.services.groundhog;
          groundhogPkg = self.packages.${pkgs.system}.default;
        in {
          options.services.groundhog = {
            enable = lib.mkEnableOption "Groundhog daily note generator";

            user = lib.mkOption {
              type = lib.types.str;
              description = "User that runs Groundhog.";
            };
          };

          config = lib.mkIf cfg.enable {
            systemd.timers.groundhog = {
              wantedBy = [ "timers.target" ];

              timerConfig = {
                OnCalendar = "daily";
                Persistent = true;
              };
            };

            systemd.services.groundhog = {
              description = "Generate daily notes with Groundhog";

              serviceConfig = {
                Type = "oneshot";
                User = cfg.user;
                ExecStart = "${groundhogPkg}/bin/groundhog";
              };
            };
          };
        };
    in
      (flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in {
          packages = {
            default = mkGroundhogPackage pkgs;
          };
        }))
      // {
        nixosModules.default = nixosModule;
      };
}