{
  config,
  lib,
  ...
}: let
  cfg = config.test-support;
in {
  options.test-support = with lib; {
    user = mkOption {
      default = "alice";
      type = types.str;
    };
    desktop-environment = mkOption {
      default = "icewm";
      type = types.enum ["icewm"];
    };
  };

  config = let
    de = cfg.desktop-environment;
    displayManager =
      if de == "icewm"
      then "lightdm"
      else throw "unreachable";
    windowManager =
      if de == "icewm"
      then "icewm"
      else throw "unreachable";
  in {
    virtualisation.memorySize = lib.mkDefault 2000; # 2gb
    services.displayManager.autoLogin.enable = true;
    services.displayManager.autoLogin.user = "alice";
    services.displayManager.defaultSession =
      if de == "icewm"
      then "none+icewm"
      else null;

    services.xserver.enable = true;
    services.xserver.windowManager.${windowManager}.enable = true;
    services.xserver.displayManager.${displayManager}.enable = true;

    environment.sessionVariables = {
      "ZED_ALLOW_EMULATED_GPU" = 1;
    };

    users.users.${cfg.user} = {
      createHome = true;
      home = "/home/${cfg.user}";
      group = "users";
      isNormalUser = true;
      password = "password";
    };
  };
}
