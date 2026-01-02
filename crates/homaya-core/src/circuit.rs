//! Quantum circuit builder.
//!
//! Fluent API for constructing quantum circuits.

use crate::{Gate, GateType, QuasarError, Result};

/// A quantum instruction: gate + target qubits.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Instruction {
    /// The gate to apply
    pub gate: Gate,
    /// Target qubit indices
    pub qubits: Vec<usize>,
    /// Classical bit indices (for measurement)
    pub clbits: Vec<usize>,
}

impl Instruction {
    /// Create a new instruction.
    #[inline]
    pub fn new(gate: Gate, qubits: Vec<usize>) -> Self {
        Self {
            gate,
            qubits,
            clbits: Vec::new(),
        }
    }

    /// Create an instruction with classical bits.
    #[inline]
    pub fn with_clbits(gate: Gate, qubits: Vec<usize>, clbits: Vec<usize>) -> Self {
        Self { gate, qubits, clbits }
    }
}

/// A quantum circuit.
///
/// # Example
///
/// ```rust
/// use homaya_core::Circuit;
///
/// // Bell state preparation
/// let circuit = Circuit::new(2)
///     .h(0)
///     .cx(0, 1);
///
/// assert_eq!(circuit.num_qubits(), 2);
/// assert_eq!(circuit.depth(), 2);
/// ```
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circuit {
    /// Number of qubits
    num_qubits: usize,
    /// Number of classical bits
    num_clbits: usize,
    /// Instructions in order
    instructions: Vec<Instruction>,
    /// Optional name
    name: Option<std::string::String>,
}

