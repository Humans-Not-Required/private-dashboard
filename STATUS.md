# STATUS.md — Private Dashboard

## Current State

**Phase:** Initial scaffold — backend + frontend + tests complete  
**Tests:** 13 passing  
**Last Updated:** 2026-02-10 18:30 UTC

## What's Done

- ✅ Project scaffold (Rust/Rocket + React/Tailwind)
- ✅ SQLite database with stats table + config table
- ✅ POST /api/v1/stats — batch submit with auth
- ✅ GET /api/v1/stats — all metrics with trend data (24h/7d/30d/90d)
- ✅ GET /api/v1/stats/:key — single stat history with period filter
- ✅ GET /api/v1/health — health check with stats count
- ✅ Auto-generated manage key on first run
- ✅ Frontend: dark theme dashboard with stat cards, sparklines, trend badges
- ✅ Frontend: responsive grid, auto-refresh 60s, empty state
- ✅ 13 HTTP tests (auth, submit, query, validation, trends)
- ✅ Dockerfile (multi-stage: frontend + backend)
- ✅ docker-compose.yml (port 3008)
- ✅ GitHub repo created, pushed to main
- ✅ DESIGN.md with full API spec

## What's Next

1. **GitHub Actions CI** — test + build + push to ghcr.io (needs workflow scope)
2. **Deploy to staging** — docker-compose up on 192.168.0.79:3008
3. **Collector cron** — Playbook that reads state files and POSTs to dashboard
4. **OpenAPI spec + llms.txt** — Standard API docs
5. **More test coverage** — edge cases, large batches, concurrent writes

## ⚠️ Gotchas

- CI workflow push requires `workflow` scope on GitHub token (blocked on all HNR repos)
- Manage key is auto-generated on first run and printed to stdout — save it
- Frontend requires `bun` for Docker build (same pattern as other HNR projects)
