let

  config = import ../example/config.nix;
  stateVersion = builtins.fromTOML (lib.readFile ../example/stateversion).${hostname};

  nixpkgs = {
    url = "https://github.com/nixos/nixpkgs/archive/nixos-${config.nixos.version}.tar.gz?ref=${config.nixos.commit}";
    tarballHash = "1bcgis62aj9slin2cm2h7ywqq4qswh4kzdlx0zgdwz80g4z10f8f";
  };

  pkgs = import
    (builtins.fetchTarball {
      url = nixpkgs.url;
      sha256 = nixpkgs.tarballHash;
    })
    { };

  lib = pkgs.lib;

  resolvePkg = pkgName:
    let
      parts = lib.strings.splitString "." pkgName;
      resolved = lib.foldl'
        (attrSet: attr:
          if attrSet != null && lib.hasAttr attr attrSet
          then lib.getAttr attr attrSet
          else attr
        )
        pkgs
        parts;
    in
    if resolved != null then resolved else lib.trace "Package not found"; #TODO: Add traces all over

  hostname = lib.strings.trim (lib.readFile /etc/hostname);
  host = config.hosts.${hostname};

  buildNixOSConfig = config:
    let
      nixOSConfig = {
        boot = {
          loader = {
            systemd-boot. enable = lib.mkDefault true;
            efi.canTouchEfiVariables = lib.mkDefault true;
          };
        };
        console.keyMap = lib.mkDefault (lib.tail config.hosts.${hostname}.keyboard);
        desktopManager = {
          plasma6.enable = lib.mkDefault (lib.elem "plasma6" config.hosts.${hostname}.desktops);
        };
        displayManager = {
          defaultSession = lib.mkDefault (lib.head config.hosts.${hostname}.desktops or null);
          sddm.enable = lib.elem "plasma6" config.hosts.${hostname}.desktops && !builtins.hasAttr "displayManagers" host; #or lib.elem "sddm" config.hosts.${hostname}.displaymanagers;
        };
        environment = {
          systemPackages = lib.mkDefault (lib.map resolvePkg config.hosts.${hostname}.packages);
        };
        font.packages = lib.mkDefault (lib.map resolvePkg config.hosts.${hostname}.fonts);
        i18n.defaultLocale = lib.mkDefault (config.hosts.${hostname}.locale + ".UTF-8");
        networking = {
          hostname = hostname;
        };
        services = lib.mkDefault
          (
            builtins.listToAttrs
              (map
                (
                  service: {
                    name = service;
                    value = {
                      enable = true;
                    };
                  }
                )
                config.hosts.${hostname}.services) // {
              xserver = {
                desktopManager = {
                  gnome.enable = lib.elem "gnome" config.hosts.${hostname}.desktops or false;
                };
                displayManager = {
                  gdm.enable =
                    builtins.elem "gdm" config.hosts.${hostname}.displaymanagers or (lib.elem "gnome" config.hosts.${hostname}.desktops && !lib.hasAttr "displayManagers" host);
                };
                xkb.layout = builtins.head config.hosts.${hostname}.keyboard;
              };
            }
          );
        stateVersion = stateVersion;
        system = lib.mkDefault config.hosts.${hostname}.platform or "x86_64-linux";
        users.users = lib.mkDefault (
          builtins.listToAttrs
            (map
              (
                user:
                if builtins.hasAttr user config.users then {
                  name = user;
                  value = {
                    isNormalUser = lib.mkDefault true;
                    description = lib.mkDefault config.users.${user}.fullname;
                    extraGroups = lib.mkDefault config.users.${user}.groups;
                    packages = lib.mkDefault (lib.map resolvePkg config.users.${user}.packages);
                    extraConfig = config.users.${user}.extraConfig;
                  };
                } else null
              )
              config.hosts.${hostname}.users));
      } // (if lib.hasAttr "extraConfig" host then config.hosts.${hostname}.extraConfig else { });
    in
    nixOSConfig;

in
buildNixOSConfig config
