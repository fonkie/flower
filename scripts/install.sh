#!/bin/bash


if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root (sudo ./service.sh)" 
   exit 1
fi


SERVICE_NAME="flower"
CWD_PATH=$(pwd)
BINARY_PATH="$CWD_PATH/flower"
WORKING_DIR="$CWD_PATH"
SERVICE_USER="flower"
SERVICE_GROUP="flower"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"


if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

chmod +x "$BINARY_PATH"

if ! id "$SERVICE_USER" &>/dev/null; then
    echo "Creating service user: $SERVICE_USER"
    useradd -r -s /bin/false -d "$WORKING_DIR" "$SERVICE_USER"
    groupadd -f "$SERVICE_GROUP"
    usermod -a -G "$SERVICE_GROUP" "$SERVICE_USER"
fi


chown -R "$SERVICE_USER":"$SERVICE_GROUP" "$WORKING_DIR"

echo "Creating systemd service file: $SERVICE_FILE"
cat > "$SERVICE_FILE" << EOF
[Unit]
Description=Flower API Service
Documentation=https://github.com/yourusername/flower
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=${SERVICE_USER}
Group=${SERVICE_GROUP}
WorkingDirectory=${WORKING_DIR}
ExecStart=${BINARY_PATH}
Restart=on-failure
RestartSec=5s

Environment=RUST_LOG=info
EnvironmentFile=${WORKING_DIR}/.env
NoNewPrivileges=true
ProtectSystem=full
PrivateTmp=true

[Install]
WantedBy=multi-user.target
EOF


echo "Reloading systemd daemon"
systemctl daemon-reload


echo "Enabling $SERVICE_NAME service"
systemctl enable "$SERVICE_NAME"


echo "Starting $SERVICE_NAME service"
systemctl start "$SERVICE_NAME"


echo "Checking service status:"
systemctl status "$SERVICE_NAME"

echo ""
echo "Installation complete! The Flower API is now running as a systemd service."
echo ""
echo "Service management commands:"
echo "  Start:   sudo systemctl start $SERVICE_NAME"
echo "  Stop:    sudo systemctl stop $SERVICE_NAME"
echo "  Restart: sudo systemctl restart $SERVICE_NAME"
echo "  Status:  sudo systemctl status $SERVICE_NAME"
echo "  Logs:    sudo journalctl -u $SERVICE_NAME -f"