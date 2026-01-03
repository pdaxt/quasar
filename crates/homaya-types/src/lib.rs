//! # HOMAYA Types
//!
//! Core types and traits for the HOMAYA quantum computing framework.
//!
//! This crate provides the foundational type definitions used across
//! all HOMAYA components, ensuring type safety and consistency.
//!
//! ## Learn More
//!
//! - **Course**: Learn quantum computing at [bskiller.com](https://bskiller.com)
//! - **Enterprise**: Production solutions at [dataxlr8.ai](https://dataxlr8.ai)
//!
//! ## Core Types
//!
//! - [`Qubit`] - Index of a quantum bit
//! - [`ClassicalBit`] - Index of a classical bit
//! - [`Amplitude`] - Complex amplitude of a quantum state
//! - [`Probability`] - Real probability value [0, 1]
//!
//! ## Core Traits
//!
//! - [`QuantumGate`] - Trait for quantum gate operations
//! - [`QuantumBackend`] - Trait for execution backends
//! - [`Optimizable`] - Trait for optimizable circuits

#![deny(missing_docs)]
#![deny(unsafe_code)]

use std::fmt;

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// Index of a quantum bit in a circuit or register.
pub type Qubit = usize;

/// Index of a classical bit for measurement results.
pub type ClassicalBit = usize;

/// Number of qubits in a system.
pub type QubitCount = usize;

/// Number of shots/samples for measurement.
pub type ShotCount = usize;

/// Real probability value in the range [0, 1].
pub type Probability = f64;

/// Angle in radians for rotation gates.
pub type Angle = f64;

// ============================================================================
// COMPLEX AMPLITUDE
// ============================================================================

/// Complex amplitude representing a quantum state coefficient.
///
/// Quantum states are represented as vectors of complex amplitudes,
/// where the probability of measuring a state is |amplitude|².
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Amplitude {
    /// Real part of the complex number
    pub re: f64,
    /// Imaginary part of the complex number
    pub im: f64,
}

impl Amplitude {
    /// Create a new complex amplitude.
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Create a purely real amplitude.
    #[inline]
    pub const fn real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    /// Create a purely imaginary amplitude.
    #[inline]
    pub const fn imag(im: f64) -> Self {
        Self { re: 0.0, im }
    }

    /// The zero amplitude.
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };

    /// The one amplitude.
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };

    /// The imaginary unit i.
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    /// Calculate the squared magnitude |z|².
    ///
    /// This equals the probability of measuring the corresponding state.
    #[inline]
    pub fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Calculate the magnitude |z|.
    #[inline]
    pub fn norm(&self) -> f64 {
        self.norm_sqr().sqrt()
    }

    /// Calculate the complex conjugate z*.
    #[inline]
    pub fn conj(&self) -> Self {
        Self { re: self.re, im: -self.im }
    }
}

impl fmt::Display for Amplitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{:.4}+{:.4}i", self.re, self.im)
        } else {
            write!(f, "{:.4}{:.4}i", self.re, self.im)
        }
    }
}

// ============================================================================
// GATE TYPES
// ============================================================================

/// Standard quantum gate types.
///
/// These represent the common gates used in quantum computing.
/// Each gate has a specific unitary matrix representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StandardGate {
    /// Identity gate (no operation)
    I,
    /// Pauli-X gate (bit flip)
    X,
    /// Pauli-Y gate
    Y,
    /// Pauli-Z gate (phase flip)
    Z,
    /// Hadamard gate (creates superposition)
    H,
    /// S gate (√Z)
    S,
    /// S-dagger gate
    Sdg,
    /// T gate (π/8 gate)
    T,
    /// T-dagger gate
    Tdg,
    /// Controlled-X (CNOT)
    CX,
    /// Controlled-Y
    CY,
    /// Controlled-Z
    CZ,
    /// Controlled-H
    CH,
    /// SWAP gate
    Swap,
    /// Toffoli (CCX)
    CCX,
    /// Fredkin (CSWAP)
    CSwap,
}

impl StandardGate {
    /// Returns the number of qubits this gate operates on.
    pub const fn num_qubits(&self) -> usize {
        match self {
            Self::I | Self::X | Self::Y | Self::Z |
            Self::H | Self::S | Self::Sdg | Self::T | Self::Tdg => 1,
            Self::CX | Self::CY | Self::CZ | Self::CH | Self::Swap => 2,
            Self::CCX | Self::CSwap => 3,
        }
    }

