{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      mkGroundhogPackage = pkgs:
        pkgs.rustPlatform.buildRustPackage {
          pname = "marmota";
          version = "2.0.0";

          cargoLock.lockFile = ./Cargo.lock;
          src = self;

          meta = with pkgs.lib; {
            description = "Templater for daily notes";
            homepage = "https://github.com/B3ella/marmota";
            license = licenses.mit;
            maintainers = with lib.maintainers; [ b3ella ];
            mainProgram = "marmota";
          };
        };

      nixosModule = { config, lib, pkgs, ... }:
        let
          cfg = config.services.marmota;
          marmotaPkg = self.packages.${pkgs.system}.default;
        in {
          options.services.marmota = {
            enable = lib.mkEnableOption "Groundhog daily note generator";

            user = lib.mkOption {
              type = lib.types.str;
              description = "User that runs Groundhog.";
            };
          };

          config = lib.mkIf cfg.enable {
            systemd.timers.marmota = {
              wantedBy = [ "timers.target" ];

              timerConfig = {
                OnCalendar = "daily";
                Persistent = true;
              };
            };

            systemd.services.marmota = {
              description = "Generate daily notes with Groundhog";

              serviceConfig = {
                Type = "oneshot";
                User = cfg.user;
                        ExecStart = "${marmotaPkg}/bin/marmota";
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
