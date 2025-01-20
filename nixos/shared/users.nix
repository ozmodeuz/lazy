{
  users = {
    groups = {
      libvirtd = {
        members = [ "oz" ];
      };
    };
    users = {
      oz = {
        home = "/home/oz";
        isNormalUser = true;
        description = "Oz Browning";
        extraGroups = [
          "wheel"
          "networkmanager"
        ];
      };
    };
  };
}
