// -----------------------------------------------------------------------------
// File: examples/dict_inspect.rs
// Project: snap-coin-opcode
// Description: Print all dictionary entries grouped by family and category
// Version: 0.1.0
// -----------------------------------------------------------------------------

use snap_coin_opcode::Dictionary;

fn main() {
    let dict = Dictionary::load("dictionary/dictionary.json").expect("failed to load dictionary");

    println!("\n=== SNAP Opcode Dictionary v{} ===\n", dict.version);

    let mut categories: Vec<String> = dict
        .all_entries()
        .values()
        .map(|e| e.category.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    categories.sort();

    for category in &categories {
        println!("--- {} ---", category.to_uppercase());

        let mut entries: Vec<_> = dict.entries_by_category(category);
        entries.sort_by(|a, b| a.1.amount.cmp(&b.1.amount));

        for (token, entry) in entries {
            let display = entry.display.as_deref().unwrap_or(token.as_str());
            println!(
                "  {}  {:>20}  {}  {}",
                entry.amount,
                token,
                display,
                entry.meaning
            );
        }
        println!();
    }

    println!("=== Total: {} opcodes ===\n", dict.all_entries().len());
}

// -----------------------------------------------------------------------------
// File: examples/dict_inspect.rs
// Project: snap-coin-opcode
// Created: 2026-03-19
// -----------------------------------------------------------------------------