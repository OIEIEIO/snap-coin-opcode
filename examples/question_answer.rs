// -----------------------------------------------------------------------------
// File: examples/question_answer.rs
// Project: snap-coin-opcode
// Description: Show question opcodes and their corresponding answer opcodes
// Version: 0.1.0
// -----------------------------------------------------------------------------

use snap_coin_opcode::Dictionary;

fn main() {
    let dict = Dictionary::load("dictionary/dictionary.json").expect("failed to load dictionary");

    println!("\n=== SNAP Opcode Question / Answer Pairs v{} ===\n", dict.version);

    let mut questions: Vec<_> = dict
        .all_entries()
        .iter()
        .filter(|(_, e)| e.category == "question")
        .collect();

    questions.sort_by(|a, b| a.1.amount.cmp(&b.1.amount));

    for (token, entry) in &questions {
        println!("Q  {}  {}  {}", entry.amount, token, entry.meaning);

        let opcode = entry.opcode.as_str();
        let mut answers: Vec<_> = dict.answers_for_question(opcode);
        answers.sort_by(|a, b| a.1.amount.cmp(&b.1.amount));

        if answers.is_empty() {
            println!("   no answers defined");
        } else {
            for (answer_token, answer_entry) in &answers {
                println!(
                    "   A  {}  {}  {}",
                    answer_entry.amount, answer_token, answer_entry.meaning
                );
            }
        }
        println!();
    }

    println!("=== Total: {} questions ===\n", questions.len());
}

// -----------------------------------------------------------------------------
// File: examples/question_answer.rs
// Project: snap-coin-opcode
// Created: 2026-03-19
// -----------------------------------------------------------------------------