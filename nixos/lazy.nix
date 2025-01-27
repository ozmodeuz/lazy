{
  sources = import sources.nix;
  hosts = {
    ozpc = {
      platform = "x86_64";
      stateVersion = "24.05";
      modules = [
        ./nixos/hosts/ozpc
        ./nixos/shared
      ];
    };
  };
  lazy = {
    allowUnfree = true;
    allowModifications = true;
  };
}
