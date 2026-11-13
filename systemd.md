```toml
[Unit]
Description=Generate daily notes with Groundhog
After=network.target
StartLimitIntervalSec=0
[Service]
Type=oneshot
Restart=always
RestartSec=1
User=Your user name here
ExecStart=~/.cargo/bin/marmota
[Timer]
OnCalenda=daily
Persistent = true

[Install]
WantedBy=multi-user.target
```