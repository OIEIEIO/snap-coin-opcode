// -----------------------------------------------------------------------------
// File: src/dictionary.rs
// Project: snap-coin-opcode
// Description: Load and query the SNAP opcode dictionary - SOT for all encode/decode
// Version: 0.1.0
// -----------------------------------------------------------------------------

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DictionaryEntry {
    pub r#type: String,
    pub family: String,
    pub opcode: String,
    pub amount: String,
    pub category: String,
    pub meaning: String,
    pub display: Option<String>,
    pub answer_family: Option<String>,
    pub answer_opcode_start: Option<String>,
    pub answers_question: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DictionaryFile {
    pub version: String,
    pub description: String,
    pub families: HashMap<String, String>,
    pub entries: HashMap<String, DictionaryEntry>,
}

#[derive(Debug, Clone)]
pub struct Dictionary {
    pub version: String,
    by_token: HashMap<String, DictionaryEntry>,
    by_amount: HashMap<String, String>,
}

impl Dictionary {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let data = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read dictionary file: {}", e))?;

        let file: DictionaryFile = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse dictionary JSON: {}", e))?;

        let mut by_amount: HashMap<String, String> = HashMap::new();
        for (token, entry) in &file.entries {
            by_amount.insert(entry.amount.clone(), token.clone());
        }

        Ok(Self {
            version: file.version,
            by_token: file.entries,
            by_amount,
        })
    }

    pub fn lookup_token(&self, token: &str) -> Option<&DictionaryEntry> {
        self.by_token.get(token)
    }

    pub fn lookup_amount(&self, amount: &str) -> Option<&DictionaryEntry> {
        let token = self.by_amount.get(amount)?;
        self.by_token.get(token)
    }

    pub fn token_name_by_amount(&self, amount: &str) -> Option<&str> {
        self.by_amount.get(amount).map(|s| s.as_str())
    }

    pub fn all_entries(&self) -> &HashMap<String, DictionaryEntry> {
        &self.by_token
    }

    pub fn entries_by_category(&self, category: &str) -> Vec<(&String, &DictionaryEntry)> {
        self.by_token
            .iter()
            .filter(|(_, e)| e.category == category)
            .collect()
    }

    pub fn answers_for_question(&self, question_opcode: &str) -> Vec<(&String, &DictionaryEntry)> {
        self.by_token
            .iter()
            .filter(|(_, e)| {
                e.answers_question.as_deref() == Some(question_opcode)
            })
            .collect()
    }
}

// -----------------------------------------------------------------------------
// File: src/dictionary.rs
// Project: snap-coin-opcode
// Created: 2026-03-19
// -----------------------------------------------------------------------------