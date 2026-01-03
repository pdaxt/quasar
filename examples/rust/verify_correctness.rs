//! HOMAYA Correctness Verification
//!
//! This demonstrates WHY the simulator is correct by checking
//! mathematical properties that MUST hold for quantum mechanics.
//!
//! Run with: cargo run --example verify_correctness -p homaya-sim

use homaya_core::Circuit;
use homaya_sim::Simulator;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║              HOMAYA Correctness Verification                  ║");
    println!("║         Proving the simulator obeys quantum mechanics          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let mut all_passed = true;

    // =========================================================================
    // TEST 1: Probability Conservation (Born Rule)
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 1: Probability Conservation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Law: All probabilities must sum to exactly 1.0");
    println!("Why: Quantum states are normalized (Born rule)");
    println!();

    let circuits = vec![
        ("Empty circuit", Circuit::new(3)),
        ("All H gates", Circuit::new(3).h(0).h(1).h(2)),
        ("Bell state", Circuit::new(2).h(0).cx(0, 1)),
        ("GHZ state", Circuit::new(3).h(0).cx(0, 1).cx(1, 2)),
        ("Complex rotations", Circuit::new(2)
            .rx(std::f64::consts::PI / 4.0, 0)
            .ry(std::f64::consts::PI / 3.0, 1)
            .cz(0, 1)),
    ];

    for (name, circuit) in &circuits {
        let mut sim = Simulator::new();
        let state = sim.run(circuit).unwrap();
        let total_prob: f64 = state.probabilities().iter().sum();
        let passed = (total_prob - 1.0).abs() < 1e-10;
        let status = if passed { "✓ PASS" } else { "✗ FAIL" };
        println!("  {}: Sum = {:.10} {}", name, total_prob, status);
        all_passed &= passed;
    }
    println!();

    // =========================================================================
    // TEST 2: Unitary Reversibility (H·H = I)
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 2: Unitary Reversibility (H² = I)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Law: Applying Hadamard twice returns to original state");
    println!("Why: H is its own inverse (H† = H, H·H = I)");
    println!();

    let circuit = Circuit::new(1).h(0).h(0);
    let mut sim = Simulator::new();
    let state = sim.run(&circuit).unwrap();

    let prob_0 = state.probability(0);
    let prob_1 = state.probability(1);
    let passed = prob_0 > 0.999 && prob_1 < 0.001;
    println!("  |0⟩ → H → H → ?");
    println!("  P(|0⟩) = {:.6} (expected: 1.0)", prob_0);
    println!("  P(|1⟩) = {:.6} (expected: 0.0)", prob_1);
    println!("  {} - State returned to |0⟩", if passed { "✓ PASS" } else { "✗ FAIL" });
    all_passed &= passed;
    println!();

    // =========================================================================
    // TEST 3: X² = I (Pauli-X is self-inverse)
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 3: Pauli-X Self-Inverse (X² = I)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Law: Applying X twice returns to original state");
    println!("Why: X flips |0⟩↔|1⟩, so X·X = I");
    println!();

    let circuit = Circuit::new(1).x(0).x(0);
    let mut sim = Simulator::new();
    let state = sim.run(&circuit).unwrap();

    let passed = state.probability(0) > 0.999;
    println!("  |0⟩ → X → X → |0⟩");
    println!("  P(|0⟩) = {:.6}", state.probability(0));
    println!("  {} - Correct", if passed { "✓ PASS" } else { "✗ FAIL" });
    all_passed &= passed;
    println!();

    // =========================================================================
    // TEST 4: Rotation by 2π = Identity
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 4: Full Rotation Returns to Origin (Rx(2π) ≈ I)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Law: Rotating 360° around any axis returns to original state");
    println!("Why: exp(-i·2π·σ/2) = -I (global phase, same state)");
    println!();

    let tau = 2.0 * std::f64::consts::PI;
    let circuit = Circuit::new(1).rx(tau, 0);
    let mut sim = Simulator::new();
    let state = sim.run(&circuit).unwrap();

    // Note: Rx(2π) gives -|0⟩, which has same probability as |0⟩
    let passed = state.probability(0) > 0.999;
    println!("  |0⟩ → Rx(2π) → ?");
    println!("  P(|0⟩) = {:.6} (expected: 1.0, may have global phase -1)", state.probability(0));
    println!("  {} - Correct (global phase doesn't affect measurement)", if passed { "✓ PASS" } else { "✗ FAIL" });
    all_passed &= passed;
    println!();

    // =========================================================================
    // TEST 5: Bell State Correlations
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 5: Bell State Perfect Correlation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Law: Bell state (|00⟩+|11⟩)/√2 has ONLY |00⟩ and |11⟩");
    println!("Why: Qubits are maximally entangled - always measure same value");
    println!();

    let circuit = Circuit::new(2).h(0).cx(0, 1);
    let mut sim = Simulator::new();
    let state = sim.run(&circuit).unwrap();

    let p00 = state.probability(0b00);
    let p01 = state.probability(0b01);
    let p10 = state.probability(0b10);
    let p11 = state.probability(0b11);

    println!("  Probabilities:");
    println!("    |00⟩: {:.6} (expected: 0.5)", p00);
    println!("    |01⟩: {:.6} (expected: 0.0)", p01);
    println!("    |10⟩: {:.6} (expected: 0.0)", p10);
    println!("    |11⟩: {:.6} (expected: 0.5)", p11);

    let passed = (p00 - 0.5).abs() < 0.01
        && p01 < 0.001
        && p10 < 0.001
        && (p11 - 0.5).abs() < 0.01;
    println!("  {} - Perfect entanglement verified", if passed { "✓ PASS" } else { "✗ FAIL" });
    all_passed &= passed;
    println!();

    // =========================================================================
    // TEST 6: CNOT Truth Table
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 6: CNOT Gate Truth Table");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Law: CNOT flips target IFF control is |1⟩");
    println!("Why: Controlled-NOT is fundamental to quantum computing");
    println!("Note: State index bit i corresponds to qubit i");
    println!();

    // State indices: bit 0 = qubit 0 (control), bit 1 = qubit 1 (target)
    // |q1 q0⟩ format, so index 0b01 = qubit0=1, qubit1=0
    let test_cases = [
        // (ctrl, tgt, input_idx, expected_idx)
        (0, 0, 0b00, 0b00), // ctrl=0,tgt=0 → ctrl=0,tgt=0 (no flip)
        (0, 1, 0b10, 0b10), // ctrl=0,tgt=1 → ctrl=0,tgt=1 (no flip, ctrl is 0)
        (1, 0, 0b01, 0b11), // ctrl=1,tgt=0 → ctrl=1,tgt=1 (flip!)
        (1, 1, 0b11, 0b01), // ctrl=1,tgt=1 → ctrl=1,tgt=0 (flip!)
    ];

    let mut cnot_passed = true;
    for (ctrl, tgt, input_idx, expected_idx) in test_cases {
        let mut circuit = Circuit::new(2);
        if ctrl == 1 { circuit = circuit.x(0); } // Set control qubit
        if tgt == 1 { circuit = circuit.x(1); }  // Set target qubit
        circuit = circuit.cx(0, 1); // CNOT: control=0, target=1

        let mut sim = Simulator::new();
        let state = sim.run(&circuit).unwrap();

        let actual_idx = state.probabilities()
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;

        let correct = actual_idx == expected_idx;
        cnot_passed &= correct;

        // Format: |target control⟩ for display (q1 q0)
        let fmt_idx = |idx: usize| format!("|{}{}⟩", (idx >> 1) & 1, idx & 1);
        println!("  {} → {} (expected {}) {}",
            fmt_idx(input_idx), fmt_idx(actual_idx), fmt_idx(expected_idx),
            if correct { "✓" } else { "✗" });
    }
    println!("  {} - CNOT truth table verified", if cnot_passed { "✓ PASS" } else { "✗ FAIL" });
    all_passed &= cnot_passed;
    println!();

    // =========================================================================
    // TEST 7: Statistical Sampling Matches Theory
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 7: Statistical Sampling Matches Theory");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Law: Repeated measurements follow Born rule probabilities");
    println!("Why: |⟨ψ|i⟩|² gives probability of measuring state |i⟩");
    println!();

    let circuit = Circuit::new(2).h(0).cx(0, 1).measure_all();
    let mut sim = Simulator::with_seed(12345);
    let counts = sim.sample(&circuit, 10000).unwrap();

    let n00 = *counts.get("00").unwrap_or(&0) as f64;
    let n11 = *counts.get("11").unwrap_or(&0) as f64;
    let n01 = *counts.get("01").unwrap_or(&0) as f64;
    let n10 = *counts.get("10").unwrap_or(&0) as f64;

    println!("  10,000 measurements of Bell state:");
    println!("    |00⟩: {} ({:.1}%, expected ~50%)", n00 as u64, n00 / 100.0);
    println!("    |11⟩: {} ({:.1}%, expected ~50%)", n11 as u64, n11 / 100.0);
    println!("    |01⟩: {} (expected 0)", n01 as u64);
    println!("    |10⟩: {} (expected 0)", n10 as u64);

    // Check within 3 standard deviations (99.7% confidence)
    // For binomial with p=0.5, n=10000, stddev ≈ 50
    let expected = 5000.0;
    let tolerance = 300.0; // ~6 sigma
    let passed = (n00 - expected).abs() < tolerance
        && (n11 - expected).abs() < tolerance
        && n01 < 1.0 && n10 < 1.0;
    println!("  {} - Sampling matches Born rule", if passed { "✓ PASS" } else { "✗ FAIL" });
    all_passed &= passed;
    println!();

    // =========================================================================
    // TEST 8: Superposition Creates Equal Probabilities
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 8: Hadamard Creates Equal Superposition");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Law: H|0⟩ = (|0⟩+|1⟩)/√2 gives exactly 50/50 probability");
    println!("Why: |1/√2|² = 0.5 for both amplitudes");
    println!();

    let circuit = Circuit::new(1).h(0);
    let mut sim = Simulator::new();
    let state = sim.run(&circuit).unwrap();

    let p0 = state.probability(0);
    let p1 = state.probability(1);
    let passed = (p0 - 0.5).abs() < 1e-10 && (p1 - 0.5).abs() < 1e-10;

    println!("  P(|0⟩) = {:.10} (expected: 0.5)", p0);
    println!("  P(|1⟩) = {:.10} (expected: 0.5)", p1);
    println!("  {} - Exact equal superposition", if passed { "✓ PASS" } else { "✗ FAIL" });
    all_passed &= passed;
    println!();

    // =========================================================================
    // FINAL VERDICT
    // =========================================================================
    println!("═══════════════════════════════════════════════════════════════");
    if all_passed {
        println!("                    ALL TESTS PASSED ✓                        ");
        println!("         The simulator correctly implements quantum mechanics  ");
    } else {
        println!("                    SOME TESTS FAILED ✗                       ");
        println!("                  Review the failures above                    ");
    }
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Learn the physics behind these tests → https://bskiller.com");
    println!("Enterprise quantum solutions         → https://dataxlr8.ai");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
