# Dev Dashboard — Team Structure Design

## Project Overview

Git + CI/CD + タスク管理 + コード品質を複合的に可視化するターミナルダッシュボードCLIツール。

## Team Members

| Name | Role | Authority |
|------|------|-----------|
| Kai（カイ） | PM / プロダクトオーナー | 最終意思決定、ユーザー代理、スコープ管理 |
| Sora（ソラ） | アーキテクト / テックリード | 技術選定、設計判断、コードレビュー統括 |
| Rin（リン） | UI/UX デザイナー | ターミナルUI設計、情報設計、ビジュアル最終判断 |
| Zen（ゼン） | バックエンド / コアエンジニア | 実装、テスト、データ取得ロジック |
| Mio（ミオ） | ドキュメント / DevRel | README、CLI help、ユーザー向けドキュメント全般 |
| Ryu（リュウ） | QA / テスト・品質管理 | 品質ゲート、リリース判断、異常系テスト |

## Communication Flow

```
User（Owner）
  │ requirements / feedback only
  ▼
Kai（PM）
  ├── design decisions ──→ Sora（Architect）
  │                          ├── implementation → Zen（Backend）
  │                          └── UI design → Rin（Designer）
  ├── quality gate ──→ Ryu（QA）
  └── documentation → Mio（DevRel）
```

## Team Rules

1. 議論は具体的に — 数値か具体例で話す
2. 反対意見は根拠+代替案とセット
3. 30秒ルール — 収束しなければKaiが裁定
4. ユーザーエスカレーション — スコープ変更、技術スタック根本変更のみ
5. 全員がコードを読める前提

## Escalation Policy

- チーム内で解決 → デフォルト
- Kaiに判断を仰ぐ → チーム内で意見が割れたとき
- ユーザーにエスカレーション → プロジェクトの方向性を変える判断のみ
