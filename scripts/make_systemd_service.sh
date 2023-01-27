#!/usr/bin/env bash

set -euxo pipefail

CURRENT_DIR=$(pwd)

cat > ${CURRENT_DIR}/poll.service << EOF
[Unit]
Description=A web service for doing ranked choice voting
After=mysql.service

[Service]
Type=simple
Restart=always
ExecStart=${CURRENT_DIR}/poll
WorkingDirectory=${CURRENT_DIR}
Environment=POLL__RUN_MODE=production

[Install]
WantedBy=multi-user.target
EOF
