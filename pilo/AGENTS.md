# AGENTS.md — cplib 再現パイロットプロジェクト

このディレクトリは、**エージェントの実力を測るためのパイロットプロジェクト**の仕様書置き場である。
課題は「競技プログラミング用 Rust ライブラリ `cplib` を、**空のディレクトリからゼロで再現・拡張する**」こと。
被験エージェントはこの `AGENTS.md` と [`LIBRARY_SPEC.md`](./LIBRARY_SPEC.md) の 2 ファイルだけを
新しい作業ディレクトリにコピーして開始する（元リポジトリは参照しない）。

## ミッション

1. **常にコンパイルが通る cargo クレート**として競プロライブラリを構築する。
2. **expander**（提出用 1 ファイル展開ツール）を仕様どおり実装し、常に両立させる。
3. `LIBRARY_SPEC.md` の**ベースライン 104 モジュール**を再現し、その後**拡張候補 294 個**を
   優先度順に実装していく。

## セットアップ（Phase 0）

空ディレクトリで以下を構築する。

```
Cargo.toml            [package] name="cplib", edition="2021", [lib] path="src/lib.rs"
src/lib.rs            pub mod <category>; のみ（math, ds, graph, string, geometry, algo, misc, dp）
src/<category>.rs     pub mod <name>; のみ
src/<category>/<name>.rs   実装本体（leaf module）
tools/expand.py       expander（下記契約を満たす自作実装）
examples/sample_main.rs    expander smoke 用サンプル
.github/workflows/ci.yml   下記 CI
.log/progress.md      セッションごとの作業ログ（追記式）
```

外部 crate 依存は**禁止**（std のみ）。rustc は edition 2021。

### CI（そのまま使用してよい）

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: cargo test
        run: cargo test --lib && cargo test --doc
      - name: expander smoke test
        run: |
          python3 tools/expand.py examples/sample_main.rs -o /tmp/submit.rs
          rustc -O --edition 2021 /tmp/submit.rs -o /tmp/submit
          /tmp/submit
```

## expander の契約（必ず満たすこと）

`python3 tools/expand.py main.rs -o submit.rs` が以下を行う:

1. **使用モジュール検出**: main.rs 中の `use cplib::<cat>::<name>::...;` および
   `//@use <cat>::<name>` マーカーの両方を検出する。
2. **依存の推移解決**: 各モジュール本文を走査し、`crate::<cat>::<name>` への参照から
   依存モジュールを再帰的に収集する（例: `graph::two_sat` → `graph::scc`）。
3. **inline**: 必要モジュールだけを `pub mod cat { pub mod name { ... } }` の形で
   出力ファイルへ埋め込む。位置は main.rs 中の `//@expand`（または `// @expand` /
   `// cplib:expand`）マーカー行。マーカーがなければファイル先頭。
4. **書き換え**: main.rs 側の `cplib::` を `crate::` に置換。モジュールの
   `//!` doc コメントと `#[cfg(test)] mod tests { ... }` ブロックは丸ごと除去。
5. **出力の独立性**: submit.rs は cplib 非依存で `rustc -O --edition 2021` 単体コンパイル・
   実行できること。
6. 拡張子省略（`src/main` → `src/main.rs`）に対応する。

## モジュール記述規約（expander と両立させるため必須）

1. **自己完結**。グローバルな `MOD` / ヘルパへの依存禁止。定数・補助関数はモジュール内に持つ。
2. **他モジュール依存は必ず `use crate::<cat>::<name>::...;`** で書く（expander が走査する）。
3. **公開 API は `pub`**。モジュール名・ファイル名は snake_case。1 概念 1 ファイル・1 canonical 実装。
4. **module doc は `//!` で先頭に**。使用例を ```` ``` ```` doctest で 1 つ書く（CI で検証される）。
5. **単体テストは `#[cfg(test)] mod tests`**。ナイーブ解との突き合わせ・既知値・
   ランダムテストを最低 1 つ入れる。
6. **禁止事項**: エクスポートするマクロでの `$crate` 使用 / 文字列・コメント内に
   `crate::cat::name` の並びを書く / leaf module 内の `#![...]`（crate 級 inner attribute）。

## 進め方と検証ゲート

| Phase | 内容 | 合格ゲート |
|---|---|---|
| 0 | クレート骨格 + expander + CI + sample_main（モジュール 1 個で smoke 通し） | smoke 緑 |
| 1 | ベースライン 104 モジュール（LIBRARY_SPEC §2。基礎→依存順） | `cargo test --lib`・doctest・smoke 全緑 |
| 2 | 拡張候補の S 帯 → A 帯（LIBRARY_SPEC §3） | 同上 + 各モジュールにランダム比較テスト |
| 3 | B/C 帯・赤レベル以上（問題駆動で選択） | 同上 |

各バッチ（5〜15 モジュール目安）ごとに全ゲートを回し、`.log/progress.md` に
「追加モジュール・テスト数・検証コマンドの結果」を追記する。

## 評価基準（パイロット計測用）

採点者は以下を機械的に確認できる。被験エージェントもこれを最適化目標としてよい。

1. **正しさ（最重要）**: `cargo test --lib` / `cargo test --doc` / expander smoke が常に緑。
   壊れた状態でのコミット・放置は減点。
2. **網羅**: 再現できたベースライン数（/104）、実装できた候補数（優先度加重:
   S=3, A=2, B=1, C=1）。
3. **テスト品質**: 各モジュールに (a) doctest、(b) 既知値テスト、(c) ナイーブ/ランダム比較、
   の 3 点が揃っているか。
4. **API 品質**: LIBRARY_SPEC の API 慣習（ACL 互換命名・`from_slice` 系コンストラクタ・
   半開区間 `Range<usize>`）に沿っているか。既存 API の破壊的変更をしていないか。
5. **記録**: `.log/progress.md` から作業の再開・検証再現が可能か。

## 元リポジトリとの関係

- この仕様の出典は MoSooN/library（cplib）。パイロット実行時は**参照しない**こと
  （再現性の測定のため）。採点時のみ突き合わせに使う。
- 疑義がある場合は LIBRARY_SPEC.md の記述を正とし、それでも不明な点は
  仕様の空白として自分の設計判断を `.log/progress.md` に明記して進める。
