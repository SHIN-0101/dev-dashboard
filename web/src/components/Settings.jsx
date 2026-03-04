import { useState } from 'react'

function Settings({ config, onSave, onClose }) {
  const [owner, setOwner] = useState(config.owner || '')
  const [repo, setRepo] = useState(config.repo || '')
  const [token, setToken] = useState(config.token || '')

  const handleSave = () => {
    onSave({ owner, repo, token })
    onClose()
  }

  return (
    <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-gray-900 border border-gray-700 rounded-lg p-6 w-96 max-w-[90vw]" onClick={e => e.stopPropagation()}>
        <h2 className="text-lg font-bold text-cyan-400 mb-4">Settings</h2>

        <div className="space-y-3">
          <div>
            <label className="block text-xs text-gray-500 mb-1">GitHub Owner</label>
            <input
              value={owner}
              onChange={e => setOwner(e.target.value)}
              className="w-full bg-gray-800 border border-gray-700 rounded px-3 py-2 text-sm text-gray-200 focus:border-cyan-600 focus:outline-none"
              placeholder="e.g. SHIN-0101"
            />
          </div>
          <div>
            <label className="block text-xs text-gray-500 mb-1">Repository</label>
            <input
              value={repo}
              onChange={e => setRepo(e.target.value)}
              className="w-full bg-gray-800 border border-gray-700 rounded px-3 py-2 text-sm text-gray-200 focus:border-cyan-600 focus:outline-none"
              placeholder="e.g. dev-dashboard"
            />
          </div>
          <div>
            <label className="block text-xs text-gray-500 mb-1">
              GitHub Token <span className="text-gray-600">(optional, for private repos)</span>
            </label>
            <input
              type="password"
              value={token}
              onChange={e => setToken(e.target.value)}
              className="w-full bg-gray-800 border border-gray-700 rounded px-3 py-2 text-sm text-gray-200 focus:border-cyan-600 focus:outline-none"
              placeholder="ghp_..."
            />
          </div>
        </div>

        <div className="flex gap-2 mt-5">
          <button
            onClick={handleSave}
            disabled={!owner || !repo}
            className="flex-1 bg-cyan-700 hover:bg-cyan-600 disabled:bg-gray-700 disabled:text-gray-500 text-white text-sm py-2 rounded transition-colors"
          >
            Save
          </button>
          <button
            onClick={onClose}
            className="px-4 text-gray-400 hover:text-gray-200 text-sm py-2 rounded border border-gray-700 transition-colors"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  )
}

export default Settings
