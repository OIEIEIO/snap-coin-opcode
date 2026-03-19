// -----------------------------------------------------------------------------
// File: src/decoder.rs
// Project: snap-coin-opcode
// Description: Decode incoming snap-coin-pay tx amounts into opcode tokens
// Version: 0.1.0
// -----------------------------------------------------------------------------

use crate::dictionary::{Dictionary, DictionaryEntry};

#[derive(Debug, Clone)]
pub struct DecodedOpcode {
    pub token: String,
    pub amount: String,
    pub meaning: String,
    pub category: String,
    pub r#type: String,
    pub display: Option<String>,
    pub answers_question: Option<String>,
    pub answer_family: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DecodedMessage {
    pub opcodes: Vec<DecodedOpcode>,
    pub is_complete: bool,
}

pub struct Decoder<'a> {
    dictionary: &'a Dictionary,
}

impl<'a> Decoder<'a> {
    pub fn new(dictionary: &'a Dictionary) -> Self {
        Self { dictionary }
    }

    // decode a single tx amount from snap-coin-pay on_confirmation
    pub fn decode_amount(&self, raw_amount: u64) -> Option<DecodedOpcode> {
        let amount_str = format_amount(raw_amount);
        let entry = self.dictionary.lookup_amount(&amount_str)?;
        Some(build_decoded_opcode(amount_str, entry))
    }

    // decode a full sequence of amounts into a message
    // is_complete is true if END opcode is present
    pub fn decode_message(&self, raw_amounts: &[u64]) -> DecodedMessage {
        let mut opcodes: Vec<DecodedOpcode> = Vec::new();
        let mut is_complete = false;

        for &raw in raw_amounts {
            let amount_str = format_amount(raw);

            match self.dictionary.lookup_amount(&amount_str) {
                Some(entry) => {
                    if entry.r#type == "control" && entry.token_is_end() {
                        is_complete = true;
                        break;
                    }
                    opcodes.push(build_decoded_opcode(amount_str, entry));
                }
                None => {
                    // unknown opcode - skip but could log here
                }
            }
        }

        DecodedMessage {
            opcodes,
            is_complete,
        }
    }

    // check if a single amount is a question opcode
    // used by UI to know when to switch keyboard to answer family
    pub fn is_question(&self, raw_amount: u64) -> bool {
        let amount_str = format_amount(raw_amount);
        self.dictionary
            .lookup_amount(&amount_str)
            .map(|e| e.category == "question")
            .unwrap_or(false)
    }

    // check if amount is the END control opcode
    pub fn is_end(&self, raw_amount: u64) -> bool {
        let amount_str = format_amount(raw_amount);
        self.dictionary
            .lookup_amount(&amount_str)
            .map(|e| e.r#type == "control")
            .unwrap_or(false)
    }
}

// format u64 nano amount back to "0.XXXXXXXX" string for dictionary lookup
fn format_amount(raw: u64) -> String {
    format!("0.{:08}", raw)
}

fn build_decoded_opcode(amount_str: String, entry: &DictionaryEntry) -> DecodedOpcode {
    DecodedOpcode {
        token: entry.r#type.clone(),
        amount: amount_str,
        meaning: entry.meaning.clone(),
        category: entry.category.clone(),
        r#type: entry.r#type.clone(),
        display: entry.display.clone(),
        answers_question: entry.answers_question.clone(),
        answer_family: entry.answer_family.clone(),
    }
}

// helper trait to avoid repeating END check
trait IsEnd {
    fn token_is_end(&self) -> bool;
}

impl IsEnd for DictionaryEntry {
    fn token_is_end(&self) -> bool {
        self.opcode == "9999" && self.family == "00"
    }
}

// -----------------------------------------------------------------------------
// File: src/decoder.rs
// Project: snap-coin-opcode
// Created: 2026-03-19
// -----------------------------------------------------------------------------