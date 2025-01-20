{ pkgs, ... }:
{
  environment = {
    sessionVariables = {
      NIXOS_OZONE_WL = "1";
      WLR_NO_HARDWARE_CURSORS = "1";
    };
    systemPackages = [
      pkgs._1password-cli
      pkgs._1password-gui
      pkgs.adwaita-qt
      pkgs.alacritty
      pkgs.apostrophe
      pkgs.bc
      pkgs.bitwarden-desktop
      pkgs.btop
      pkgs.cf-terraforming
      pkgs.coreutils
      pkgs.curl
      pkgs.devenv
      pkgs.discord
      pkgs.emacs
      pkgs.eyedropper
      pkgs.fd
      pkgs.fuzzel
      pkgs.ghostty
      pkgs.google-chrome
      pkgs.gparted
      pkgs.hyfetch
      pkgs.inkscape
      pkgs.ironbar
      pkgs.jdk
      pkgs.jetbrains.idea-community
      pkgs.kdePackages.karousel
      pkgs.kdePackages.krohnkite
      pkgs.lazygit
      pkgs.libgnome-keyring
      pkgs.micro
      pkgs.nix-index
      pkgs.nixpkgs-fmt
      pkgs.nixos-icons
      pkgs.nodejs_22
      pkgs.nuclear
      pkgs.nvd
      pkgs.nwg-panel
      pkgs.packwiz
      pkgs.pam_u2f
      pkgs.pavucontrol
      pkgs.pika-backup
      pkgs.prismlauncher
      pkgs.ripgrep
      pkgs.rofi-wayland
      pkgs.scribus
      pkgs.shellcheck
      pkgs.swaylock
      pkgs.swig
      pkgs.terraform
      pkgs.vim
      pkgs.vlc
      pkgs.vscode
      pkgs.waybar
      pkgs.xwayland-satellite
      pkgs.yaru-theme
      pkgs.yubioath-flutter
    ];
  };
}
