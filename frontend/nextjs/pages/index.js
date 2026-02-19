import Head from "next/head";
import { useState, useCallback } from "react";
import LanguageCard from "../components/LanguageCard";

const BACKEND = process.env.NEXT_PUBLIC_BACKEND_URL || "http://localhost:8080";

const LANGS = [
  { key: "rust",   label: "Rust",           color: "text-orange-400", emoji: "🦀" },
  { key: "cpp",    label: "C++",            color: "text-blue-400",   emoji: "⚙️" },
  { key: "csharp", label: "C#",             color: "text-purple-400", emoji: "🔷" },
  { key: "python", label: "Python3/FastAPI",color: "text-yellow-400", emoji: "🐍" },
  { key: "zig",    label: "Zig",            color: "text-amber-400",  emoji: "⚡" },
  { key: "mojo",   label: "Mojo",           color: "text-red-400",    emoji: "🔥" },
  { key: "fstar",  label: "F*",             color: "text-green-400",  emoji: "✅" },
  { key: "dafny",  label: "Dafny",          color: "text-teal-400",   emoji: "🔐" },
];

export default function Home() {
  const [results,    setResults]    = useState({});
  const [loading,    setLoading]    = useState({});
  const [sqlResult,  setSqlResult]  = useState(null);
  const [phpResult,  setPhpResult]  = useState(null);
  const [sqlLoading, setSqlLoading] = useState(false);
  const [phpLoading, setPhpLoading] = useState(false);

  const callLang = useCallback(async (key) => {
    setLoading((p) => ({ ...p, [key]: true }));
    try {
      const r    = await fetch(`/api/proxy/${key}`);
      const data = await r.json();
      setResults((p) => ({ ...p, [key]: data }));
    } catch (e) {
      setResults((p) => ({ ...p, [key]: { status: "error", result: String(e) } }));
    } finally {
      setLoading((p) => ({ ...p, [key]: false }));
    }
  }, []);

  const callAll = () => LANGS.forEach(({ key }) => callLang(key));

  const runSql = async () => {
    setSqlLoading(true);
    try {
      const r = await fetch("/api/sqljs", {
        method:  "POST",
        headers: { "Content-Type": "application/json" },
        body:    JSON.stringify({ query: "SELECT 42 AS answer, 'SQL.js WASM' AS engine" }),
      });
      setSqlResult(await r.json());
    } catch (e) {
      setSqlResult({ error: String(e) });
    } finally {
      setSqlLoading(false);
    }
  };

  const runPhp = async () => {
    setPhpLoading(true);
    try {
      const r = await fetch("/api/phpjs", {
        method:  "POST",
        headers: { "Content-Type": "application/json" },
        body:    JSON.stringify({
          code: '<?php $n=15; $a=0; $b=1; for($i=2;$i<=$n;$i++){$c=$a+$b;$a=$b;$b=$c;} echo "PHP-WASM | fib(15)=$b\\n";',
        }),
      });
      setPhpResult(await r.json());
    } catch (e) {
      setPhpResult({ error: String(e) });
    } finally {
      setPhpLoading(false);
    }
  };

  return (
    <>
      <Head><title>Polyglot Web Service</title></Head>

      <header className="sticky top-0 z-10 border-b border-slate-800 bg-slate-950/90 backdrop-blur px-6 py-4 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">🌐 Polyglot Web Service</h1>
          <p className="text-slate-400 text-sm mt-0.5">
            Render (Rust·C++·C#·Python·Zig·Mojo·F*·Dafny) × Vercel (Next.js·HTMX·Alpine·Hyperscript·SQL.js·PHP-WASM)
          </p>
        </div>
        <span className="code-badge">v1.0.0</span>
      </header>

      <main className="max-w-6xl mx-auto px-6 py-10 space-y-14">

        <section>
          <h2 className="section-title">
            <span className="code-badge">HTMX</span> Live Backend Poll
          </h2>
          <div className="card space-y-4">
            <p className="text-slate-400 text-sm">
              Fetches <code className="code-badge">/health</code> from the Rust/Actix backend
              via HTMX — declarative, no JavaScript logic required.
            </p>
            <div className="flex items-center gap-4 flex-wrap">
              <button
                className="btn-primary"
                hx-get={`${BACKEND}/health`}
                hx-target="#htmx-result"
                hx-swap="innerHTML"
                hx-indicator="#htmx-spinner"
              >
                HTMX → /health
              </button>
              <span
                id="htmx-spinner"
                className="htmx-indicator text-brand-500 text-sm animate-pulse"
              >
                Fetching…
              </span>
            </div>
            <pre
              id="htmx-result"
              className="bg-slate-900 rounded-lg p-3 text-xs text-green-400 min-h-[2.5rem] overflow-auto"
            />
          </div>
        </section>

        <section>
          <h2 className="section-title">
            <span className="code-badge">Alpine.js</span> Reactive Counter
          </h2>
          <div
            className="card"
            x-data="{ count: 0, history: [] }"
          >
            <p className="text-slate-400 text-sm mb-4">
              Alpine.js manages reactive state entirely in-HTML — no framework boilerplate.
            </p>
            <div className="flex items-center gap-3 flex-wrap">
              <button
                className="btn-primary"
                x-on:click="count++; history.push(count)"
              >
                Increment
              </button>
              <button
                className="btn-outline"
                x-on:click="count = 0; history = []"
              >
                Reset
              </button>
              <span className="text-brand-500 font-mono text-2xl font-bold" x-text="count" />
            </div>
            <p className="text-slate-500 text-xs mt-3">
              History: <span className="font-mono text-slate-400" x-text="history.join(', ') || '—'" />
            </p>
          </div>
        </section>

        <section>
          <h2 className="section-title">
            <span className="code-badge">Hyperscript</span> DOM Event Magic
          </h2>
          <div className="card space-y-4">
            <p className="text-slate-400 text-sm">
              Hyperscript handles DOM manipulation with human-readable inline scripts.
            </p>
            <div className="flex gap-3 flex-wrap">
              <button
                className="btn-outline"
                _="on click toggle .hidden on #hs-panel
                   then put 'Panel toggled by Hyperscript ✓' into #hs-msg"
              >
                Toggle Panel
              </button>
              <button
                className="btn-outline"
                _="on click add .text-brand-500 to #hs-msg
                   then wait 1s then remove .text-brand-500 from #hs-msg"
              >
                Flash Message
              </button>
            </div>
            <div
              id="hs-panel"
              className="bg-slate-900 rounded-lg p-4 text-green-400 text-sm border border-slate-700"
            >
              Hyperscript-controlled panel. Toggle it above.
            </div>
            <p id="hs-msg" className="text-xs font-mono text-slate-500 min-h-[1rem]" />
          </div>
        </section>

        <section>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-semibold">Backend Language Results</h2>
            <button className="btn-primary text-sm" onClick={callAll}>
              ▶ Run All
            </button>
          </div>
          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
            {LANGS.map(({ key, label, color, emoji }) => (
              <LanguageCard
                key={key}
                language={label}
                emoji={emoji}
                color={color}
                result={results[key]?.result}
                durationMs={results[key]?.duration_ms}
                status={results[key]?.status}
                loading={!!loading[key]}
                onRun={() => callLang(key)}
              />
            ))}
          </div>
        </section>

        <section>
          <h2 className="section-title">
            <span className="code-badge">SQL.js</span> WebAssembly SQLite
          </h2>
          <div className="card space-y-4">
            <p className="text-slate-400 text-sm">
              SQLite compiled to WASM via <code className="code-badge">sql.js</code> — runs in Node.js
              inside a Next.js API route with no native binaries.
            </p>
            <button
              className="btn-primary"
              onClick={runSql}
              disabled={sqlLoading}
            >
              {sqlLoading ? "Querying…" : "Execute SQL Query"}
            </button>
            {sqlResult && (
              <pre className="bg-slate-900 rounded p-3 text-xs text-green-400 overflow-auto">
                {JSON.stringify(sqlResult, null, 2)}
              </pre>
            )}
          </div>
        </section>

        <section>
          <h2 className="section-title">
            <span className="code-badge">PHP-WASM</span> PHP in WebAssembly
          </h2>
          <div className="card space-y-4">
            <p className="text-slate-400 text-sm">
              PHP compiled to WebAssembly via{" "}
              <code className="code-badge">@php-wasm/node</code> — executes PHP code
              server-side in a Next.js API route.
            </p>
            <button
              className="btn-primary"
              onClick={runPhp}
              disabled={phpLoading}
            >
              {phpLoading ? "Executing PHP…" : "Run PHP-WASM"}
            </button>
            {phpResult && (
              <pre className="bg-slate-900 rounded p-3 text-xs text-green-400 overflow-auto">
                {JSON.stringify(phpResult, null, 2)}
              </pre>
            )}
          </div>
        </section>

      </main>

      <footer className="border-t border-slate-800 px-6 py-6 text-center text-slate-500 text-sm space-y-1">
        <p>Render (Rust/Actix) · Vercel (Next.js) · No Dockerfile</p>
        <p className="text-xs text-slate-600">
          C++ · C# · Python3/FastAPI · Zig · Mojo · F* · Dafny · SQL.js · PHP-WASM
        </p>
      </footer>
    </>
  );
}
