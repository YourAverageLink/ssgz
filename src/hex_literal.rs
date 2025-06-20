// Convenience macro for making hex bytes literal
#[macro_export]
macro_rules! hex {
    ($hex:literal) => {{
        const HEX_STR: &str = $hex;
        const LEN: usize = HEX_STR.len();

        const _: () = assert!(
            LEN % 2 == 0,
            "Must use even number of digits for full bytes."
        );

        const BYTE_LEN: usize = LEN / 2;
        const fn hex_to_bytes() -> [u8; BYTE_LEN] {
            let hex_bytes = HEX_STR.as_bytes();
            let mut result = [0u8; BYTE_LEN];
            let mut i = 0;

            while i < BYTE_LEN {
                let high = hex_char_to_nibble(hex_bytes[i * 2]);
                let low = hex_char_to_nibble(hex_bytes[i * 2 + 1]);
                result[i] = (high << 4) | low;
                i += 1;
            }
            result
        }

        const fn hex_char_to_nibble(c: u8) -> u8 {
            match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                b'A'..=b'F' => c - b'A' + 10,
                _ => panic!("Invalid hex character"),
            }
        }

        hex_to_bytes()
    }};
}
