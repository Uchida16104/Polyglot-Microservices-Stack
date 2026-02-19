export default async function handler(req, res) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method Not Allowed — use POST" });
  }

  const { code } = req.body || {};
  if (!code || typeof code !== "string") {
    return res.status(400).json({ error: "Body must contain a 'code' string" });
  }

  try {
    const { PhpNode } = await import("@php-wasm/node");

    const php = await PhpNode.create({
      requestHandler: {
        documentRoot: "/",
        absoluteUrl:  "http://localhost",
      },
    });

    const result = await php.run({ code });

    return res.status(200).json({
      engine:    "@php-wasm/node",
      output:    result.text,
      exit_code: result.exitCode,
    });
  } catch (error) {
    return res.status(500).json({
      engine: "@php-wasm/node",
      error:  error.message,
      hint:   "Ensure @php-wasm/node is installed via: npm install @php-wasm/node",
    });
  }
}
