// This is free and unencumbered software released into the public domain.

use prost::Message;

use crate::{
    Block,
    InPort, InputPort, Port, prelude::{Box, vec, Vec},
};

/// A machine-readable identifier for a block in a system.
///
/// Only valid within the scope of that system.
pub type BlockID = usize;

struct SystemBlock {
    block: Box<dyn Block>,
    inputs: Vec<Box<dyn InPort<Box<dyn Message>>>>,
}

/// A system is a collection of blocks that are connected together.
pub struct System {
    /// The registered blocks in the system.
    blocks: Vec<SystemBlock>,
}

// pub type Subsystem = System;

impl System {
    /// Instantiates a new system.
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    /// Instantiates a block in the system.
    pub fn block(&mut self, block: Box<dyn Block>) -> BlockID {
        let system_block = SystemBlock {
            block,
            inputs: vec![],
        };
        self.blocks.push(system_block);
        self.blocks.len() - 1
    }

    /// Connects two ports of two blocks in the system.
    pub fn connect_flume(
        &mut self,
        source_block: BlockID,
        source_port: &str,
        target_block: BlockID,
        target_port: &str,
    ) -> Result<(), ()> {
        // Check if the source block port name exists.
        let source_block = self.blocks.get_mut(source_block).ok_or(())?;
        source_block
            .block
            .inputs()
            .iter_mut()
            .find(|input| input.name() == source_port)
            .ok_or(())?;
        let thing = InputPort::new();

        // Check if the target block port name exists.
        let target_block = self.blocks.get_mut(target_block).ok_or(())?;
        target_block
            .block
            .inputs()
            .iter_mut()
            .find(|input| input.name() == target_port)
            .ok_or(())?;

        Ok(())
    }
}
