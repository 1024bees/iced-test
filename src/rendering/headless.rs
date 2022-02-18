//! Interfaces for headless rendering
use super::screenshot::Screenshot;

pub trait VirtualCompositor {
    /// Reads the framebuffer pixels on the provided region into the provided buffer.
    fn read(&self) -> Option<Screenshot>;
}