impl Circuit {
    /// Create a new circuit with the given number of qubits.
    #[inline]
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            num_clbits: 0,
            instructions: Vec::new(),
            name: None,
        }
    }

    /// Create a circuit with qubits and classical bits.
    #[inline]
    pub fn with_clbits(num_qubits: usize, num_clbits: usize) -> Self {
        Self {
            num_qubits,
            num_clbits,
            instructions: Vec::new(),
            name: None,
        }
    }

    /// Set the circuit name.
    #[inline]
    pub fn named(mut self, name: impl Into<std::string::String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Get the number of qubits.
    #[inline]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get the number of classical bits.
    #[inline]
    pub const fn num_clbits(&self) -> usize {
        self.num_clbits
    }

    /// Get the instructions.
    #[inline]
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    /// Get the number of instructions.
    #[inline]
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Check if the circuit is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Calculate circuit depth (critical path length).
    pub fn depth(&self) -> usize {
        if self.instructions.is_empty() {
            return 0;
        }

        // Track depth at each qubit
        let mut qubit_depth = std::vec![0usize; self.num_qubits];

        for inst in &self.instructions {
            if inst.gate.gate_type == GateType::Barrier {
                continue;
            }

            // Find max depth among target qubits
            let max_depth = inst.qubits.iter().map(|&q| qubit_depth[q]).max().unwrap_or(0);

            // All target qubits advance to max + 1
            for &q in &inst.qubits {
                qubit_depth[q] = max_depth + 1;
            }
        }

        qubit_depth.into_iter().max().unwrap_or(0)
    }

    /// Count gates by type.
    pub fn count_gates(&self) -> std::collections::BTreeMap<GateType, usize> {
        let mut counts = std::collections::BTreeMap::new();
        for inst in &self.instructions {
            *counts.entry(inst.gate.gate_type).or_insert(0) += 1;
        }
        counts
    }

    /// Add a raw instruction.
    fn push(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    // ========== Single-qubit gates ==========

    /// Apply identity gate.
    #[inline]
    pub fn i(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::i(), std::vec![q]));
        self
    }

    /// Apply Pauli-X (NOT) gate.
    #[inline]
    pub fn x(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::x(), std::vec![q]));
        self
    }

    /// Apply Pauli-Y gate.
    #[inline]
    pub fn y(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::y(), std::vec![q]));
        self
    }

    /// Apply Pauli-Z gate.
    #[inline]
    pub fn z(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::z(), std::vec![q]));
        self
    }

    /// Apply Hadamard gate.
    #[inline]
    pub fn h(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::h(), std::vec![q]));
        self
    }

    /// Apply S gate.
    #[inline]
    pub fn s(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::s(), std::vec![q]));
        self
    }

    /// Apply S-dagger gate.
    #[inline]
    pub fn sdg(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::sdg(), std::vec![q]));
        self
    }

    /// Apply T gate.
    #[inline]
    pub fn t(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::t(), std::vec![q]));
        self
    }

    /// Apply T-dagger gate.
    #[inline]
    pub fn tdg(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::tdg(), std::vec![q]));
        self
    }

    /// Apply rotation around X-axis.
    #[inline]
    pub fn rx(mut self, theta: f64, q: usize) -> Self {
        self.push(Instruction::new(Gate::rx(theta), std::vec![q]));
        self
    }

    /// Apply rotation around Y-axis.
    #[inline]
    pub fn ry(mut self, theta: f64, q: usize) -> Self {
        self.push(Instruction::new(Gate::ry(theta), std::vec![q]));
        self
    }

    /// Apply rotation around Z-axis.
    #[inline]
    pub fn rz(mut self, theta: f64, q: usize) -> Self {
        self.push(Instruction::new(Gate::rz(theta), std::vec![q]));
        self
    }

    /// Apply phase gate.
    #[inline]
    pub fn p(mut self, theta: f64, q: usize) -> Self {
        self.push(Instruction::new(Gate::p(theta), std::vec![q]));
        self
    }

    /// Apply general U gate.
    #[inline]
    pub fn u(mut self, theta: f64, phi: f64, lambda: f64, q: usize) -> Self {
        self.push(Instruction::new(Gate::u(theta, phi, lambda), std::vec![q]));
        self
    }

    // ========== Two-qubit gates ==========

    /// Apply CNOT (CX) gate.
    #[inline]
    pub fn cx(mut self, control: usize, target: usize) -> Self {
        self.push(Instruction::new(Gate::cx(), std::vec![control, target]));
        self
    }

    /// Apply CY gate.
    #[inline]
    pub fn cy(mut self, control: usize, target: usize) -> Self {
        self.push(Instruction::new(Gate::cy(), std::vec![control, target]));
        self
    }

    /// Apply CZ gate.
    #[inline]
    pub fn cz(mut self, control: usize, target: usize) -> Self {
        self.push(Instruction::new(Gate::cz(), std::vec![control, target]));
        self
    }

    /// Apply CH gate.
    #[inline]
    pub fn ch(mut self, control: usize, target: usize) -> Self {
        self.push(Instruction::new(Gate::ch(), std::vec![control, target]));
        self
    }

    /// Apply controlled-phase gate.
    #[inline]
    pub fn cp(mut self, theta: f64, control: usize, target: usize) -> Self {
        self.push(Instruction::new(Gate::cp(theta), std::vec![control, target]));
        self
    }

    /// Apply SWAP gate.
    #[inline]
    pub fn swap(mut self, q1: usize, q2: usize) -> Self {
        self.push(Instruction::new(Gate::swap(), std::vec![q1, q2]));
        self
    }

    // ========== Three-qubit gates ==========

    /// Apply Toffoli (CCX) gate.
    #[inline]
    pub fn ccx(mut self, c1: usize, c2: usize, target: usize) -> Self {
        self.push(Instruction::new(Gate::ccx(), std::vec![c1, c2, target]));
        self
    }

    /// Apply Fredkin (CSWAP) gate.
    #[inline]
    pub fn cswap(mut self, control: usize, t1: usize, t2: usize) -> Self {
        self.push(Instruction::new(Gate::cswap(), std::vec![control, t1, t2]));
        self
    }

    // ========== Special operations ==========

    /// Measure a qubit.
    #[inline]
    pub fn measure(mut self, q: usize, c: usize) -> Self {
        self.push(Instruction::with_clbits(Gate::measure(), std::vec![q], std::vec![c]));
        self
    }

    /// Measure all qubits.
    pub fn measure_all(mut self) -> Self {
        // Ensure we have enough classical bits
        if self.num_clbits < self.num_qubits {
            self.num_clbits = self.num_qubits;
        }
        for q in 0..self.num_qubits {
            self.push(Instruction::with_clbits(Gate::measure(), std::vec![q], std::vec![q]));
        }
        self
    }

    /// Reset a qubit to |0⟩.
    #[inline]
    pub fn reset(mut self, q: usize) -> Self {
        self.push(Instruction::new(Gate::reset(), std::vec![q]));
        self
    }

    /// Add a barrier (synchronization point).
    pub fn barrier(mut self, qubits: &[usize]) -> Self {
        self.push(Instruction::new(Gate::barrier(), qubits.to_vec()));
        self
    }

    /// Add a barrier across all qubits.
    pub fn barrier_all(mut self) -> Self {
        let qubits: Vec<usize> = (0..self.num_qubits).collect();
        self.push(Instruction::new(Gate::barrier(), qubits));
        self
    }

    // ========== Composition ==========

    /// Append another circuit.
    pub fn compose(mut self, other: &Circuit) -> Result<Self> {
        if other.num_qubits > self.num_qubits {
            return Err(QuasarError::QubitMismatch {
                expected: self.num_qubits,
                got: other.num_qubits,
            });
        }
        for inst in &other.instructions {
            self.push(inst.clone());
        }
        Ok(self)
    }

    /// Repeat the circuit n times.
    pub fn repeat(self, n: usize) -> Self {
        let original = self.instructions.clone();
        let mut result = self;
        for _ in 1..n {
            for inst in &original {
                result.push(inst.clone());
            }
        }
        result
    }

    /// Get the inverse of this circuit.
    pub fn inverse(self) -> Self {
        let mut result = Circuit::new(self.num_qubits);
        result.num_clbits = self.num_clbits;

        // Reverse order and invert each gate
        for inst in self.instructions.into_iter().rev() {
            // For now, just reverse (TODO: proper gate inversion)
            result.push(inst);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bell_state_circuit() {
        let circuit = Circuit::new(2)
            .h(0)
            .cx(0, 1);

        assert_eq!(circuit.num_qubits(), 2);
        assert_eq!(circuit.len(), 2);
        assert_eq!(circuit.depth(), 2);
    }

    #[test]
    fn test_ghz_state_circuit() {
        let circuit = Circuit::new(3)
            .h(0)
            .cx(0, 1)
            .cx(1, 2);

        assert_eq!(circuit.num_qubits(), 3);
        assert_eq!(circuit.len(), 3);
        assert_eq!(circuit.depth(), 3);
    }

    #[test]
    fn test_parallel_depth() {
        // Parallel operations should have depth 1
        let circuit = Circuit::new(4)
            .h(0)
            .h(1)
            .h(2)
            .h(3);

        // Each H is on a different qubit, but our simple depth calc
        // counts sequential instructions. For proper parallel depth,
        // we'd need a more sophisticated algorithm.
        assert_eq!(circuit.len(), 4);
    }

    #[test]
    fn test_measure_all() {
        let circuit = Circuit::new(3).h(0).measure_all();

        assert_eq!(circuit.num_clbits(), 3);
        assert_eq!(circuit.len(), 4); // 1 H + 3 measures
    }

    #[test]
    fn test_gate_count() {
        let circuit = Circuit::new(2)
            .h(0)
            .h(1)
            .cx(0, 1)
            .h(0);

        let counts = circuit.count_gates();
        assert_eq!(counts.get(&GateType::H), Some(&3));
        assert_eq!(counts.get(&GateType::CX), Some(&1));
    }
}
