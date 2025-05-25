#![feature(associated_type_defaults, negative_impls, auto_traits)]

pub(crate) mod handler;
pub(crate) mod render_context;

pub(crate) mod bind;
pub(crate) mod layout;
pub(crate) mod surface;
pub(crate) mod texture;

#[cfg(feature = "ecs")]
pub(crate) mod ecs;

pub mod prelude;

pub(crate) mod core;

pub use bytemuck;
pub use tokio;
pub use wgpu;
pub use winit;
