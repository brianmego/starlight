RED := '\033[0;31m'
YELLOW := '\033[1;33m'
NC := '\033[0m'

run_backend:
    cargo watch -w src -cx 'run'

run_frontend:
    cd ui && pnpm run dev

run_db:
    surreal start surrealkv://surreal.db -u root -p root -l debug

seed_db:
    surreal import --conn http://localhost:8000 --user root --pass root --ns scouts --db scouts seed_data.sql

deploy_be: upload_deploy_script
    cargo build --release
    ssh starlightcookies -t "sudo systemctl stop backend"
    scp target/release/starlight starlightcookies:~/starlight
    ssh starlightcookies -t "sudo systemctl start backend"

deploy_fe: upload_deploy_script
    cd ui && \
    rm -rf .next && \
    pnpm run build && \
    rm -f starlight_js.tar starlight_js.tar.xz && \
    tar --create --file starlight_js.tar .next && \
    xz -f starlight_js.tar && \
    scp starlight_js.tar.xz starlightcookies:~/frontend/starlight/ui && \
    ssh starlightcookies -t "rm ~/frontend/starlight/ui && cd ~/frontend/starlight/ui && tar xvf starlight_js.tar.xz && pm restart starlight"

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

