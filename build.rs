use spirv_builder::{MetadataPrintout, SpirvBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    SpirvBuilder::new("./kernel", "spirv-unknown-spv1.5")
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    Ok(())
}