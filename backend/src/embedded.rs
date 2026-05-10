#[cfg(feature = "bundle-frontend")]
use rust_embed::RustEmbed;

#[cfg(feature = "bundle-frontend")]
#[derive(RustEmbed)]
#[folder = "../frontend/out"]
pub struct Frontend;
