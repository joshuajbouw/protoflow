// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

use std::sync::Mutex;
use std::time::Instant;

use crate::{Port, prelude::Duration, Runtime, RuntimeStatus, Scheduler, System};

pub struct StdThread {
    system: System,
    status: Mutex<RuntimeStatus>,
}

impl Runtime for StdThread {
    fn new(system: System) -> Self {
        Self {
            system,
            status: Mutex::new(RuntimeStatus::NotStarted),
        }
    }

    fn is_stopped() -> bool {
        todo!()
    }

    fn has_error() -> bool {
        todo!()
    }

    fn status() -> RuntimeStatus {
        todo!()
    }

    fn run(&mut self) {
        todo!()
    }
}

impl Scheduler for StdThread {
    fn is_alive(&self) -> bool {
        match *self.status.lock().expect("Status unexpectedly poisoned") {
            RuntimeStatus::Running | RuntimeStatus::Dormant => true,
            _ => false,
        }
    }

    fn sleep_for(&self, duration: Duration) {
        std::thread::sleep(duration);
    }

    fn sleep_until(&self, instant: Instant) {
        let now = Instant::now();
        if now < instant {
            std::thread::sleep(instant - now);
        }
    }

    fn wait_for(&self, port: &dyn Port) -> Result<(), BlockError> {
        while self.is_alive() && !port.is_connected() {
            self.yield_now()?;
        }
        if self.is_alive() {
            Ok(())
        } else {
            Err(BlockError::Terminated)
        }
    }

    fn yield_now(&self) -> Result<(), ()> {
        std::thread::yield_now();
        Ok(())
    }
}
