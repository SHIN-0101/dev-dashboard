const GITHUB_API = 'https://api.github.com'

function headers(token) {
  const h = { Accept: 'application/vnd.github+json' }
  if (token) h.Authorization = `Bearer ${token}`
  return h
}

export async function fetchGitData(owner, repo, token) {
  const [commitsRes, repoRes] = await Promise.all([
    fetch(`${GITHUB_API}/repos/${owner}/${repo}/commits?per_page=15`, { headers: headers(token) }),
    fetch(`${GITHUB_API}/repos/${owner}/${repo}`, { headers: headers(token) }),
  ])

  if (!commitsRes.ok) throw new Error(`GitHub API error: ${commitsRes.status}`)

  const commits = await commitsRes.json()
  const repoData = await repoRes.json()

  return {
    branch: repoData.default_branch || 'main',
    commits: commits.map(c => ({
      hash: c.sha.slice(0, 7),
      message: c.commit.message.split('\n')[0],
      author: c.commit.author.name,
      timestamp: c.commit.author.date,
    })),
    branches: [],
    changed_files: 0,
    staged_files: 0,
  }
}

export async function fetchCiData(owner, repo, token) {
  const res = await fetch(
    `${GITHUB_API}/repos/${owner}/${repo}/actions/runs?per_page=10`,
    { headers: headers(token) }
  )

  if (!res.ok) throw new Error(`GitHub API error: ${res.status}`)
  const data = await res.json()

  return {
    runs: (data.workflow_runs || []).map(run => ({
      id: String(run.id),
      name: run.name,
      status: mapRunStatus(run),
      branch: run.head_branch,
      duration_secs: run.updated_at && run.run_started_at
        ? Math.round((new Date(run.updated_at) - new Date(run.run_started_at)) / 1000)
        : null,
      started_at: run.run_started_at,
    })),
  }
}

function mapRunStatus(run) {
  if (run.status === 'completed') {
    if (run.conclusion === 'success') return 'Success'
    if (run.conclusion === 'failure') return 'Failure'
    if (run.conclusion === 'cancelled') return 'Cancelled'
    return 'Failure'
  }
  if (run.status === 'in_progress') return 'Running'
  return 'Pending'
}

export async function fetchTasksData(owner, repo, token) {
  const res = await fetch(
    `${GITHUB_API}/repos/${owner}/${repo}/issues?state=all&per_page=20&sort=updated`,
    { headers: headers(token) }
  )

  if (!res.ok) throw new Error(`GitHub API error: ${res.status}`)
  const issues = await res.json()

  return {
    tasks: issues
      .filter(i => !i.pull_request)
      .map(i => ({
        id: `#${i.number}`,
        title: i.title,
        status: mapIssueStatus(i),
        assignee: i.assignee?.login || null,
      })),
  }
}

function mapIssueStatus(issue) {
  if (issue.state === 'closed') return 'Done'
  const labels = (issue.labels || []).map(l => l.name.toLowerCase())
  if (labels.some(l => l.includes('blocked'))) return 'Blocked'
  if (labels.some(l => l.includes('progress') || l.includes('wip'))) return 'InProgress'
  return 'Todo'
}

export async function fetchQualityData(owner, repo, token) {
  // Quality metrics derived from recent workflow runs
  const res = await fetch(
    `${GITHUB_API}/repos/${owner}/${repo}/actions/runs?per_page=20&status=completed`,
    { headers: headers(token) }
  )

  if (!res.ok) throw new Error(`GitHub API error: ${res.status}`)
  const data = await res.json()
  const runs = data.workflow_runs || []

  const total = runs.length
  const success = runs.filter(r => r.conclusion === 'success').length
  const failures = runs.filter(r => r.conclusion === 'failure').length

  return {
    test_coverage: total > 0 ? (success / total) * 100 : 0,
    lint_warnings: 0,
    lint_errors: failures,
    security_issues: 0,
  }
}
