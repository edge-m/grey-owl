# Grey Owl Wiki Skills

Grey OwlのWiki操作を、目的ごとに分けたSkill群です。

## Skills

- `wiki-overview`: Wikiの全体像を取得する
- `wiki-add-article`: 指定されたソースファイルを起点に記事追加後の確認を行う
- `wiki-maintenance`: `growl check` を中心にWikiの状態を確認する
- `wiki-search`: Wiki内の情報を検索する

## 共通方針

- 操作前に設定とWikiの現在状態を確認する
- 未定義のtype、field、配置ルールを推測しない
- 変更後は `growl check` を実行する
- 未解決の検証エラーを成功として扱わない

## CLI workflow

Use the following commands when the generated CLI is available:

```text
growl overview directories
growl overview types
growl overview types --type <type>
growl search --query 'field:value'
growl check
growl check --file <path>
```

Commands resolve the wiki root from `growl.yml` in the current directory by
default. Use `--config <path>` to select another configuration file. Use `rg`
for searching Markdown body text; `growl search` is for structured frontmatter.

## TODO

- [ ] 各Skillの詳細な実行手順を定義する
- [ ] CLIで提供する情報取得・検索機能との対応を定義する
- [ ] 出力形式（human / JSON）を整理する
