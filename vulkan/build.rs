use spirv_builder::{MemoryModel, SpirvBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    SpirvBuilder::new("./kernel")
        .spirv_version(1, 5)
        .print_metadata(true)
        .memory_model(MemoryModel::Vulkan)
        .build()?;
    Ok(())
}
