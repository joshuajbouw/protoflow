// This is free and unencumbered software released into the public domain.

use crate::{Scheduler, System};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimeStatus {
    NotStarted,
    Running,
    Dormant,
    Stopping,
    Stopped,
    Error,
}

pub trait Runtime: Scheduler {
    fn new(system: System) -> Self;

    fn is_stopped() -> bool;

    fn has_error() -> bool;

    fn status() -> RuntimeStatus;

    fn run(&mut self);
}
