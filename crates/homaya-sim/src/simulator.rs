//! Circuit simulator engine.
//!
//! Executes quantum circuits on state vectors.

use homaya_core::{Circuit, Complex, Gate, GateType, GateParams, QuasarError, Result, INV_SQRT_2, PI};
use crate::StateVector;

/// Measurement results from circuit execution.
#[derive(Clone, Debug, Default)]
pub struct MeasurementResult {
    /// Classical bit values (index → value)
    pub bits: Vec<u8>,
}

impl MeasurementResult {
    /// Create a new measurement result.
    pub fn new(num_clbits: usize) -> Self {
        Self {
            bits: vec![0; num_clbits],
        }
    }

    /// Get result as a bitstring.
    pub fn bitstring(&self) -> String {
        self.bits.iter().map(|b| if *b == 0 { '0' } else { '1' }).collect()
    }

    /// Get result as an integer (little-endian).
    pub fn as_int(&self) -> u64 {
        self.bits.iter().enumerate().fold(0u64, |acc, (i, &b)| {
            acc | ((b as u64) << i)
        })
    }
}

/// The quantum circuit simulator.
///
/// Simulates circuits using state vector representation.
///
/// # Example
///
/// ```rust
/// use homaya_core::Circuit;
/// use homaya_sim::Simulator;
///
/// let circuit = Circuit::new(2).h(0).cx(0, 1);
/// let mut sim = Simulator::new();
/// let state = sim.run(&circuit).unwrap();
///
/// // Check Bell state
/// assert!(state.probability(0) > 0.49); // |00⟩
/// assert!(state.probability(3) > 0.49); // |11⟩
/// ```
#[derive(Clone, Debug)]
pub struct Simulator {
    /// Random seed for measurements
    seed: Option<u64>,
    /// Current random state
    rng_state: u64,
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulator {
    /// Create a new simulator.
    pub fn new() -> Self {
        Self {
            seed: None,
            rng_state: 0x853c49e6748fea9b, // Default seed
        }
    }

    /// Create a simulator with a specific seed for reproducibility.
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed: Some(seed),
            rng_state: seed,
        }
    }

    /// Simple xorshift64 PRNG for fast random numbers.
    fn next_random(&mut self) -> f64 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng_state = x;
        (x as f64) / (u64::MAX as f64)
    }

    /// Run a circuit and return the final state.
    pub fn run(&mut self, circuit: &Circuit) -> Result<StateVector> {
        self.run_from_state(circuit, StateVector::new(circuit.num_qubits()))
    }

    /// Run a circuit starting from a given state.
    pub fn run_from_state(&mut self, circuit: &Circuit, state: StateVector) -> Result<StateVector> {
        if state.num_qubits() != circuit.num_qubits() {
            return Err(QuasarError::QubitMismatch {
                expected: circuit.num_qubits(),
                got: state.num_qubits(),
            });
        }

        let mut state = state;
        let mut measurements = MeasurementResult::new(circuit.num_clbits());

        for inst in circuit.instructions() {
            self.apply_instruction(&mut state, &inst.gate, &inst.qubits, &inst.clbits, &mut measurements)?;
        }

        Ok(state)
    }

    /// Run a circuit with measurements and return both state and results.
    pub fn run_with_measurements(&mut self, circuit: &Circuit) -> Result<(StateVector, MeasurementResult)> {
        let mut state = StateVector::new(circuit.num_qubits());
        let mut measurements = MeasurementResult::new(circuit.num_clbits());

        for inst in circuit.instructions() {
            self.apply_instruction(&mut state, &inst.gate, &inst.qubits, &inst.clbits, &mut measurements)?;
        }

        Ok((state, measurements))
    }

    /// Sample the circuit multiple times.
    pub fn sample(&mut self, circuit: &Circuit, shots: usize) -> Result<std::collections::HashMap<String, usize>> {
        let mut counts = std::collections::HashMap::new();

        // Reset seed if specified
        if let Some(seed) = self.seed {
            self.rng_state = seed;
        }

        for _ in 0..shots {
            let (_, result) = self.run_with_measurements(circuit)?;
            *counts.entry(result.bitstring()).or_insert(0) += 1;
        }

        Ok(counts)
    }

    /// Apply a single instruction to the state.
    fn apply_instruction(
        &mut self,
        state: &mut StateVector,
        gate: &Gate,
        qubits: &[usize],
        clbits: &[usize],
        measurements: &mut MeasurementResult,
    ) -> Result<()> {
        use GateType::*;

        match gate.gate_type {
            // Single-qubit gates
            I | X | Y | Z | H | S | Sdg | T | Tdg | Rx | Ry | Rz | P | U => {
                let matrix = self.get_single_qubit_matrix(gate)?;
                state.apply_single(qubits[0], matrix);
            }

            // Controlled gates
            CX => {
                let x_matrix = [[Complex::ZERO, Complex::ONE], [Complex::ONE, Complex::ZERO]];
                state.apply_controlled(qubits[0], qubits[1], x_matrix);
            }

            CY => {
                let y_matrix = [[Complex::ZERO, -Complex::I], [Complex::I, Complex::ZERO]];
                state.apply_controlled(qubits[0], qubits[1], y_matrix);
            }

            CZ => {
                let z_matrix = [[Complex::ONE, Complex::ZERO], [Complex::ZERO, -Complex::ONE]];
                state.apply_controlled(qubits[0], qubits[1], z_matrix);
            }

            CH => {
                let h = Complex::from_real(INV_SQRT_2);
                let h_matrix = [[h, h], [h, -h]];
                state.apply_controlled(qubits[0], qubits[1], h_matrix);
            }

            CP => {
                if let GateParams::Angle(theta) = gate.params {
                    let phase = Complex::from_polar(1.0, theta);
                    let cp_matrix = [[Complex::ONE, Complex::ZERO], [Complex::ZERO, phase]];
                    state.apply_controlled(qubits[0], qubits[1], cp_matrix);
                }
            }

            Swap => {
                let swap_matrix = [
                    [Complex::ONE, Complex::ZERO, Complex::ZERO, Complex::ZERO],
                    [Complex::ZERO, Complex::ZERO, Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::ONE, Complex::ZERO, Complex::ZERO],
                    [Complex::ZERO, Complex::ZERO, Complex::ZERO, Complex::ONE],
                ];
                state.apply_two(qubits[0], qubits[1], swap_matrix);
            }

            // Three-qubit gates (decomposed)
            CCX => {
                // Toffoli decomposition using 6 CNOTs and single-qubit gates
                self.apply_ccx(state, qubits[0], qubits[1], qubits[2]);
            }

            CSwap => {
                // Fredkin = CNOT + Toffoli + CNOT
                self.apply_cswap(state, qubits[0], qubits[1], qubits[2]);
            }

            // Measurement
            Measure => {
                let random = self.next_random();
                let result = state.measure(qubits[0], random);
                if !clbits.is_empty() {
                    measurements.bits[clbits[0]] = result;
                }
            }

            Reset => {
                let random = self.next_random();
                state.reset(qubits[0], random);
            }

            Barrier => {
                // No-op for simulation
            }

            _ => {
                return Err(QuasarError::NotSupported {
                    operation: "gate type not implemented",
                });
            }
        }

        Ok(())
    }

    /// Get the 2x2 matrix for a single-qubit gate.
    fn get_single_qubit_matrix(&self, gate: &Gate) -> Result<[[Complex; 2]; 2]> {
        gate.matrix_2x2().ok_or(QuasarError::NotSupported {
            operation: "gate has no 2x2 matrix",
        })
    }

    /// Apply Toffoli (CCX) gate using decomposition.
    fn apply_ccx(&mut self, state: &mut StateVector, c1: usize, c2: usize, target: usize) {
        // Standard Toffoli decomposition
        let h = Complex::from_real(INV_SQRT_2);
        let h_matrix = [[h, h], [h, -h]];
        let t = Complex::from_polar(1.0, PI / 4.0);
        let tdg = Complex::from_polar(1.0, -PI / 4.0);
        let t_matrix = [[Complex::ONE, Complex::ZERO], [Complex::ZERO, t]];
        let tdg_matrix = [[Complex::ONE, Complex::ZERO], [Complex::ZERO, tdg]];
        let x_matrix = [[Complex::ZERO, Complex::ONE], [Complex::ONE, Complex::ZERO]];

        state.apply_single(target, h_matrix);
        state.apply_controlled(c2, target, x_matrix);
        state.apply_single(target, tdg_matrix);
        state.apply_controlled(c1, target, x_matrix);
        state.apply_single(target, t_matrix);
        state.apply_controlled(c2, target, x_matrix);
        state.apply_single(target, tdg_matrix);
        state.apply_controlled(c1, target, x_matrix);
        state.apply_single(c2, t_matrix);
        state.apply_single(target, t_matrix);
        state.apply_single(target, h_matrix);
        state.apply_controlled(c1, c2, x_matrix);
        state.apply_single(c2, tdg_matrix);
        state.apply_controlled(c1, c2, x_matrix);
        state.apply_single(c1, t_matrix);
        state.apply_single(c2, [[Complex::ONE, Complex::ZERO], [Complex::ZERO, Complex::I]]);
    }

    /// Apply Fredkin (CSWAP) gate.
    fn apply_cswap(&mut self, state: &mut StateVector, control: usize, t1: usize, t2: usize) {
        let x_matrix = [[Complex::ZERO, Complex::ONE], [Complex::ONE, Complex::ZERO]];

        // CSWAP = CNOT(t2, t1) + Toffoli(control, t1, t2) + CNOT(t2, t1)
        state.apply_controlled(t2, t1, x_matrix);
        self.apply_ccx(state, control, t1, t2);
        state.apply_controlled(t2, t1, x_matrix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use homaya_core::Circuit;

    #[test]
    fn test_simple_circuit() {
        let circuit = Circuit::new(1).x(0);
        let mut sim = Simulator::new();
        let state = sim.run(&circuit).unwrap();

        assert!(state.probability(0) < 0.01);
        assert!(state.probability(1) > 0.99);
    }

    #[test]
    fn test_bell_state() {
        let circuit = Circuit::new(2).h(0).cx(0, 1);
        let mut sim = Simulator::new();
        let state = sim.run(&circuit).unwrap();

        // Bell state: (|00⟩ + |11⟩)/√2
        assert!(state.probability(0) > 0.49); // |00⟩
        assert!(state.probability(1) < 0.01); // |01⟩
        assert!(state.probability(2) < 0.01); // |10⟩
        assert!(state.probability(3) > 0.49); // |11⟩
    }

    #[test]
    fn test_ghz_state() {
        let circuit = Circuit::new(3).h(0).cx(0, 1).cx(1, 2);
        let mut sim = Simulator::new();
        let state = sim.run(&circuit).unwrap();

        // GHZ state: (|000⟩ + |111⟩)/√2
        assert!(state.probability(0) > 0.49); // |000⟩
        assert!(state.probability(7) > 0.49); // |111⟩
    }

    #[test]
    fn test_measurement_sampling() {
        let circuit = Circuit::new(2)
            .h(0)
            .cx(0, 1)
            .measure_all();

        let mut sim = Simulator::with_seed(42);
        let counts = sim.sample(&circuit, 1000).unwrap();

        // Should get roughly 50% |00⟩ and 50% |11⟩
        let count_00 = counts.get("00").copied().unwrap_or(0);
        let count_11 = counts.get("11").copied().unwrap_or(0);

        assert!(count_00 > 400);
        assert!(count_00 < 600);
        assert!(count_11 > 400);
        assert!(count_11 < 600);
    }

    #[test]
    fn test_rotation_gates() {
        use std::f64::consts::PI;

        // Rx(π) = X
        let circuit = Circuit::new(1).rx(PI, 0);
        let mut sim = Simulator::new();
        let state = sim.run(&circuit).unwrap();

        assert!(state.probability(0) < 0.01);
        assert!(state.probability(1) > 0.99);
    }

    #[test]
    fn test_toffoli_truth_table() {
        let mut sim = Simulator::new();

        // CCX flips target only when both controls are |1⟩
        // Input: |110⟩ → Output: |111⟩
        let circuit = Circuit::new(3).x(0).x(1).ccx(0, 1, 2);
        let state = sim.run(&circuit).unwrap();

        assert!(state.probability(0b111) > 0.99);
    }

    #[test]
    fn test_swap() {
        let circuit = Circuit::new(2).x(0).swap(0, 1);
        let mut sim = Simulator::new();
        let state = sim.run(&circuit).unwrap();

        // |01⟩ → |10⟩
        assert!(state.probability(0b10) > 0.99);
    }
}
