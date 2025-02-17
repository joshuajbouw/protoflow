// This is free and unencumbered software released into the public domain.

use crate::PortState;

/// The common interface for ports, whether for input or output.
pub trait Port {
    /// The current state of this port.
    fn state(&self) -> PortState;

    /// The machine-readable name of this port.
    fn name(&self) -> &str;

    /// A human-readable label for this port.
    fn label(&self) -> Option<&str>;

    /// Checks whether this port is currently closed.
    fn is_closed(&self) -> bool {
        self.state() == PortState::Closed
    }

    /// Checks whether this port is currently open.
    fn is_open(&self) -> bool {
        self.state() == PortState::Open
    }

    /// Checks whether this port is currently connected.
    fn is_connected(&self) -> bool {
        self.state() == PortState::Connected
    }
}
