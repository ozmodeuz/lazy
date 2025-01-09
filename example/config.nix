{
  nixos = {
    version = "unstable";
    commit = "cbd8ec4de4469333c82ff40d057350c30e9f7d36";
  };

  sources = {
    stylix = {
      type = "git";
      url = "github:danth/stylix";
      ref = "release-24.11";
      commit = "9015d5d0d5d100f849129c43d257b827d300b089";
    };
  };

  hosts = {
    ozpc = {
      desktops = [
        "plasma6"
        "gnome"
      ];
      fonts = [
        "nerd-fonts._0xproto"
        "nerd-fonts.fira-code"
        "nerd-fonts.jetbrains-mono"
        "nerd-fonts.symbols-only"
      ];
      keyboard = [ "gb" "uk" ];
      locale = "en_GB";
      packages = [
        "alacritty"
        "blesh"
        "devenv"
        "fuzzel"
        "git"
        "nushell"
        "starship"
        "wallust"
      ];
      services = [
        "openssh"
        "pcsd"
        "printing"
      ];
      users = [
        "oz"
      ];
    };
  };

  users = {
    oz = {
      fullname = "Oz Browning";
      sudo = true;
      groups = [
        "mc"
        "networkmanager"
        "wheel"
      ];
      packages = [
        "firefox"
        "micro"
        "nvd"
        "ripgrep"
      ];
    };
  };
}
