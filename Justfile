RED := '\033[0;31m'
YELLOW := '\033[1;33m'
NC := '\033[0m'

run_backend:
    cargo watch -cx 'run'

run_frontend:
    cd ui && pnpm run dev

run_db:
    surreal start surrealkv://surreal.db -u root -p root -l debug

seed_db:
    surreal import --conn http://localhost:8000 --user root --pass root --ns scouts --db scouts seed_data.sql
