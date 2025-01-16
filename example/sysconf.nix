{
  sources = {
    stylix = {
      url = "github:danth/stylix";
      ref = "release-24.11";
    };
  };
  overlays = [
    ./overlays/sylix.nix;
  ];
  hosts = {
    book = {
      platform = "x86_64";
      version = "unstable";
      stateVersion = "24.05";
      modules = [
        ./hosts/book
        ./shared
      ];
    };
    ozpc = {
      platform = "x86_64";
      version = "unstable";
      stateVersion = "24.05";
      modules = [
        ./hosts/book
        ./shared
      ];
    };
    think = {
      platform = "x86_64";
      version = "unstable";
      stateVersion = "24.11";
      modules = [
        ./hosts/think
        ./shared
      ];
    };
  };
}
