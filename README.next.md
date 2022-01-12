# TrustGraph::Holochain

TrustGraph::Holochain is a Rust library, intended to allow for [Hololchain](https://www.holochain.org) developers to easily use the [Trust Graph](https://github.com/trustgraph/trustgraph) protocol in their Happs.

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

let target: EntryHashB64 = "...".into(); // TODO // coming soon: HC HREL format hc://appid//hashid
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
    - TrustGraph (`EntryHashB64`) (described below)
1. Holochain Link `target` == TrustAtom `target` - entity being rated/reviewed/etc - one of:
    - `EntryHashB64`
    - `AgentPubKeyB64` (?)
1. Holochain Link `tag` (max 999 bytes) - formatted as UTF-8 string: one or more of:
  - TrustAtom header bytes: `[0xC5][0xA6]` (which together comprise the unicode character `Ŧ`)
  - Direction byte:
      - `[0x21][0x92]` (unicode `→`) means: HC target = TA target
      - `[0x21][0xA9]` (unicode `↩`) means: HC target = TA source
  - TrustAtom `content` - semantic info (eg sushi) - max 9xx bytes (max we can fit!)
  - Separator: null byte `[0x00]`
  - TrustAtom `value` - rating ( `"-0.999999999"` to `"0.999999999"`) - max 12 chars
  - Separator: null byte `[0x00]`
  - Random 9 characters for bucketing purposes
  - Separator: null byte `[0x00]`
  - Canonical data including additional attributes - `EntryHashB64`
      - Entry contains attributes formatted in: `BTreeMap<String, String>`
      - You will find full content here; if content exceeds link tag limts it ends with `…` as a hint
      - If value is 1.0, we use "0.999999999" in link tag, but 1.0 here
      - ~~Entry hash is raw bytes, not a string representation converted to bytes~~

// search on base,
// so have links in both directions?
// or at the happ level, you can choose
// which you treat as base
// maybe reserve 1 byte at beginning -- bitmask which encodes direction and other things

// maybe attributes contain source and target info, esp do not exist in DHT

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

## Scratchpad

```
Add 9-digit random after value:

"Category~Pop~80s~user_ratings[0x00]0.9[0x00]328425615[0x00]uHEntityId"
"Category~Pop~user_ratings[0x00]0.9[0x00]328425615[0x00]uHEntityId"
"user_ratings[0x00]0.9[0x00]328425615[0x00]uHEntityId"


"Ŧ→Category~Pop~80s~user_ratings�0.9�328425615�uHEntityId"

"Ŧ→pop�0.999999999�328425615�"
"Ŧ→pop�0.999999999��uHEntityId"

---

Link Tags:

"2021-12-29T00:00:234"

"Category~Pop~80s[0x00]0.999999999[0x00]"
"[0x00]0.999999999[0x00]" // thumbs up
"[0x00]0.0[0x00]" // thumbs down
"[0x00]-0.999999999[0x00]" // flag/spam/abuse

"drums[0x00]0.999999999[0x00]"

// sort by sales
// sort by artist
// sort by album
// sort by recently rated?
```

---

If a thumbs up is a "1" value (perfect score), what rollup score should we assign to an album with 30k thumbs ups vs another with 30 thumbs ups?  This invokes the reputon field [`sample-size`](https://datatracker.ietf.org/doc/html/rfc7071#section-6.3) -- "how many ratings this reputon is synthesized from"...

----

Guardian has Sally's privately shared TG in their TG

- protects her by only publishing rollup TAs, of her and other TGs they follow

What if being invited to a DHT implies access to the next level of DHTs?

hash bound source chain queries
