# UI localisation

The desktop app offers **Čeština** (`cs`), **Deutsch** (`de`), **한국어** (`ko`), **Русский** (`ru`), **简体中文** (`zh-hans`)
and **繁體中文** (`zh-hant`), alongside English, French, Japanese and **Português (Brasil)** (`pt-br`), in **Settings → General → Language**. The choice is saved in the
existing `language` setting. **System** uses the operating system's preferred
supported language; if none matches, it uses English. An explicit choice takes
precedence over the system language.

Chinese locales without a script, including `zh`, `zh-CN` and `zh-SG`, select
Simplified Chinese. `zh-TW`, `zh-HK` and `zh-MO` select Traditional Chinese.
An explicit `Hans` or `Hant` script takes precedence over the region. Locale
matching accepts underscores, case differences and Unix encoding suffixes.

## German

**Deutsch** (`de`) uses the informal "du". `de`, `de-DE`, `de-AT` and `de-CH` select it, as does the
system language when it is German. `crates/ui-egui/src/i18n/de.tsv` has the same keys as the Chinese
catalogs (a unit test compares them), written from the English source strings, not from the Chinese.
It was drafted with an AI and not yet reviewed by a native speaker; corrections are welcome.
Product and technology names, tool ids and shortcuts keep their spelling. The kind detection that reads
a request ("a game", "a website") also knows the German words *Spiel*, *Webseite* and *Internetseite*,
so a translated suggestion keeps its project kind.

## Brazilian Portuguese

**Português (Brasil)** (`pt-br`) uses plain Brazilian Portuguese. `pt`, `pt-BR`, POSIX tags such as
`pt_BR.UTF-8` and other Portuguese regions select this catalog until another Portuguese locale is
available. The system-language choice follows the same resolution.

`crates/ui-egui/src/i18n/pt-br.tsv` has the same keys as the maintained reference catalogs.
It translates the English source meaning. An editorial pass covered every catalog entry and
`README.pt.md`, checking meaning, Brazilian usage, grammar, labels and consistent terminology.
Independent community review remains welcome. Automated UI checks cover startup, switching between English and
Portuguese, saved selection, translated-title search, number formatting and translated suggestions.
Interactive native-window checks across platforms remain pending.

Terms include *Sala limpa* (Clean room), *Profundo* (Deep), *evidências* (evidence), *agente*
(the builder), *versão fixada* (pinned build) and *cópia independente* (frozen copy). *Criar* names
the action, while *versão* names its saved result. Technical terms without a clearer everyday
equivalent, such as *engine*, *stack*, *backend*, *strings*, *disassembly*, *source map* and *dump*,
keep their established English spelling. Counts use labels such as `Itens: {n}` to avoid
parenthesized plural suffixes and ambiguous gender when runtime values are inserted. Product names, tool identifiers and
placeholders keep their spelling. USD usage estimates use a decimal comma without changing stored
numeric values. Requests containing *jogo*, *jogabilidade* or *página web*, including the translated
home suggestions, keep their intended project kind. Game requests also accept *em*, *com*,
*usando*, *no* and *na* before an engine name, including the Rust example in
[`README.pt.md`](../README.pt.md). A Portuguese game-related topic, such as *um site sobre jogos*,
does not change an explicitly requested website into a game; a saved game-engine stack still wins.
That README translates the English documentation and preserves executable names, flags, API/tool
identifiers and file names; technical terms such as *stack*, *backend* and *sandbox* remain in English.

## Scope and format

`crates/ui-egui/src/i18n/cs.tsv`, `de.tsv`, `ko.tsv`, `pt-br.tsv`, `ru.tsv`, `zh-hans.tsv` and `zh-hant.tsv` contain original
translations under the repository's MIT OR Apache-2.0 licence. Each UTF-8 line
has two columns: `English source<TAB>translation`. Comments begin with `#`;
`\n` represents a newline. The language registry in `i18n.rs` embeds the catalogs.

The catalogs cover first run, new projects, project controls, Settings, the
workbench, tool titles, tool groups, target descriptions, tooltips and
accessibility labels. The command palette searches translated tool titles as
well as English titles and stable tool IDs. Missing translations fall back to
the original English text. Product and technology names retain their spelling.

User text, AI responses, evidence and raw tool results, tool summaries and schema
descriptions, logs and exported reports retain their original text. The CLI,
MCP, tool IDs, stored project data and the separate browser viewer are unchanged.

Keep placeholders such as `{name}` and `{n}` intact, along with command IDs,
shortcuts, escapes and trailing `…`. Update every catalog (`cs`, `de`, `ko`, `pt-br`, `ru`, `zh-hans`, `zh-hant`)
when English source keys change; the tests require matching key sets. Common terms include 净室 / 淨室 (Clean room), 深度 (Deep),
构建 / 建置 (Build), 证据 / 證據 (Evidence), and 项目 / 專案 (Project).
The Korean catalog `ko.tsv` uses 클린룸 (Clean room), 심층 (Deep), 분석 / 만들기 / 검사 / 완성
(Study / Build / Check / Ready), 근거 (Evidence) and 프로젝트 (Project); short labels are nouns and
sentences use the formal 합니다 style.

## Validation

Run:

```sh
cargo test -p craft-anything-ui-egui
cargo test -p xtask
cargo clippy -p craft-anything-ui-egui -p xtask --all-targets -- -D warnings
cargo xtask i18n-coverage
```

Catalog tests check duplicate and empty entries, matching key sets, placeholders
and ellipses. The coverage command checks literal translation calls, common
widget labels and registered engine tool titles and groups; it also runs in
`cargo xtask ci`. It does not prove coverage of arbitrary runtime text or labels
held in tables, so review those when adding or changing UI code.

The existing snapshot example accepts any catalog code with `--lang`, such as
`--lang zh-hant` or `--lang ko`, for example:

```sh
cargo run -p craft-anything-ui-egui --example snapshot -- --lang zh-hant --only settings --out-dir target/snapshots/zh-hant
```

Chinese and Korean glyphs use the existing craft-fonts fallback, which release and CI builds
already include. Local builds must set `CRAFT_FONTS_DIR` to a craft-fonts checkout
before compiling to get the same glyph coverage. The catalogs add no font assets.
Check each script, clipping, narrow windows and native file dialogs visually
on each platform; catalog coverage alone does not verify layout or fonts.
