{ pkgs, ... }:
let
  mkMinecraftService = name: path: jdkPackage: {
    enable = true;
    description = "Minecraft (${name})";
    path = [
      pkgs.bash
      jdkPackage
    ];
    serviceConfig = {
      ExecStart = "${pkgs.bash}/bin/bash /home/oz/.minecraft/${name}/server/start.sh";
      Restart = "always";
      WorkingDirectory = "/home/oz/.minecraft/${name}/server";
      StandardInput = "socket";
      StandardOutput = "journal";
    };
    wantedBy = [ "multi-user.target" ];
  };

  mkMinecraftSocket = name: {
    description = "Minecraft ${name} Socket";
    wantedBy = [ "sockets.target" ];
    socketConfig.ListenFIFO = [ "/run/minecraft/${name}" ];
  };

in
{
  users.groups = {
    mc = { gid = 10001; };
  };
  systemd.services.mc4eva = mkMinecraftService "mc4eva" "/home/oz/.minecraft/mc4eva/server" pkgs.jdk;
  systemd.services.skyblock = mkMinecraftService "skyblock" "/home/oz/.minecraft/skyblock/server" pkgs.jdk;
  systemd.sockets.mc4eva = mkMinecraftSocket "mc4eva";
  systemd.sockets.skyblock = mkMinecraftSocket "skyblock";
  #environment.systemPackages = [ pkgs.mcman ];
}
