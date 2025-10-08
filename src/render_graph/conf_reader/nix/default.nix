{defaultResolution}: let
  lib = import ../nix/lib.nix;
  defaultBuiltinNames = {
    time = "u_time";
    resolution = "u_resolution";
    mouse = "u_mouse";
    position = "a_position";
  };
  conf = {
    inherit defaultResolution defaultBuiltinNames lib;
  };
in {
  mkTransform = import ../nix/mkTransform.nix conf;
  mkTexture = import ../nix/mkTexture.nix conf;
  inherit defaultResolution;
}
