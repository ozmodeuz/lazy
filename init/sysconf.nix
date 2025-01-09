{

  # Other sources of packages, services, and other options that you want to use.
  # E.g.
  # sources = {
  #   stylix = {
  #     url = "github:danth/stylix/release-24.11";
  #     commit = "9015d5d0d5d100f849129c43d257b827d300b089";
  # };

  # Set of hosts by hostname; each with its own configuration.
  hosts = {

    ozpc = {

      # List of desktop environments to enable on this host.
      desktops = [
        "plasma6"
      ];

      # List of display managers to enable on this host.
      # If no display manager is set, but a desktop environment is set, then the display manager will be set according to the desktop environment.
      # E.g.
      # displaymanagers = [
      #   "sddm"
      # ];

      # Extra standard NixOS configuration to include in the resulting config.
      # E.g.
      # extraConfig = {
      #   imports = [
      #     "yubikey.nix"
      #   ];
      # };

      # List of font packages to install globally on the host.
      # E.g.
      # fonts = [
      #   "nerd-fonts.fira-code"
      # ];

      # Keyboard layout and console map.
      keyboard = [ "us" "us" ];

      # Locale
      locale = "en_US";

      # List of packages to install globally on the host.
      # E.g.
      # packages = [
      #   "curl"
      #   "git"
      #   "vim"
      # ];

      # List of services to enable globally on the host.
      # E.g.
      # services = [
      #   "openssh"
      #   "printing"
      # ];

      # list of users (from the user set below) to add to the host.
      users = [
        "oz"
      ];
    };
  };

  # Set of users by username.
  # Note that users will not be added to a host unless added to that host's 'users' list.
  # If this is a new deployment of sysconf; run 'sysconf init' to populate for this user.
  users = {

    oz = {

      # Whether the user should have sudo privilege.
      # E.g.
      # sudo = true;

      # List of groups to add the user to.
      # Groups can be configured in the groups set below, otherwise groups will be created based on the list of users assigned to the host.
      # E.g.
      # groups = [
      #   my-group
      # ];

      #TODO: Improve the examples for extraConfig here and in host config above

      # Extra standard NixOS configuration to include in the resulting config.
      # Default: {}
      # extraConfig = {
      #   imports = [
      #     ./myoptions.nix
      #   ];
      # };

      # List of ssh public keys that are authorized for this user.
      # E.g.
      # sshKeys = [
      #   "ssh-ed25519 00000000000000000000000000000000000000000000000000000000000000000000"
      # ];

      # List of packages to install for this user.
      # E.g.
      # packages = [
      #   firefox
      #   micro
      #   ripgrep
      # ];
    };
  };
}
