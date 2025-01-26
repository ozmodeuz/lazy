let
  hostname = "ozpc";
  sources = builtins.mapAttrs (
    name: source:
    let
      ref = if builtins.hasAttr "ref" source then source.ref else "HEAD";
      store_name = if ref != "HEAD" then "${name}-${source.ref}" else "${name}";
    in
    fetchTarball {
      name = store_name;
      url = "${source.url}/archive/${ref}.tar.gz";
      sha256 = source.sha256;
    }
  ) (import ../example/sources.nix);
  config = import ../example/config.nix { inherit sources; };
  host = config.hosts.${hostname};
  system = host.platform + "-linux";
in
{
  nixpkgs = sources.nixpkgs;
  nixos = {
    imports = host.modules;
    system.stateVersion = host.stateVersion;
    nixpkgs = sources.nixpkgs;
    inherit sources;
  };
}
