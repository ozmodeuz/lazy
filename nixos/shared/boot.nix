{
  boot = {
    loader = {
      efi.canTouchEfiVariables = true;
      systemd-boot.enable = true;
      timeout = 0;
    };
    plymouth = {
      enable = true;
      theme = "breeze";
    };
  };
}
