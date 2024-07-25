// This is free and unencumbered software released into the public domain.

use core::marker::PhantomData;

#[cfg(all(not(feature = "flume"), feature = "std"))]
use std::sync::mpsc::Sender;

use crate::{
    BlockError,
    Message, Port, PortState, prelude::{fmt, String},
};
use crate::transport::Sender;
use crate::transports::FlumeSender;

pub struct OutputPort<M: Message, S: Sender<M>> {
    state: PortState,
    name: String,
    label: Option<String>,
    sender: Option<S>,
    _phantom: PhantomData<M>,
}

impl<M: Message> OutputPort<M, FlumeSender<M>> {
    pub fn new_with_flume(name: impl Into<String>) -> Self {
        Self {
            state: PortState::default(),
            name: name.into(),
            label: None,
            sender: None,
            _phantom: PhantomData,
        }
    }

    pub fn new_flume_with_label(name: impl Into<String>, label: Option<impl Into<String>>) -> Self {
        Self {
            state: PortState::default(),
            name: name.into(),
            label: label.map(|s| s.into()),
            sender: None,
            _phantom: PhantomData,
        }
    }

    pub fn open(&mut self, sender: FlumeSender<M>) -> Result<(), BlockError> {
        self.state = PortState::Open;
        self.sender = Some(sender);
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), BlockError> {
        self.state = PortState::Closed;
        Ok(())
    }

    pub fn send(&self, _message: &T) -> Result<(), BlockError> {
        Ok(()) // TODO
    }
}

impl<T: Message> Port for OutputPort<T> {
    fn state(&self) -> PortState {
        self.state
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl<T: Message> fmt::Display for OutputPort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}→", self.name)
    }
}

impl<T: Message> From<&str> for OutputPort<T> {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl<T: Message> From<String> for OutputPort<T> {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl<T: Message> AsRef<str> for OutputPort<T> {
    fn as_ref(&self) -> &str {
        self.name()
    }
}
