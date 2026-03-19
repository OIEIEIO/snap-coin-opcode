// -----------------------------------------------------------------------------
// File: src/lib.rs
// Project: snap-coin-opcode
// Description: SNAP Coin opcode encoder/decoder library - public API
// Version: 0.1.0
// -----------------------------------------------------------------------------

pub mod dictionary;
pub mod compiler;
pub mod decoder;

pub use dictionary::{Dictionary, DictionaryEntry};
pub use compiler::{Compiler, CompiledMessage};
pub use decoder::{Decoder, DecodedOpcode, DecodedMessage};


#[cfg(test)]
mod tests {
    use super::*;

    fn load_dict() -> Dictionary {
        Dictionary::load("dictionary/dictionary.json").expect("failed to load dictionary")
    }

    #[test]
    fn test_dictionary_loads() {
        let dict = load_dict();
        assert!(!dict.version.is_empty());
    }

    #[test]
    fn test_compile_hello() {
        let dict = load_dict();
        let compiler = Compiler::new(&dict);
        let result = compiler.compile(&["HELLO"]).expect("compile failed");
        // HELLO + END = 2 amounts
        assert_eq!(result.amounts.len(), 2);
        assert_eq!(result.tokens[0], "HELLO");
        assert_eq!(result.tokens[1], "END");
    }

    #[test]
    fn test_decode_hello() {
        let dict = load_dict();
        let compiler = Compiler::new(&dict);
        let decoder = Decoder::new(&dict);
        let compiled = compiler.compile(&["HELLO"]).expect("compile failed");
        let decoded = decoder.decode_message(&compiled.amounts);
        assert!(decoded.is_complete);
        assert_eq!(decoded.opcodes.len(), 1);
        assert_eq!(decoded.opcodes[0].meaning, "Initiate contact");
    }

    #[test]
    fn test_roundtrip_meet_tomorrow_1400() {
        let dict = load_dict();
        let compiler = Compiler::new(&dict);
        let decoder = Decoder::new(&dict);
        let compiled = compiler.compile(&["MEET_TOMORROW_1400"]).expect("compile failed");
        let decoded = decoder.decode_message(&compiled.amounts);
        assert!(decoded.is_complete);
        assert_eq!(decoded.opcodes[0].meaning, "Meeting requested tomorrow at 14:00");
    }

    #[test]
    fn test_question_detected() {
        let dict = load_dict();
        let compiler = Compiler::new(&dict);
        let decoder = Decoder::new(&dict);
        let compiled = compiler.compile(&["AVAILABLE?"]).expect("compile failed");
        assert!(decoder.is_question(compiled.amounts[0]));
    }

    #[test]
    fn test_unknown_amount_skipped() {
        let dict = load_dict();
        let decoder = Decoder::new(&dict);
        // 99999999 is not in dictionary
        let decoded = decoder.decode_message(&[99999999]);
        assert!(!decoded.is_complete);
        assert_eq!(decoded.opcodes.len(), 0);
    }
}
// -----------------------------------------------------------------------------
// File: src/lib.rs
// Project: snap-coin-opcode
// Created: 2026-03-19
// -----------------------------------------------------------------------------