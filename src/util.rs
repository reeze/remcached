

use std::ascii;

// Take from tikv
pub fn escape(data: &[u8]) -> String {
    let mut escaped = Vec::with_capacity(data.len() * 4);
    for &c in data {
        match c {
            b'"' => escaped.extend_from_slice(b"\\\""),
            b'\\' => escaped.extend_from_slice(br"\\"),
            b'\'' => escaped.push(b'\''),
            _ if c > b'\x7e' => {
                escaped.push(b'\\');
                escaped.push(b'0' + (c >> 6));
                escaped.push(b'0' + ((c >> 3) & 7));
                escaped.push(b'0' + (c & 7));
            }
            _ => escaped.extend(ascii::escape_default(c)),
        }
    }
    escaped.shrink_to_fit();
    unsafe { String::from_utf8_unchecked(escaped) }
}
