//! Quantum gate definitions.
//!
//! All standard gates optimized for speed.

use crate::{Complex, INV_SQRT_2, PI};

/// The type of a quantum gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GateType {
    // Single-qubit gates
    /// Identity gate
    I,
    /// Pauli-X (NOT) gate
    X,
    /// Pauli-Y gate
    Y,
    /// Pauli-Z gate
    Z,
    /// Hadamard gate
    H,
    /// S gate (√Z)
    S,
    /// S-dagger gate
    Sdg,
    /// T gate (√S)
    T,
    /// T-dagger gate
    Tdg,
    /// Rotation around X-axis
    Rx,
    /// Rotation around Y-axis
    Ry,
    /// Rotation around Z-axis
    Rz,
    /// Phase gate
    P,
    /// U gate (general single-qubit)
    U,

    // Two-qubit gates
    /// Controlled-X (CNOT)
    CX,
    /// Controlled-Y
    CY,
    /// Controlled-Z
    CZ,
    /// Controlled-H
    CH,
    /// Controlled-Phase
    CP,
    /// Controlled-U
    CU,
    /// SWAP gate
    Swap,
    /// iSWAP gate
    ISwap,
    /// √SWAP gate
    SqrtSwap,

    // Three-qubit gates
    /// Toffoli (CCX)
    CCX,
    /// Controlled-SWAP (Fredkin)
    CSwap,

    // Measurement
    /// Measure qubit
    Measure,
    /// Reset qubit to |0⟩
    Reset,
    /// Barrier (no-op for timing)
    Barrier,
}

/// A quantum gate with its parameters.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gate {
    /// The type of gate
    pub gate_type: GateType,
    /// Parameters (angles for rotation gates)
    pub params: GateParams,
}

/// Gate parameters.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GateParams {
    /// No parameters
    None,
    /// Single angle (Rx, Ry, Rz, P)
    Angle(f64),
    /// Three angles (U gate: theta, phi, lambda)
    Angles3(f64, f64, f64),
}

impl Gate {
    // ========== Single-qubit gates ==========

    /// Identity gate.
    #[inline]
    pub const fn i() -> Self {
        Self {
            gate_type: GateType::I,
            params: GateParams::None,
        }
    }

    /// Pauli-X (NOT) gate.
    /// ```text
    /// |0⟩ → |1⟩
    /// |1⟩ → |0⟩
    /// ```
    #[inline]
    pub const fn x() -> Self {
        Self {
            gate_type: GateType::X,
            params: GateParams::None,
        }
    }

    /// Pauli-Y gate.
    #[inline]
    pub const fn y() -> Self {
        Self {
            gate_type: GateType::Y,
            params: GateParams::None,
        }
    }

    /// Pauli-Z gate.
    /// ```text
    /// |0⟩ → |0⟩
    /// |1⟩ → -|1⟩
    /// ```
    #[inline]
    pub const fn z() -> Self {
        Self {
            gate_type: GateType::Z,
            params: GateParams::None,
        }
    }

    /// Hadamard gate - creates superposition.
    /// ```text
    /// |0⟩ → (|0⟩ + |1⟩)/√2
    /// |1⟩ → (|0⟩ - |1⟩)/√2
    /// ```
    #[inline]
    pub const fn h() -> Self {
        Self {
            gate_type: GateType::H,
            params: GateParams::None,
        }
    }

    /// S gate (√Z).
    #[inline]
    pub const fn s() -> Self {
        Self {
            gate_type: GateType::S,
            params: GateParams::None,
        }
    }

    /// S-dagger gate.
    #[inline]
    pub const fn sdg() -> Self {
        Self {
            gate_type: GateType::Sdg,
            params: GateParams::None,
        }
    }

    /// T gate (√S).
    #[inline]
    pub const fn t() -> Self {
        Self {
            gate_type: GateType::T,
            params: GateParams::None,
        }
    }

    /// T-dagger gate.
    #[inline]
    pub const fn tdg() -> Self {
        Self {
            gate_type: GateType::Tdg,
            params: GateParams::None,
        }
    }

    /// Rotation around X-axis.
    #[inline]
    pub const fn rx(theta: f64) -> Self {
        Self {
            gate_type: GateType::Rx,
            params: GateParams::Angle(theta),
        }
    }

    /// Rotation around Y-axis.
    #[inline]
    pub const fn ry(theta: f64) -> Self {
        Self {
            gate_type: GateType::Ry,
            params: GateParams::Angle(theta),
        }
    }

