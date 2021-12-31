# HoloTrust

HoloTrust is a Rust library, intended to allow for [Hololchain](https://www.holochain.org) developers to easily use the [Trust Graph](https://github.com/trustgraph/trustgraph) protocol in their Happs.

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

let target: EntryHashB64 = "...".into(); // TODO
let content: String = "sushi".into();
let value: f32 = 0.8;
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

let trust_atoms: Vector<TrustAtom> = trust_graph.highest(
  content_starts_with: "Category~Pop~80s".into(),
  exclude: vec![
    // TA's which client has already seen, ie previous "pages" (or screens in infinite scroll)
  ]
)
// TODO ask HC core team about pagination support <-- and/or max # results for link queries
// what if a million links?  a billion?

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
    - creating agent (`AgentPubKeyB64`) - TODO source must be an entry - do we need to make an entry pointing to the current user?
    - TrustGraph (`EntryHashB64`) (described below)
1. Holochain Link `target` == TrustAtom `target` - entity being rated/reviewed/etc - one of:
    - `EntryHashB64`
    - `AgentPubKeyB64` (?)
1. Holochain Link `tag` - formatted as UTF-8 string: one or more of:
  - TrustAtom `content` - semantic info (eg sushi) - max 100 chars
  - TrustAtom `value` - rating ( `"-0.999999999"` to `"0.999999999"`) - max 12 chars
  - additional attributes - `EntryHashB64`, where the entry contains attributes formatted in: `BTreeMap<String, String>`

## Link Tags

This format is designed to allow us to encode trust atoms as Holochain links, and search them by their tags.  Holochain can search for all links _starting_ with a given set of bytes (characters).

Each link tag is a UTF8 string:
1. optional: semantic content -- if hierarchical chunks, separated by `~`
    - example: `"Category~Pop~80s~Boy Band"`
1. required: null byte (x00)
1. optional: value as a string -- `"-0.999999999"` to `"0.999999999"`
1. required: null byte (x00)

Therefore, the minimum link tag is two null bytes: `[x00][x00]`, meaning no semantic content or value is attached.

Example of only semantic content: `"Category~Pop~80s~Boy Band[x00][x00]"`

Example of only value: `"[x00]0.8[x00]"`

*TODO* describe searching, both of semantic content and values

## Scratchpad

```
Add 9-digit random after value:

"Category~Pop~80s~user_ratings[x00]0.9[x00]328425615[x00]uHEntityId"
"Category~Pop~user_ratings[x00]0.9[x00]328425615[x00]uHEntityId"
"user_ratings[x00]0.9[x00]328425615[x00]uHEntityId"

---

Link Tags:

"2021-12-29T00:00:234"

"Category~Pop~80s[x00]0.999999999[x00]"
"[x00]0.999999999[x00]" // thumbs up
"[x00]0.0[x00]" // thumbs down
"[x00]-0.999999999[x00]" // flag/spam/abuse

"drums[x00]0.999999999[x00]"

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
