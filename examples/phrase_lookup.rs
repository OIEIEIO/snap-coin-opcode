// -----------------------------------------------------------------------------
// File: examples/phrase_lookup.rs
// Project: snap-coin-opcode
// Description: Show all phrase opcodes available in the dictionary
// Version: 0.1.0
// -----------------------------------------------------------------------------

use snap_coin_opcode::Dictionary;

fn main() {
    let dict = Dictionary::load("dictionary/dictionary.json").expect("failed to load dictionary");

    println!("\n=== SNAP Opcode Phrases v{} ===\n", dict.version);

    let mut phrases: Vec<_> = dict
        .all_entries()
        .iter()
        .filter(|(_, e)| e.r#type == "phrase")
        .collect();

    phrases.sort_by(|a, b| a.1.amount.cmp(&b.1.amount));

    println!(
        "{:<16}  {:<24}  {:<28}  {}",
        "AMOUNT", "TOKEN", "DISPLAY", "MEANING"
    );
    println!("{}", "-".repeat(90));

    for (token, entry) in &phrases {
        let display = entry.display.as_deref().unwrap_or(token.as_str());
        println!(
            "{:<16}  {:<24}  {:<28}  {}",
            entry.amount,
            token,
            display,
            entry.meaning
        );
    }

    println!("\n=== Total: {} phrases ===\n", phrases.len());
}

// -----------------------------------------------------------------------------
// File: examples/phrase_lookup.rs
// Project: snap-coin-opcode
// Created: 2026-03-19
// -----------------------------------------------------------------------------