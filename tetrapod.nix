{ config, userName, ... }:
{
  systemd = {
    services.tetrapod = {
      description = "Get a daily new wallpaper from Nasa";
      script = ''
        /home/${userName}/Documents/coding/rust/Tetrapod/target/release/Tetrapod
      '';
      serviceConfig = {
        Type = "oneshot";
        User = userName;
      };
    };

    timers.tetrapod = {
      description = "Tetrapod timer";
      wantedBy = [ "timers.target" ];
      timerConfig = {
        OnBootSec = "5min";
        OnUnitActiveSec = "2h";
        Persistent = true;
      };
    };
  };
}
