export default async function handler(req, res) {
  const { path } = req.query;
  const route      = Array.isArray(path) ? path.join("/") : path;
  const backendUrl = process.env.NEXT_PUBLIC_BACKEND_URL || "http://localhost:8080";
  const url        = `${backendUrl}/api/${route}`;

  try {
    const upstream = await fetch(url, {
      method:  req.method,
      headers: { "Content-Type": "application/json" },
      body:
        req.method !== "GET" && req.method !== "HEAD"
          ? JSON.stringify(req.body)
          : undefined,
    });

    const data = await upstream.json();
    res.status(upstream.status).json(data);
  } catch (error) {
    res.status(502).json({
      status:   "error",
      result:   `Proxy error: ${error.message}`,
      language: route,
    });
  }
}
