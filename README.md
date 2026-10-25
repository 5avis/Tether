# Tether (`tether`)

<div align="center">
  <h3>Unified Multi-Cloud Infrastructure, Quota Watchdog & Instant Preview Tunnel Control Plane</h3>
  <p><strong>Matte Graphite & Coral Palette</strong> • <strong>Sub-10 MB Native Binary (6.7 MB)</strong> • <strong>Sub-25 MB Idle RAM</strong> • <strong>100% Local-First (POSIX 0600)</strong></p>
</div>

---

## 🌟 Architectural Features

1. **Instant Cloudflare Preview Tunnels**:
   - Zero-DNS public preview addresses via `cloudflared tunnel --url http://localhost:<port>`.
   - Guaranteed subprocess hygiene: child handles are tracked asynchronously and gracefully terminated on application exit, window close, or `SIGTERM`.

2. **Multi-Cloud Free-Tier Watchdog & Inactivity Guard**:
   - Monitors compute quotas across Fly.io, Cloudflare, Supabase, Neon Serverless, and Render.
   - Calculates database dormancy intervals and triggers automated or manual **Keep-Alive Pings** to reset 7-day lifecycle retention clocks back to 168 hours.

3. **Parallel Multi-Cloud Deployer**:
   - Concurrently triggers build and release pipelines across all linked container targets simultaneously using `tokio` async pools.
   - Live pulse animations and stage badges (`Queued` ➔ `Building` ➔ `Deploying` ➔ `Live`).

4. **Secret Synchronizer**:
   - Parses local `.env` files and pushes variables across Fly.io, Cloudflare Workers, Supabase, and Render in a single concurrent operation.

5. **Unified Real-Time Log Interleaver**:
   - Multiplexes deployment, quick tunnel, and runtime log streams into a unified, color-coded, searchable terminal stream and slide-up drawer.

6. **Local-First Security**:
   - Configurations and provider keys reside strictly at `~/.config/tether/config.json`.
   - Strictly enforced POSIX permissions: `0700` (`drwx------`) for the directory and `0600` (`-rw-------`) for the config file.
   - Zero telemetry or external relay servers.

---

## 🚀 Running the Compiled Standalone Binary

The standalone desktop executable is compiled and ready to run with **zero build overhead**:

```bash
cd ~/Projects/Tether
./src-tauri/target/release/tether
```

*Binary size: **6.7 MB** (Target: < 10 MB). Idle RAM: **< 25 MB**.*

---

## 🛠️ Development & Building

### Prerequisites
- Rust 1.80+ (`rustup`)
- Node.js 18+ & npm
- WebKitGTK 4.1 (`webkit2gtk-4.1`) & GTK3

### Commands
```bash
# Install dependencies
npm install

# Run desktop dev environment (hot-reload)
npm run tauri dev

# Run Rust unit tests (all 8 tests passing)
cd src-tauri && cargo test

# Build optimized production release binary
npx tauri build --no-bundle
```
