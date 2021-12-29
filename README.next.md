# HoloTrust

HoloTrust is a Rust library, intended to allow for [Hololchain](https://www.holochain.org) developers to easily use the [Trust Graph](https://github.com/trustgraph/trustgraph) protocol in their Happs.

## Usage

```rs
use trust_graph::prelude::*;

let target: EntryHashB64 = ... // TODO
let content: String = "sushi".into()
let value: f32 = 0.9
let attributes: BTreeMap<String, String> = ... // TODO

let trust_atom = TrustAtom.create(
  target: target,
  content: content,
  value: value,
  attributes: attributes,
)

```

## Data format

It encodes TrustAtoms as links, with the following components:

1. Holochain Link `base` == TrustAtom `source` - one of:
    - creating agent (`AgentPubKeyB64`) - TODO source must be an entry - do we need to make an entry pointing to the current user?
    - TrustGraph (`EntryHashB64`) (described below)
1. Holochain Link `target` == TrustAtom `target` - entity being rated/reviewed/etc - one of:
    - `EntryHashB64`
    - `AgentPubKeyB64` (?)
1. Holochain Link `tag` - one or more of:
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
