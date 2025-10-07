{
  mkTexture,
  mkTransform,
  defaultResolution,
  ...
}: let
  texture1 = mkTexture {
    name = "texture1";
    resolution = {
      width = 512;
      height = 512;
    };
    src = ./dummy;
    format = "png";
  };
  texture2 = mkTexture {
    name = "texture2";
    resolution = {
      width = 512;
      height = 512;
    };
    src = ./dummy;
    format = "png";
  };
  transform1 = mkTransform {
    name = "transform1";
    resolution = {
      width = defaultResolution.width / 2;
      height = defaultResolution.height / 2;
    };
    vertexShaderSrc = ./dummy;
    fragmentShaderSrc = ./dummy;
    inputs = {
      texture1 = texture1;
    };
    outputs = ["output1"];
    frameRate = 30;
  };
  transform2 = mkTransform {
    name = "transform2";
    vertexShader = "for parsing test only, omitted";
    fragmentShader = "for parsing test only, omitted";
    inputs = {
      texture2 = texture2;
      transform1Output1 = transform1;
    };
    outputs = ["output2"];
    frameRate = 30;
  };
in
  transform2
