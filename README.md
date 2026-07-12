# ternary-turing

Turing machines over the **ternary tape alphabet {-1, 0, +1}**, with support for custom transition functions, halting analysis, busy beaver search, and base-3 tape encoding. `#![no_std]` and `#![forbid(unsafe_code)]`.

## Why It Matters

The ternary alphabet {-1, 0, +1} is the minimal symmetric tape alphabet: it has a natural additive structure that binary {0, 1} lacks. This makes ternary Turing machines the cleanest model for studying computation over signed symbols — a setup that connects directly to the ternary agent ecosystem where agents hold values in exactly this set.

Turing machines are the foundational model of computation. Understanding which ternary-language machines halt, and how productive they are before halting, grounds the theory of ternary agent decision-making in computability.

## How It Works

### Machine Definition

A Turing machine is a tuple **M = (Q, Σ, δ, q₀, q_halt)**:

- **Q**: finite set of states (represented as `usize`)
- **Σ**: tape alphabet = {-1, 0, +1} (clamped from `i8`)
- **δ**: transition function: **(state, read) → (write, direction, next_state)**
- **q₀**: initial state (= 0)
- **q_halt**: halting state

### Transition Function

Each transition is:

```
δ(q, a) = (b, D, q')
```

Where *b* ∈ {-1, 0, +1} is the symbol written, *D* ∈ {L, R, S} is the head direction, and *q'* is the next state.

If no transition matches `(state, read)`, the machine enters the halt state.

### Tape Encoding

The tape is encoded as a base-3 number using the mapping:

```
-1 → digit 0
 0 → digit 1
+1 → digit 2
```

This is the **balanced ternary offset encoding**. For a tape of length *n*, the encoding is:

```
encode(tape) = Σᵢ digit(tape[i]) · 3^i
```

- **Complexity:** O(n) time, O(n) space
- **Range:** [0, 3ⁿ − 1]

### Halting and Step Complexity

Each `step()` is **O(|δ|)** (linear scan of transition table). The `run(max_steps)` function is O(max_steps · |δ|).

- **Worst case:** machine never halts — `run()` stops at `max_steps`.
- **Best case:** machine halts immediately (no matching transition).

### Busy Beaver

The busy beaver function **S(n)** asks: what is the maximum number of non-zero cells written by an *n*-state halting ternary Turing machine starting from the all-zero tape?

```
S(n) = max { non_zero_count(M) : M halts, |Q| = n }
```

This crate implements a simplified (non-exhaustive) search over a subset of machines. The true busy beaver function grows faster than any computable function — it is **non-computable** (Turing, 1936).

For the ternary alphabet, the problem is related to the **generalized busy beaver** over k-symbol machines, which is even harder than the classical 2-symbol version.

### Counter Machine

A pre-built machine that cycles the starting cell through the full ternary alphabet `0 → +1 → -1 → 0` and then halts. All transitions use `Direction::S` (stay) so the head re-reads the symbol it just wrote — moving away after each write would leave the written symbol on a fresh `0` cell and the cycle could never progress past the first step:

```
δ(0, 0)  → (1,  S, 0)    // 0 → +1, stay
δ(0, 1)  → (-1, S, 0)    // +1 → -1, stay
δ(0, -1) → (0,  S, 1)    // -1 → 0, halt
```

Starting from an all-zero tape, the machine halts in exactly 3 steps with the starting cell back at `0`.

## Quick Start

```rust
use ternary_turing::*;

// Build a simple machine: flip all zeros to ones, halt on non-zero
let transitions = vec![
    Transition { state: 0, read: 0,  write: 1,  dir: Direction::R, next: 0 },
    Transition { state: 0, read: 1,  write: 1,  dir: Direction::R, next: 1 },
];
let tape = Tape::new(10);
let mut tm = TuringMachine::new(tape, transitions, 1);

let steps = tm.run(1000);
println!("Ran {} steps, {} non-zero cells", steps, tm.tape.non_zero_count());
```

## API

| Type | Purpose |
|---|---|
| `Tape` | Infinite-extent tape (auto-extends in both directions) |
| `Transition` | Single δ entry: (state, read) → (write, dir, next) |
| `TuringMachine` | State machine + tape + transition table |
| `Direction` | L, R, or S (Stay) |
| `busy_beaver(n, tape_size)` | Search for productive n-state machines |
| `counter_machine(size)` | Pre-built cycling machine |

## Architecture Notes

The ternary tape alphabet {-1, 0, +1} is the same trit space used throughout the ecosystem. The base-3 encoding of the tape maps directly to the ternary tuple space of `ternary-tuple`, and the balanced-ternary arithmetic connects to the **γ + η = C** conservation law: on a halting tape, the sum of +1 cells minus the sum of −1 cells gives the net ternary bias, which must satisfy conservation constraints when interpreted as an agent population distribution.

## References

- Turing, A. M. (1936). *"On Computable Numbers, with an Application to the Entscheidungsproblem."* Proc. LMS.
- Rado, T. (1962). *"On Non-Computable Functions."* Bell System Technical Journal. — Original busy beaver definition.
- Knuth, D. E. (1997). *The Art of Computer Programming, Vol. 2.* §4.1: Positional Number Systems (balanced ternary).

## License

MIT
