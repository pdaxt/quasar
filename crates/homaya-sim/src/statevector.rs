//! Quantum state vector representation.
//!
//! Optimized for cache-friendly access patterns.

use homaya_core::{Complex, QuasarError, Result};

/// A quantum state vector.
///
/// Represents the full quantum state as a vector of 2^n complex amplitudes.
/// This is the workhorse of state vector simulation.
///
/// # Memory Layout
///
/// Amplitudes are stored in computational basis order:
/// - Index 0: |00...0⟩
/// - Index 1: |00...1⟩
/// - Index 2: |00...10⟩
/// - etc.
///
/// # Example
///
/// ```rust
/// use homaya_sim::StateVector;
///
/// // Create |00⟩ state
/// let state = StateVector::new(2);
/// assert_eq!(state.num_qubits(), 2);
/// assert_eq!(state.dimension(), 4);
/// ```
#[derive(Clone, Debug)]
pub struct StateVector {
    /// Number of qubits
    num_qubits: usize,
    /// Amplitude vector (length = 2^num_qubits)
    amplitudes: Vec<Complex>,
}

impl StateVector {
    /// Maximum supported qubits (limited by memory).
    pub const MAX_QUBITS: usize = 30;

    /// Create a new state vector initialized to |0...0⟩.
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let mut amplitudes = vec![Complex::ZERO; dim];
        amplitudes[0] = Complex::ONE;
        Self {
            num_qubits,
            amplitudes,
        }
    }

    /// Create a state vector from amplitudes.
    ///
    /// # Errors
    ///
    /// Returns error if amplitudes length is not a power of 2 or not normalized.
    pub fn from_amplitudes(amplitudes: Vec<Complex>) -> Result<Self> {
        let dim = amplitudes.len();
        if dim == 0 || (dim & (dim - 1)) != 0 {
            return Err(QuasarError::StateDimensionMismatch {
                expected: 0,
                got: dim,
            });
        }

        let num_qubits = dim.trailing_zeros() as usize;

        let norm_sqr: f64 = amplitudes.iter().map(|c| c.norm_sqr()).sum();
        if (norm_sqr - 1.0).abs() > 1e-10 {
            return Err(QuasarError::StateNotNormalized { norm: norm_sqr.sqrt() });
        }

        Ok(Self {
            num_qubits,
            amplitudes,
        })
    }

    /// Create a uniform superposition over all basis states.
    pub fn uniform(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let amp = Complex::from_real(1.0 / (dim as f64).sqrt());
        Self {
            num_qubits,
            amplitudes: vec![amp; dim],
        }
    }

    /// Get the number of qubits.
    #[inline]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get the dimension (2^n).
    #[inline]
    pub fn dimension(&self) -> usize {
        self.amplitudes.len()
    }

    /// Get a reference to the amplitudes.
    #[inline]
    pub fn amplitudes(&self) -> &[Complex] {
        &self.amplitudes
    }

    /// Get a mutable reference to the amplitudes.
    #[inline]
    pub fn amplitudes_mut(&mut self) -> &mut [Complex] {
        &mut self.amplitudes
    }

    /// Get amplitude at index.
    #[inline]
    pub fn get(&self, index: usize) -> Complex {
        self.amplitudes[index]
    }

    /// Set amplitude at index.
    #[inline]
    pub fn set(&mut self, index: usize, value: Complex) {
        self.amplitudes[index] = value;
    }

    /// Get the probability of measuring a specific basis state.
    #[inline]
    pub fn probability(&self, index: usize) -> f64 {
        self.amplitudes[index].norm_sqr()
    }

    /// Get all probabilities.
    pub fn probabilities(&self) -> Vec<f64> {
        self.amplitudes.iter().map(|c| c.norm_sqr()).collect()
    }

    /// Normalize the state vector in-place.
    pub fn normalize(&mut self) {
        let norm_sqr: f64 = self.amplitudes.iter().map(|c| c.norm_sqr()).sum();
        if norm_sqr > 0.0 {
            let inv_norm = 1.0 / norm_sqr.sqrt();
            for amp in &mut self.amplitudes {
                *amp = *amp * inv_norm;
            }
        }
    }

    /// Apply a single-qubit gate.
    ///
    /// Uses an optimized algorithm that minimizes cache misses.
    pub fn apply_single(&mut self, qubit: usize, matrix: [[Complex; 2]; 2]) {
        let dim = self.dimension();
        let mask = 1 << qubit;

        // Process pairs of amplitudes where they differ only in the qubit bit
        for i in 0..dim {
            // Only process when the qubit bit is 0
            if (i & mask) == 0 {
                let i0 = i;          // qubit bit = 0
                let i1 = i | mask;   // qubit bit = 1

                let a0 = self.amplitudes[i0];
                let a1 = self.amplitudes[i1];

                self.amplitudes[i0] = matrix[0][0] * a0 + matrix[0][1] * a1;
                self.amplitudes[i1] = matrix[1][0] * a0 + matrix[1][1] * a1;
            }
        }
    }

    /// Apply a two-qubit gate.
    ///
    /// Optimized for controlled gates and SWAP-like operations.
    pub fn apply_two(&mut self, q0: usize, q1: usize, matrix: [[Complex; 4]; 4]) {
        let dim = self.dimension();
        let mask0 = 1 << q0;
        let mask1 = 1 << q1;

        // Process groups of 4 amplitudes where both qubits are 0
        for i in 0..dim {
            // Only process when both qubit bits are 0
            if (i & mask0) == 0 && (i & mask1) == 0 {
                let i00 = i;
                let i01 = i | mask0;
                let i10 = i | mask1;
                let i11 = i | mask0 | mask1;

                let a00 = self.amplitudes[i00];
                let a01 = self.amplitudes[i01];
                let a10 = self.amplitudes[i10];
                let a11 = self.amplitudes[i11];

                self.amplitudes[i00] = matrix[0][0] * a00 + matrix[0][1] * a01 + matrix[0][2] * a10 + matrix[0][3] * a11;
                self.amplitudes[i01] = matrix[1][0] * a00 + matrix[1][1] * a01 + matrix[1][2] * a10 + matrix[1][3] * a11;
                self.amplitudes[i10] = matrix[2][0] * a00 + matrix[2][1] * a01 + matrix[2][2] * a10 + matrix[2][3] * a11;
                self.amplitudes[i11] = matrix[3][0] * a00 + matrix[3][1] * a01 + matrix[3][2] * a10 + matrix[3][3] * a11;
            }
        }
    }

    /// Apply a controlled single-qubit gate (optimized).
    ///
    /// This is more efficient than the general two-qubit gate for CX, CZ, etc.
    pub fn apply_controlled(&mut self, control: usize, target: usize, matrix: [[Complex; 2]; 2]) {
        let dim = self.dimension();
        let control_mask = 1 << control;
        let target_mask = 1 << target;

        // Iterate over pairs that differ only in target qubit
        // Only apply when control qubit is 1
        for i in 0..dim {
            // Only process when: control=1 and target=0
            if (i & control_mask) != 0 && (i & target_mask) == 0 {
                let i0 = i;              // target = 0
                let i1 = i | target_mask; // target = 1

                let a0 = self.amplitudes[i0];
                let a1 = self.amplitudes[i1];

                self.amplitudes[i0] = matrix[0][0] * a0 + matrix[0][1] * a1;
                self.amplitudes[i1] = matrix[1][0] * a0 + matrix[1][1] * a1;
            }
        }
    }

    /// Measure a single qubit, collapsing the state.
    ///
    /// Returns the measurement result (0 or 1).
    pub fn measure(&mut self, qubit: usize, random: f64) -> u8 {
        let mask = 1 << qubit;

        // Calculate probability of measuring |0⟩
        let prob_0: f64 = self.amplitudes
            .iter()
            .enumerate()
            .filter(|(i, _)| i & mask == 0)
            .map(|(_, c)| c.norm_sqr())
            .sum();

        let result = if random < prob_0 { 0 } else { 1 };

        // Collapse the state
        let norm = if result == 0 { prob_0 } else { 1.0 - prob_0 };
        let inv_sqrt_norm = 1.0 / norm.sqrt();

        for i in 0..self.dimension() {
            let qubit_is_one = (i & mask) != 0;
            if (result == 0 && qubit_is_one) || (result == 1 && !qubit_is_one) {
                self.amplitudes[i] = Complex::ZERO;
            } else {
                self.amplitudes[i] = self.amplitudes[i] * inv_sqrt_norm;
            }
        }

        result
    }

    /// Reset a qubit to |0⟩.
    pub fn reset(&mut self, qubit: usize, random: f64) {
        let result = self.measure(qubit, random);
        if result == 1 {
            // Apply X gate to flip back to |0⟩
            let x_matrix = [[Complex::ZERO, Complex::ONE], [Complex::ONE, Complex::ZERO]];
            self.apply_single(qubit, x_matrix);
        }
    }

    /// Sample a measurement outcome without collapsing.
    pub fn sample(&self, random: f64) -> usize {
        let mut cumulative = 0.0;
        for (i, amp) in self.amplitudes.iter().enumerate() {
            cumulative += amp.norm_sqr();
            if random < cumulative {
                return i;
            }
        }
        self.dimension() - 1
    }

    /// Get the inner product with another state.
    pub fn inner_product(&self, other: &StateVector) -> Complex {
        self.amplitudes
            .iter()
            .zip(other.amplitudes.iter())
            .map(|(a, b)| a.conj() * *b)
            .fold(Complex::ZERO, |acc, x| acc + x)
    }

    /// Calculate the fidelity with another state.
    pub fn fidelity(&self, other: &StateVector) -> f64 {
        self.inner_product(other).norm_sqr()
    }
}

