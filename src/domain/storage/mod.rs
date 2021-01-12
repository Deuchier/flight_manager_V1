//! Storage.
//!
//! Provides interface for persistent storage objects.
//!
//! # Multiprocessor Parallelism
//! All storage objects should support synchronization mechanisms. For this reason, even the `self`
//! references in the modifying method calls are declared as immutable. The implementors should
//! use synchronization facilities carefully.

pub mod data;
pub mod items;
pub mod users;
