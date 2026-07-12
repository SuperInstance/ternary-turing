//! # ternary-turing
//!
//! Turing machines over ternary alphabet {-1, 0, 1}.

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

/// Direction: Left, Right, or Stay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    L,
    R,
    S,
}

/// A transition: (state, read) → (write, direction, next_state)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transition {
    pub state: usize,
    pub read: i8,
    pub write: i8,
    pub dir: Direction,
    pub next: usize,
}

/// Turing machine tape
#[derive(Debug, Clone)]
pub struct Tape {
    pub cells: Vec<i8>,
    pub head: usize,
}

impl Tape {
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![0; size],
            head: size / 2,
        }
    }

    pub fn from_vec(data: Vec<i8>) -> Self {
        let mut tape = Self {
            cells: data,
            head: 0,
        };
        for c in tape.cells.iter_mut() {
            *c = (*c).clamp(-1, 1);
        }
        tape
    }

    pub fn read(&self) -> i8 {
        self.cells.get(self.head).copied().unwrap_or(0)
    }

    pub fn write(&mut self, v: i8) {
        if self.head < self.cells.len() {
            self.cells[self.head] = v.clamp(-1, 1);
        }
    }

    /// Move the head one cell in `dir`.
    ///
    /// The tape is infinite in *both* directions: moving right past the last
    /// cell appends a fresh `0`, and moving left from cell 0 prepends a fresh
    /// `0` (leaving `head` at 0, now pointing at the new cell). `S` leaves the
    /// head in place. This never silently no-ops: a requested move always
    /// actually moves the head to a distinct cell.
    pub fn move_head(&mut self, dir: Direction) {
        match dir {
            Direction::L => {
                if self.head == 0 {
                    self.cells.insert(0, 0);
                    // head stays 0, now over the freshly prepended cell
                } else {
                    self.head -= 1;
                }
            }
            Direction::R => {
                self.head += 1;
                if self.head >= self.cells.len() {
                    self.cells.push(0);
                }
            }
            Direction::S => {}
        }
    }

    /// Count non-zero cells
    pub fn non_zero_count(&self) -> usize {
        self.cells.iter().filter(|&&v| v != 0).count()
    }

    /// Encode tape as a single number (base 3)
    pub fn encode(&self) -> usize {
        let mut n = 0usize;
        let mut place = 1usize;
        for &v in &self.cells {
            let digit = ((v + 1) % 3) as usize;
            n += digit * place;
            place *= 3;
        }
        n
    }
}

/// A Turing machine
#[derive(Debug, Clone)]
pub struct TuringMachine {
    pub tape: Tape,
    pub state: usize,
    pub transitions: Vec<Transition>,
    pub halt_state: usize,
}

