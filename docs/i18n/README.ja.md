# Grey Owl

![Grey Owl logo](../img/wide.webp)

Grey Owlは、構造化されたMarkdown Wiki向けの高速な設定駆動型バリデーターです。YAML frontmatterを検証し、人間向けの診断結果を出力します。

プロジェクト名とライブラリ名は`grey-owl`、CLI実行ファイル名は`growl`です。

## Why Grey Owl?

名前は**OWL**、つまり**O**KF **W**iki **L**interに由来します。Open Knowledge Format（OKF）Wikiの慣例から始まったプロジェクトですが、データモデルと検証ルールはOKFや特定のWiki製品に依存しない設計です。

## できること

- Wikiルート以下のMarkdownファイルを再帰的に走査
- YAML frontmatterの解析
- 必須フィールド、文書種別、値の型、許可値の検証
- 識別子の欠落と重複の検出
- 人間向けの診断出力
- スクリプトやCIで利用できる終了コード
- 初期設定ファイルとAgent Skillの生成
- Agent向けの導入手順の表示

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

Agent Skillを生成し、AI Agentが利用できる場所に配置します。

```sh
growl skill <agent-skills-directory>
```

ファイルを変更せずに、セットアップ全体の流れを表示することもできます。

```sh
growl onboard
```

`growl init`は`growl.yml`を作成します。既存のファイルは上書きしません。

### AI AgentでWikiを運用する流れ

1. `growl init`でWikiを初期化し、`growl.yml`にWikiルート、ディレクトリ、
   文書種別、必須frontmatterを設定します。生成したSkillとWikiへのアクセスを
   AI Agentに渡します。
2. 記事の追加を依頼すると、AgentがルールとWikiの状態を確認し、文書種別と配置先を
   選び、必須のYAML frontmatter付きMarkdownを作成します。
3. Agentが`growl check --file <path>`で新しい記事を検証した後、
   `growl check`でWiki全体を検証し、エラー、リンク切れ、孤立ページを修正します。

人間は基本的にAI Agentと対話し、`growl`コマンドはAgentがWikiを調査・検証するための
ツールとして使います。

また、`growl onboard`を実行すると表示された手順をAgentに渡せます。Agentは
`wiki-config` Skillを使って対話的に`growl.yml`を設計・初期化し、その後ほかの
Skillを使ってWikiの構築、検索、保守を行えます。

## 設定

生成される設定は、シンプルなYAMLスキーマです。`wiki_root`は設定ファイルからの相対パスとして解決され、`growl`コマンドがWikiを見つけるために使われます。`directories`では説明付きのディレクトリ構成をネストして定義できます。データソース用の`raw`にはファイルを自由に追加できます。`mandatory_fields`は全ドキュメントで必須となり、`optional: true`は指定できません。type固有のフィールドは、`optional: true`を指定しない限り必須です。配列の`items`やオブジェクトの`fields`で、値の構造をネストして定義できます。

```yaml
growl_version: 0.1.0
wiki_root: .

directories:
  raw:
    description: データソース。ファイルを自由に追加
    directories:
      inbox:
        description: 取り込み前のファイル

mandatory_fields:
  type:
    type: string
  title:
    type: string
  description:
    type: string
  tags:
    type: array
    items:
      type: string
  sources:
    type: array
    items:
      type: string
  generated:
    type: object
    fields:
      at:
        type: datetime
      by:
        type: string
  stale_after:
    type: date
    description: 情報が古いとみなす日。Agentは通常、作成日の14日後を設定し、情報の変化の速さに応じて調整します。

types:
  note:
    description: 一般的なノート

wiki_lint:
  exclude:
    - '**/.*'
    - '**/README.md'
    - raw/**
config_lint:
  max_nesting_depth: 1
```

未知のfrontmatterフィールドは保持され、エラーにはなりません。ディレクトリ設定は説明専用で、ネストできます。設定したWikiルートを使う場合は`growl check --config ./growl.yml`を実行します。`--config`は設定ファイル自体のパス、`wiki_root`はその設定ファイル内で指定するWikiルートのパスです。
`wiki_lint.exclude`には、`wiki_root`からの相対パスで、すべてのWiki検査（frontmatter、リンク、グラフ、孤立ページ検出）から除外するMarkdownパスを指定できます。`growl init`では、ドットで始まるパス、`README.md`、`raw/**`をデフォルトで除外します。`*`、`?`、`**`が使えます。`config_lint`には設定ファイル自体を検証するための設定を記述します。配列やオブジェクトのフィールドは、それぞれ`items`や`fields`でネストして定義できます。
`growl_version`には設定を生成した`growl`のバージョンが記録されます。`growl config validate`では、設定を生成したバージョンが現在の`growl`の互換範囲に含まれるか確認します。

## コマンド

```text
growl init
growl onboard
growl config validate [--config <file>]
growl graph [--config <file>]
growl schema [--config <file>] [--format text|json]
growl maintain [--config <file>] [--stale-before YYYY-MM-DD] [--dry-run] [--format human|json]
growl overview directories [--config <file>] [--statistics]
growl overview types [--config <file>] [--statistics] [--type <type>]
growl search [--config <file>] --query <query>
growl check [--config <file>] [--file <path>]
growl skill <output-directory>
```

`--config`を省略した場合、コマンドはカレントディレクトリの`growl.yml`を探し、
そこから`wiki_root`を解決します。`--file`なしの`check`はWiki全体を検証し、
`index.md`から到達できないページも検出します。

`growl skill`は静的なAgent Skillを次の場所へ出力します。

```text
<output-directory>/growl/SKILL.md
<output-directory>/growl/wiki-config/SKILL.md
<output-directory>/growl/wiki-overview/SKILL.md
<output-directory>/growl/wiki-add-article/SKILL.md
<output-directory>/growl/wiki-maintenance/SKILL.md
<output-directory>/growl/wiki-search/SKILL.md
```

生成されるSkillは、Agentによる設定、Wiki全体像の把握、記事追加、検索、保守の
ワークフローを提供します。

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

## ビルドプロファイル

開発ビルドでは再ビルドを高速にするため、codegen unitを128に設定しています。リリースビルドでは実行時の最適化を最大化するため、最大LTO（`lto = true`）、`opt-level = 3`、codegen unitを1に設定しています。リリースプロファイルではstripとpanicの動作は変更しません。
