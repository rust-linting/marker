use std::marker::PhantomData;

use crate::{
    common::{GenericId, ItemId, TyDefId},
    sem::generic::SemGenericArgs,
};

/// The semantic representation of an abstract data type. This can be an
/// [`Enum`], [`Struct`], or [`Union`].
///
/// [`Struct`]: https://doc.rust-lang.org/reference/types/struct.html
/// [`Enum`]: https://doc.rust-lang.org/reference/types/enum.html
/// [`Union`]: https://doc.rust-lang.org/reference/types/union.html
#[repr(C)]
#[derive(Debug)]
pub struct SemAdtTy<'ast> {
    def_id: TyDefId,
    generics: SemGenericArgs<'ast>,
}

impl<'ast> SemAdtTy<'ast> {
    /// This returns the [`TyDefId`] of the abstract data type.
    pub fn def_id(&self) -> TyDefId {
        self.def_id
    }

    /// This returns the [`SemGenericArgs`] used by the type
    pub fn generics(&self) -> &SemGenericArgs<'ast> {
        &self.generics
    }
}

#[cfg(feature = "driver-api")]
impl<'ast> SemAdtTy<'ast> {
    pub fn new(def_id: TyDefId, generics: SemGenericArgs<'ast>) -> Self {
        Self { def_id, generics }
    }
}

/// The semantic representation of a generic type. For example
///
/// ```
/// fn function<T: Default>() {
///     let _ = T::default();
///     //      ^^^^^^^^^^^^ This will have the generic type `T`
/// }
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct SemGenericTy<'ast> {
    _lifetime: PhantomData<&'ast ()>,
    generic_id: GenericId,
}

impl<'ast> SemGenericTy<'ast> {
    /// This returns the [`GenericId`] assigned to the generic parameter.
    /// This id can be used to retrieve more information from the item that
    /// defines the generic.
    pub fn generic_id(&self) -> GenericId {
        self.generic_id
    }
}

#[cfg(feature = "driver-api")]
impl<'ast> SemGenericTy<'ast> {
    pub fn new(generic_id: GenericId) -> Self {
        Self {
            _lifetime: PhantomData,
            generic_id,
        }
    }
}

/// The semantic representation of a type alias.
///
/// Aliases in semantic type representations are usually resolved directly. This
/// kind, is primarily used for instances, where the concrete aliased type is not yet
/// known.
#[repr(C)]
#[derive(Debug)]
pub struct SemAliasTy<'ast> {
    _lifetime: PhantomData<&'ast ()>,
    alias_item: ItemId,
}

impl<'ast> SemAliasTy<'ast> {
    /// This [`ItemId`] identifies the item that defined the alias
    pub fn alias_item(&self) -> ItemId {
        self.alias_item
    }
}

#[cfg(feature = "driver-api")]
impl<'ast> SemAliasTy<'ast> {
    pub fn new(alias_item: ItemId) -> Self {
        Self {
            _lifetime: PhantomData,
            alias_item,
        }
    }
}
