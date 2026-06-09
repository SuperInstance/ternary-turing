# ternary-turing

*A ternary Turing machine. The simplest possible model of ternary computation — an infinite tape with {-1, 0, +1}, a head, and a state machine. If it can't be computed here, it can't be computed with ternary.*

## Why This Exists

Before you build ternary neural networks, ternary GPUs, or ternary programming languages, you should ask: *what can ternary systems actually compute?* The answer is: everything. A ternary Turing machine is Turing-complete — adding the third symbol (0) gives you more expressive power per cell than binary, but the computational class is the same.

This crate is the formal foundation. If a ternary algorithm works here, on this minimal machine, it works everywhere.

## Architecture

```
         Tape: [... 0, -1,  1,  0, -1,  0, ...]
                         ↑
                       Head (state: Q3)
                         │
                    Read: +1
                    Rule: (Q3, +1) → write(-1), move(R), next(Q5)
```

### Key Types

- **`TernaryTape`** — Infinite tape with {-1, 0, +1} values. Extends in both directions. Supports read, write, move left/right.
- **`TernaryState`** — Machine state (u32 wrapper with display).
- **`TernaryRule`** — (state, read_trit) → (write_trit, direction, next_state).
- **`TernaryTuringMachine`** — Execute rules on a tape. Step, run until halt, detect infinite loops.
- **`busy_beaver_ternary(n)`** — Compute the ternary busy beaver function for small n.

## Usage

```rust
use ternary_turing::*;

// Create a machine with rules
let rules = vec![
    TernaryRule::new(0, 0, 1, Dir::Right, 1),  // State 0, read 0 → write 1, go right, state 1
    TernaryRule::new(1, 0, -1, Dir::Left, 0),   // State 1, read 0 → write -1, go left, state 0
    // Halting: no rule for (1, 1) → machine halts
];

let mut tm = TernaryTuringMachine::new(rules, vec![0, 0, 1, 0, 0]);
tm.run(100); // Run up to 100 steps

assert!(tm.halted());
println!("Tape: {:?}", tm.tape().nonzero_cells());
println!("Steps: {}", tm.steps());
```

## Busy Beaver

The busy beaver function BB(n) = the maximum number of steps an n-state ternary Turing machine can take before halting, starting from an all-zero tape. This is uncomputable in general, but for small n we can enumerate:

| States | Steps | Non-zeros written |
|--------|-------|--------------------|
| 1      | 2     | 1                  |
| 2      | 8     | 4                  |
| 3      | ~40   | ~13                |

The ternary busy beaver grows faster than the binary version — the extra symbol gives more room for complexity.

## The Deeper Idea

This crate makes ternary computation tangible. You can see exactly how a ternary algorithm works — every read, write, and state transition is explicit. It's the assembly language of ternary.

The educational value is direct: if you can write a ternary Turing machine program that solves a problem, you understand the problem at its most fundamental level. The 0 symbol isn't just "neutral" — it's *potential*. Binary tapes have on/off. Ternary tapes have no/latent/yes. That third state enables qualitatively different programs.

## Related Crates

- `ternary-compiler` — Compiles ternary expressions to bytecode (higher-level than this)
- `ternary-game-of-life` — Another ternary cellular automaton (2D instead of 1D tape)
- `ternary-weather` — Ternary simulation (continuous instead of discrete states)
