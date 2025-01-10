#!/bin/bash
set -e

if [ "$EUID" -ne 0 ]
  then echo "Only root user may run this script"
  exit
fi

THIS_DIR=$(dirname ${BASH_SOURCE[0]})
cd $THIS_DIR

tar -xzvf starlight_services.tgz
systemctl stop backend.service database.service

mkdir -p /var/log/starlight
cp -v deploy/backend.service /etc/systemd/system/backend.service
cp -v deploy/database.service /etc/systemd/system/database.service
cp -v deploy/starlight.nginx.conf /etc/nginx/conf.d/starlight.nginx.conf
systemctl daemon-reload
systemctl start database.service
sleep 30
systemctl start backend.service
runuser -u ec2-user -- /home/ec2-user/.local/share/pnpm/pm2 restart starlight
systemctl reload nginx
