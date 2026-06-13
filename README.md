# Ternary Turing — Turing Machines over Balanced-Ternary Alphabet

**Ternary Turing** implements Turing machines over the alphabet {-1, 0, +1}. The machine reads ternary symbols from the tape, writes ternary symbols back, moves left/right/stays, and transitions between states. It provides tape encoding/decoding, non-zero counting, and supports the halting problem and busy beaver experiments in the ternary domain.

## Why It Matters

The Turing machine is the foundational model of computation. Studying it over a ternary alphabet reveals how computational power relates to the alphabet size. While binary Turing machines can compute exactly the same functions as ternary ones, the ternary version often uses fewer states and tape cells — balanced ternary is the most efficient radix for positional number systems. The busy beaver problem (finding the machine that writes the most non-zero symbols before halting) has different answers in ternary: ternary busy beaver values grow faster than binary ones, making the ternary version both more interesting and harder to analyze.

## How It Works

### Tape

The `Tape` is a vector of ternary values {-1, 0, +1} with a head position. Operations:
- `read()`: Return the trit at head position. O(1).
- `write(v)`: Write trit v at head. O(1).
- `move_head(dir)`: Move left (decrement), right (increment, extend if needed), or stay. O(1) amortized.

### Transitions

Each `Transition` specifies:
- Current state and read symbol
- Write symbol
- Direction (L, R, S)
- Next state

The machine stores transitions in a lookup table. Each step is O(1) table lookup.

### Machine Execution

```
TuringMachine::run():
  while current_state ≠ HALT:
    symbol = tape.read()
    transition = lookup(current_state, symbol)
    tape.write(transition.write)
    tape.move_head(transition.dir)
    current_state = transition.next
```

Each step is O(1). Total steps depend on the machine — may not halt.

### Tape Encoding

`encode()` converts the tape to a single number using balanced ternary positional notation:

```
n = Σ ((vᵢ + 1) mod 3) × 3ⁱ   for all tape cells
```

This provides a canonical fingerprint for tape configurations. O(cells).

### Non-Zero Count

Counts cells with non-zero values — the "score" for busy beaver problems. O(cells).

## Quick Start

```rust
use ternary_turing::{Tape, Transition, Direction};

let mut tape = Tape::new(10);
tape.write(1);
tape.move_head(Direction::R);
tape.write(-1);

// Build a simple machine that alternates +1 and -1
let transitions = vec![
    Transition { state: 0, read: 0, write: 1, dir: Direction::R, next: 1 },
    Transition { state: 1, read: 0, write: -1, dir: Direction::R, next: 0 },
];

let non_zero = tape.non_zero_count();
let encoded = tape.encode();
```

```bash
cargo add ternary-turing
```

## API

| Type / Function | Description |
|---|---|
| `Tape` | `{ cells: Vec<i8>, head: usize }` |
| `Transition` | `{ state, read, write, dir, next }` |
| `Direction` | `L`, `R`, `S` |
| `Tape::encode()` | Balanced ternary → integer |
| `Tape::non_zero_count()` | Count of ±1 cells |

## Architecture Notes

The Turing machine is the theoretical foundation for ternary computation in **SuperInstance**. It proves that {-1, 0, +1} is computationally universal — any computable function can be expressed as a ternary Turing machine. The γ + η = C conservation manifests in the tape: non-zero cells contribute γ (information), zero cells contribute η (entropy/blank space), and their sum is the tape length. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References:

- Turing, Alan. "On Computable Numbers," *Proc. London Math. Soc.*, 42, 1936 — original Turing machine.
- Knuth, Donald. *The Art of Computer Programming, Vol. 2*, §4.1 — balanced ternary efficiency.
| Rado, Tibor. "On Non-Computable Functions," *Bell System Tech. J.*, 41(3), 1962 — busy beaver problem.

## License

MIT
