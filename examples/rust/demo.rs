//! QUASAR Demo - Quantum Circuit Builder
//!
//! Run with: cargo run --example demo

use homaya_core::{Circuit, Gate, GateType};

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                         QUASAR Demo                           ║");
    println!("║     Quantum Unified Architecture for Simulation And Runtime    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Demo 1: Bell State
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 1: Bell State (Entanglement)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Circuit: |00⟩ → H(q0) → CNOT(q0,q1) → (|00⟩ + |11⟩)/√2");
    println!();

    let bell = Circuit::new(2)
        .h(0)        // Hadamard on qubit 0
        .cx(0, 1)    // CNOT: control=0, target=1
        .measure_all();

    println!("  ┌───┐     ");
    println!("q0┤ H ├──●──M");
    println!("  └───┘  │   ");
    println!("       ┌─┴─┐ ");
    println!("q1─────┤ X ├─M");
    println!("       └───┘ ");
    println!();
    println!("Circuit stats:");
    println!("  • Qubits: {}", bell.num_qubits());
    println!("  • Gates:  {}", bell.len());
    println!("  • Depth:  {}", bell.depth());
    println!();

    // Demo 2: GHZ State
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 2: GHZ State (3-qubit entanglement)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Circuit: |000⟩ → (|000⟩ + |111⟩)/√2");
    println!();

    let ghz = Circuit::new(3)
        .h(0)
        .cx(0, 1)
        .cx(1, 2)
        .measure_all();

    println!("  ┌───┐          ");
    println!("q0┤ H ├──●───────M");
    println!("  └───┘  │        ");
    println!("       ┌─┴─┐      ");
    println!("q1─────┤ X ├──●──M");
    println!("       └───┘  │   ");
    println!("            ┌─┴─┐ ");
    println!("q2──────────┤ X ├M");
    println!("            └───┘ ");
    println!();
    println!("Circuit stats:");
    println!("  • Qubits: {}", ghz.num_qubits());
    println!("  • Gates:  {}", ghz.len());
    println!("  • Depth:  {}", ghz.depth());
    println!();

    // Demo 3: Rotation gates
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 3: Rotation Gates");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let pi = std::f64::consts::PI;
    let rotations = Circuit::new(1)
        .rx(pi / 4.0, 0)
        .ry(pi / 2.0, 0)
        .rz(pi, 0);

    println!("  ┌─────────┐┌─────────┐┌─────────┐");
    println!("q0┤ Rx(π/4) ├┤ Ry(π/2) ├┤  Rz(π)  ├");
    println!("  └─────────┘└─────────┘└─────────┘");
    println!();
    println!("Circuit stats:");
    println!("  • Qubits: {}", rotations.num_qubits());
    println!("  • Gates:  {}", rotations.len());
    println!("  • Depth:  {}", rotations.depth());
    println!();

    // Demo 4: Gate matrices
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 4: Gate Matrices");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let h = Gate::h();
    if let Some(m) = h.matrix_2x2() {
        println!("Hadamard (H) gate matrix:");
        println!("  ┌                     ┐");
        println!("  │ {:>7.4}  {:>7.4}  │", m[0][0].re, m[0][1].re);
        println!("  │ {:>7.4}  {:>7.4}  │", m[1][0].re, m[1][1].re);
        println!("  └                     ┘");
        println!("  (= 1/√2 × [[1,1],[1,-1]])");
    }
    println!();

    let x = Gate::x();
    if let Some(m) = x.matrix_2x2() {
        println!("Pauli-X (NOT) gate matrix:");
        println!("  ┌           ┐");
        println!("  │ {:>3.0}   {:>3.0} │", m[0][0].re, m[0][1].re);
        println!("  │ {:>3.0}   {:>3.0} │", m[1][0].re, m[1][1].re);
        println!("  └           ┘");
    }
    println!();

    // Demo 5: Gate counting
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Demo 5: Circuit Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let complex = Circuit::new(4)
        .h(0).h(1).h(2).h(3)  // Superposition on all
        .cx(0, 1).cx(1, 2).cx(2, 3)  // Entangle
        .t(0).t(1).t(2).t(3)  // T gates
        .measure_all();

    println!("Complex 4-qubit circuit:");
    let counts = complex.count_gates();
    for (gate_type, count) in &counts {
        println!("  • {:?}: {}", gate_type, count);
    }
    println!();
    println!("Total: {} gates, depth {}", complex.len(), complex.depth());
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    Demo Complete!");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("QUASAR is ready. The quantum revolution starts here.");
    println!();
}
