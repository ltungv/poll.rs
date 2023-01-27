#!/usr/bin/env bash

set -euxo pipefail

SERVICE_WORKING_DIR=$1
CURRENT_DIR=$(pwd)

cat > ${CURRENT_DIR}/poll.service << EOF
[Unit]
Description=A web service for doing ranked choice voting
After=mysql.service

[Service]
Type=simple
Restart=always
ExecStart=${SERVICE_WORKING_DIR}/poll
WorkingDirectory=${SERVICE_WORKING_DIR}
Environment=POLL__RUN_MODE=production

[Install]
WantedBy=multi-user.target
EOF
