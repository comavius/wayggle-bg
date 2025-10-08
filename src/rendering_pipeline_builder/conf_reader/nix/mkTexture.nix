{...}: let
  nullResolution = {
    width = -1;
    height = -1;
  };
in
  {
    name,
    resolution ? nullResolution,
    src,
    format ? "auto",
  }: {
    texturePass = {
      path = src;
      inherit name resolution format;
    };
  }
