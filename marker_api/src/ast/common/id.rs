macro_rules! new_id {
    ($(#[$attr:meta])* $vis:vis $name:ident: $data_ty:ty) => {
        $(#[$attr])*
        ///
        /// **Stability notice**:
        /// * The ID is not stable between different sessions.
        /// * IDs should never be stored by lint crates, as drivers might change
        ///   IDs between different `check_*` function calls.
        /// * The layout and size of this type might change. The ID will continue
        ///   to provide the current trait implementations.
        #[repr(C)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name {
            /// The layout of the data is up to the driver implementation. The API will never
            /// create custom IDs and pass them to the driver. The size of this type might
            /// change. Drivers should validate the size with tests.
            data: $data_ty,
        }

        #[cfg(feature = "driver-api")]
        impl $name {
            #[must_use]
            pub fn new(data: $data_ty) -> Self {
                Self { data }
            }

            pub fn data(self) -> $data_ty {
                self.data
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(concat!(stringify!($name), "(..)")).finish()
            }
        }
    };
}

use new_id;

new_id!(
    /// This ID uniquely identifies a crate during linting.
    pub CrateId: u32
);

new_id! {
    ///  This ID uniquely identifies an item during linting.
    pub ItemId: u64
}

new_id! {
    ///  This ID uniquely identifies an enum variant during linting.
    pub VariantId: u64
}

new_id! {
    /// This ID uniquely identifies a field inside a struct during linting.
    pub FieldId: u64
}

new_id! {
    /// This ID uniquely identifies a user defined type during linting.
    pub TyDefId: u64
}

new_id! {
    /// This ID uniquely identifies a generic parameter during linting.
    pub GenericId: u64
}

new_id! {
    /// This ID uniquely identifies a macro during linting.
    pub MacroId: u64
}

new_id! {
    /// This ID uniquely identifies a body during linting.
    pub BodyId: u64
}

new_id! {
    /// This ID uniquely identifies a variable during linting.
    ///
    /// A variable can have several declaration spots. This can happen if they
    /// originate from an or binding. Like this:
    /// ```
    /// # #[allow(dead_code)]
    /// # enum Helper {
    /// #     One(&'static str),
    /// #     Two(&'static str),
    /// #     Three(&'static str),
    /// # }
    /// # let source = Helper::Three("duck");
    /// if let Helper::One(msg) | Helper::Two(msg) = source {
    ///     println!("{msg}");
    /// }
    /// ```
    pub VarId: u64
}

new_id! {
    /// This ID uniquely identifies an expression during linting.
    pub ExprId: u64
}

new_id! {
    /// **Unstable**
    ///
    /// This id is used to identify `Span`s. This type is only intended for internal
    /// use. Lint crates should always get a `Span` object.
    #[cfg_attr(feature = "driver-api", visibility::make(pub))]
    pub(crate) SpanId: u64
}

new_id! {
    /// **Unstable**
    ///
    /// This id is used to identify the source of a `Span`. This type is only intended for internal
    /// use. For now it's only intended for drivers to map spans back
    #[cfg_attr(feature = "driver-api", visibility::make(pub))]
    pub(crate) SpanSrcId: u32
}

new_id! {
    /// **Unstable**
    ///
    /// This id is used to identify a specific expansion. This type is only intended for internal
    /// use. For now it's only intended for drivers to map spans back
    #[cfg_attr(feature = "driver-api", visibility::make(pub))]
    pub(crate) ExpnId: u64
}

new_id! {
    /// **Unstable**
    ///
    /// This id is used to identify symbols. This type is only intended for internal
    /// use. Lint crates should always get [`String`] or `&str`.
    #[cfg_attr(feature = "driver-api", visibility::make(pub))]
    pub(crate) SymbolId: u32
}

new_id! {
    /// This ID uniquely identifies a statement during linting.
    pub StmtId: StmtIdInner
}

impl StmtId {
    /// This is an extra constructor for api internal use. The `new_id` macro
    /// only generates methods for drivers.
    pub(crate) fn ast_new(data: StmtIdInner) -> Self {
        Self { data }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "driver-api", visibility::make(pub))]
#[allow(clippy::exhaustive_enums)] // Only driver public
pub(crate) enum StmtIdInner {
    Expr(ExprId),
    Item(ItemId),
    LetStmt(LetStmtId),
}

new_id! {
    /// **Unstable**
    ///
    /// This id is used to identify a `let` statement. It's intended to be used
    /// inside [`StmtIdInner`]
    #[cfg_attr(feature = "driver-api", visibility::make(pub))]
    pub(crate) LetStmtId: u64
}
