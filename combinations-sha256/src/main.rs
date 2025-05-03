use itertools::Itertools;
use rayon::prelude::*;
use sha2;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// as appearing in styles.css as classes
const FLAG_FRAGMENTS: [(u8, &str); 15] = [
    (4, "\u{0044}\u{0044}\u{0043}"), // DDC
    (9, "\u{007B}\u{0043}\u{0035}"), // {C5
    (6, "\u{0035}\u{005F}\u{0031}"),
    (2, "\u{0035}\u{005F}\u{0048}"),
    (10, "\u{0034}\u{0052}\u{0044}"),
    (7, "\u{005F}\u{004D}\u{0034}"),
    (5, "\u{004E}\u{005F}\u{0035}"),
    (1, "\u{004B}\u{0031}\u{0038}"),
    (8, "\u{0031}\u{0044}\u{0031}"),
    (3, "\u{007D}\u{0020}\u{0020}"), // `}  ` (two spaces)
    (11, "\u{0034}\u{0034}\u{0034}"),
    (15, "\u{0039}\u{004B}\u{0038}"),
    (12, "\u{0042}\u{0043}\u{0033}"),
    (13, "\u{005A}\u{0035}\u{0034}"),
    (17, "\u{0043}\u{0034}\u{0032}"),
];

fn main() {
    assert_ne!(CORRECT_HASH_BYTES.as_slice(), CORRECT_HASH_STR.as_bytes());

    let mut map = HashMap::from(FLAG_FRAGMENTS);

    let a = map.remove(&4).unwrap(); // `DDC`
    let b = map.remove(&9).unwrap(); // `{C5`
    let _c = map.remove(&3).unwrap(); // `}  `
    
    // known because of cut-off: This is (27-6)/3 (28 length minus start (`DDC{C5`) and end (`}`)
    let k = 7;
    let string = map
        .into_values()
        .permutations(k)
        .enumerate()
        .par_bridge()
        .find_map_any(|(i, x)| {
            let string = format!("{}{}{}{}", a, b, x.join(""), '}');
            if i % 100000 == 0 {
                eprintln!(
                    "Did {} iterations. Arbitrary pick: {}",
                    i,
                    string
                );
            }
            if check_hash(string.as_str()) {
                Some(string)
            } else {
                None
            }
        });
    match string {
        Some(string) => {
            println!("HASH MATCHED! The string is: {}", string);
        }
        None => {
            eprintln!("String not found.");
        }
    }
}

/// Formatted as a series of code points. See function below.
const CORRECT_HASH_STR: &str = "2643e1d88572f77df01a77da0c3752c1f173b4618ab443db60660fac4a05b023";
const CORRECT_HASH_BYTES: [u8; 32] = [
    0x26, 0x43, 0xe1, 0xd8, 0x85, 0x72, 0xf7, 0x7d, 0xf0, 0x1a, 0x77, 0xda, 0x0c, 0x37, 0x52, 0xc1,
    0xf1, 0x73, 0xb4, 0x61, 0x8a, 0xb4, 0x43, 0xdb, 0x60, 0x66, 0x0f, 0xac, 0x4a, 0x05, 0xb0, 0x23,
];

/// Based on the following:
/// ```js
///  // Function to compute SHA-256
///  async function computeSHA256(message) {
///      const encoder = new TextEncoder();
///      const data = encoder.encode(message);
///      const hashBuffer = await crypto.subtle.digest('SHA-256', data);
///      const hashArray = Array.from(new Uint8Array(hashBuffer));
///      return hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('');
///  }
/// ```
///
/// Converts from a string to an array of bytes. Afterward, we write it out as code points?
/// A codepoints are written in a string with hexadecimals: 0b00000000 (0x0) is "00"
///
#[inline]
fn check_hash(message: &str) -> bool {
    // assume message is encoded like we want and that the SHA256 output can be compared to our
    // output (probably incorrect?)
    // string is already utf-8 bytes, we can pass it directly and compare
    Sha256::digest(message.as_bytes()).as_slice() == CORRECT_HASH_BYTES
}
