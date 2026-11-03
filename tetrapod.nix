{ config, userName, ... }:
{
  systemd.user = {
    services.tetrapod = {
      description = "Get a daily new wallpaper from Nasa";
      serviceConfig = {
        Type = "oneshot";
        WorkingDirectory = "/home/${userName}/Documents/coding/rust/Tetrapod";
        ExecStart = "/home/${userName}/Documents/coding/rust/Tetrapod/target/release/Tetrapod";
      };
    };

    timers.tetrapod = {
      description = "Tetrapod timer";
      wantedBy = [ "timers.target" ];
      timerConfig = {
        OnBootSec = "5min";
        OnUnitActiveSec = "1h";
        Persistent = true;
      };
    };
  };
}
