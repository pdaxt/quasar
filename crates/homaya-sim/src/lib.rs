//! QUASAR Simulator - Ultra-fast quantum state vector simulation.
//!
//! This crate provides the core simulation engine for QUASAR, offering:
//!
//! - **Fast state vector simulation** optimized for cache efficiency
//! - **Full gate support** including all standard gates
//! - **Measurement** with proper state collapse
//! - **Sampling** for running multiple shots
//!
//! # Example
//!
//! ```rust
//! use homaya_core::Circuit;
//! use homaya_sim::Simulator;
//!
//! // Create a Bell state
//! let circuit = Circuit::new(2)
//!     .h(0)
//!     .cx(0, 1);
//!
//! // Simulate
//! let mut sim = Simulator::new();
//! let state = sim.run(&circuit).unwrap();
//!
//! // Check probabilities
//! assert!(state.probability(0) > 0.49); // |00⟩
//! assert!(state.probability(3) > 0.49); // |11⟩
//! ```
//!
//! # Sampling Multiple Shots
//!
//! ```rust
//! use homaya_core::Circuit;
//! use homaya_sim::Simulator;
//!
//! let circuit = Circuit::new(2)
//!     .h(0)
//!     .cx(0, 1)
//!     .measure_all();
//!
//! let mut sim = Simulator::with_seed(42);
//! let counts = sim.sample(&circuit, 1000).unwrap();
//!
//! // Bell state gives ~50% |00⟩ and ~50% |11⟩
//! println!("Results: {:?}", counts);
//! ```

#![deny(missing_docs)]

mod statevector;
mod simulator;

pub use statevector::StateVector;
pub use simulator::{Simulator, MeasurementResult};
