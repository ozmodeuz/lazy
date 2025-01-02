{
  sysconf = {
    oz = {
      groups = [
        mc
        networkmanager
        wheel
      ];
      keys = [
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIK6iKmnjRIMjVGYgN5SG8gVIqGT8r6DXpszo8P1FZk6j"
      ];
      packages = [
        _1password-cli
        _1password-gui
        apostrophe
        bc
        btop
        coreutils
        curl
        discord
        eyedropper
        fd
        firefox
        google-chrome
        gparted
        graphviz
        inkscape
        lazygit
        libgnome-keyring
        mcman
        micro
        nixpkgs-fmt
        nuclear
        nvd
        packwiz
        pika-backup
        prismlauncher
        ripgrep
        scribus
        shellcheck
        swig
        vim
        vscode
        yubioath-flutter
      ];
    };
  };
  programs = {
    chromium = {
      enable = true;
      extensions = [
        "aeblfdkhhhdcdjpifhhbdiojplfjncoa" # 1password
        "cjpalhdlnbpafiamejdnhcphjbkeiagm" # ublock origin
      ];
    };
    git = {
      extraconfig.init.defaultbranch = "main";
      username = "Oz Browning";
      useremail = "56755170+ozmodeuz@users.noreply.github.com";
    };
    ssh = {
      enable = true;
      extraConfig = ''
        AddKeysToAgent yes
        Host *
          IdentityAgent ~/.1password/agent.sock
      '';
    };
  };
}
