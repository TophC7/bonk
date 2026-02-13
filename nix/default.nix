# NixOS module for bonk
#
# This module configures bonk via environment variables and ensures
# required dependencies are available system-wide.
#
# Usage in your NixOS config:
#
#   programs.bonk = {
#     enable = true;
#     flakePath = "/repo/Nix/dot.nix";
#     buildHost = null;  # null = local builds
#     extraArgs = [ "--impure" ];
#   };

bonkFlake:

{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.bonk;
in
{
  options.programs.bonk = {
    enable = lib.mkEnableOption "bonk CLI tool";

    flakePath = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      description = ''
        Default flake path for rebuild/update commands.
        If null, bonk will look for flake.nix in the current directory
        or use the $FLAKE environment variable.
      '';
      example = "/repo/Nix/dot.nix";
    };

    buildHost = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = ''
        Default remote build host.
        If null, builds are performed locally.
        Can be overridden with --build-host or --local flags.
      '';
      example = "buildserver";
    };

    extraArgs = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ "--impure" ];
      description = ''
        Extra arguments passed to nh/nix commands.
        These are appended after the -- separator.
      '';
      example = [
        "--impure"
        "--verbose"
      ];
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [
      bonkFlake.packages.${pkgs.system}.default
      # Use nh from bonk's flake, faster updates
      bonkFlake.inputs.nh.packages.${pkgs.system}.default
    ];

    environment.sessionVariables = lib.filterAttrs (_: v: v != null) {
      BONK_FLAKE_PATH = lib.mkIf (cfg.flakePath != null) (toString cfg.flakePath);
      BONK_BUILD_HOST = lib.mkIf (cfg.buildHost != null) cfg.buildHost;
      BONK_EXTRA_ARGS = lib.mkIf (cfg.extraArgs != [ ]) (lib.concatStringsSep ":" cfg.extraArgs);
    };
  };
}
