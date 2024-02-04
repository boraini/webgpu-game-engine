use std::num::NonZeroU64;

use wgpu::{util::StagingBelt, Buffer, CommandEncoder, Device};

pub struct StagingBeltAndCommandEncoder {
    pub staging_belt: StagingBelt,
    pub command_encoder: CommandEncoder,
}

impl StagingBeltAndCommandEncoder {
    pub fn new(staging_belt: StagingBelt, command_encoder: CommandEncoder) -> Self {
        Self {
            staging_belt,
            command_encoder,
        }
    }
    pub fn write_buffer(&mut self, target: &Buffer, offset: u64, slice: &[u8], device: &Device) {
        self.staging_belt
            .write_buffer(
                &mut self.command_encoder,
                target,
                offset,
                NonZeroU64::new(std::mem::size_of_val(&*(slice)) as u64).unwrap(),
                device,
            )
            .copy_from_slice(slice);
    }
}
