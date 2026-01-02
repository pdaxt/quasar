//! Error types for QUASAR.

use core::fmt;

/// Errors that can occur in QUASAR operations.
#[derive(Clone, Debug, PartialEq)]
pub enum QuasarError {
    /// Qubit index out of range.
    QubitOutOfRange {
        /// The invalid qubit index
        qubit: usize,
        /// Maximum valid index + 1
        max: usize,
    },

    /// Classical bit index out of range.
    ClbitOutOfRange {
        /// The invalid clbit index
        clbit: usize,
        /// Maximum valid index + 1
        max: usize,
    },

    /// Qubit count mismatch between circuits.
    QubitMismatch {
        /// Expected number of qubits
        expected: usize,
        /// Actual number of qubits
        got: usize,
    },

    /// Invalid gate parameters.
    InvalidGateParams {
        /// Gate name
        gate: &'static str,
        /// Error message
        message: &'static str,
    },

    /// Duplicate qubit in instruction.
    DuplicateQubit {
        /// The duplicated qubit index
        qubit: usize,
    },

    /// Circuit is too large for operation.
    CircuitTooLarge {
        /// Number of qubits
        qubits: usize,
        /// Maximum supported
        max: usize,
    },

    /// State vector dimension mismatch.
    StateDimensionMismatch {
        /// Expected dimension
        expected: usize,
        /// Actual dimension
        got: usize,
    },

    /// State vector not normalized.
    StateNotNormalized {
        /// Actual norm
        norm: f64,
    },

    /// Invalid probability (not in [0, 1]).
    InvalidProbability {
        /// The invalid value
        value: f64,
    },

    /// Operation not supported.
    NotSupported {
        /// Description of unsupported operation
        operation: &'static str,
    },

    /// Simulation error.
    SimulationError {
        /// Error message
        message: std::string::String,
    },

    /// Backend error.
    BackendError {
        /// Backend name
        backend: &'static str,
        /// Error message
        message: std::string::String,
    },
}

impl fmt::Display for QuasarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitOutOfRange { qubit, max } => {
                write!(f, "qubit index {} out of range (max {})", qubit, max - 1)
            }
            Self::ClbitOutOfRange { clbit, max } => {
                write!(f, "classical bit index {} out of range (max {})", clbit, max - 1)
            }
            Self::QubitMismatch { expected, got } => {
                write!(f, "qubit count mismatch: expected {}, got {}", expected, got)
            }
            Self::InvalidGateParams { gate, message } => {
                write!(f, "invalid parameters for {}: {}", gate, message)
            }
            Self::DuplicateQubit { qubit } => {
                write!(f, "duplicate qubit {} in instruction", qubit)
            }
            Self::CircuitTooLarge { qubits, max } => {
                write!(f, "circuit with {} qubits exceeds maximum of {}", qubits, max)
            }
            Self::StateDimensionMismatch { expected, got } => {
                write!(f, "state dimension mismatch: expected {}, got {}", expected, got)
            }
            Self::StateNotNormalized { norm } => {
                write!(f, "state not normalized (norm = {})", norm)
            }
            Self::InvalidProbability { value } => {
                write!(f, "invalid probability: {} not in [0, 1]", value)
            }
            Self::NotSupported { operation } => {
                write!(f, "operation not supported: {}", operation)
            }
            Self::SimulationError { message } => {
                write!(f, "simulation error: {}", message)
            }
            Self::BackendError { backend, message } => {
                write!(f, "{} backend error: {}", backend, message)
            }
        }
    }
}

impl std::error::Error for QuasarError {}
