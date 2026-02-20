const SQL_WASM_CDN =
  "https://cdnjs.cloudflare.com/ajax/libs/sql.js/1.12.0/sql-wasm.wasm";

let SQLInstance = null;

async function getSQL() {
  if (SQLInstance) return SQLInstance;
  const initSqlJs = (await import("sql.js")).default;
  SQLInstance = await initSqlJs({
    locateFile: () => SQL_WASM_CDN,
  });
  return SQLInstance;
}

export default async function handler(req, res) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method Not Allowed" });
  }

  const { query } = req.body || {};
  if (!query || typeof query !== "string") {
    return res.status(400).json({ error: "Body must contain a 'query' string" });
  }

  try {
    const SqlJs = await getSQL();
    const db    = new SqlJs.Database();

    db.run(`
      CREATE TABLE IF NOT EXISTS compute_results (
        id         INTEGER PRIMARY KEY AUTOINCREMENT,
        language   TEXT    NOT NULL,
        result     TEXT    NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `);

    db.run("INSERT INTO compute_results (language, result) VALUES (?, ?)", [
      "SQL.js",
      "SQLite WASM running in Node.js via sql.js",
    ]);

    const stmt = db.prepare(query);
    const rows = [];
    while (stmt.step()) {
      rows.push(stmt.getAsObject());
    }
    stmt.free();

    const countStmt = db.prepare("SELECT COUNT(*) AS total FROM compute_results");
    countStmt.step();
    const { total } = countStmt.getAsObject();
    countStmt.free();
    db.close();

    return res.status(200).json({
      engine:              "sql.js (SQLite WebAssembly via CDN WASM)",
      query,
      rows,
      total_rows_in_table: total,
    });
  } catch (error) {
    return res.status(500).json({ error: error.message });
  }
}
