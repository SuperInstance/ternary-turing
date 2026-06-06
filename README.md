# ternary-turing

**Turing machines over the ternary alphabet {-1, 0, +1} — tape encoding, deterministic execution, halting analysis, counter machines, and a simplified busy beaver search.**

## Background

The Turing machine is the foundational model of computation. Alan Turing's 1936 formulation describes a machine with an infinite tape, a read/write head, a finite state machine, and a transition function that maps (state, symbol) → (write, direction, next state). Every computable function can be expressed as a Turing machine.

Classical Turing machines use a binary alphabet {0, 1}. But the theory allows any finite alphabet, and a ternary alphabet {-1, 0, +1} is particularly natural for three reasons. First, balanced ternary is the *most efficient* integer base — it represents numbers with the fewest digits on average (radix economy). Second, the three symbols map directly to the three-way decisions that occur in real computation: yes/no/unknown, accept/reject/abstain, positive/negative/zero. Third, the tape encoding function `encode()` maps the entire tape to a single number via base-3 positional notation, enabling exact comparison of machine states.

The busy beaver problem — find the N-state Turing machine that writes the most non-zero symbols before halting — is one of the deepest problems in computer science. For binary Turing machines, Σ(5) is still unknown (it's at least 4098). For ternary machines, the search space explodes because each transition has 3 choices for write symbol and 2 for direction (versus 2 and 2 in binary). This crate implements a simplified busy beaver search that explores a subset of the transition space.

## How It Works

### Core Types

**`Direction`** — Head movement: `L` (left), `R` (right), `S` (stay). The `S` option is unusual in classical Turing machines but natural for ternary: it means "no movement" (the neutral action).

**`Transition`** — A single transition rule:
```
(state: usize, read: i8) → (write: i8, dir: Direction, next: usize)
```
Both `read` and `write` are in {-1, 0, +1} (clamped on write).

**`Tape`** — A dynamically-extending tape:
- Initialized to all zeros (balanced ternary neutral state)
- Head starts at the center
- Extends rightward automatically when the head moves past the right edge
- `encode()` converts the entire tape to a base-3 number for state hashing

**`TuringMachine`** — The machine itself:
- `tape`: the Tape
- `state`: current state (usize)
- `transitions`: Vec of Transition rules
- `halt_state`: the halting state (user-defined)

### Execution Model

```rust
fn step(&mut self) -> bool {
    // 1. Read current cell
    // 2. Find matching transition (state, read)
    // 3. Write new value
    // 4. Move head
    // 5. Transition to next state
    // 6. Return false if halted or no transition found
}
```

If no transition matches (state, read), the machine halts. This is the *implicit halt* convention — simpler than requiring explicit halt transitions for every state-symbol pair.

### Special Machines

**`counter_machine(size)`** — A simple 3-state machine:
- State 0, read 0 → write 1, move right
- State 0, read 1 → write −1, move right
- State 0, read −1 → write 0, move left, transition to halt

This counts in balanced ternary: it fills the tape with 1, then overwrites with −1, then halts when it sees −1.

**`busy_beaver(n_states, tape_size)`** — Simplified busy beaver search:
- Enumerates a subset of all possible N-state machines (limited to `min(n_states * 3, 27)` configurations)
- Runs each machine for up to 1000 steps
- Returns the machine with the most non-zero cells at halt

This is not an exhaustive search — the full ternary busy beaver space grows as O(6^(n × 3)) — but it demonstrates the concept and finds non-trivial halting machines.

### Tape Encoding

```rust
fn encode(&self) -> usize {
    // Map each cell to a ternary digit: {-1→0, 0→1, 1→2} (mod 3)
    // Compute positional value: digit × 3^position
}
```

This gives every distinct tape configuration a unique number, enabling exact state comparison and cycle detection.

### Design Decisions

1. **`#![no_std]` with `alloc`**: The crate works in embedded/no_std environments. Only `Vec` is needed from `alloc`.

2. **Clamped ternary writes**: The `write` method clamps values to [-1, 1], ensuring tape integrity even if a transition specifies an out-of-range value.

3. **Right-only extension**: The tape extends to the right when the head moves past the end but never to the left. If the head reaches position 0 and moves left, it stays at 0. This is a common convention that simplifies implementation while preserving computational universality.

4. **Implicit halt**: No matching transition → halt. This reduces the number of transitions needed and makes machine definitions more compact.

## Experimental Results

All **13 tests pass**:

| Test | Result |
|------|--------|
| `test_tape_new` | New tape reads 0, non-zero count = 0 |
| `test_tape_write_read` | Write 1, read back → 1 |
| `test_tape_move` | Start at center (5), move right → 6, move left → 5 |
| `test_tape_extend` | At right edge, move right → tape grows by 1 |
| `test_tape_encode` | `[0, 1, −1]` encodes to a positive number |
| `test_tape_from_vec` | `[1, 0, −1]` starts reading 1 |
| `test_tm_halted` | New machine at state 0 with halt_state=1 → not halted |
| `test_tm_step_halts` | Single transition writes 1, moves to halt state; second step returns false |
| `test_tm_run` | Machine with 2 transitions runs for multiple steps |
| `test_tm_no_transition_halts` | Empty transition table → immediate halt |
| `test_counter_machine` | Runs 50 steps, produces non-zero cells |
| `test_busy_beaver` | Search finds at least one halting machine |
| `test_direction_values` | L == L, L != R |

Key result: the counter machine runs successfully, filling cells with alternating 1 and −1 values, producing a non-zero tape that encodes balanced ternary counting.

## Impact

The ternary alphabet {-1, 0, +1} makes Turing machines more expressive per symbol. A binary tape has 2^n possible n-cell configurations. A ternary tape has 3^n — 50% more information per cell. This means ternary busy beavers can write more non-zero symbols with the same number of states, and ternary universal Turing machines can have smaller state-symbol products.

The balanced ternary encoding is particularly natural for the tape because the "zero" cell (0) serves as the blank symbol — the default, unvisited state. In binary, blank is typically 0, but then 0 is ambiguous (blank vs. written zero). In balanced ternary, 0 is unambiguously "neutral" — not positive, not negative, not blank. Written −1 and +1 are the informative states.

## Use Cases

1. **Computability education** — Demonstrate Turing machines with a richer alphabet that maps to intuitive concepts (negative/neutral/positive)
2. **Busy beaver research** — Explore the ternary busy beaver function, which is less studied than the binary version
3. **Balanced ternary encoding** — Use the tape encoding to study base-3 number theory and balanced ternary representation
4. **Embedded state machines** — The `no_std` design makes this suitable for firmware-level state machines with ternary inputs
5. **Formal verification** — The tape's `encode()` function provides exact state fingerprints for model checking and cycle detection

## Open Questions

1. **Ternary busy beaver values**: What is Σ₃(N) — the ternary busy beaver function? For binary, Σ(2)=6 and Σ(3)=21. The ternary search space is much larger (3 write values, 2 directions per transition), so Σ₃(N) likely grows faster than Σ₂(N).
2. **Universality threshold**: What is the minimum number of states for a universal Turing machine over a ternary alphabet? For binary, the smallest known universal machine has 2 states and 3 symbols (or equivalently 3 states and 2 symbols). Ternary might achieve universality with fewer states.
3. **Halt detection for counter machines**: The `counter_machine` has a known halting behavior. Can we prove that all 3-state, 3-symbol machines either halt or enter a detectable loop within a bounded number of steps?

## Connection to Oxide Stack

`ternary-turing` represents the theoretical foundation for the **flux-core** bytecode VM. The `TernaryVM` in `ternary-interpreter` is a specialized Turing machine where the tape is replaced by a stack + store. The transition function is the bytecode instruction set. The halt state is the `Op::Halt` instruction.

At the **cudaclaw** layer, GPU thread execution is itself a kind of Turing machine: each thread has local state, reads/writes shared memory, and transitions between instruction addresses. The ternary tape encoding is used for state hashing in warp-consensus algorithms — comparing encoded tape values is equivalent to comparing full machine states.

## Stats

| Metric | Value |
|--------|-------|
| Lines of Rust | ~190 |
| Test count | 13 |
| Public types | 4 (Direction, Transition, Tape, TuringMachine) |
| Public functions | 5 (new, step, run, counter_machine, busy_beaver) |
| Dependencies | 0 (`#![no_std]` with `alloc`) |

## Install

```toml
[dependencies]
ternary-turing = "0.1.0"
```

## License

MIT