    /// Rotation around Z-axis.
    #[inline]
    pub const fn rz(theta: f64) -> Self {
        Self {
            gate_type: GateType::Rz,
            params: GateParams::Angle(theta),
        }
    }

    /// Phase gate.
    #[inline]
    pub const fn p(theta: f64) -> Self {
        Self {
            gate_type: GateType::P,
            params: GateParams::Angle(theta),
        }
    }

    /// General single-qubit unitary.
    #[inline]
    pub const fn u(theta: f64, phi: f64, lambda: f64) -> Self {
        Self {
            gate_type: GateType::U,
            params: GateParams::Angles3(theta, phi, lambda),
        }
    }

    // ========== Two-qubit gates ==========

    /// Controlled-X (CNOT) gate.
    #[inline]
    pub const fn cx() -> Self {
        Self {
            gate_type: GateType::CX,
            params: GateParams::None,
        }
    }

    /// Controlled-Y gate.
    #[inline]
    pub const fn cy() -> Self {
        Self {
            gate_type: GateType::CY,
            params: GateParams::None,
        }
    }

    /// Controlled-Z gate.
    #[inline]
    pub const fn cz() -> Self {
        Self {
            gate_type: GateType::CZ,
            params: GateParams::None,
        }
    }

    /// Controlled-H gate.
    #[inline]
    pub const fn ch() -> Self {
        Self {
            gate_type: GateType::CH,
            params: GateParams::None,
        }
    }

    /// Controlled-Phase gate.
    #[inline]
    pub const fn cp(theta: f64) -> Self {
        Self {
            gate_type: GateType::CP,
            params: GateParams::Angle(theta),
        }
    }

    /// SWAP gate.
    #[inline]
    pub const fn swap() -> Self {
        Self {
            gate_type: GateType::Swap,
            params: GateParams::None,
        }
    }

    // ========== Three-qubit gates ==========

    /// Toffoli (CCX) gate.
    #[inline]
    pub const fn ccx() -> Self {
        Self {
            gate_type: GateType::CCX,
            params: GateParams::None,
        }
    }

    /// Controlled-SWAP (Fredkin) gate.
    #[inline]
    pub const fn cswap() -> Self {
        Self {
            gate_type: GateType::CSwap,
            params: GateParams::None,
        }
    }

    // ========== Special operations ==========

    /// Measure qubit.
    #[inline]
    pub const fn measure() -> Self {
        Self {
            gate_type: GateType::Measure,
            params: GateParams::None,
        }
    }

    /// Reset qubit to |0⟩.
    #[inline]
    pub const fn reset() -> Self {
        Self {
            gate_type: GateType::Reset,
            params: GateParams::None,
        }
    }

    /// Barrier (synchronization point).
    #[inline]
    pub const fn barrier() -> Self {
        Self {
            gate_type: GateType::Barrier,
            params: GateParams::None,
        }
    }

    // ========== Matrix representation ==========

    /// Get the 2x2 matrix for a single-qubit gate.
    ///
    /// Returns `[[a, b], [c, d]]` where the matrix is:
    /// ```text
    /// ┌     ┐
    /// │ a b │
    /// │ c d │
    /// └     ┘
    /// ```
    pub fn matrix_2x2(&self) -> Option<[[Complex; 2]; 2]> {
        use GateType::*;

        let zero = Complex::ZERO;
        let one = Complex::ONE;
        let i = Complex::I;
        let h = Complex::from_real(INV_SQRT_2);

        match self.gate_type {
            I => Some([[one, zero], [zero, one]]),

            X => Some([[zero, one], [one, zero]]),

            Y => Some([[zero, -i], [i, zero]]),

            Z => Some([[one, zero], [zero, -one]]),

            H => Some([[h, h], [h, -h]]),

            S => Some([[one, zero], [zero, i]]),

            Sdg => Some([[one, zero], [zero, -i]]),

            T => {
                let t = Complex::from_polar(1.0, PI / 4.0);
                Some([[one, zero], [zero, t]])
            }

            Tdg => {
                let t = Complex::from_polar(1.0, -PI / 4.0);
                Some([[one, zero], [zero, t]])
            }

            Rx => {
                if let GateParams::Angle(theta) = self.params {
                    let cos = Complex::from_real((theta / 2.0).cos());
                    let sin = Complex::new(0.0, -(theta / 2.0).sin());
                    Some([[cos, sin], [sin, cos]])
                } else {
                    None
                }
            }

            Ry => {
                if let GateParams::Angle(theta) = self.params {
                    let cos = Complex::from_real((theta / 2.0).cos());
                    let sin = Complex::from_real((theta / 2.0).sin());
                    Some([[cos, -sin], [sin, cos]])
                } else {
                    None
                }
            }

            Rz => {
                if let GateParams::Angle(theta) = self.params {
                    let e_neg = Complex::from_polar(1.0, -theta / 2.0);
                    let e_pos = Complex::from_polar(1.0, theta / 2.0);
                    Some([[e_neg, zero], [zero, e_pos]])
                } else {
                    None
                }
            }

            P => {
                if let GateParams::Angle(theta) = self.params {
                    let phase = Complex::from_polar(1.0, theta);
                    Some([[one, zero], [zero, phase]])
                } else {
                    None
                }
            }

            U => {
                if let GateParams::Angles3(theta, phi, lambda) = self.params {
                    let cos = (theta / 2.0).cos();
                    let sin = (theta / 2.0).sin();
                    Some([
                        [
                            Complex::from_real(cos),
                            -Complex::from_polar(1.0, lambda) * sin,
                        ],
                        [
                            Complex::from_polar(1.0, phi) * sin,
                            Complex::from_polar(1.0, phi + lambda) * cos,
                        ],
                    ])
                } else {
                    None
                }
            }

            _ => None, // Multi-qubit gates don't have 2x2 matrices
        }
    }

