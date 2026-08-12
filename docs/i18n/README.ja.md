# Grey Owl

Grey Owlは、構造化されたMarkdown Wiki向けの高速な設定駆動型バリデーターです。YAML frontmatterを検証し、人間向けまたはJSON形式で診断結果を出力します。

プロジェクト名とライブラリ名は`grey-owl`、CLI実行ファイル名は`growl`です。

## Why Grey Owl?

名前は**OWL**、つまり**O**KF **W**iki **L**interに由来します。Open Knowledge Format（OKF）Wikiの慣例から始まったプロジェクトですが、データモデルと検証ルールはOKFや特定のWiki製品に依存しない設計です。

## できること

- Wikiルート以下のMarkdownファイルを再帰的に走査
- YAML frontmatterの解析
- 必須フィールド、文書種別、値の型、許可値の検証
- 識別子の欠落と重複の検出
- 人間向けまたはJSON形式での診断出力
- スクリプトやCIで利用できる終了コード
- 初期設定ファイルとAgent Skillの生成

Markdown本文は不透明なコンテンツとして扱い、意味や文章品質の評価は行いません。

## クイックスタート

現在のチェックアウトをローカルにインストールします。

```sh
make dev-install
```

Wikiのディレクトリで設定を作成します。

```sh
cd path/to/wiki
growl init
```

Wikiを検証します。

```sh
growl check . --config ./growl.yml
growl check . --config ./growl.yml --format json
```

`growl init`は`growl.yml`を作成します。既存のファイルは上書きしません。

## 設定

生成される設定は、シンプルなYAMLスキーマです。`id`と`type`は予約フィールドです。`optional: true`がないフィールドは必須になります。

```yaml
common_fields:
  id:
    type: string
  type:
    type: string

types:
  note:
    fields:
      title:
        type: string
      status:
        type: string
        optional: true
        values: [draft, active]
```

未知のfrontmatterフィールドは保持され、エラーにはなりません。現在の実装では、設定ファイルは`--config`で明示的に指定する必要があり、自動探索は行いません。

## コマンド

```text
growl init
growl check <wiki-path> [--config <file>] [--format human|json]
growl skill <output-directory>
```

`growl skill`は静的なAgent Skillを次の場所へ出力します。

```text
<output-directory>/growl/SKILL.md
```

終了コード：

- `0` — 検証がエラーなしで完了
- `1` — 検証は完了したがエラーを検出
- `2` — 引数、設定、I/Oエラーなどでコマンドを実行できなかった

## 開発

stable Rustを使用しています。リポジトリルートで次を実行します。

```sh
make format-check
make lint
make test
```

その他のターゲットとして`make build`、`make check`、`make format`、`make dev-install`があります。コントリビューションと検証のルールは[AGENTS.md](../../AGENTS.md)を参照してください。

英語版の一次READMEは[README.md](../../README.md)です。ユーザー向け情報を変更する場合は、英語版とこの日本語版を同期してください。
