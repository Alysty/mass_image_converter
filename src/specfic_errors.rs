use std::{error::Error, fmt};


#[derive(thiserror::Error)]
pub enum SpecificErrors{
    #[error("failed to read the png image file\n")]
    FileRead(#[source] std::io::Error),
    
    #[error("failed to decode the png\n")]
    Image(#[source]image::error::ImageError),

    #[error("failed to encode the image to webp\n")]
    Webp(String)
}

impl fmt::Debug for SpecificErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}
