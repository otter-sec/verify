/// Programs indicate success with a return value of 0
pub const SUCCESS: u64 = 0;

pub use crate::program::ProgramResult;

/// Start address of the memory region used for program heap.
pub const HEAP_START_ADDRESS: u64 = 0x300000000;
/// Length of the heap memory region used for program heap.
pub const HEAP_LENGTH: usize = 32 * 1024;