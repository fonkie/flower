#!/bin/bash

# add-flower-api-proxy.sh - Add Flower API proxy configuration to Nginx
# This script adds the necessary location blocks to your Nginx configuration
# to proxy requests to the Flower API service running on port 10086

# Text formatting
BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration variables
NGINX_CONF_DIR="/www/server/panel/vhost/nginx"
SITE_DOMAIN="sanyuan.xn--bww30p.com"
SITE_CONF="${NGINX_CONF_DIR}/${SITE_DOMAIN}.conf"
BACKUP_SUFFIX=".bak.$(date +%Y%m%d%H%M%S)"
FLOWER_PORT="10086"

# Flower API configuration block
read -r -d '' API_CONFIG << 'EOF'
    location /api/ {
        proxy_pass http://127.0.0.1:10086/api/;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_buffering off;
        proxy_read_timeout 90s;
        client_max_body_size 10m;
        
        add_header 'Access-Control-Allow-Origin' '*' always;
        add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS' always;
        add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization' always;
        
        if ($request_method = 'OPTIONS') {
            add_header 'Access-Control-Allow-Origin' '*';
            add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS';
            add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization';
            add_header 'Access-Control-Max-Age' 1728000;
            add_header 'Content-Type' 'text/plain; charset=utf-8';
            add_header 'Content-Length' 0;
            return 204;
        }
    }
    
    location = /api {
        proxy_pass http://127.0.0.1:10086/;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
EOF

# Function to print status messages
print_status() {
    local type=$1
    local message=$2
    
    case "$type" in
        "info")
            echo -e "[${BOLD}INFO${NC}] $message"
            ;;
        "success")
            echo -e "[${GREEN}SUCCESS${NC}] $message"
            ;;
        "warning")
            echo -e "[${YELLOW}WARNING${NC}] $message"
            ;;
        "error")
            echo -e "[${RED}ERROR${NC}] $message"
            ;;
        *)
            echo -e "$message"
            ;;
    esac
}

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    print_status "error" "This script must be run as root (use sudo)"
    exit 1
fi

# Check if Nginx is installed
if ! command -v nginx &>/dev/null; then
    print_status "error" "Nginx is not installed. Please install Nginx before running this script."
    exit 1
fi

# Check if Flower API is running
if ! netstat -tuln | grep -q ":$FLOWER_PORT "; then
    print_status "warning" "Flower API does not appear to be running on port $FLOWER_PORT"
    read -p "Continue anyway? (y/n): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_status "info" "Exiting without making changes"
        exit 0
    fi
fi

# Check if the site configuration exists
if [ ! -f "$SITE_CONF" ]; then
    print_status "error" "Site configuration not found: $SITE_CONF"
    print_status "info" "Available site configurations:"
    ls -1 "$NGINX_CONF_DIR"/*.conf 2>/dev/null || echo "  None found"
    exit 1
fi

# Check if configuration already contains API proxy settings
if grep -q "location /api/" "$SITE_CONF"; then
    print_status "warning" "API proxy configuration already exists in this file"
    read -p "Do you want to replace it? (y/n): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_status "info" "Exiting without making changes"
        exit 0
    fi
    # If replacing, we'll remove the existing configuration
    print_status "info" "Will replace existing API proxy configuration"
    REPLACE_EXISTING=true
else
    REPLACE_EXISTING=false
fi

# Back up the original configuration
print_status "info" "Creating backup of original configuration"
cp "$SITE_CONF" "${SITE_CONF}${BACKUP_SUFFIX}"
if [ $? -ne 0 ]; then
    print_status "error" "Failed to create backup file"
    exit 1
fi
print_status "success" "Backup created: ${SITE_CONF}${BACKUP_SUFFIX}"

# Temporary file for the new configuration
TEMP_CONF=$(mktemp)

if $REPLACE_EXISTING; then
    # Remove existing API proxy configuration
    print_status "info" "Removing existing API proxy configuration"
    sed '/location \/api\//,/}/d' "$SITE_CONF" | sed '/location = \/api/,/}/d' > "$TEMP_CONF"
else
    # Copy original configuration
    cp "$SITE_CONF" "$TEMP_CONF"
fi

# Find the position to insert our configuration
# We'll insert it right before the closing brace of the server block
SERVER_END_LINE=$(grep -n "}" "$TEMP_CONF" | tail -1 | cut -d: -f1)

if [ -z "$SERVER_END_LINE" ]; then
    print_status "error" "Could not find server block closing brace in configuration"
    rm "$TEMP_CONF"
    exit 1
fi

# Split the configuration file into two parts
head -n $(($SERVER_END_LINE - 1)) "$TEMP_CONF" > "${TEMP_CONF}.part1"
tail -n +$SERVER_END_LINE "$TEMP_CONF" > "${TEMP_CONF}.part2"

# Create the new configuration by combining the parts with our API configuration
cat "${TEMP_CONF}.part1" > "${TEMP_CONF}.new"
echo "$API_CONFIG" >> "${TEMP_CONF}.new"
cat "${TEMP_CONF}.part2" >> "${TEMP_CONF}.new"

# Test the new configuration
print_status "info" "Testing new configuration"
nginx -t -c "${TEMP_CONF}.new" &>/dev/null
if [ $? -ne 0 ]; then
    print_status "error" "New configuration is invalid. See details below:"
    nginx -t -c "${TEMP_CONF}.new"
    print_status "info" "Original configuration restored, no changes made"
    rm "${TEMP_CONF}"*
    exit 1
fi

# Move the new configuration into place
print_status "info" "Applying new configuration"
cat "${TEMP_CONF}.new" > "$SITE_CONF"

# Clean up temporary files
rm "${TEMP_CONF}"*

# Test the installed configuration
print_status "info" "Verifying installed configuration"
nginx -t
if [ $? -ne 0 ]; then
    print_status "error" "Something went wrong with the configuration"
    print_status "info" "Restoring backup from ${SITE_CONF}${BACKUP_SUFFIX}"
    cp "${SITE_CONF}${BACKUP_SUFFIX}" "$SITE_CONF"
    nginx -t
    exit 1
fi

# Reload Nginx to apply changes
print_status "info" "Reloading Nginx service"
systemctl reload nginx
if [ $? -ne 0 ]; then
    print_status "error" "Failed to reload Nginx"
    print_status "warning" "Configuration is valid but changes may not be applied"
    exit 1
fi

print_status "success" "Flower API proxy configuration has been successfully added to Nginx"
print_status "info" "Your API is now accessible at: http://$SITE_DOMAIN/api"
print_status "info" "To test, run: curl -s http://$SITE_DOMAIN/api | grep version"

exit 0