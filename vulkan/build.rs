use spirv_builder::{MemoryModel, SpirvBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    SpirvBuilder::new("./kernel")
        .print_metadata(true)
        .memory_model(MemoryModel::Vulkan)
        .build()?;
    Ok(())
}
