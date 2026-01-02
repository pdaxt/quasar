# Contributing to HOMAYA

**Welcome! We're learning quantum computing together.**

You don't need to be an expert. In fact, questions from beginners help us write better explanations.

## Ways to Contribute

### 1. Tell Us What's Confusing

This is the **most valuable** contribution. If something doesn't make sense:

1. Open an [Issue](https://github.com/pdaxt/homaya/issues/new)
2. Tell us what lesson/section confused you
3. Tell us what you expected vs what you found

We'll either fix the explanation or realize we don't understand it ourselves (and learn together!).

### 2. Try the Course and Give Feedback

- [Take the Interactive Course](https://pdaxt.github.io/homaya/)
- What clicked? What didn't?
- What analogy helped? What fell flat?

Open an issue or reach out directly:
- [LinkedIn - Pran](https://www.linkedin.com/in/pran-dataxlr8)
- [Substack](https://bsbskiller.com)

### 3. Improve Explanations

Found a better way to explain something?

1. Fork the repo
2. Edit `docs/learn/course.html`
3. Open a Pull Request
4. Tell us *why* your explanation is clearer

### 4. Add to the Simulator

Want to add features to `homaya-core` or `homaya-sim`?

1. Fork the repo
2. Create a branch: `git checkout -b feature/your-feature`
3. Write tests (we verify everything!)
4. Make your changes
5. Run tests: `cargo test`
6. Open a Pull Request

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/homaya.git
cd homaya

# Run tests
cargo test

# Run the verification suite
cargo run --example verify_correctness -p homaya-sim

# Open the course locally
open docs/learn/course.html
```

## Code Style

- We prefer clarity over cleverness
- Comment the "why", not the "what"
- If you learned something while coding, add a comment explaining it

## Questions?

Open an issue! We're friendly. We're learning too.

---

**Remember: If you're confused, that's a bug in our explanation, not a bug in you.**
