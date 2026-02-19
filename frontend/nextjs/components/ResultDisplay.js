export default function ResultDisplay({ data, loading }) {
  if (loading) {
    return (
      <div className="flex items-center gap-2 text-brand-500 text-sm">
        <svg
          className="animate-spin h-4 w-4"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle
            cx="12" cy="12" r="10"
            stroke="currentColor" strokeWidth="4"
            className="opacity-25"
          />
          <path
            fill="currentColor" className="opacity-75"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
          />
        </svg>
        Executing…
      </div>
    );
  }
  if (!data) return null;
  return (
    <pre className="bg-slate-900 rounded-lg p-3 text-xs text-green-400 overflow-auto whitespace-pre-wrap">
      {typeof data === "string" ? data : JSON.stringify(data, null, 2)}
    </pre>
  );
}
