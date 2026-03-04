# Web Dashboard Design

**Goal:** dev-dashboardにWebモードを追加。`--web`フラグでAxum APIサーバー + React SPAが起動し、ブラウザでダッシュボードを表示する。

**Architecture:** 既存のRustデータ層をAxumのAPIハンドラーで公開。React + Tailwind SPAがAPIをpollingして表示。ビルド済みSPAをAxumの静的ファイル配信で提供（単一バイナリ）。

## API Endpoints

| Endpoint | Response |
|----------|----------|
| `GET /api/git` | GitStatus |
| `GET /api/ci` | CiStatus |
| `GET /api/tasks` | TasksStatus |
| `GET /api/quality` | QualityMetrics |
| `GET /api/config` | { owner, repo, path } |
| `GET /*` | Static SPA files |

## CLI

`cargo run -- --web` → APIサーバーモード (port 3000)
`cargo run` → 従来のTUIモード

## Frontend

- React + Tailwind, ダークテーマ
- 4パネルグリッド
- 5秒ごとAPI polling
- `web/` ディレクトリに配置
