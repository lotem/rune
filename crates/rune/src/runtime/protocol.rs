use crate::alloc;
#[cfg(feature = "doc")]
use crate::alloc::Vec;
use crate::compile::meta;
use crate::module::{AssociatedFunctionName, ToInstance};
use crate::Hash;

#[doc(inline)]
pub use rune_core::Protocol;

impl ToInstance for Protocol {
    #[inline]
    fn to_instance(self) -> alloc::Result<AssociatedFunctionName> {
        Ok(AssociatedFunctionName {
            associated: meta::AssociatedKind::Protocol(self),
            function_parameters: Hash::EMPTY,
            #[cfg(feature = "doc")]
            parameter_types: Vec::new(),
        })
    }
}
