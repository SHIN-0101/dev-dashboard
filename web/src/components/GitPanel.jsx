function GitPanel({ data, active }) {
  const borderColor = active ? 'border-cyan-700 shadow-cyan-900/20 shadow-lg' : 'border-gray-800'

  return (
    <div className={`rounded-lg border ${borderColor} bg-gray-900 p-4 overflow-hidden flex flex-col`}>
      <h2 className="text-sm font-semibold text-gray-400 mb-3 flex items-center gap-2">
        <span className="text-cyan-400">●</span> Git
      </h2>

      {!data ? (
        <p className="text-gray-600 text-sm">Fetching git data...</p>
      ) : (
        <>
          <div className="flex items-center gap-3 mb-3 text-sm">
            <span className="text-gray-500">branch:</span>
            <span className="text-green-400 font-bold">{data.branch}</span>
            <span className="text-yellow-400 text-xs">
              {data.changed_files}M {data.staged_files}S
            </span>
          </div>

          <div className="flex-1 overflow-auto">
            <table className="w-full text-xs">
              <thead>
                <tr className="text-gray-600 border-b border-gray-800">
                  <th className="text-left py-1 pr-3 w-20">Hash</th>
                  <th className="text-left py-1 pr-3">Message</th>
                  <th className="text-left py-1 w-24">Author</th>
                </tr>
              </thead>
              <tbody>
                {data.commits?.slice(0, 15).map((c, i) => (
                  <tr key={i} className="border-b border-gray-800/50 hover:bg-gray-800/30">
                    <td className="py-1 pr-3 text-yellow-400 font-mono">{c.hash}</td>
                    <td className="py-1 pr-3 text-gray-300 truncate max-w-[200px]">{c.message}</td>
                    <td className="py-1 text-blue-400">{c.author}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </>
      )}
    </div>
  )
}

export default GitPanel
