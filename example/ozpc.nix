{
  ozpc = {
    imports = [
      "cloudflared.nix"
      "fancontrol.nix"
      "graphics.nix"
      "tailscale.nix"
      "yubikey.nix"
    ];
    desktops = [
      "plasma6"
      "gnome"
    ];
    # displaymanagers = [
    #   "gdm"
    #   "sddm"
    # ];
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
      "nixos-icons"
      "nushell"
      "pam_u2f"
      "starship"
      "wallust"
      "yaru-theme"
    ];
    programs = [
      "direnv"
      "firefox"
      "niri"
    ];
    services = [
      #"openssh"
      "pcsd"
      "printing"
    ];
    users = [
      "oz"
    ];
  };
}
