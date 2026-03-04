function QualityPanel({ data, active }) {
  const borderColor = active ? 'border-cyan-700 shadow-cyan-900/20 shadow-lg' : 'border-gray-800'

  const coverageColor = (v) => {
    if (v >= 80) return 'bg-green-500'
    if (v >= 60) return 'bg-yellow-500'
    return 'bg-red-500'
  }

  const coverageTextColor = (v) => {
    if (v >= 80) return 'text-green-400'
    if (v >= 60) return 'text-yellow-400'
    return 'text-red-400'
  }

  return (
    <div className={`rounded-lg border ${borderColor} bg-gray-900 p-4 overflow-hidden flex flex-col`}>
      <h2 className="text-sm font-semibold text-gray-400 mb-3 flex items-center gap-2">
        <span className="text-cyan-400">●</span> Quality
      </h2>

      {!data ? (
        <p className="text-gray-600 text-sm">Running quality checks...</p>
      ) : (
        <div className="space-y-4">
          {/* Coverage bar */}
          <div>
            <div className="flex justify-between items-center mb-1">
              <span className="text-xs text-gray-500">Test Coverage</span>
              <span className={`text-sm font-bold ${coverageTextColor(data.test_coverage)}`}>
                {data.test_coverage.toFixed(1)}%
              </span>
            </div>
            <div className="w-full bg-gray-800 rounded-full h-3">
              <div
                className={`h-3 rounded-full transition-all duration-500 ${coverageColor(data.test_coverage)}`}
                style={{ width: `${Math.min(data.test_coverage, 100)}%` }}
              />
            </div>
          </div>

          {/* Metrics */}
          <div className="grid grid-cols-3 gap-3">
            <div className="bg-gray-800/50 rounded-lg p-3 text-center">
              <div className={`text-2xl font-bold ${data.lint_warnings > 0 ? 'text-yellow-400' : 'text-green-400'}`}>
                {data.lint_warnings}
              </div>
              <div className="text-xs text-gray-500 mt-1">Warnings</div>
            </div>
            <div className="bg-gray-800/50 rounded-lg p-3 text-center">
              <div className={`text-2xl font-bold ${data.lint_errors > 0 ? 'text-red-400' : 'text-green-400'}`}>
                {data.lint_errors}
              </div>
              <div className="text-xs text-gray-500 mt-1">Errors</div>
            </div>
            <div className="bg-gray-800/50 rounded-lg p-3 text-center">
              <div className={`text-2xl font-bold ${data.security_issues > 0 ? 'text-red-400' : 'text-green-400'}`}>
                {data.security_issues}
              </div>
              <div className="text-xs text-gray-500 mt-1">Security</div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default QualityPanel
