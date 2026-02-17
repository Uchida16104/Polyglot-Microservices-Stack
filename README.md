# Polyglot Microservices Stack

Production-grade polyglot architecture with Laravel 11 (Auth/Gateway), Rust/Axum (Core Compile/Execute), C# ASP.NET Core 8 (Business Logic), Python FastAPI (ML/Data), Python FastAPI (Research Runtimes: Zig, Mojo, Dafny, F*), and SvelteKit + Tailwind (Frontend on Vercel).

---

## Service Architecture

| Service | Language / Framework | Port | Responsibility |
|---|---|---|---|
| `laravel-gateway` | PHP 8.3 / Laravel 11 | 8000 | Auth, JWT sessions, API gateway proxy |
| `rust-core` | Rust / Axum 0.7 | 8001 | Compile/execute pipeline, sandboxed jobs |
| `csharp-business` | C# 12 / ASP.NET Core 8 | 8002 | Projects, tasks, Kanban state machine |
| `python-ml` | Python 3.12 / FastAPI | 8003 | ML model registry, training/inference jobs |
| `research-runtimes` | Python 3.12 / FastAPI | 8004 | Zig, Mojo, Dafny, F* compile and verification |
| `frontend` | TypeScript / SvelteKit | 3000 | SSR frontend deployed on Vercel |

**Database:** CockroachDB Serverless (Postgres-compatible, free tier)
**Cache and Queues:** Redis (Render managed or Upstash)
**Frontend Hosting:** Vercel
**Services Hosting:** Render (Docker, free tier)
**CI/CD:** GitHub Actions

---

## Research Runtimes — Zig, Mojo, Dafny, F*

The `research-runtimes` service exposes a unified compile/execute endpoint that dispatches to four specialized toolchains installed in its Docker image.

- **Zig** — Systems programming. Compiled via `zig build-exe`, then executed as a native binary.
- **Mojo** — AI-accelerated Python superset by Modular. Executed directly via `mojo run`.
- **Dafny** — Formal verification language. Runs `dafny verify` first to check all proof obligations, then `dafny run` to execute.
- **F*** — Proof assistant and dependently-typed programming language. Runs SMT-based verification via `fstar.exe`.

All four runtimes share the `research_jobs` table in CockroachDB and the `research_queue` Redis list. The state machine and retry logic (max 3 retries, 60-second timeout) are identical to compile and ML jobs.

The gateway exposes these runtimes under `/api/research/compile`.

---

## Database Migrations

Apply migrations in order to your CockroachDB Serverless cluster:

```
001_users_auth.sql         — users, sessions, oauth_tokens
002_projects_tasks.sql     — projects, project_members, tasks, files
003_compile_execute_jobs.sql — compile_jobs, execute_jobs
004_ml_audit.sql           — ml_models, ml_jobs, audit_logs
005_research_jobs.sql      — research_jobs (Zig/Mojo/Dafny/F*)
```

```bash
export DATABASE_URL="postgresql://user:pass@cluster.crdb.io:26257/polyglot?sslmode=verify-full"
for f in migrations/*.sql; do
  echo "Applying $f"
  cockroach sql --url "$DATABASE_URL" < "$f"
done
```

---

## Local Development

```bash
cp services/laravel-gateway/.env.example services/laravel-gateway/.env
docker compose up --build
```

Services are available at:

- Frontend: http://localhost:3000
- Laravel Gateway: http://localhost:8000
- Rust Core: http://localhost:8001
- C# Business: http://localhost:8002
- Python ML: http://localhost:8003
- Research Runtimes: http://localhost:8004
- CockroachDB Admin UI: http://localhost:8080

---

## Deployment to Render + Vercel

1. Create a CockroachDB Serverless cluster and apply all five migration files.
2. Connect your GitHub repository to Render and create a Blueprint instance from `render.yaml`.
3. Set the required secrets in each service (DB connection strings, APP_KEY for Laravel).
4. Deploy the frontend with `npx vercel --prod` from `services/frontend/`.
5. Add GitHub Actions secrets for CI/CD: `VERCEL_TOKEN`, `VERCEL_ORG_ID`, `VERCEL_PROJECT_ID`, `RENDER_DEPLOY_HOOK_LARAVEL`, `RENDER_DEPLOY_HOOK_RUST`, `RENDER_DEPLOY_HOOK_CSHARP`, `RENDER_DEPLOY_HOOK_PYTHON`, `RENDER_DEPLOY_HOOK_RESEARCH`.

## Developer
***Hirotoshi Uchida***

---

## Mermaid Architecture Diagrams

Open `mermaid-diagrams.html` in any modern browser to view all nine interactive diagrams: ER diagram, architecture topology, auth sequence, compile-execute sequence, research runtimes sequence, ML job sequence, task state machine, job state machine, and deployment topology.
