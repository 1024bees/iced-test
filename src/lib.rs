//! Additional widgets for the Iced GUI library.
#![deny(missing_docs)]
#![deny(unused_results)]
//#![forbid(unsafe_code)]
#![warn(
    clippy::nursery,

    // Restriction lints
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::let_underscore_must_use,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
)]
#![allow(
    clippy::suboptimal_flops,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions
)]

pub mod rendering;
pub mod runners;
pub mod trace_events;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "glow")))]
use iced_wgpu as renderer;

#[cfg(all(not(target_arch = "wasm32"), feature = "glow"))]
use iced_glow as renderer;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "glow"),))]
use iced_winit as runtime;

#[cfg(all(not(target_arch = "wasm32"), feature = "glow"))]
use iced_glutin as runtime;

pub use rendering::screenshot::Screenshot;
pub use trace_events::TraceEvent;
