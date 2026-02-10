# 📊 Private Dashboard

Local network stats dashboard for the [Humans Not Required](https://github.com/Humans-Not-Required) ecosystem. Displays key operational metrics with trend data across multiple time windows.

## Features

- **Time-series metrics storage** — Submit stats via API, query with trends
- **Trend analysis** — 24h/7d/30d/90d windows with percentage change
- **Sparkline charts** — Visual 24h mini-charts per metric
- **Dark theme** — Fullscreen dashboard designed for always-on displays
- **Auto-refresh** — Updates every 60 seconds
- **Token auth** — Write-protected with auto-generated manage key

## Quick Start

```bash
# Build and run
cargo run

# The manage key is printed on first run
# 🔑 Generated new manage key: dash_xxxx

# Submit stats
curl -X POST http://localhost:8000/api/v1/stats \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer dash_xxxx" \
  -d '[{"key":"agents_discovered","value":645},{"key":"repos_count","value":7}]'

# View stats
curl http://localhost:8000/api/v1/stats

# View history
curl http://localhost:8000/api/v1/stats/agents_discovered?period=7d
```

## API

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v1/health` | No | Health check |
| POST | `/api/v1/stats` | Bearer | Submit stat batch (up to 100) |
| GET | `/api/v1/stats` | No | All stats with trends |
| GET | `/api/v1/stats/:key` | No | Single stat history |

## Docker

```bash
docker compose up -d
# Accessible at http://localhost:3008
```

## Tech Stack

- **Backend:** Rust (Rocket), SQLite
- **Frontend:** React, Tailwind CSS, Vite
- **Deployment:** Docker, Watchtower auto-deploy

## License

MIT