impl PartialEq for StateVector {
    fn eq(&self, other: &Self) -> bool {
        if self.num_qubits != other.num_qubits {
            return false;
        }
        self.amplitudes
            .iter()
            .zip(other.amplitudes.iter())
            .all(|(a, b)| a.approx_eq(*b, 1e-10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = StateVector::new(2);
        assert_eq!(state.num_qubits(), 2);
        assert_eq!(state.dimension(), 4);
        assert_eq!(state.get(0), Complex::ONE);
        assert_eq!(state.get(1), Complex::ZERO);
        assert_eq!(state.get(2), Complex::ZERO);
        assert_eq!(state.get(3), Complex::ZERO);
    }

    #[test]
    fn test_uniform_superposition() {
        let state = StateVector::uniform(2);
        let expected = Complex::from_real(0.5); // 1/sqrt(4)
        for i in 0..4 {
            assert!(state.get(i).approx_eq(expected, 1e-10));
        }
    }

    #[test]
    fn test_probabilities_normalized() {
        let state = StateVector::new(3);
        let probs = state.probabilities();
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_hadamard_creates_superposition() {
        let mut state = StateVector::new(1);
        let h = Complex::from_real(std::f64::consts::FRAC_1_SQRT_2);
        let h_matrix = [[h, h], [h, -h]];

        state.apply_single(0, h_matrix);

        // Should be |+⟩ = (|0⟩ + |1⟩)/√2
        assert!(state.get(0).approx_eq(h, 1e-10));
        assert!(state.get(1).approx_eq(h, 1e-10));
    }

    #[test]
    fn test_x_gate_flips() {
        let mut state = StateVector::new(1);
        let x_matrix = [[Complex::ZERO, Complex::ONE], [Complex::ONE, Complex::ZERO]];

        state.apply_single(0, x_matrix);

        // Should be |1⟩
        assert_eq!(state.get(0), Complex::ZERO);
        assert_eq!(state.get(1), Complex::ONE);
    }

    #[test]
    fn test_cnot_creates_bell_state() {
        let mut state = StateVector::new(2);
        let h = Complex::from_real(std::f64::consts::FRAC_1_SQRT_2);

        // H on qubit 0
        state.apply_single(0, [[h, h], [h, -h]]);

        // CNOT with control=0, target=1
        let x_matrix = [[Complex::ZERO, Complex::ONE], [Complex::ONE, Complex::ZERO]];
        state.apply_controlled(0, 1, x_matrix);

        // Should be Bell state: (|00⟩ + |11⟩)/√2
        assert!(state.get(0).approx_eq(h, 1e-10)); // |00⟩
        assert!(state.get(1).approx_eq(Complex::ZERO, 1e-10)); // |01⟩
        assert!(state.get(2).approx_eq(Complex::ZERO, 1e-10)); // |10⟩
        assert!(state.get(3).approx_eq(h, 1e-10)); // |11⟩
    }

    #[test]
    fn test_measurement_collapses() {
        let mut state = StateVector::uniform(1);

        // Measure with random = 0.3 (should give |0⟩ since prob_0 = 0.5)
        let result = state.measure(0, 0.3);
        assert_eq!(result, 0);
        assert_eq!(state.get(0), Complex::ONE);
        assert_eq!(state.get(1), Complex::ZERO);
    }

    #[test]
    fn test_fidelity_with_self() {
        let state = StateVector::uniform(3);
        let fidelity = state.fidelity(&state);
        assert!((fidelity - 1.0).abs() < 1e-10);
    }
}
