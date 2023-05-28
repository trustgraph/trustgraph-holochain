pub const UNICODE_NUL_STR: &str = "\u{0}"; // Unicode NUL character
pub const LINK_TAG_HEADER: [u8; 2] = [197, 166]; // Unicode "Ŧ" // hex bytes: [0xC5][0xA6]
pub const LINK_TAG_ARROW_FORWARD: [u8; 3] = [226, 134, 146]; // Unicode "→" // hex bytes: [0xE2][0x86][0x92]
pub const LINK_TAG_ARROW_REVERSE: [u8; 3] = [226, 134, 169]; // Unicode "↩" // hex bytes: [0xE2][0x86][0xA9]

pub fn build_forward_header() -> Vec<u8> {
  let mut forward_bytes = vec![];
  forward_bytes.extend_from_slice(&LINK_TAG_HEADER);
  forward_bytes.extend_from_slice(&LINK_TAG_ARROW_FORWARD);

  forward_bytes
}

pub fn build_reverse_header() -> Vec<u8> {
  let mut reverse_bytes = vec![];
  reverse_bytes.extend_from_slice(&LINK_TAG_HEADER);
  reverse_bytes.extend_from_slice(&LINK_TAG_ARROW_REVERSE);

  reverse_bytes
}
