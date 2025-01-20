{
  security = {
    pam = {
      sshAgentAuth.enable = true;
      u2f = {
        enable = true;
        settings = {
          cue = true;
          #debug = true;
        };
      };
      services = {
        login = {
          kwallet.enable = true;
          u2fAuth = true;
          enableGnomeKeyring = true;
        };
        sudo = {
          u2fAuth = true;
          sshAgentAuth = true;
        };
      };
    };
    polkit.enable = true;
    sudo.extraConfig = ''
      Defaults lecture = never
      Defaults pwfeedback
      Defaults timestamp_timeout=120
      Defaults env_keep+=SSH_AUTH_SOCK
    '';
  };
}

