//! # HOMAYA Algorithms Showcase
//!
//! Demonstrating the power of quantum algorithms.
//!
//! Run with: `cargo run --example algorithms_showcase`

use homaya_sim::Simulator;
use homaya_algorithms::{
    GroverSearch,
    DeutschJozsa,
    deutsch::FunctionType,
    BernsteinVazirani,
};

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║              HOMAYA Quantum Algorithms Showcase               ║");
    println!("║           Some connections transcend distance                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    grover_demo();
    println!();
    deutsch_jozsa_demo();
    println!();
    bernstein_vazirani_demo();
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    All demonstrations complete                 ");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Learn how these algorithms work    → https://bskiller.com");
    println!("Enterprise quantum solutions       → https://dataxlr8.ai");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn grover_demo() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("GROVER'S SEARCH ALGORITHM");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Problem: Find item 5 in an unsorted database of 8 items.");
    println!("Classical: O(N) queries needed on average");
    println!("Quantum:   O(√N) queries - that's just 2 iterations!");
    println!();

    let target = 5;
    let grover = GroverSearch::new(3, target);

    println!("Configuration:");
    println!("  • Search space: 8 items (3 qubits)");
    println!("  • Target: {}", target);
    println!("  • Optimal iterations: {}", grover.optimal_iterations());
    println!("  • Theoretical success probability: {:.1}%", grover.success_probability() * 100.0);
    println!();

    let circuit = grover.build();
    let mut sim = Simulator::new();
    let counts = sim.sample(&circuit, 1000).unwrap();

    println!("Results (1000 measurements):");
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    for (state, count) in sorted.iter().take(5) {
        let decimal = usize::from_str_radix(state, 2).unwrap();
        let marker = if decimal == target { " ← TARGET FOUND!" } else { "" };
        println!("  |{}⟩ ({}): {} times ({:.1}%){}", state, decimal, count, **count as f64 / 10.0, marker);
    }
    println!();
    println!("✓ Quantum speedup demonstrated!");
}

fn deutsch_jozsa_demo() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("DEUTSCH-JOZSA ALGORITHM");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Problem: Determine if a function is CONSTANT or BALANCED.");
    println!("Classical: Up to 2^(n-1) + 1 queries in worst case");
    println!("Quantum:   Just ONE query!");
    println!();

    let mut sim = Simulator::new();

    // Test constant function
    println!("Testing CONSTANT function (f(x) = 0 for all x):");
    let dj_const = DeutschJozsa::new(3, FunctionType::ConstantZero);
    let circuit = dj_const.build();
    let counts = sim.sample(&circuit, 100).unwrap();

    for (state, count) in &counts {
        // Only look at first 3 bits (query qubits)
        let query_bits = &state[..3];
        let is_const = DeutschJozsa::is_constant(query_bits);
        println!("  Measured: {} → {}", query_bits, if is_const { "CONSTANT" } else { "BALANCED" });
        if *count == 100 {
            break;
        }
    }

    // Test balanced function
    println!();
    println!("Testing BALANCED function (f(x) = parity of x):");
    let dj_bal = DeutschJozsa::new(3, FunctionType::BalancedParity);
    let circuit = dj_bal.build();
    let counts = sim.sample(&circuit, 100).unwrap();

    for (state, count) in &counts {
        let query_bits = &state[..3];
        let is_const = DeutschJozsa::is_constant(query_bits);
        println!("  Measured: {} → {}", query_bits, if is_const { "CONSTANT" } else { "BALANCED" });
        if *count > 50 {
            break;
        }
    }
    println!();
    println!("✓ One query tells us everything!");
}

fn bernstein_vazirani_demo() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("BERNSTEIN-VAZIRANI ALGORITHM");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Problem: Find a hidden n-bit string s.");
    println!("Classical: Need n queries (one per bit)");
    println!("Quantum:   ONE query reveals the entire string!");
    println!();

    let secret = 0b1011;  // Binary: 1011 = 11 in decimal
    let bv = BernsteinVazirani::new(4, secret);

    println!("Configuration:");
    println!("  • Hidden string: {} ({})", bv.secret_as_binary(), secret);
    println!("  • String length: 4 bits");
    println!();

    let circuit = bv.build();
    let mut sim = Simulator::new();
    let counts = sim.sample(&circuit, 100).unwrap();

    println!("Results (100 measurements):");
    for (state, count) in &counts {
        // Only look at first 4 bits (query qubits)
        let query_bits = &state[..4];
        let found = usize::from_str_radix(query_bits, 2).unwrap();
        let correct = found == secret;
        println!(
            "  Measured: {} ({}) - {} times {}",
            query_bits,
            found,
            count,
            if correct { "✓ SECRET FOUND!" } else { "" }
        );
    }
    println!();
    println!("✓ Hidden string revealed in one shot!");
}
