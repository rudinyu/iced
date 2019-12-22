/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! iced-x86 is an x86/x64 disassembler, decoder, encoder written in Rust

#![doc(html_logo_url = "https://raw.githubusercontent.com/0xd4d/iced/master/logo.png")]
#![doc(html_root_url = "https://docs.rs/iced-x86/0.0.0")]
#![allow(unknown_lints)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_lossless))]
// Requires rustc >= 1.26.0
#![cfg_attr(feature = "cargo-clippy", allow(clippy::range_plus_one))]
// '_ requires rustc >= 1.31.0
#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_lifetimes))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::useless_let_if_seq))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::collapsible_if))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::verbose_bit_mask))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::cognitive_complexity))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::dbg_macro))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::default_trait_access))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::doc_markdown))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::fallible_impl_from))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::large_digit_groups))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::unimplemented))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::used_underscore_binding))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::must_use_candidate))]
//TODO: enable this when rustc 1.41.0 has been released
//#![cfg_attr(feature = "cargo-clippy", warn(clippy::missing_inline_in_public_items))]
#![deny(absolute_paths_not_starting_with_crate)]
#![deny(deprecated_in_future)]
#![deny(keyword_idents)]
#![deny(missing_copy_implementations)]
#![deny(missing_docs)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_must_use)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![allow(bare_trait_objects)] // dyn syntax supported by rustc >= 1.27.0
#![allow(dead_code)] //TODO: REMOVE

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate static_assertions;

mod code;
#[cfg(any(feature = "decoder", feature = "encoder"))]
mod constant_offsets;
#[cfg(feature = "decoder")]
mod decoder;
#[cfg(feature = "encoder")]
mod encoder;
mod enums;
#[cfg(any(
	feature = "gas_formatter",
	feature = "intel_formatter",
	feature = "masm_formatter",
	feature = "nasm_formatter",
	feature = "all_formatters",
))]
mod formatter;
pub(crate) mod iced_constants;
mod iced_features;
#[cfg(feature = "instr_info")]
mod info;
mod instruction;
mod instruction_internal;
mod instruction_memory_sizes;
mod instruction_op_counts;
mod memory_size;
mod mnemonic;
mod mnemonics;
mod register;
#[cfg(test)]
pub(crate) mod test_utils;

pub use self::code::*;
#[cfg(any(feature = "decoder", feature = "encoder"))]
pub use self::constant_offsets::*;
#[cfg(feature = "decoder")]
pub use self::decoder::*;
#[cfg(feature = "encoder")]
pub use self::encoder::*;
pub use self::enums::*;
#[cfg(any(
	feature = "gas_formatter",
	feature = "intel_formatter",
	feature = "masm_formatter",
	feature = "nasm_formatter",
	feature = "all_formatters",
))]
pub use self::formatter::*;
pub use self::iced_features::*;
#[cfg(feature = "instr_info")]
pub use self::info::*;
pub use self::instruction::*;
pub use self::memory_size::*;
pub use self::mnemonic::*;
pub use self::register::*;
