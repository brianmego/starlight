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

deploy:
    cargo build --release
    scp -i ~/aws/MegoPersonal.pem target/release/starlight ec2-user@ec2-34-209-85-43.us-west-2.compute.amazonaws.com:~

deploy_fe:
    cd ui && pnpm run build
