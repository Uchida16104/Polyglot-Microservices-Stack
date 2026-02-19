export default function LanguageCard({ language, emoji, color, result, durationMs, status, loading, onRun }) {
  const borderColor =
    status === "error" ? "border-red-500" : status === "ok" ? "border-green-600" : "";

  return (
    <div className={`card flex flex-col gap-3 ${borderColor}`}>
      <div className="flex items-center justify-between">
        <span className={`font-semibold text-sm ${color}`}>{emoji} {language}</span>
        {durationMs !== undefined && (
          <span className="text-slate-500 text-xs">{durationMs}ms</span>
        )}
      </div>
      <pre className="text-xs bg-slate-900 rounded p-2 overflow-auto min-h-[3.5rem] text-slate-300 whitespace-pre-wrap leading-relaxed">
        {loading ? "Executing..." : result || "-"}
      </pre>
      <button className="btn-outline text-xs mt-auto" onClick={onRun} disabled={loading}>
        {loading ? "Running..." : `Run ${language}`}
      </button>
    </div>
  );
}
