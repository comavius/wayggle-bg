let
  checkHardInternal = {
    ctx,
    cond,
    val,
  }:
    if cond
    then val
    else builtins.abort ctx;
in {
  checkHard = {
    ctx,
    cond,
  }: val:
    checkHardInternal {
      inherit ctx cond val;
    };

  isSet = ctx: val:
    checkHardInternal {
      ctx = "${ctx}: expected a set but got a ${builtins.typeOf val}";
      cond = builtins.typeOf val == "set";
      val = val;
    };

  pipe = v: fs: builtins.foldl' (acc: f: f acc) v fs;
}
