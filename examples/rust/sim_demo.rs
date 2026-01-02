//! QUASAR Simulator Demo - Quantum State Vector Simulation
//!
//! Run with: cargo run --example sim_demo

use homaya_core::Circuit;
use homaya_sim::Simulator;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    QUASAR Simulator Demo                      ║");
    println!("║       Quantum State Vector Simulation Engine                   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Demo 1: Bell State Simulation
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 1: Bell State Simulation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let bell = Circuit::new(2)
        .h(0)
        .cx(0, 1);

    let mut sim = Simulator::new();
    let state = sim.run(&bell).unwrap();

    println!("Circuit: H(q0) → CNOT(q0, q1)");
    println!();
    println!("State vector probabilities:");
    println!("  |00⟩: {:.4} ({:.1}%)", state.probability(0), state.probability(0) * 100.0);
    println!("  |01⟩: {:.4} ({:.1}%)", state.probability(1), state.probability(1) * 100.0);
    println!("  |10⟩: {:.4} ({:.1}%)", state.probability(2), state.probability(2) * 100.0);
    println!("  |11⟩: {:.4} ({:.1}%)", state.probability(3), state.probability(3) * 100.0);
    println!();
    println!("This is the Bell state: (|00⟩ + |11⟩)/√2 - maximum entanglement!");
    println!();

    // Demo 2: GHZ State with Measurement Sampling
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 2: GHZ State with 1000 Measurement Shots");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let ghz = Circuit::new(3)
        .h(0)
        .cx(0, 1)
        .cx(1, 2)
        .measure_all();

    let mut sim = Simulator::with_seed(42);
    let counts = sim.sample(&ghz, 1000).unwrap();

    println!("Circuit: H(q0) → CNOT(q0,q1) → CNOT(q1,q2) → Measure");
    println!();
    println!("Measurement results (1000 shots):");
    let mut sorted_counts: Vec<_> = counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));
    for (bitstring, count) in sorted_counts {
        let bar_len = (*count as f64 / 10.0) as usize;
        let bar: String = "█".repeat(bar_len);
        println!("  |{}⟩: {:>4} {}", bitstring, count, bar);
    }
    println!();
    println!("GHZ state: only |000⟩ and |111⟩ are possible - perfect 3-way entanglement!");
    println!();

    // Demo 3: Quantum Interference
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 3: Quantum Interference (Mach-Zehnder)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // H-H gives |0⟩ (constructive interference)
    let hh = Circuit::new(1).h(0).h(0);
    let mut sim = Simulator::new();
    let state = sim.run(&hh).unwrap();

    println!("Circuit: H → H (two beam splitters)");
    println!();
    println!("Result:");
    println!("  |0⟩: {:.4} ({:.1}%)", state.probability(0), state.probability(0) * 100.0);
    println!("  |1⟩: {:.4} ({:.1}%)", state.probability(1), state.probability(1) * 100.0);
    println!();
    println!("Two Hadamards cancel out! H² = I (constructive interference at |0⟩)");
    println!();

    // Demo 4: Quantum Phase Kickback
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 4: Phase Kickback (Key to Quantum Algorithms)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // |+⟩|1⟩ → CZ → |-⟩|1⟩
    let kickback = Circuit::new(2)
        .x(1)       // Target = |1⟩
        .h(0)       // Control = |+⟩
        .cz(0, 1);  // CZ: phase kicks back to control

    let mut sim = Simulator::new();
    let state = sim.run(&kickback).unwrap();

    println!("Circuit: X(q1) → H(q0) → CZ(q0, q1)");
    println!();
    println!("Before CZ: q0 = |+⟩ = (|0⟩+|1⟩)/√2");
    println!("After CZ:  q0 = |-⟩ = (|0⟩-|1⟩)/√2  ← Phase kicked back!");
    println!();
    println!("State amplitudes:");
    let amps = state.amplitudes();
    for i in 0..4 {
        let re = amps[i].re;
        let im = amps[i].im;
        let basis = match i {
            0 => "00",
            1 => "01",
            2 => "10",
            3 => "11",
            _ => unreachable!(),
        };
        if re.abs() > 0.01 || im.abs() > 0.01 {
            println!("  |{}⟩: {:+.4}{:+.4}i", basis, re, im);
        }
    }
    println!();
    println!("The -0.7071 on |11⟩ shows the phase kickback in action!");
    println!("This is the core of Deutsch-Jozsa, Grover's, and Shor's algorithms.");
    println!();

    // Demo 5: Toffoli (CCX) Gate
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 5: Toffoli Gate (Universal Quantum Computing)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    println!("Toffoli gate truth table (flips target only when both controls = 1):");
    println!();
    println!("  Input  → Output");

    for c1 in 0..2 {
        for c2 in 0..2 {
            for t in 0..2 {
                let mut circuit = Circuit::new(3);
                if c1 == 1 { circuit = circuit.x(0); }
                if c2 == 1 { circuit = circuit.x(1); }
                if t == 1 { circuit = circuit.x(2); }
                circuit = circuit.ccx(0, 1, 2);

                let mut sim = Simulator::new();
                let state = sim.run(&circuit).unwrap();

                // Find the output state
                let output_idx = state.probabilities()
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .unwrap()
                    .0;

                let out_c1 = (output_idx >> 0) & 1;
                let out_c2 = (output_idx >> 1) & 1;
                let out_t = (output_idx >> 2) & 1;

                println!("  |{}{}{}⟩ → |{}{}{}⟩", c1, c2, t, out_c1, out_c2, out_t);
            }
        }
    }
    println!();
    println!("Toffoli + Hadamard = Universal Quantum Computing!");
    println!();

    // Demo 6: Performance
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 6: Performance - Large Circuit");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let n_qubits = 16;
    let start = std::time::Instant::now();

    // Create a 16-qubit circuit with many gates
    let mut big_circuit = Circuit::new(n_qubits);
    for q in 0..n_qubits {
        big_circuit = big_circuit.h(q);
    }
    for q in 0..(n_qubits - 1) {
        big_circuit = big_circuit.cx(q, q + 1);
    }
    for q in 0..n_qubits {
        big_circuit = big_circuit.t(q);
    }

    let mut sim = Simulator::new();
    let state = sim.run(&big_circuit).unwrap();
    let elapsed = start.elapsed();

    println!("Simulated {} qubits ({} amplitudes)", n_qubits, state.dimension());
    println!("Gates: {} H + {} CNOT + {} T = {} total", n_qubits, n_qubits - 1, n_qubits, n_qubits * 3 - 1);
    println!("Time: {:.2?}", elapsed);
    println!();

    let amps_per_sec = state.dimension() as f64 / elapsed.as_secs_f64();
    println!("Performance: {:.0} amplitude ops/sec", amps_per_sec * (n_qubits * 3 - 1) as f64);
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("                   Simulator Demo Complete!                    ");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("QUASAR Simulator is operational. Ready for quantum computation.");
    println!();
}
