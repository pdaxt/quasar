<p align="center">
  <h1 align="center">HOMAYA</h1>
</p>

<p align="center">
  <em>Some connections transcend distance.</em>
</p>

<p align="center">
  A quantum computing framework built from scratch.<br>
  Named for my wife. Like entangled qubits, some bonds don't need explanation.
</p>

---

<h2 align="center">THE MISSION</h2>

<p align="center">
  <strong>Make quantum computing accessible to everyone.</strong>
</p>

<table align="center">
  <tr>
    <td align="center"><strong>WHAT</strong></td>
    <td>Build a complete quantum computing framework from scratch</td>
  </tr>
  <tr>
    <td align="center"><strong>HOW</strong></td>
    <td>Learn it ourselves and document everything along the way</td>
  </tr>
  <tr>
    <td align="center"><strong>WHY</strong></td>
    <td>So anyone curious can learn quantum computing - no PhD required</td>
  </tr>
</table>

<p align="center">
  <em>We're not waiting until we're experts. We're learning in public and building as we go.</em>
</p>

---

<p align="center">
  <a href="#try-it-now">Try It Now</a> •
  <a href="#the-journey">The Journey</a> •
  <a href="#the-interactive-course">Course</a> •
  <a href="#learn-with-us">Learn With Us</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.75+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/status-learning%20project-purple.svg" alt="Status">
</p>

---

## Try It Now

**No installation needed! Start learning quantum computing in your browser:**

### **[Launch Interactive Course](https://pdaxt.github.io/homaya/docs/learn/course.html)**

Click the link above to start learning. Works on any device.

---

## The Journey

**This is us learning quantum computing. In public. From scratch.**

We're not quantum physicists. We're not academics. We're just curious people who wanted to understand how quantum computers actually work.

Every resource we found was either:
- Written for physics PhDs (too hard)
- Oversimplified YouTube videos (too shallow)

So we decided: **let's build a quantum computer simulator ourselves and see if we actually understand it.**

### The Crazy Part

If our simulator passes physics tests, that means we *actually* understand the math. We can't fake it. The computer will tell us if we're wrong.

```
✓ Probability Conservation - All probabilities sum to 1
✓ H² = I - Hadamard is self-inverse
✓ Bell State - Perfect entanglement correlation
✓ CNOT Truth Table - Controlled operations work
... and 4 more physics tests
```

**It passes. We're learning.**

---

## The Interactive Course

As we learn, we write explanations. Not for experts - for ourselves. For people who have never touched quantum computing.

**[Take the Course](https://pdaxt.github.io/homaya/docs/learn/course.html)**

### What You'll Learn

| Lesson | What You'll Understand |
|--------|----------------------|
| **Light Switches** | How regular computers work (bits = on/off switches) |
| **The Spinning Coin** | What makes quantum different (qubits can be BOTH) |
| **Weighted Coins** | How we control quantum probabilities |
| **The H Button** | The most important quantum operation (Hadamard) |
| **Building Circuits** | Chaining operations together |
| **Two Qubits** | Exponential power (2 qubits = 4 states at once!) |
| **CNOT Gate** | Making qubits work together |
| **Entanglement** | The "spooky" thing Einstein hated |

### No Prerequisites

- No physics degree
- No linear algebra
- No quantum mechanics background
- Just curiosity

We explain everything like you're 10 years old. Because that's how we had to explain it to ourselves.

---

## Our Learning Philosophy

```
1. Build it → Forces you to truly understand
2. Test it → Can't fake understanding to a computer
3. Explain it simply → If you can't explain it simply, you don't understand it
4. Share it → Others help you find gaps in your knowledge
```

### What We've Built So Far

| Component | What It Does | Did We Learn It? |
|-----------|-------------|------------------|
| **homaya-core** | Qubits, quantum gates, circuits | Yes |
| **homaya-sim** | Simulates quantum circuits | Yes |
| **Interactive Course** | Teaches what we learned | Writing as we go |
| **Verification Tests** | Proves our math is right | All passing |

---

## For Developers

Want to run the simulator locally?

```bash
# Clone and enter
git clone https://github.com/pdaxt/homaya.git
cd homaya

# Run a demo
cargo run --example sim_demo -p homaya-sim

# Prove it's mathematically correct
cargo run --example verify_correctness -p homaya-sim
```

### Build Your First Quantum Circuit

```rust
use homaya_core::Circuit;
use homaya_sim::Simulator;

// Create entangled qubits (Bell state)
let circuit = Circuit::new(2)
    .h(0)        // Put qubit 0 in superposition
    .cx(0, 1)    // Entangle qubit 1 with qubit 0
    .measure_all();

// Simulate it
let mut sim = Simulator::new();
let counts = sim.sample(&circuit, 1000).unwrap();

// You'll get ~50% "00" and ~50% "11"
// Never "01" or "10" - they're entangled!
println!("{:?}", counts);
```

---

## Learn With Us

**You don't need to be an expert to contribute. We're not experts either!**

### Ways to Help

| If you're... | You can... |
|-------------|-----------|
| **A beginner** | Tell us what's confusing - seriously, this helps us write better explanations |
| **Learning too** | Try the course and let us know what clicks (or doesn't) |
| **A developer** | Add features to the simulator |
| **Quantum-curious** | Share it with others who might want to learn |

### Ask Questions!

If something doesn't make sense, that's a bug in our explanation, not a bug in you.

Open an issue. We'll either:
1. Improve the explanation, or
2. Realize we don't understand it either (and learn together!)

---

## Project Status

### What Works

- Full state vector quantum simulation
- All standard gates: X, Y, Z, H, S, T, Rx, Ry, Rz, CNOT, CZ, SWAP, Toffoli
- Measurement and sampling
- Interactive browser-based course
- 8 physics verification tests (all passing)

### What We're Learning Next

- [ ] Python bindings (so more people can use it)
- [ ] Browser/WASM support (run simulations in the course!)
- [ ] More course lessons (quantum algorithms!)
- [ ] GPU acceleration (for bigger simulations)

---

## Project Structure

```
homaya/
├── crates/
│   ├── homaya-core/      # The fundamentals (we learned this first)
│   ├── homaya-sim/       # The simulator (we learned this second)
│   └── ...               # More to come as we learn
├── docs/
│   └── learn/            # Interactive course (our notes, but prettier)
└── examples/
    └── rust/             # Working examples (proof we understand it)
```

---

## About

Built by **Pran** - learning quantum computing one qubit at a time.

Named for **Homaya** - because like entangled qubits, some connections transcend distance.

- [LinkedIn](https://www.linkedin.com/in/pran-dataxlr8) - Let's connect!
- [Substack](https://bskiller.com) - Follow the learning journey

---

## License

MIT / Apache 2.0 - Use it however you want.

---

<p align="center">
  <strong>Learning in public. Building in the open.</strong>
</p>

<p align="center">
  <em>We started confused. We're getting less confused. Join us.</em>
</p>

<p align="center">
  <a href="https://pdaxt.github.io/homaya/docs/learn/course.html">
    <strong>Start Learning Now</strong>
  </a>
</p>
