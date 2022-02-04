<h1 align="center">
  <div>👋 Welcome to the</div>
  <img src="./doc/img/logo.png" alt="Logo" height="125">
  <div>Holochain library</div>
</h1>

<div align="center">

[![license](https://img.shields.io/github/license/trustgraph/amazing-github-template.svg?style=flat-square)](LICENSE.md)
[![PRs welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/trustgraph/amazing-github-template/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![made with hearth by trustgraph](https://img.shields.io/badge/made%20with%20%E2%99%A5%20-cc14cc.svg?style=flat-square)](https://github.com/trustgraph)

</div>


TrustGraph::Holochain is a Rust library, intended to allow for [Hololchain](https://www.holochain.org) developers to easily use the [Trust Graph](https://github.com/trustgraph/trustgraph) protocol in their Happs.


## Prerequisites

- rust >= 1.56

## Install

In your `Cargo.toml`:

```rs
hc_zome_trust_atom = {git = "https://github.com/trustgraph/trustgraph-holochain.git", rev="x.y.z", package = "hc_zome_trust_atom"}
```

Replace `x.y.z` with the tag corresponding to the version you want. See the list of [available tags](https://github.com/trustgraph/trustgraph-holochain/tags).

HDK version correspondence:

- version `0.0.1` works with `hdk` version `0.0.116`


## stuff

A TrustGraph is

- initiated from a collection of TAs - generally to people or orgs in order to follow their webs of trust
- [someone] rolls up the network into top level TAs for that TG (ongoing)

TG

- public viewable or private viewable
- private viewing is by invite code/cap tokent
- has an underlying DHT
  - or two: 1 for read, one for write ?

Rollups

- an agent could only roll up what is visible to them
- trust atoms that are an amalgam _value_ for each _target_ / _content_ combination

## Usage

```rs
use holo_trust::prelude::*;

let target: EntryHashB64 = "...".into(); // TODO // coming soon: HC HREL format hc://appid//hashid -- maybe should be hc://dna_hash//entry_hash|header_hash|agent_pub_key
let content: String = "sushi".into();
let value: f32 = 0.8;  // TODO decimal
let attributes: BTreeMap<String, String> = BTreeMap::from([
  ("original_rating_type".into(), "stars".into()),
  ("original_rating_min".into(), "1".into()),
  ("original_rating_max".into(), "5".into()),
  ("original_rating_value".into(), "4".into()),
])

// todo TrustAtom.create_5_star(...)
// todo TrustAtom.create_like(...)
// todo TrustAtom.create_clap(...)
// ...

let trust_atom = TrustAtom.create(
  target: target,
  content: content,
  value: value,
  attributes: attributes,
);

let trust_graph_atoms: Vector<TrustAtom> = vec![
  // TrustAtom where target = Alice's TrustGraph
  // TrustAtom where target = Bo's TrustGraph
  // TrustAtom where target = New York Times' TrustGraph
]

let trust_graph = TrustGraph.create(
  private: true,
  trust_atoms: trust_graph_atoms
)

let trust_atoms: Vec<TrustAtom> = trust_graph.where(
  content_starts_with: Some("Category~Pop~80s".into()),
  min_rating: Some("0.0"),
  source: Some("agentpubkey TODO"),
  target: Some("entry hash b64 TODO"),
)

let trust_atoms: Vec<TrustAtom> = trust_graph.where(
  content_starts_with: "Category~Pop~80s".into(),
  min_rating: "0.0"
)

let trust_graph_2 = trust_graph.copy(
    with: vec![
      // TrustAtoms to add to new TrustGraph
      ],
    without: vec![
      // TrustAtoms to exclude from new TrustGraph
      ],
  )

trust_graph.rollup() // <-- do useful work of synthesizing TG to top level; maybe should be paid, in trust, tokens, or both

```

## Data format

It encodes TrustAtoms as links, with the following components:

1. Holochain Link `base` == TrustAtom `source` - one of:
    - creating agent (`AgentPubKeyB64`)
    - TrustGraph (`EntryHashB64`)
1. Holochain Link `target` == TrustAtom `target` - entity being rated/reviewed/etc - one of:
    - `EntryHashB64`
    - `AgentPubKeyB64`
1. Holochain Link `tag`* (max 999 bytes) - formatted as UTF-8 string
  - TrustAtom header bytes: `[0xC5][0xA6]` (which together comprise the unicode character `Ŧ`) (required)
  - Direction byte:
      - `[0x21][0x92]` (unicode `→`) means: HC target = TA target
      - `[0x21][0xA9]` (unicode `↩`) means: HC target = TA source
  - TrustAtom `content` - semantic info (eg sushi) - max 900 bytes
  - Separator: null byte `[0x00]`
  - TrustAtom `value` - rating ( `"-0.999999999"` to `"0.999999999"`) - max 12 chars
  - Separator: null byte `[0x00]`
  - Random 9 characters for bucketing purposes
  - Separator: null byte `[0x00]`
  - Canonical data including additional attributes - `EntryHashB64`
      - Entry contains attributes formatted in: `BTreeMap<String, String>`
      - You will find full content here; if content exceeds link tag limts it ends with `…` as a hint
      - If value is 1.0, we use "0.999999999" in link tag, but 1.0 here
      - Entry hash is a sring version of EntryHashB64 for debugging purposes, not raw bytes

*This format is designed to allow us to encode trust atoms as Holochain links, and search them by their tags.  Holochain can search for all links _starting_ with a given set of bytes (characters).

### Full Example Link Tags

```
Ŧ→sushi[0x00]0.999999999[0x00]892412523[0x00]uhCEk…UFnFF
Ŧ↩sushi[0x00]0.999999999[0x00]892412523[0x00]uhCEk…UFnFF

Ŧ→sushi[0x00]0.800000000[0x00]087423432[0x00]uhCEk…qS5wc
Ŧ↩sushi[0x00]0.800000000[0x00]087423432[0x00]uhCEk…qS5wc

Ŧ→spam[0x00]-0.999999999[0x00]328425615[0x00]uhCEk…VaaDd
Ŧ→block[0x00]-0.999999999[0x00]837592944[0x00]uhCEk…VaaDd
```

## Link Tags

This format is designed to allow us to encode trust atoms as Holochain links, and search them by their tags.  Holochain can search for all links _starting_ with a given set of bytes (characters).

Each link tag is a UTF8 string:
1. optional: semantic content -- if hierarchical chunks, separated by `~`
    - example: `"Category~Pop~80s~Boy Band"`
1. required: null byte [0x00]
1. optional: value as a string -- `"-0.999999999"` to `"0.999999999"`
1. required: null byte [0x00]

Therefore, the minimum link tag is two null bytes: `[0x00][0x00]`, meaning no semantic content or value is attached.

Example of only semantic content: `"Category~Pop~80s~Boy Band[0x00][0x00]"`

Example of only value: `"[0x00]0.8[0x00]"`

*TODO* describe searching, both of semantic content and values


---

Link Tags:

"2021-12-29T00:00:234"

"Category~Pop~80s[0x00]0.999999999[0x00]"
"[0x00]0.999999999[0x00]" // thumbs up
"[0x00]0.0[0x00]" // thumbs down
"[0x00]-0.999999999[0x00]" // flag/spam/abuse

TrustAtom Creation:

```rs
pub struct TrustAtomInput {
    pub target: EntryHash,
    pub content: String,
    pub value: String,
    pub attributes: BTreeMap<String, String>,
}


#[hdk_extern]
pub fn create(input: TrustAtomInput) -> ExternResult<()> {
    // ...
}
```

TrustAtom Query:

```rs
pub struct QueryInput {
    pub source: Option<EntryHash>,
    pub target: Option<EntryHash>,
    pub content_starts_with: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
}

#[hdk_extern]
pub fn query(input: QueryInput) -> ExternResult<Vec<TrustAtom>> {
    // ...
}

```

Client-facing representation of a Trust Atom (this is what is returned to client from a `query`)

```rs
pub struct TrustAtom {
    pub source: String,
    pub target: String,
    pub content: String,
    pub value: String,
    pub source_entry_hash: EntryHashB64,
    pub target_entry_hash: EntryHashB64,
    pub attributes: BTreeMap<String, String>,
}
```

## Author

👤 **Harlan T Wood (https://github.com/harlantwood)**

* Website: https://trustgraph.net
* Github: [@trustgraph](https://github.com/trustgraph)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!<br />

<!-- Feel free to check [issues page](https://github.com/trustgraph/js-trustgraph-core/issues). -->
<!-- You can also take a look at the [contributing guide](https://github.com/trustgraph/js-trustgraph-core/blob/master/CONTRIBUTING.md). -->

<a href="https://github.com/trustgraph/amazing-github-template/issues/new?assignees=&labels=bug&template=01_BUG_REPORT.md&title=bug%3A+">Report a Bug</a>
·
<a href="https://github.com/trustgraph/amazing-github-template/issues/new?assignees=&labels=enhancement&template=02_FEATURE_REQUEST.md&title=feat%3A+">Request a Feature</a>
·
<a href="https://github.com/trustgraph/amazing-github-template/discussions">Ask a Question</a>




## Show your support

Give a ⭐️ if you like the project!

## 📝 License

Copyright © 2022 [Harlan T Wood (https://github.com/harlantwood)](https://github.com/trustgraph).<br />
This project is [Apache-2.0](https://github.com/trustgraph/js-trustgraph-core/blob/master/LICENSE) licensed.
