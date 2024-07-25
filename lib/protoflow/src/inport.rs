use prost::Message;

use crate::{Port, PortError};

pub trait InPort<M: Message>: Port {
    fn recv(&mut self) -> Result<Option<M>, PortError>;
}
