# 🌐 Polyglot-Microservices-Stack

> A cloud-native web service demonstrating ten programming languages and runtimes across two managed hosting platforms — **zero Dockerfiles required**.

[![Deploy on Render](https://render.com/images/deploy-to-render-button.svg)](https://render.com/deploy)

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Architecture](#2-architecture)
3. [Technology Stack](#3-technology-stack)
4. [Repository Structure](#4-repository-structure)
5. [Prerequisites](#5-prerequisites)
6. [Render Deployment — Backend](#6-render-deployment--backend)
   - 6.1 [Connect Repository](#61-connect-repository-to-render)
   - 6.2 [Service 1 — Rust/Actix-Web](#62-service-1--rustactix-web-api-gateway)
   - 6.3 [Service 2 — Python 3/FastAPI](#63-service-2--python-3--fastapi)
   - 6.4 [render.yaml Reference](#64-renderyaml-reference)
   - 6.5 [Verifying the Render Deployment](#65-verifying-the-render-deployment)
7. [Vercel Deployment — Frontend](#7-vercel-deployment--frontend)
   - 7.1 [Connect Repository](#71-connect-repository-to-vercel)
   - 7.2 [Project Configuration](#72-project-configuration)
   - 7.3 [Environment Variables](#73-environment-variables)
   - 7.4 [vercel.json Reference](#74-verceljson-reference)
   - 7.5 [Verifying the Vercel Deployment](#75-verifying-the-vercel-deployment)
8. [Local Development](#8-local-development)
9. [API Reference](#9-api-reference)
10. [Mermaid Diagrams](#10-mermaid-diagrams)
11. [Optional Language Setup](#11-optional-language-setup)
12. [Troubleshooting](#12-troubleshooting)
13. [Quick Reference Card](#13-quick-reference-card)
14. [License](#14-license)

---

## 1. Project Overview

**Polyglot-Microservices-Stack** is a production-ready template for embedding multiple programming languages into a single backend service and deploying the full stack to managed cloud platforms without container orchestration. The backend — hosted on **Render** — is a Rust/Actix-Web API gateway that dispatches requests to C++, C#, Python 3, Zig, Mojo, F\*, and Dafny through subprocess invocations and HTTP proxying. The frontend — hosted on **Vercel** — is a Next.js 14 application that integrates HTMX, Alpine.js, Hyperscript, and TailwindCSS for progressive enhancement, while SQL.js and PHP-WASM run as WebAssembly modules inside Next.js API routes.

| Platform | Service | Runtime |
|----------|---------|---------|
| **Render** | `polyglot-backend` | Rust (Actix-Web 4) |
| **Render** | `python-fastapi` | Python 3 (FastAPI) |
| **Vercel** | `polyglot-microservices-stack` | Next.js 14 |

---

## 2. Architecture

The following Mermaid diagrams are included in the repository root and render natively on GitHub:

| File | Type | Purpose |
|------|------|---------|
| `README.mmd` | Flowchart | Top-level Vercel/Render stack overview |
| `ARCHITECTURE.mmd` | C4 Context | System boundaries and user flows |
| `SEQUENCE.mmd` | Sequence Diagram | Full request lifecycle per language |
| `FLOWCHART.mmd` | Flowchart LR | Backend language dispatch routing |
| `DEPLOY.mmd` | Flowchart | CI/CD pipeline from push to production |

**Request flow summary:** The browser interacts with Next.js on Vercel. HTMX, Alpine.js, and Hyperscript handle frontend interactivity declaratively. Backend API calls are routed through Next.js proxy API routes (`/api/proxy/[...path]`) to the Render Rust service, which then dispatches to the appropriate language handler and returns a unified `LangResponse` JSON object.

---

## 3. Technology Stack

### Backend (Render)

| Language | Version | Integration Method | Source File |
|----------|---------|-------------------|-------------|
| **Rust** | 1.82+ | Native (Actix-Web handler) | `src/handlers/rust_handler.rs` |
| **C++** | C++17 | `g++` compile + subprocess | `cpp/compute.cpp` |
| **C#** | .NET 8 | `dotnet run` subprocess | `csharp/Processor.cs` |
| **Python 3** | 3.11+ | HTTP → FastAPI (`reqwest`) | `backend/python-fastapi/main.py` |
| **Zig** | 0.13.0 | `zig run` subprocess | `zig/compute.zig` |
| **Mojo** | Latest (Magic SDK) | `mojo run` subprocess | `mojo/compute.mojo` |
| **F\*** | Latest (OPAM) | `fstar.exe` subprocess | `fstar/Verify.fst` |
| **Dafny** | Latest (dotnet tool) | `dafny verify` subprocess | `dafny/Verify.dfy` |

### Frontend (Vercel)

| Technology | Version | Role |
|-----------|---------|------|
| **Next.js** | 14.2.15 | SSR/CSR framework |
| **TailwindCSS** | 3.4 | Utility-first styling |
| **HTMX** | 1.9.12 | Declarative HTTP interactions (CDN) |
| **Alpine.js** | 3.14.1 | Reactive in-HTML state (CDN) |
| **Hyperscript** | 0.9.13 | DOM scripting (CDN) |
| **SQL.js** | 1.12.0 | SQLite compiled to WASM (npm) |
| **@php-wasm/node** | 0.0.24 | PHP compiled to WASM (npm) |

---

## 4. Repository Structure

```
Polyglot-Microservices-Stack/
├── README.mmd              # Mermaid: project overview flowchart
├── ARCHITECTURE.mmd        # Mermaid: C4 context diagram
├── SEQUENCE.mmd            # Mermaid: request sequence diagram
├── FLOWCHART.mmd           # Mermaid: backend dispatch flowchart
├── DEPLOY.mmd              # Mermaid: CI/CD deploy flow
├── render.yaml             # Render Blueprint (2 services)
├── vercel.json             # Vercel project configuration
│
├── backend/
│   ├── rust-actix/                     ← Render Service 1
│   │   ├── Cargo.toml
│   │   ├── build.sh                    ← Installs .NET, Zig; compiles C++; cargo build
│   │   ├── .env.example
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── models/mod.rs           ← LangResponse, HealthResponse
│   │   │   ├── routes/mod.rs           ← Route registration
│   │   │   └── handlers/
│   │   │       ├── mod.rs
│   │   │       ├── health_handler.rs
│   │   │       ├── rust_handler.rs
│   │   │       ├── cpp_handler.rs
│   │   │       ├── csharp_handler.rs
│   │   │       ├── python_handler.rs
│   │   │       ├── zig_handler.rs
│   │   │       ├── mojo_handler.rs
│   │   │       ├── fstar_handler.rs
│   │   │       ├── dafny_handler.rs
│   │   │       └── all_handler.rs
│   │   ├── cpp/compute.cpp + compute.h
│   │   ├── csharp/Processor.csproj + Processor.cs
│   │   ├── zig/compute.zig
│   │   ├── mojo/compute.mojo
│   │   ├── fstar/Verify.fst
│   │   └── dafny/Verify.dfy
│   │
│   └── python-fastapi/                 ← Render Service 2
│       ├── requirements.txt
│       └── main.py
│
└── frontend/nextjs/                    ← Vercel
    ├── package.json
    ├── next.config.js
    ├── tailwind.config.js
    ├── postcss.config.js
    ├── styles/globals.css
    ├── pages/
    │   ├── _app.js                     ← Loads HTMX, Alpine, Hyperscript via CDN
    │   ├── _document.js
    │   ├── index.js                    ← Main UI with all integrations
    │   └── api/
    │       ├── proxy/[...path].js      ← Backend proxy (avoids CORS)
    │       ├── sqljs.js                ← SQL.js WASM API route
    │       └── phpjs.js                ← PHP-WASM API route
    ├── components/
    │   ├── LanguageCard.js
    │   ├── Layout.js
    │   └── ResultDisplay.js
    └── public/
        └── favicon.svg
```

---

## 5. Prerequisites

Before deploying, confirm the following are available.

**Accounts required:**
- [GitHub](https://github.com) account with the repository pushed to a remote named `Polyglot-Microservices-Stack`
- [Render](https://render.com) account — Free tier is sufficient for initial deployment; Starter plan ($7/month) is recommended for production to avoid instance sleep on inactivity
- [Vercel](https://vercel.com) account — Free Hobby tier is sufficient

**Local tools (for local development only):**
- Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Node.js 20+: https://nodejs.org
- Python 3.11+: https://python.org
- Vercel CLI (optional): `npm install -g vercel`

---

## 6. Render Deployment — Backend

Render hosts two services defined in `render.yaml`. Both are connected to the same repository and deploy automatically on every push to `main`.

> **Free Tier Notice:** Render's free tier spins down services after 15 minutes of inactivity. Cold starts may take 30–60 seconds. For production, upgrade to a paid plan or use an uptime monitor to ping `/health` every 14 minutes.

### 6.1 Connect Repository to Render

1. Navigate to [https://render.com](https://render.com) and sign in.
2. Click **New → Blueprint** from the dashboard.
3. Select your Git provider, search for `Polyglot-Microservices-Stack`, and click **Connect**.
4. Render detects the root-level `render.yaml` and displays both services. Review and click **Apply**.

> **Alternative:** Create each service manually via **New → Web Service** using the configuration values in Sections 6.2 and 6.3.

---

### 6.2 Service 1 — Rust/Actix-Web API Gateway

#### Service Settings

| Field | Value |
|-------|-------|
| **Service Name** | `polyglot-backend` |
| **Service Type** | Web Service |
| **Runtime** | Rust |
| **Region** | Oregon (US West) — or nearest available |
| **Plan** | Free (dev) / Starter $7/mo (production) |
| **Root Directory** | `backend/rust-actix` |
| **Build Command** | `bash build.sh` |
| **Start Command** | `./target/release/polyglot-backend` |
| **Output Directory** | *(not applicable — Rust binary at `target/release/`)* |
| **Health Check Path** | `/health` |
| **Auto-Deploy** | Yes — on push to `main` |

#### Root Directory

```
backend/rust-actix
```

Set this in **Settings → Build & Deploy → Root Directory**. All `build.sh` and `cargo` commands execute from this path.

#### Build Command

```bash
bash build.sh
```

`build.sh` performs the following in sequence:
1. `apt-get install -y build-essential g++ wget curl xz-utils ca-certificates`
2. Downloads and installs .NET 8 SDK via `dotnet-install.sh` to `/usr/local/dotnet`
3. Downloads Zig 0.13.0 tarball and installs to `/usr/local/zig`
4. Compiles `cpp/compute.cpp` → `libcompute.so` via `g++ -O2 -shared -fPIC`
5. `dotnet build csharp/Processor.csproj -c Release`
6. `cargo build --release`
7. Copies all language source files to `target/release/assets/`

#### Start Command

```bash
./target/release/polyglot-backend
```

#### Output Directory

Render does not require an explicit Output Directory for Rust services. The compiled binary is produced at `target/release/polyglot-backend` and referenced directly by the Start Command.

#### Environment Variables — Service 1

Configure these under **Environment → Environment Variables** in the Render dashboard:

| Variable Name | Value / Description | Required |
|--------------|---------------------|----------|
| `RUST_LOG` | `info` | Required |
| `PORT` | `8080` | Required |
| `PYTHON_FASTAPI_URL` | `https://python-fastapi.onrender.com` | Required |
| `ZIG_PATH` | `/usr/local/zig/zig` | Required |
| `DOTNET_PATH` | `/usr/local/dotnet/dotnet` | Required |
| `DOTNET_ROOT` | `/usr/local/dotnet` | Required |
| `MOJO_PATH` | `mojo` | Optional |
| `FSTAR_PATH` | `fstar.exe` | Optional |
| `DAFNY_PATH` | `dafny` | Optional |

> **Note on `PYTHON_FASTAPI_URL`:** Deploy the `python-fastapi` service first (Section 6.3), then copy its live URL here. When using Blueprints (`render.yaml`), Render resolves this automatically via the `fromService` reference.

---

### 6.3 Service 2 — Python 3 / FastAPI

#### Service Settings

| Field | Value |
|-------|-------|
| **Service Name** | `python-fastapi` |
| **Service Type** | Web Service |
| **Runtime** | Python 3 |
| **Region** | Oregon (US West) — match Service 1 |
| **Plan** | Free (dev) / Starter $7/mo (production) |
| **Root Directory** | `backend/python-fastapi` |
| **Build Command** | `pip install -r requirements.txt` |
| **Start Command** | `uvicorn main:app --host 0.0.0.0 --port $PORT` |
| **Output Directory** | *(not applicable — Python source served directly)* |
| **Health Check Path** | `/health` |
| **Auto-Deploy** | Yes — on push to `main` |

#### Root Directory

```
backend/python-fastapi
```

#### Build Command

```bash
pip install -r requirements.txt
```

Installs `fastapi==0.115.0`, `uvicorn[standard]==0.30.6`, `numpy==2.1.2`, `pydantic==2.9.2`, and `httpx==0.27.2`.

#### Start Command

```bash
uvicorn main:app --host 0.0.0.0 --port $PORT
```

`$PORT` is injected automatically by Render. The service exposes `/compute` (Fibonacci + Sieve of Eratosthenes) and `/health`.

#### Output Directory

Python services on Render do not require an Output Directory. The `main.py` module is served directly from the Root Directory.

#### Environment Variables — Service 2

| Variable Name | Value / Description | Required |
|--------------|---------------------|----------|
| `PORT` | Injected automatically by Render | Required |
| `PYTHONUNBUFFERED` | `1` | Recommended |
| `PYTHONDONTWRITEBYTECODE` | `1` | Recommended |

---

### 6.4 render.yaml Reference

```yaml
services:
  - type: web
    name: polyglot-backend
    runtime: rust
    rootDir: backend/rust-actix
    buildCommand: bash build.sh
    startCommand: ./target/release/polyglot-backend
    envVars:
      - key: RUST_LOG
        value: info
      - key: PORT
        value: "8080"
      - key: PYTHON_FASTAPI_URL
        fromService:
          name: python-fastapi
          type: web
          property: hostport
    healthCheckPath: /health

  - type: web
    name: python-fastapi
    runtime: python
    rootDir: backend/python-fastapi
    buildCommand: pip install -r requirements.txt
    startCommand: uvicorn main:app --host 0.0.0.0 --port $PORT
    envVars:
      - key: PORT
        value: "8001"
    healthCheckPath: /health
```

---

### 6.5 Verifying the Render Deployment

1. Wait for both services to reach **Running** status in the Render dashboard. The Rust service typically takes 5–12 minutes on first build.
2. Open `https://polyglot-backend.onrender.com/health` — expected response:
   ```json
   {
     "status": "healthy",
     "timestamp": "2025-01-01T00:00:00Z",
     "version": "0.1.0",
     "langs": ["Rust","C++","C#","Python3/FastAPI","Zig","Mojo","F*","Dafny"]
   }
   ```
3. Test `https://polyglot-backend.onrender.com/api/rust` for Rust computation.
4. Test `https://polyglot-backend.onrender.com/api/python` — proxies to the FastAPI service.
5. Test `https://polyglot-backend.onrender.com/api/all` — returns all language results in a single response.

---

## 7. Vercel Deployment — Frontend

Vercel hosts the Next.js frontend. The `vercel.json` at the repository root pre-configures all build settings. Frontend-to-backend communication is routed exclusively through Next.js API proxy routes to avoid CORS issues.

### 7.1 Connect Repository to Vercel

1. Navigate to [https://vercel.com](https://vercel.com) and sign in.
2. Click **Add New… → Project**.
3. Import `Polyglot-Microservices-Stack` from your Git provider.
4. Vercel detects `vercel.json` and pre-fills all configuration fields. Set `NEXT_PUBLIC_BACKEND_URL` under **Environment Variables** before clicking **Deploy**.

---

### 7.2 Project Configuration

| Field | Value |
|-------|-------|
| **Project Name** | `polyglot-microservices-stack` |
| **Framework Preset** | Next.js *(auto-detected)* |
| **Root Directory** | `frontend/nextjs` |
| **Install Command** | `npm install` |
| **Build Command** | `npm run build` |
| **Output Directory** | `.next` *(Next.js default — auto-resolved)* |
| **Start Command** | *(managed by Vercel — not applicable)* |
| **Node.js Version** | `20.x` *(set in Settings → General)* |
| **Auto-Deploy** | Yes — on push to `main` |

#### Root Directory

```
frontend/nextjs
```

Set this in the Vercel import UI under **Build and Output Settings → Root Directory**. This tells Vercel to treat `frontend/nextjs` as the project root, running all commands (`npm install`, `npm run build`) from that subdirectory.

#### Install Command

```bash
npm install
```

Runs from within the Root Directory. Installs all dependencies declared in `package.json`: `next`, `react`, `react-dom`, `sql.js`, `@php-wasm/node`, `axios`, `tailwindcss`, `autoprefixer`, and `postcss`.

#### Build Command

```bash
npm run build
```

Equivalent to `next build`. Compiles the Next.js application for production: TailwindCSS purge, WASM module bundling (`asyncWebAssembly: true` in `next.config.js`), and SSR/static pre-rendering.

#### Output Directory

```
.next
```

Vercel resolves this automatically for Next.js projects. No manual configuration is required when the Framework Preset is set to Next.js. The `.next` directory contains server bundles, client chunks, static assets, and API route handlers.

#### Start Command

Vercel manages the production server internally for Next.js projects. **No Start Command is configured.** Vercel serves the `.next` build using its own edge runtime and serverless function infrastructure. The local equivalent is `npm run start` (`next start`), used only for local production testing.

---

### 7.3 Environment Variables

Set these in the Vercel dashboard under **Settings → Environment Variables**. Apply each to **Production**, **Preview**, and **Development** environments unless otherwise noted.

| Variable Name | Value / Description | Environment | Required |
|--------------|---------------------|-------------|----------|
| `NEXT_PUBLIC_BACKEND_URL` | `https://polyglot-backend.onrender.com` | Production | Required |
| `NEXT_PUBLIC_BACKEND_URL` | `http://localhost:8080` | Development | Required |
| `NEXT_TELEMETRY_DISABLED` | `1` | All | Recommended |
| `NODE_ENV` | `production` | Production | Auto-set by Vercel |

> **Important — `NEXT_PUBLIC_` prefix:** Variables prefixed with `NEXT_PUBLIC_` are embedded into the client-side JavaScript bundle at build time. `NEXT_PUBLIC_BACKEND_URL` must carry this prefix so HTMX `hx-get` attributes and Alpine.js fetch calls can read it in the browser. Adding or modifying this variable after deployment requires a redeployment to take effect.

---

### 7.4 vercel.json Reference

```json
{
  "framework": "nextjs",
  "buildCommand": "cd frontend/nextjs && npm run build",
  "outputDirectory": "frontend/nextjs/.next",
  "installCommand": "cd frontend/nextjs && npm install",
  "env": {
    "NEXT_PUBLIC_BACKEND_URL": "https://polyglot-backend.onrender.com"
  },
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        { "key": "X-Content-Type-Options", "value": "nosniff" },
        { "key": "X-Frame-Options",        "value": "DENY"    },
        { "key": "Referrer-Policy",        "value": "strict-origin-when-cross-origin" }
      ]
    }
  ]
}
```

> **Note:** `vercel.json` takes precedence over settings in the Vercel project UI. To make permanent configuration changes, edit `vercel.json` in the repository and push to `main`.

---

### 7.5 Verifying the Vercel Deployment

1. Monitor the deployment log in the Vercel dashboard — build typically completes in 60–120 seconds.
2. Open the assigned Vercel URL (e.g., `https://polyglot-microservices-stack.vercel.app`).
3. Click **HTMX → /health** — the health panel should populate with JSON from the Render backend.
4. Click **Run All Languages** — all eight language cards should return results within 30 seconds.
5. Click **Execute SQL Query** — SQL.js WASM should return `{ "rows": [{ "answer": 42 }] }`.
6. Click **Run PHP-WASM** — PHP-WASM should return output from the PHP Fibonacci script with `exit_code: 0`.

---

## 8. Local Development

### Step 1 — Clone the Repository

```bash
git clone https://github.com/<your-org>/Polyglot-Microservices-Stack.git
cd Polyglot-Microservices-Stack
```

### Step 2 — Run the Python FastAPI Service

```bash
cd backend/python-fastapi
python -m venv .venv
source .venv/bin/activate      # Windows: .venv\Scripts\activate
pip install -r requirements.txt
uvicorn main:app --host 0.0.0.0 --port 8001 --reload
# Available at: http://localhost:8001
```

### Step 3 — Run the Rust/Actix Backend

```bash
cd backend/rust-actix
cp .env.example .env
# Edit .env: set PYTHON_FASTAPI_URL=http://localhost:8001
cargo run
# Available at: http://localhost:8080
```

### Step 4 — Run the Next.js Frontend

```bash
cd frontend/nextjs
npm install
echo "NEXT_PUBLIC_BACKEND_URL=http://localhost:8080" > .env.local
npm run dev
# Available at: http://localhost:3000
```

---

## 9. API Reference

All backend endpoints return JSON matching the `LangResponse` schema:

```json
{
  "language":    "string",
  "result":      "string",
  "duration_ms": 0,
  "status":      "ok | error"
}
```

| Method | Endpoint | Handler | Description |
|--------|---------|---------|-------------|
| `GET` | `/health` | Rust (native) | Returns version, timestamp, and supported language list |
| `GET` | `/api/rust` | Rust (native) | Fibonacci + prime sieve in native Rust |
| `GET` | `/api/cpp` | C++ subprocess | `g++` compiles and executes `cpp/compute.cpp` |
| `GET` | `/api/csharp` | C# subprocess | `dotnet run` executes `csharp/Processor.cs` |
| `GET` | `/api/python` | HTTP → FastAPI | `reqwest` proxies to `python-fastapi /compute` |
| `GET` | `/api/zig` | Zig subprocess | `zig run` executes `zig/compute.zig` |
| `GET` | `/api/mojo` | Mojo subprocess | `mojo run` executes `mojo/compute.mojo` |
| `GET` | `/api/fstar` | F* subprocess | `fstar.exe --admit_smt_queries true fstar/Verify.fst` |
| `GET` | `/api/dafny` | Dafny subprocess | `dafny verify dafny/Verify.dfy` |
| `GET` | `/api/all` | Aggregator | Dispatches to all handlers; returns array of `LangResponse` |

### Frontend API Routes (Next.js)

| Method | Endpoint | Description |
|--------|---------|-------------|
| `GET/POST` | `/api/proxy/[...path]` | Transparent proxy to `NEXT_PUBLIC_BACKEND_URL/api/[path]` |
| `POST` | `/api/sqljs` | Executes SQL via `sql.js` (SQLite WASM) in Node.js |
| `POST` | `/api/phpjs` | Executes PHP code via `@php-wasm/node` (PHP WASM) in Node.js |

---

## 10. Mermaid Diagrams

Render these `.mmd` files at [https://mermaid.live](https://mermaid.live) or with the VS Code Mermaid Preview extension. GitHub renders them natively when the file extension is `.mmd` and a compatible viewer is used.

| File | Diagram Type | Content |
|------|-------------|---------|
| `README.mmd` | `flowchart TD` | Vercel/Render stack overview with language embeddings |
| `ARCHITECTURE.mmd` | `C4Context` | C4 model: user → Vercel → Render → SQLite |
| `SEQUENCE.mmd` | `sequenceDiagram` | Full request lifecycle for each language path |
| `FLOWCHART.mmd` | `flowchart LR` | Backend route dispatch: each `/api/*` route to its handler |
| `DEPLOY.mmd` | `flowchart TD` | CI/CD: git push → Render Blueprint + Vercel auto-deploy |

---

## 11. Optional Language Setup

Mojo, F\*, and Dafny are not installed by `build.sh` by default due to their large installation footprint on Render's free plan build instances. Their API handlers return informative error messages when the binaries are absent, leaving all other endpoints fully operational.

### Mojo (Modular Magic SDK)

Add to `build.sh` before `cargo build --release`:

```bash
curl -ssL https://magic.modular.com | bash
export PATH="$HOME/.modular/bin:$PATH"
magic install mojo
export MOJO_PATH="$HOME/.modular/bin/mojo"
```

Then add `MOJO_PATH` to the Render environment variables for `polyglot-backend`.

### F* (Proof Assistant via OPAM)

```bash
apt-get install -y opam
opam init --disable-sandboxing -y
opam install fstar -y
eval $(opam env)
export FSTAR_PATH="$(which fstar.exe)"
```

### Dafny (via .NET Global Tool)

Requires .NET 8, which is already installed by `build.sh`:

```bash
dotnet tool install --global dafny
export PATH="$HOME/.dotnet/tools:$PATH"
export DAFNY_PATH="dafny"
```

---

## 12. Troubleshooting

### Render — Common Issues

**`cargo: command not found` during build.**
Verify that the Service Runtime is set to **Rust** in the Render dashboard, not Node or Python. Render only installs the Rust toolchain for Rust-runtime services.

**C++ handler returns `g++ not found`.**
Confirm `build.sh` includes `apt-get install -y build-essential g++` and that this line runs before `cargo build --release`. Review the Render build log for `apt-get` errors.

**Python handler returns `HTTP error: connection refused`.**
The `PYTHON_FASTAPI_URL` environment variable points to an unreachable address. Confirm the `python-fastapi` service is running (green status) and that `PYTHON_FASTAPI_URL` is set to its exact Render URL, not `localhost`.

**Cold starts take 30–60 seconds (Free tier).**
Expected behavior on Render's free tier. Upgrade to Starter plan or configure an uptime monitor (e.g., UptimeRobot) to ping `/health` every 14 minutes.

### Vercel — Common Issues

**`Cannot find module 'sql.js'` during build.**
Verify Root Directory is `frontend/nextjs` and `sql.js` is listed in `dependencies` (not `devDependencies`) in `package.json`.

**HTMX requests fail with a CORS error in the browser.**
HTMX `hx-get` attributes in `index.js` point directly to the Render backend. The Rust service configures `allow_any_origin()` via `actix-cors`, so CORS should be permitted. If errors persist, route HTMX requests through the Next.js proxy (`/api/proxy/*`) instead.

**`NEXT_PUBLIC_BACKEND_URL` is `undefined` at runtime.**
`NEXT_PUBLIC_` variables are embedded at build time. Setting them after deployment has no effect. Add the variable in Vercel and trigger a new deployment from the dashboard.

**`@php-wasm/node` fails to import.**
Set Node.js version to `20.x` in **Vercel → Settings → General → Node.js Version**. The package bundles a PHP WebAssembly binary that requires a compatible runtime.

---

## 13. Quick Reference Card

### Render — `polyglot-backend` (Rust/Actix)

| Setting | Value |
|---------|-------|
| Root Directory | `backend/rust-actix` |
| Build Command | `bash build.sh` |
| Start Command | `./target/release/polyglot-backend` |
| Output Directory | *(not applicable)* |
| Health Check Path | `/health` |
| `RUST_LOG` | `info` |
| `PORT` | `8080` |
| `PYTHON_FASTAPI_URL` | `https://python-fastapi.onrender.com` |
| `ZIG_PATH` | `/usr/local/zig/zig` |
| `DOTNET_PATH` | `/usr/local/dotnet/dotnet` |

### Render — `python-fastapi` (Python 3)

| Setting | Value |
|---------|-------|
| Root Directory | `backend/python-fastapi` |
| Build Command | `pip install -r requirements.txt` |
| Start Command | `uvicorn main:app --host 0.0.0.0 --port $PORT` |
| Output Directory | *(not applicable)* |
| Health Check Path | `/health` |
| `PYTHONUNBUFFERED` | `1` |

### Vercel — `polyglot-microservices-stack` (Next.js)

| Setting | Value |
|---------|-------|
| Framework Preset | Next.js |
| Root Directory | `frontend/nextjs` |
| Install Command | `npm install` |
| Build Command | `npm run build` |
| Output Directory | `.next` |
| Start Command | *(managed by Vercel)* |
| Node.js Version | `20.x` |
| `NEXT_PUBLIC_BACKEND_URL` | `https://polyglot-backend.onrender.com` |
| `NEXT_TELEMETRY_DISABLED` | `1` |

---

## 14. License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

---

*Polyglot-Microservices-Stack · v1.0.0 · Render (Rust/Actix) × Vercel (Next.js) · No Dockerfile*
