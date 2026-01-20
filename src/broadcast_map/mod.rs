pub(crate) mod r#impl;
pub(crate) mod r#struct;
pub(crate) mod r#trait;
pub(crate) mod r#type;

#[cfg(test)]
mod test;

pub use {r#struct::*, r#trait::*, r#type::*};
