//! # QUASAR Core
//!
//! Ultra-fast quantum computing primitives.
//!
//! This crate provides the fundamental building blocks for quantum computation:
//! - [`Complex`] - High-performance complex number operations
//! - [`Gate`] - Quantum gate definitions
//! - [`Circuit`] - Quantum circuit builder
//!
//! ## Philosophy
//!
//! Every line has purpose. No bloat. Pure speed.
//!
//! ## Example
//!
//! ```rust
//! use homaya_core::{Circuit, Gate};
//!
//! // Create a Bell state: |00⟩ + |11⟩
//! let circuit = Circuit::new(2)
//!     .h(0)        // Hadamard on qubit 0
//!     .cx(0, 1);   // CNOT: control=0, target=1
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]

mod complex;
mod gate;
mod circuit;
mod error;

pub use complex::Complex;
pub use gate::{Gate, GateType, GateParams};
pub use circuit::{Circuit, Instruction};
pub use error::QuasarError;

/// Result type for QUASAR operations
pub type Result<T> = core::result::Result<T, QuasarError>;

/// The mathematical constant π
pub const PI: f64 = core::f64::consts::PI;

/// The mathematical constant τ = 2π
pub const TAU: f64 = core::f64::consts::TAU;

/// Square root of 2, precomputed for performance
pub const SQRT_2: f64 = core::f64::consts::SQRT_2;

/// 1/√2, used frequently in quantum gates
pub const INV_SQRT_2: f64 = 0.7071067811865476;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!((INV_SQRT_2 * SQRT_2 - 1.0).abs() < 1e-15);
    }
}
