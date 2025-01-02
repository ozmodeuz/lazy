let
  pkgs = import <nixpkgs>;
  lib = import <nixpkgs/lib>;

  hostname = lib.strings.trim (builtins.readFile /etc/hostname);
  sysconf = (import ./ozpc.nix).${hostname};

  desktopsAndDisplayManagers = {
    plasma6.enable = sysconf.desktops.plasma6 or false;
    displayManager = {
      defaultSession = builtins.head sysconf.desktops or null;
      sddm.enable =
        if !builtins.hasAttr "displayManagers" sysconf && builtins.elem "plasma6"
          sysconf.desktops then true
        else builtins.elem "sddm" sysconf.displaymanagers;
    };
    xserver = {
      desktopManager = {
        gnome.enable = builtins.elem "gnome" sysconf.desktops or false;
      };
      displayManager = {
        gdm.enable =
          if !builtins.hasAttr "displayManagers" sysconf && builtins.elem "gnome" sysconf.desktops then true
          else builtins.elem "gdm" sysconf.displaymanagers;
      };
      xkb.layout = builtins.head sysconf.keyboard;
    };
  };

  config = {
    boot = {
      loader = {
        systemd-boot.enable = true;
        efi.canTouchEfiVariables = true;
      };
    };
    console.keyMap = builtins.tail sysconf.keyboard;
    environment = {
      systemPackages = with pkgs; sysconf.packages;
    };
    fonts.packages = with pkgs; sysconf.fonts;
    i18n.defaultLocale = sysconf.locale + ".UTF-8";
    networking = {
      hostname = hostname;
    };
    services = builtins.listToAttrs
      (map
        (
          service: {
            name = service;
            value = {
              enable = true;
            };
          }
        )
        sysconf.services) // desktopsAndDisplayManagers;
  };

in
builtins.toJSON config