    /// Returns the number of qubits this gate operates on.
    #[inline]
    pub const fn num_qubits(&self) -> usize {
        use GateType::*;
        match self.gate_type {
            I | X | Y | Z | H | S | Sdg | T | Tdg | Rx | Ry | Rz | P | U | Measure | Reset => 1,
            CX | CY | CZ | CH | CP | CU | Swap | ISwap | SqrtSwap => 2,
            CCX | CSwap => 3,
            Barrier => 0, // Barrier can span any number
        }
    }

    /// Returns true if this gate is a controlled gate.
    #[inline]
    pub const fn is_controlled(&self) -> bool {
        use GateType::*;
        matches!(
            self.gate_type,
            CX | CY | CZ | CH | CP | CU | CCX | CSwap
        )
    }

    /// Returns true if this gate modifies the quantum state.
    #[inline]
    pub const fn is_unitary(&self) -> bool {
        !matches!(self.gate_type, GateType::Measure | GateType::Reset | GateType::Barrier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_matrix() {
        let h = Gate::h();
        let m = h.matrix_2x2().unwrap();

        // H² = I
        let h_squared = [
            [
                m[0][0] * m[0][0] + m[0][1] * m[1][0],
                m[0][0] * m[0][1] + m[0][1] * m[1][1],
            ],
            [
                m[1][0] * m[0][0] + m[1][1] * m[1][0],
                m[1][0] * m[0][1] + m[1][1] * m[1][1],
            ],
        ];

        assert!(h_squared[0][0].approx_eq(Complex::ONE, 1e-10));
        assert!(h_squared[0][1].approx_eq(Complex::ZERO, 1e-10));
        assert!(h_squared[1][0].approx_eq(Complex::ZERO, 1e-10));
        assert!(h_squared[1][1].approx_eq(Complex::ONE, 1e-10));
    }

    #[test]
    fn test_pauli_anticommutation() {
        // XY = iZ, YX = -iZ → XY + YX = 0
        let x = Gate::x().matrix_2x2().unwrap();
        let y = Gate::y().matrix_2x2().unwrap();

        // XY
        let xy = [
            [
                x[0][0] * y[0][0] + x[0][1] * y[1][0],
                x[0][0] * y[0][1] + x[0][1] * y[1][1],
            ],
            [
                x[1][0] * y[0][0] + x[1][1] * y[1][0],
                x[1][0] * y[0][1] + x[1][1] * y[1][1],
            ],
        ];

        // YX
        let yx = [
            [
                y[0][0] * x[0][0] + y[0][1] * x[1][0],
                y[0][0] * x[0][1] + y[0][1] * x[1][1],
            ],
            [
                y[1][0] * x[0][0] + y[1][1] * x[1][0],
                y[1][0] * x[0][1] + y[1][1] * x[1][1],
            ],
        ];

        // XY + YX should be zero matrix
        for i in 0..2 {
            for j in 0..2 {
                assert!((xy[i][j] + yx[i][j]).is_zero(1e-10));
            }
        }
    }
}
