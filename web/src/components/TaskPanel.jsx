const statusConfig = {
  Todo: { icon: '○', color: 'text-gray-500' },
  InProgress: { icon: '●', color: 'text-yellow-400' },
  Done: { icon: '✓', color: 'text-green-400' },
  Blocked: { icon: '✗', color: 'text-red-400' },
}

function TaskPanel({ data, active }) {
  const borderColor = active ? 'border-cyan-700 shadow-cyan-900/20 shadow-lg' : 'border-gray-800'

  return (
    <div className={`rounded-lg border ${borderColor} bg-gray-900 p-4 overflow-hidden flex flex-col`}>
      <h2 className="text-sm font-semibold text-gray-400 mb-3 flex items-center gap-2">
        <span className="text-cyan-400">●</span> Tasks
      </h2>

      {!data ? (
        <p className="text-gray-600 text-sm">Fetching tasks...</p>
      ) : (
        <div className="flex-1 overflow-auto space-y-1">
          {data.tasks?.map((task, i) => {
            const cfg = statusConfig[task.status] || statusConfig.Todo
            return (
              <div key={i} className="flex items-center gap-3 py-1.5 px-2 rounded hover:bg-gray-800/30">
                <span className={`${cfg.color} w-4 text-center`}>{cfg.icon}</span>
                <span className="text-sm text-gray-300 flex-1 truncate">{task.title}</span>
                <span className="text-xs text-gray-600">{task.id}</span>
                {task.assignee && (
                  <span className="text-xs text-blue-400">{task.assignee}</span>
                )}
              </div>
            )
          })}
          {(!data.tasks || data.tasks.length === 0) && (
            <p className="text-gray-600 text-sm">No issues found</p>
          )}
        </div>
      )}
    </div>
  )
}

export default TaskPanel
