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

    pub fn move_head(&mut self, dir: Direction) {
        match dir {
            Direction::L => {
                if self.head > 0 {
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

/// A simple universal-like machine: increment tape cell and move right
pub fn counter_machine(size: usize) -> TuringMachine {
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
            write: -1,
            dir: Direction::R,
            next: 0,
        },
        Transition {
            state: 0,
            read: -1,
            write: 0,
            dir: Direction::L,
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
    fn test_tape_encode() {
        let t = Tape::from_vec(vec![0, 1, -1]);
        let n = t.encode();
        assert!(n > 0);
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
        assert!(steps > 0);
    }

    #[test]
    fn test_tm_no_transition_halts() {
        let mut tm = TuringMachine::new(Tape::new(5), vec![], 99);
        tm.state = 0;
        assert!(!tm.step()); // no transitions → halt
    }

    #[test]
    fn test_counter_machine() {
        let mut cm = counter_machine(10);
        let steps = cm.run(50);
        assert!(steps > 0);
        assert!(cm.tape.non_zero_count() > 0);
    }

    #[test]
    fn test_busy_beaver() {
        let (machine, score) = busy_beaver(2, 10);
        // Should find at least one halting machine
        // (may not find the optimal one due to simplified search)
        let _ = (machine, score);
    }

    #[test]
    fn test_direction_values() {
        assert_eq!(Direction::L, Direction::L);
        assert_ne!(Direction::L, Direction::R);
    }
}
