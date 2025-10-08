{
  defaultResolution,
  defaultBuiltinNames,
  lib,
  ...
}: {
  name ? null, # string
  resolution ? defaultResolution, # { width: int, height: int }
  vertexShaderSrc ? null, # path
  vertexShader ? null, # string
  fragmentShaderSrc ? null, # path
  fragmentShader ? null, # string
  builtinNames ? {}, # { [string]: string }
  inputs ? {}, # { [string]: texturePass | transformPass }
  outputs, # [string]
  frameRate ? 30, # int
}: let
  _functionName = "mkTransform";
  builtinNames' = lib.pipe builtinNames [
    (lib.isSet "${_functionName}: `builtinNames`")
    (x: defaultBuiltinNames // x)
  ];
in {
  transformPass = {
    inherit name resolution inputs outputs frameRate;
    vertexShader =
      if (vertexShader != null && vertexShaderSrc != null)
      then throw "${_functionName}: only one of `vertexShader` and `vertexShaderSrc` should be provided"
      else if vertexShader != null
      then vertexShader
      else if vertexShaderSrc != null
      then builtins.readFile vertexShaderSrc
      else throw "${_functionName}: one of `vertexShader` or `vertexShaderSrc` must be provided";
    fragmentShader =
      if (fragmentShader != null && fragmentShaderSrc != null)
      then throw "${_functionName}: only one of `fragmentShader` and `fragmentShaderSrc` should be provided"
      else if fragmentShader != null
      then fragmentShader
      else if fragmentShaderSrc != null
      then builtins.readFile fragmentShaderSrc
      else throw "${_functionName}: one of `fragmentShader` or `fragmentShaderSrc` must be provided";
    builtinNames = builtinNames';
  };
}
