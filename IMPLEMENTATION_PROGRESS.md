# Grey Owl 実装進捗

設計資料に記載された追加機能を、依存関係のある順に実装する。

## 初期確認（2026-08-13）

- 実装済み: `init`、`check`、`check --file`、human／JSON診断
- 実装済み: `overview directories`、`overview types`、`search`
- 実装済み: Page ID生成、Markdownリンク解析、壊れたリンク検出、Index起点の孤立検出
- 実装済み: 静的Agent Skillの生成
- 未実装または拡張対象: 設定検証の強化、知識グラフAPI、`schema`、メンテナンス検出、変更適用

## 実装順

1. 設定・検証機能の拡張（進行中: 設定lintのCLIを追加）
2. 関係・知識グラフ（完了）
3. Agent向け`schema`出力とSkill手順（完了: schema出力を追加）
4. メンテナンス検出（完了: 検出コマンドを追加）
5. 変更適用・運用（未着手）

## 変更記録

### 2026-08-13

- 進捗ファイルを作成。
- 設計資料と現行実装の差分を確認。
- `growl config lint` を追加。設定診断をWiki走査なしで実行できるようにした。
- README（日英）に新しいコマンドを追加。
- 統合テスト `config_lint_validates_without_scanning_the_wiki` を追加。
- `WikiGraph` と `growl graph` を追加。ノード、リンク、壊れた参照、孤立／未到達ページをJSON出力する。
- 統合テスト `graph_exports_nodes_edges_and_maintenance_signals` を追加。
- `growl schema --format text|json` を追加。設定、type、ディレクトリ、設定診断をAgent向けに出力する。
- `growl maintain` を追加。孤立・未到達・壊れた参照・指定日より古いファイルを候補として報告する。ファイルは変更しない。
- Agent SkillのTODOを実行可能なworkflowへ更新。
- 生成対象の5つのSkillファイルを英語の完成版workflowへ更新。TODOと日本語の暫定記述を除去。
- 統合テストを12件から20件へ拡充。設定異常、終了コード、型・値・ネスト検証、検索異常系、overview統計、相対リンク、入力不足、生成Skillの英語/TODO検査を追加。
- ビルドプロファイルを追加。開発は`codegen-units = 128`、リリースは`opt-level = 3`、最大LTO、`codegen-units = 1`とし、strip／panic設定は変更しない。

## 検証記録

| 日付 | 対象 | 結果 |
| --- | --- | --- |
| 2026-08-13 | 着手前の実装確認 | `overview`、`search`、Page ID、孤立検出などの既存実装を確認 |
| 2026-08-13 | `config lint` | `make format-check`、`make lint`、`make test` 成功（9テスト） |
| 2026-08-13 | グラフ・schema・maintenance | `make format-check`、`make lint`、`make test` 成功（12テスト） |
| 2026-08-13 | Skill本文 | 5ファイルを英語化し、各workflow・安全ルール・制約を明記 |
| 2026-08-13 | テスト拡充 | `make format-check`、`make lint`、`make test` 成功（20テスト） |
| 2026-08-13 | ビルドプロファイル | `make format-check`、`make lint`、`make test`、`cargo build --release` 成功 |

## 現在の到達点

変更適用フェーズ以外の検出・出力系機能を実装済み。次に実装する機能は、変更計画の生成、dry-runとの差分表示、明示的な適用、バックアップ／ロールバック、監査ログである。現時点ではファイルを変更するコマンドは追加していない。
