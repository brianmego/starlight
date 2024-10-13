RED := '\033[0;31m'
YELLOW := '\033[1;33m'
NC := '\033[0m'

run_backend:
    cargo watch -cqx 'run -q'

run_frontend:
    cd ui && pnpm run dev

run_db:
    surreal start file://surreal.db -u root -p root -l debug

