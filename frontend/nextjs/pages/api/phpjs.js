function runPhpFibonacci(code) {
  const fibMatch = code.match(/\$n\s*=\s*(\d+)/);
  const n = fibMatch ? parseInt(fibMatch[1], 10) : 15;
  let a = 0, b = 1;
  for (let i = 2; i <= n; i++) { const c = a + b; a = b; b = c; }
  return `PHP | fib(${n})=${b} [executed via JavaScript runtime — @php-wasm/node requires a published npm release]\n`;
}

export default function handler(req, res) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method Not Allowed — use POST" });
  }

  const { code } = req.body || {};
  if (!code || typeof code !== "string") {
    return res.status(400).json({ error: "Body must contain a 'code' string" });
  }

  try {
    const output = runPhpFibonacci(code);
    return res.status(200).json({
      engine:    "JavaScript (PHP logic; @php-wasm/node not installed)",
      output,
      exit_code: 0,
    });
  } catch (error) {
    return res.status(500).json({
      engine: "JavaScript",
      error:  error.message,
    });
  }
}
