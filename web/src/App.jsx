import { useState, useEffect, useCallback } from 'react'
import GitPanel from './components/GitPanel'
import CiPanel from './components/CiPanel'
import TaskPanel from './components/TaskPanel'
import QualityPanel from './components/QualityPanel'
import Settings from './components/Settings'
import { fetchGitData, fetchCiData, fetchTasksData, fetchQualityData } from './lib/github'

const STORAGE_KEY = 'dev-dashboard-config'

function loadConfig() {
  try {
    return JSON.parse(localStorage.getItem(STORAGE_KEY)) || {}
  } catch {
    return {}
  }
}

function App() {
  const [git, setGit] = useState(null)
  const [ci, setCi] = useState(null)
  const [tasks, setTasks] = useState(null)
  const [quality, setQuality] = useState(null)
  const [activePanel, setActivePanel] = useState(0)
  const [config, setConfig] = useState(loadConfig)
  const [showSettings, setShowSettings] = useState(false)
  const [error, setError] = useState(null)

  const isConfigured = config.owner && config.repo

  const fetchData = useCallback(async () => {
    if (!isConfigured) return
    const { owner, repo, token } = config
    setError(null)

    try {
      const [gitData, ciData, tasksData, qualityData] = await Promise.all([
        fetchGitData(owner, repo, token).catch(() => null),
        fetchCiData(owner, repo, token).catch(() => null),
        fetchTasksData(owner, repo, token).catch(() => null),
        fetchQualityData(owner, repo, token).catch(() => null),
      ])
      if (gitData) setGit(gitData)
      if (ciData) setCi(ciData)
      if (tasksData) setTasks(tasksData)
      if (qualityData) setQuality(qualityData)
    } catch (e) {
      setError(e.message)
    }
  }, [config, isConfigured])

  useEffect(() => {
    if (!isConfigured) {
      setShowSettings(true)
      return
    }
    fetchData()
    const interval = setInterval(fetchData, 30000)
    return () => clearInterval(interval)
  }, [fetchData, isConfigured])

  const handleSaveConfig = (newConfig) => {
    setConfig(newConfig)
    localStorage.setItem(STORAGE_KEY, JSON.stringify(newConfig))
    setGit(null)
    setCi(null)
    setTasks(null)
    setQuality(null)
  }

  const tabs = ['Git', 'CI/CD', 'Tasks', 'Quality']

  return (
    <div className="min-h-screen bg-gray-950 text-gray-100 p-4 font-mono">
      {/* Header */}
      <div className="mb-4 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <h1 className="text-xl font-bold text-cyan-400">dev-dashboard</h1>
          {isConfigured && (
            <span className="text-sm text-gray-500">
              {config.owner}/{config.repo}
            </span>
          )}
        </div>
        <div className="flex items-center gap-3">
          <div className="flex gap-1">
            {tabs.map((tab, i) => (
              <button
                key={tab}
                onClick={() => setActivePanel(i)}
                className={`px-3 py-1 text-sm rounded transition-colors ${
                  activePanel === i
                    ? 'bg-cyan-900/50 text-cyan-400 border border-cyan-700'
                    : 'text-gray-500 hover:text-gray-300 border border-transparent'
                }`}
              >
                {tab}
              </button>
            ))}
          </div>
          <button
            onClick={() => setShowSettings(true)}
            className="text-gray-500 hover:text-gray-300 text-sm px-2 py-1 border border-gray-800 rounded transition-colors"
          >
            Settings
          </button>
        </div>
      </div>

      {error && (
        <div className="mb-4 p-2 bg-red-900/30 border border-red-800 rounded text-sm text-red-400">
          {error}
        </div>
      )}

      {!isConfigured ? (
        <div className="flex items-center justify-center h-[60vh]">
          <p className="text-gray-500">Configure your GitHub repository in Settings to get started.</p>
        </div>
      ) : (
        <div className="grid grid-cols-2 gap-4 h-[calc(100vh-100px)]">
          <GitPanel data={git} active={activePanel === 0} />
          <CiPanel data={ci} active={activePanel === 1} />
          <TaskPanel data={tasks} active={activePanel === 2} />
          <QualityPanel data={quality} active={activePanel === 3} />
        </div>
      )}

      {showSettings && (
        <Settings
          config={config}
          onSave={handleSaveConfig}
          onClose={() => setShowSettings(false)}
        />
      )}
    </div>
  )
}

export default App