    /// Returns true if this is a controlled gate.
    pub const fn is_controlled(&self) -> bool {
        matches!(self, Self::CX | Self::CY | Self::CZ | Self::CH | Self::CCX | Self::CSwap)
    }
}

// ============================================================================
// RESULT TYPES
// ============================================================================

/// Result of measuring a quantum circuit.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MeasurementResult {
    /// Bit string results (e.g., "00", "11")
    pub counts: std::collections::HashMap<String, usize>,
    /// Total number of shots
    pub shots: ShotCount,
}

impl MeasurementResult {
    /// Create a new measurement result.
    pub fn new(shots: ShotCount) -> Self {
        Self {
            counts: std::collections::HashMap::new(),
            shots,
        }
    }

    /// Get the probability of a specific outcome.
    pub fn probability(&self, outcome: &str) -> Probability {
        self.counts.get(outcome).copied().unwrap_or(0) as f64 / self.shots as f64
    }

    /// Get the most likely outcome.
    pub fn most_likely(&self) -> Option<(&str, Probability)> {
        self.counts
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(outcome, &count)| (outcome.as_str(), count as f64 / self.shots as f64))
    }
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Error type for HOMAYA operations.
#[derive(Debug, Clone, PartialEq)]
pub enum HomayaError {
    /// Qubit index out of range
    QubitOutOfRange {
        /// The invalid qubit index
        qubit: Qubit,
        /// Maximum allowed index
        max: Qubit,
    },
    /// Invalid gate parameters
    InvalidParameter {
        /// Parameter name
        name: &'static str,
        /// Error message
        message: String,
    },
    /// Operation not supported
    NotSupported {
        /// Description of unsupported operation
        operation: &'static str,
    },
    /// Backend execution error
    BackendError {
        /// Error message from backend
        message: String,
    },
}

impl fmt::Display for HomayaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitOutOfRange { qubit, max } => {
                write!(f, "Qubit {} out of range (max: {})", qubit, max)
            }
            Self::InvalidParameter { name, message } => {
                write!(f, "Invalid parameter '{}': {}", name, message)
            }
            Self::NotSupported { operation } => {
                write!(f, "Operation not supported: {}", operation)
            }
            Self::BackendError { message } => {
                write!(f, "Backend error: {}", message)
            }
        }
    }
}

impl std::error::Error for HomayaError {}

/// Result type for HOMAYA operations.
pub type Result<T> = std::result::Result<T, HomayaError>;

// ============================================================================
// TRAITS
// ============================================================================

/// Trait for types that can be optimized.
///
/// Implemented by circuits and other structures that can be
/// transformed to reduce resource usage.
pub trait Optimizable {
    /// Apply optimization and return the optimized version.
    fn optimize(&self) -> Self;

    /// Returns the current "cost" (e.g., gate count).
    fn cost(&self) -> usize;
}

// ============================================================================
// CONSTANTS
// ============================================================================

/// Mathematical constants used in quantum computing.
pub mod constants {
    /// π
    pub const PI: f64 = std::f64::consts::PI;

    /// τ = 2π
    pub const TAU: f64 = std::f64::consts::TAU;

    /// √2
    pub const SQRT_2: f64 = std::f64::consts::SQRT_2;

    /// 1/√2 (used frequently in quantum gates)
    pub const INV_SQRT_2: f64 = 0.7071067811865476;

    /// Default tolerance for floating point comparisons
    pub const EPSILON: f64 = 1e-10;
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplitude_norm() {
        let a = Amplitude::new(3.0, 4.0);
        assert!((a.norm() - 5.0).abs() < 1e-10);
        assert!((a.norm_sqr() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_amplitude_conj() {
        let a = Amplitude::new(1.0, 2.0);
        let c = a.conj();
        assert_eq!(c.re, 1.0);
        assert_eq!(c.im, -2.0);
    }

    #[test]
    fn test_gate_qubits() {
        assert_eq!(StandardGate::H.num_qubits(), 1);
        assert_eq!(StandardGate::CX.num_qubits(), 2);
        assert_eq!(StandardGate::CCX.num_qubits(), 3);
    }

    #[test]
    fn test_measurement_probability() {
        let mut result = MeasurementResult::new(100);
        result.counts.insert("00".to_string(), 60);
        result.counts.insert("11".to_string(), 40);

        assert!((result.probability("00") - 0.6).abs() < 1e-10);
        assert!((result.probability("11") - 0.4).abs() < 1e-10);
        assert!((result.probability("01") - 0.0).abs() < 1e-10);
    }
}
