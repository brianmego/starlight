RED := '\033[0;31m'
YELLOW := '\033[1;33m'
NC := '\033[0m'

run_backend:
    bacon -j run

run_frontend:
    cd ui && pnpm run dev

run_db:
    surreal start surrealkv://surreal.db -u root -p root -l debug

seed_db:
    surreal import --conn http://localhost:8000 --user root --pass root --ns scouts --db scouts seed_data.sql

deploy_be: upload_deploy_script
    cargo build --release
    scp target/release/starlight starlightcookies:~/starlight/starlight.new
    ssh starlightcookies -t "sudo systemctl stop backend && mv ~/starlight/starlight{,.bak} && mv ~/starlight/starlight{.new,} && sudo systemctl start backend"

revert_be:
    ssh starlightcookies -t "sudo systemctl stop backend && mv ~/starlight/starlight{,.tmp} && mv ~/starlight/starlight{.bak,} && mv ~/starlight/starlight{.tmp,.bak} && sudo systemctl start backend"

deploy_fe: upload_deploy_script
    cd ui && \
    rm -rf .next && \
    pnpm run build && \
    rm -f starlight_js.tar starlight_js.tar.xz && \
    tar --create --file starlight_js.tar .next && \
    xz -f starlight_js.tar && \
    scp starlight_js.tar.xz starlightcookies:~/frontend/starlight/ui && \
    ssh starlightcookies -t "rm -rf ~/frontend/starlight/ui/.next && cd ~/frontend/starlight/ui && tar xvf starlight_js.tar.xz && pm2 restart starlight"

deploy_systemd_services: upload_deploy_script
    mkdir -p dist
    tar -Zcvf dist/starlight_services.tgz \
        deploy/backend.service \
        deploy/database.service \
        deploy/starlight.nginx.conf
    scp dist/starlight_services.tgz starlightcookies:~/starlight
    ssh starlightcookies -t "sudo ~/starlight/deploy.sh"

upload_deploy_script:
    scp deploy/deploy.sh starlightcookies:~/starlight

alias t := test
test:
    cargo nextest run
