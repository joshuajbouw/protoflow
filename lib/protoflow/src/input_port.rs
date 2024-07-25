// This is free and unencumbered software released into the public domain.

#[cfg(all(not(feature = "flume"), feature = "std"))]
use std::sync::mpsc::Receiver;

use crate::{
    BlockError,
    InPort, Message, Port, PortError, PortState, prelude::{fmt, String},
};
use crate::{vec, Vec};
use crate::transport::Receiver;
use crate::transports::{FlumeReceiver, ZmqReceiver};

#[derive(Debug)]
pub struct InputPort<M: Message, R: Receiver<M>> {
    state: PortState,
    name: String,
    label: Option<String>,
    stack: Vec<M>,
    receiver: Option<R>,
}

#[cfg(feature = "flume")]
impl<M: Message> InputPort<M, FlumeReceiver<M>> {
    pub fn new_flume(name: impl Into<String>) -> Self {
        Self {
            state: PortState::default(),
            name: name.into(),
            label: None,
            stack: vec![],
            receiver: None,
        }
    }

    pub fn new_flume_with_label(name: impl Into<String>, label: Option<impl Into<String>>) -> Self {
        Self {
            state: PortState::default(),
            name: name.into(),
            label: label.map(|s| s.into()),
            stack: vec![],
            receiver: None,
        }
    }

    pub fn open(&mut self, receiver: FlumeReceiver<M>) -> Result<(), BlockError> {
        self.state = PortState::Open;
        self.receiver = Some(receiver);
        Ok(())
    }
}

#[cfg(feature = "zeromq")]
impl<M: Message + Default> InputPort<M, ZmqReceiver<M>> {
    pub fn new_zmq(name: impl Into<String>) -> Self {
        Self {
            state: PortState::default(),
            name: name.into(),
            label: None,
            stack: vec![],
            receiver: None,
        }
    }

    pub fn new_zmq_with_label(name: impl Into<String>, label: Option<impl Into<String>>) -> Self {
        Self {
            state: PortState::default(),
            name: name.into(),
            label: label.map(|s| s.into()),
            stack: vec![],
            receiver: None,
        }
    }

    pub fn open(&mut self, endpoint: &str) -> Result<(), BlockError> {
        let mut receiver = ZmqReceiver::new();
        receiver
            .open(endpoint)
            .map_err(|_| BlockError::PortError(PortError::NotEstablished))?;
        self.state = PortState::Open;
        self.receiver = Some(receiver);
        Ok(())
    }
}

impl<M: Message, R: Receiver<M>> InputPort<M, R> {
    pub fn close(&mut self) -> Result<(), BlockError> {
        self.state = PortState::Closed;
        if let Some(receiver) = self.receiver.take() {
            drop(receiver);
        }
        Ok(())
    }
}

impl<M: Message, R: Receiver<M>> Port for InputPort<M, R> {
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

impl<M: Message, R: Receiver<M>> InPort<M> for InputPort<M, R> {
    fn recv(&mut self) -> Result<Option<M>, PortError> {
        if let Some(receiver) = &mut self.receiver {
            receiver
                .recv()
                .map(|m| Some(m))
                .map_err(|_| PortError::NotEstablished)
        } else {
            Err(PortError::NotEstablished)
        }
    }
}

impl<M: Message, R: Receiver<M>> fmt::Display for InputPort<M, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "→{}", self.name)
    }
}

#[cfg(feature = "flume")]
impl<M: Message> From<&str> for InputPort<M, FlumeReceiver<M>> {
    fn from(name: &str) -> Self {
        Self::new_flume(name)
    }
}

#[cfg(feature = "zeromq")]
impl<M: Message + Default> From<&str> for InputPort<M, ZmqReceiver<M>> {
    fn from(name: &str) -> Self {
        Self::new_zmq(name)
    }
}

#[cfg(feature = "flume")]
impl<M: Message> From<String> for InputPort<M, FlumeReceiver<M>> {
    fn from(name: String) -> Self {
        Self::new_flume(name)
    }
}

#[cfg(feature = "zeromq")]
impl<M: Message + Default> From<String> for InputPort<M, ZmqReceiver<M>> {
    fn from(name: String) -> Self {
        Self::new_zmq(name)
    }
}

impl<M: Message, R: Receiver<M>> AsRef<str> for InputPort<M, R> {
    fn as_ref(&self) -> &str {
        self.name()
    }
}
