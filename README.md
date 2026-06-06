# ternary-turing

Turing machines with ternary tape — computation, halting, and busy beaver

## Why This Matters

# ternary-turing
Turing machines over ternary alphabet {-1, 0, 1}.

## The Five-Layer Stack

This crate is part of the **Oxide Stack** — a distributed GPU runtime built on five layers:

```
┌─────────────────┐
│  cudaclaw        │  Persistent GPU kernels, warp consensus, SmartCRDT
├─────────────────┤
│  cuda-oxide      │  Flux → MIR → Pliron → NVVM → PTX compiler
├─────────────────┤
│  flux-core       │  Bytecode VM + A2A agent protocol
├─────────────────┤
│  pincher         │  "Vector DB as runtime, LLM as compiler"
├─────────────────┤
│  open-parallel   │  Async runtime (tokio fork)
└─────────────────┘
```

The key insight: **ternary values {-1, 0, +1} map directly to GPU compute**. They pack 16× denser than FP32, enable XNOR+popcount matmul, and conservation laws become compile-time checks.

## Design

Every value in this crate follows **ternary algebra** (Z₃):

| Value | Meaning | GPU Analog |
|-------|---------|------------|
| +1 | Positive / Active / Healthy | Warp vote yes |
| 0 | Neutral / Pending / Balanced | Warp vote abstain |
| -1 | Negative / Failed / Overloaded | Warp vote no |

This isn't arbitrary — ternary is the natural encoding for:
1. **BitNet b1.58** (Microsoft) — ternary LLMs at 60% less power
2. **GPU warp voting** — hardware ballot returns ternary consensus
3. **Conservation laws** — {-1, 0, +1} preserves quantity

## Key Types

```rust
pub enum Direction
pub struct Transition
pub struct Tape
pub fn new
pub fn from_vec
pub fn read
pub fn write
pub fn move_head
pub fn non_zero_count
pub fn encode
pub struct TuringMachine
pub fn new
```

## Usage

```toml
[dependencies]
ternary-turing = "0.1.0"
```

```rust
use ternary_turing::*;
// See src/lib.rs tests for complete working examples
```

## Testing

```bash
git clone https://github.com/SuperInstance/ternary-turing.git
cd ternary-turing
cargo test    # 13 tests
```

## Stats

| Metric | Value |
|--------|-------|
| Tests | 13 |
| Lines of Rust | 289 |
| Public API | 17 items |

## License

Apache-2.0
