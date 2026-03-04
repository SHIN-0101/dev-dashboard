# dev-dashboard

Terminal dashboard that shows your development status at a glance. Git activity, CI/CD pipelines, tasks, and code quality — all in one screen.

## Why

Switching between GitHub, CI dashboards, Jira, and coverage reports kills focus. This tool puts everything in your terminal where you already live.

## Quick Start

```bash
# Install
cargo install --path .

# Run in your project directory
dev-dashboard

# Or point to a specific repo
dev-dashboard --path /path/to/repo
```

## What You See

```
┌─ dev-dashboard ──────────────────────────────────────────┐
│ Git │ CI/CD │ Tasks │ Quality                            │
├──────────────────────────┬───────────────────────────────┤
│  Git                     │  CI/CD                        │
│  branch: main   2M 0S   │  ✓ Build & Test  main    94s  │
│  Hash    Message  Author │  ● Deploy        main    —    │
│  abc1234 feat:... zen    │  ✗ Lint       feature   12s  │
├──────────────────────────┼───────────────────────────────┤
│  Tasks                   │  Quality                      │
│  ● Auth system    Zen    │  Coverage: ████████░░ 85.5%   │
│  ○ API docs       Mio    │  Warnings: 3                  │
│  ✓ Setup CI       Kai    │  Errors: 0                    │
│  ✗ Blocked task   —      │  Security: 1                  │
└──────────────────────────┴───────────────────────────────┘
```

## Keybindings

| Key | Action |
|-----|--------|
| `1-4` | Jump to panel |
| `Tab` | Next panel |
| `Shift+Tab` | Previous panel |
| `q` | Quit |
| `Ctrl+C` | Quit |

## Configuration

```bash
# Custom refresh interval (default: 5 seconds)
dev-dashboard --refresh 10

# Specify repo path
dev-dashboard --path ~/projects/my-app
```

## Requirements

- Rust 1.70+
- A git repository

## Build from Source

```bash
git clone <repo-url>
cd dev-dashboard
cargo build --release
```

## License

MIT
