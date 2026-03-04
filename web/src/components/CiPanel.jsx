const statusConfig = {
  Success: { icon: '✓', color: 'text-green-400', bg: 'bg-green-400/10' },
  Failure: { icon: '✗', color: 'text-red-400', bg: 'bg-red-400/10' },
  Running: { icon: '●', color: 'text-yellow-400', bg: 'bg-yellow-400/10' },
  Pending: { icon: '○', color: 'text-gray-500', bg: 'bg-gray-500/10' },
  Cancelled: { icon: '—', color: 'text-gray-500', bg: 'bg-gray-500/10' },
}

function CiPanel({ data, active }) {
  const borderColor = active ? 'border-cyan-700 shadow-cyan-900/20 shadow-lg' : 'border-gray-800'

  return (
    <div className={`rounded-lg border ${borderColor} bg-gray-900 p-4 overflow-hidden flex flex-col`}>
      <h2 className="text-sm font-semibold text-gray-400 mb-3 flex items-center gap-2">
        <span className="text-cyan-400">●</span> CI/CD
      </h2>

      {!data ? (
        <p className="text-gray-600 text-sm">Fetching CI/CD data...</p>
      ) : (
        <div className="flex-1 overflow-auto space-y-2">
          {data.runs?.map((run, i) => {
            const cfg = statusConfig[run.status] || statusConfig.Pending
            return (
              <div key={i} className={`flex items-center gap-3 p-2 rounded ${cfg.bg}`}>
                <span className={`${cfg.color} text-lg w-6 text-center`}>{cfg.icon}</span>
                <div className="flex-1 min-w-0">
                  <div className="text-sm text-gray-200 truncate">{run.name}</div>
                  <div className="text-xs text-blue-400">{run.branch}</div>
                </div>
                <span className="text-xs text-gray-500">
                  {run.duration_secs ? `${run.duration_secs}s` : '—'}
                </span>
              </div>
            )
          })}
          {(!data.runs || data.runs.length === 0) && (
            <p className="text-gray-600 text-sm">No pipeline runs found</p>
          )}
        </div>
      )}
    </div>
  )
}

export default CiPanel