impl TuringMachine {
    pub fn new(tape: Tape, transitions: Vec<Transition>, halt_state: usize) -> Self {
        Self {
            tape,
            state: 0,
            transitions,
            halt_state,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.state == self.halt_state
    }

    /// Find matching transition
    fn find_transition(&self) -> Option<&Transition> {
        let read = self.tape.read();
        self.transitions
            .iter()
            .find(|t| t.state == self.state && t.read == read)
    }

    /// Execute one step. Returns false if the machine halted on this call
    /// (either it was already halted, or no transition matched the current
    /// `(state, read)` pair, which forces the machine into the halt state).
    pub fn step(&mut self) -> bool {
        if self.is_halted() {
            return false;
        }
        match self.find_transition().copied() {
            Some(t) => {
                self.tape.write(t.write);
                self.tape.move_head(t.dir);
                self.state = t.next;
                true
            }
            None => {
                self.state = self.halt_state;
                false
            }
        }
    }

    /// Run up to max_steps. Returns number of steps actually executed.
    pub fn run(&mut self, max_steps: usize) -> usize {
        let mut count = 0;
        for _ in 0..max_steps {
            if !self.step() {
                break;
            }
            count += 1;
        }
        count
    }
}

/// Busy beaver: find the machine with N states that writes the most 1s before halting
/// Starts with all-0 tape, transitions on {-1, 0, 1}
pub fn busy_beaver(n_states: usize, tape_size: usize) -> (Vec<Transition>, usize) {
    let halt = n_states;
    let symbols = [0i8, 1, -1]; // simplified: only use 0 and 1 for busy beaver
    let directions = [Direction::L, Direction::R];

    // For small n_states, enumerate a subset of machines
    let mut best_machine = vec![];
    let mut best_score = 0usize;

    // Generate a simple class of machines: for each state, pick a deterministic transition for symbol 0
    // This is a simplified search, not exhaustive
    for config in 0..(n_states * 3).min(27) {
        let mut transitions = vec![];
        for s in 0..n_states {
            let write_idx = (config + s) % 2;
            let dir_idx = (config + s) % 2;
            let next = (s + config + 1) % (halt + 1);
            transitions.push(Transition {
                state: s,
                read: 0,
                write: symbols[write_idx],
                dir: directions[dir_idx],
                next,
            });
        }

        let tape = Tape::new(tape_size);
        let mut tm = TuringMachine::new(tape, transitions.clone(), halt);
        let steps = tm.run(1000);
        // A machine that never halts (ran the full budget) is not a valid
        // busy-beaver candidate — skip it.
        if steps >= 1000 || !tm.is_halted() {
            continue;
        }
        let score = tm.tape.non_zero_count();
        if score > best_score {
            best_score = score;
            best_machine = transitions;
        }
    }

    (best_machine, best_score)
}

/// A simple machine that cycles the starting cell through the full ternary
/// alphabet `0 → +1 → -1 → 0` and then halts. All transitions use `Direction::S`
/// so the head re-reads the cell it just wrote; moving rightward instead would
/// leave the written symbol behind on a fresh `0` cell and the cycle would
/// never progress past the first transition.
///
/// Hand trace on an all-zero tape (head at the centre cell):
///   step 1: read 0  → write +1, state 0
///   step 2: read +1 → write -1, state 0
///   step 3: read -1 → write  0, state 1 (halt)
/// After 3 steps the machine is halted and the cell is back to 0.
pub fn counter_machine(size: usize) -> TuringMachine {
    let transitions = vec![
        Transition {
            state: 0,
            read: 0,
            write: 1,
            dir: Direction::S,
            next: 0,
        },
        Transition {
            state: 0,
            read: 1,
            write: -1,
            dir: Direction::S,
            next: 0,
        },
        Transition {
            state: 0,
            read: -1,
            write: 0,
            dir: Direction::S,
            next: 1,
        },
    ];
    TuringMachine::new(Tape::new(size), transitions, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tape_new() {
        let t = Tape::new(10);
        assert_eq!(t.read(), 0);
        assert_eq!(t.non_zero_count(), 0);
    }

    #[test]
    fn test_tape_write_read() {
        let mut t = Tape::new(10);
        t.write(1);
        assert_eq!(t.read(), 1);
    }

    #[test]
    fn test_tape_move() {
        let mut t = Tape::new(10);
        t.move_head(Direction::R);
        assert_eq!(t.head, 6); // started at 5
        t.move_head(Direction::L);
        assert_eq!(t.head, 5);
    }

    #[test]
    fn test_tape_extend() {
        let mut t = Tape::new(3);
        t.head = 2;
        t.move_head(Direction::R);
        assert_eq!(t.cells.len(), 4);
    }

    #[test]
    fn test_tape_extend_left_edge() {
        // Moving left from cell 0 must extend the tape leftward (not silently
        // no-op), keeping head at 0 over the freshly prepended 0.
        let mut t = Tape::from_vec(vec![1]);
        assert_eq!(t.head, 0);
        assert_eq!(t.cells.len(), 1);
        t.move_head(Direction::L);
        assert_eq!(t.cells.len(), 2);
        assert_eq!(t.head, 0);
        assert_eq!(t.read(), 0); // new cell is blank
                                 // The original cell shifted right by one.
        assert_eq!(t.cells[1], 1);
    }

    #[test]
    fn test_tape_encode() {
        // Documented digit map: -1 → 0, 0 → 1, +1 → 2;
        // encode(tape) = Σᵢ digit(tape[i]) · 3^i, range [0, 3^n − 1].
        // [0, 1, -1] → digits [1, 2, 0] → 1·1 + 2·3 + 0·9 = 7
        assert_eq!(Tape::from_vec(vec![0, 1, -1]).encode(), 7);
        // [1, 0, -1] → digits [2, 1, 0] → 2·1 + 1·3 + 0·9 = 5
        assert_eq!(Tape::from_vec(vec![1, 0, -1]).encode(), 5);
        // all -1 → all digits 0 → 0 (minimum of the documented range)
        assert_eq!(Tape::from_vec(vec![-1, -1, -1]).encode(), 0);
        // all +1 → all digits 2 → 2·(1 + 3 + 9) = 26 = 3^3 − 1 (maximum)
        assert_eq!(Tape::from_vec(vec![1, 1, 1]).encode(), 26);
    }

    #[test]
    fn test_tape_from_vec() {
        let t = Tape::from_vec(vec![1, 0, -1]);
        assert_eq!(t.read(), 1);
    }

    #[test]
    fn test_tm_halted() {
        let tm = TuringMachine::new(Tape::new(5), vec![], 1);
        assert!(!tm.is_halted()); // state 0, halt_state 1, not halted yet
    }

    #[test]
    fn test_tm_step_halts() {
        let transitions = vec![Transition {
            state: 0,
            read: 0,
            write: 1,
            dir: Direction::S,
            next: 1,
        }];
        let mut tm = TuringMachine::new(Tape::new(5), transitions, 1);
        assert!(tm.step()); // executes transition
        assert!(tm.is_halted()); // now at state 1
        assert!(!tm.step()); // can't step further
    }

    #[test]
    fn test_tm_run() {
        // This machine writes 1 and moves R on reading 0, staying in state 0.
        // Because it always steps onto a fresh 0 cell, the (read 1) transition
        // never fires and the machine never halts — so run() must exhaust the
        // full step budget and return exactly that count, with the machine
        // still running.
        let transitions = vec![
            Transition {
                state: 0,
                read: 0,
                write: 1,
                dir: Direction::R,
                next: 0,
            },
            Transition {
                state: 0,
                read: 1,
                write: 0,
                dir: Direction::R,
                next: 1,
            },
        ];
        let mut tm = TuringMachine::new(Tape::new(10), transitions, 1);
        let steps = tm.run(100);
        assert_eq!(steps, 100);
        assert!(!tm.is_halted());
    }

    #[test]
    fn test_two_ones_then_halt_hand_traced() {
        // Independent hand trace (the core "execute transitions then halt"
        // path), used to sabotage-verify test_tm_step_halts and the simulator
        // generally:
        //   tape = 10 zeros, head = 5 (Tape::new centres the head)
        //   t0: (state 0, read 0) → write 1, R, next 1
        //   t1: (state 1, read 0) → write 1, R, next 2   (halt_state = 2)
        //   step 1: read 0 @5 → write 1 @5, head→6, state→1
        //   step 2: read 0 @6 → write 1 @6, head→7, state→2 (halt)
        //   step 3: is_halted → step() returns false, run() stops
        //   run() therefore returns 2.
        let transitions = vec![
            Transition {
                state: 0,
                read: 0,
                write: 1,
                dir: Direction::R,
                next: 1,
            },
            Transition {
                state: 1,
                read: 0,
                write: 1,
                dir: Direction::R,
                next: 2,
            },
        ];
        let mut tm = TuringMachine::new(Tape::new(10), transitions, 2);
        let steps = tm.run(1000);
        assert_eq!(steps, 2);
        assert!(tm.is_halted());
        assert_eq!(tm.tape.head, 7);
        assert_eq!(tm.tape.cells[5], 1);
        assert_eq!(tm.tape.cells[6], 1);
        assert_eq!(tm.tape.non_zero_count(), 2);
    }

    #[test]
    fn test_tm_no_transition_halts() {
        let mut tm = TuringMachine::new(Tape::new(5), vec![], 99);
        tm.state = 0;
        assert!(!tm.step()); // no transitions → halt
    }

    #[test]
    fn test_counter_machine() {
        // Hand trace (head starts at the centre of an all-zero tape):
        //   step 1: read 0  → write +1, stay, state 0
        //   step 2: read +1 → write -1, stay, state 0
        //   step 3: read -1 → write  0, stay, state 1 (halt)
        // Net: 3 steps, machine halted, head unmoved, starting cell back to 0.
        let mut cm = counter_machine(10);
        let centre = cm.tape.head;
        let steps = cm.run(50);
        assert_eq!(steps, 3);
        assert!(cm.is_halted());
        assert_eq!(cm.tape.head, centre);
        assert_eq!(cm.tape.non_zero_count(), 0);
        assert_eq!(cm.tape.read(), 0);
    }

    #[test]
    fn test_busy_beaver_finds_halting_machine() {
        // For n_states = 2 the (small, simplified) search space includes
        // machines that write at least one non-zero cell and halt — verified
        // by hand-enumerating the configs the search generates.
        let (machine, score) = busy_beaver(2, 10);
        assert!(
            score >= 1,
            "expected non-zero busy-beaver score, got {score}"
        );
        assert!(
            !machine.is_empty(),
            "busy_beaver returned an empty machine for a non-trivial search"
        );
        // The returned machine must actually reproduce the claimed score when
        // re-executed from a fresh tape (halt_state for n=2 is 2).
        let mut tm = TuringMachine::new(Tape::new(10), machine.clone(), 2);
        let steps = tm.run(1000);
        assert!(tm.is_halted(), "returned machine did not halt");
        assert!(
            steps < 1000,
            "returned machine ran the full step budget without halting"
        );
        assert_eq!(
            tm.tape.non_zero_count(),
            score,
            "re-executed score did not match reported score"
        );
    }

    #[test]
    fn test_direction_values() {
        assert_eq!(Direction::L, Direction::L);
        assert_ne!(Direction::L, Direction::R);
    }
}
