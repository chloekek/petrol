#![
    doc(
        html_logo_url = "../../../marketing/logo.svg",
        html_favicon_url = "../../../marketing/logo.svg",
    )
]

//! The compiler takes a series of values as input, interprets them as a
//! computer program, translates that computer program to machine code, and
//! emits an object file.
//!
//! Because there is no “eval” functionality. the compiler is a standalone
//! program and it is not available at runtime.
//!
//! # Memory management
//!
//! The compiler uses the typed-arena crate for most of its memory management.
//! This allows for more efficient memory allocation and easier management of
//! life times, at the cost of higher memory usage. See the [Arenas] struct for
//! an overview of the various arenas and what they are used for.
//!
//! # Internal representation
//!
//! Because there are no additional steps between reading and generating ANF,
//! a separate representation for the AST is not too useful to have, so values
//! are used for the AST precisely as they came in.
//!
//! ANF is an internal representation that lends itself to ease optimizing and
//! generating code. For a thorough overview of ANF, see the [anf] module.
//!
//! See the [Value] struct for the in-compiler representation of values.
//!
//! # Lowering
//!
//! Lowering is the process whereby an AST is converted into ANF. Lowering is
//! implemented in the [lower] module.
//!
//! [anf]: anf/index.html
//! [lower]: lower/index.html
//! [Arenas]: struct.Arenas.html
//! [Value]: struct.Value.html

extern crate typed_arena;

pub mod anf;
pub mod lower;
pub mod parse;

mod arenas;
mod hash;
mod position;
mod value;

pub use arenas::*;
pub use hash::*;
pub use position::*;
pub use value::*;
