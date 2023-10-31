#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::index_refutable_slice)]
#![allow(clippy::module_name_repetitions)]

mod error;
mod loader;

pub mod context;

pub use error::{Error, Result};
pub use loader::LintCrateInfo;

use loader::LintCrateRegistry;
use marker_api::Lint;
use marker_api::{
    ast::{Body, Crate, EnumVariant, ExprKind, ItemField, ItemKind, StmtKind},
    context::MarkerContext,
    LintPass, LintPassInfo,
};
use marker_utils::visitor::{self, Visitor};
use std::{cell::RefCell, ops::ControlFlow};

pub const LINT_CRATES_ENV: &str = "MARKER_LINT_CRATES";

/// This struct is the interface used by lint drivers to load lint crates, pass
/// `marker_api` objects to external lint passes and all other magic you can think of.
#[derive(Debug)]
pub struct Adapter {
    /// [`LintPass`] functions are called with a mutable `self` parameter as the
    /// first argument. This `RefCell` acts as a wrapper to hide the internal
    /// mutability from drivers.
    ///
    /// The effects of the mutability should never reach the driver anyways and
    /// this just makes it way easier to handle the adapter in drivers.
    inner: RefCell<AdapterInner>,
}

#[derive(Debug)]
struct AdapterInner {
    external_lint_crates: LintCrateRegistry,
}

impl Adapter {
    /// This creates a new [`Adapter`] instance
    ///
    /// # Errors
    ///
    /// This function will return an error if an error occurs during the lint
    /// loading process.
    pub fn new(lint_crates: &[LintCrateInfo]) -> Result<Self> {
        let external_lint_crates = LintCrateRegistry::new(lint_crates)?;
        Ok(Self {
            inner: RefCell::new(AdapterInner { external_lint_crates }),
        })
    }

    pub fn marker_lints(&self) -> Vec<&'static Lint> {
        self.lint_pass_infos()
            .iter()
            .flat_map(marker_api::LintPassInfo::lints)
            .copied()
            .collect()
    }

    #[must_use]
    fn lint_pass_infos(&self) -> Vec<LintPassInfo> {
        self.inner.borrow().external_lint_crates.collect_lint_pass_info()
    }

    pub fn process_krate<'ast>(&self, cx: &'ast MarkerContext<'ast>, krate: &'ast Crate<'ast>) {
        let inner = &mut *self.inner.borrow_mut();

        inner.external_lint_crates.set_ast_context(cx);

        inner.external_lint_crates.check_crate(cx, krate);
        visitor::traverse_item::<()>(cx, inner, ItemKind::Mod(krate.root_mod()));
    }
}

impl Visitor<()> for AdapterInner {
    fn scope(&self) -> visitor::VisitorScope {
        visitor::VisitorScope::AllBodies
    }

    fn visit_item<'ast>(&mut self, cx: &'ast MarkerContext<'ast>, item: ItemKind<'ast>) -> ControlFlow<()> {
        self.external_lint_crates.check_item(cx, item);
        ControlFlow::Continue(())
    }

    fn visit_field<'ast>(&mut self, cx: &'ast MarkerContext<'ast>, field: &'ast ItemField<'ast>) -> ControlFlow<()> {
        self.external_lint_crates.check_field(cx, field);
        ControlFlow::Continue(())
    }

    fn visit_variant<'ast>(
        &mut self,
        cx: &'ast MarkerContext<'ast>,
        variant: &'ast EnumVariant<'ast>,
    ) -> ControlFlow<()> {
        self.external_lint_crates.check_variant(cx, variant);
        ControlFlow::Continue(())
    }

    fn visit_body<'ast>(&mut self, cx: &'ast MarkerContext<'ast>, body: &'ast Body<'ast>) -> ControlFlow<()> {
        self.external_lint_crates.check_body(cx, body);
        ControlFlow::Continue(())
    }

    fn visit_stmt<'ast>(&mut self, cx: &'ast MarkerContext<'ast>, stmt: StmtKind<'ast>) -> ControlFlow<()> {
        self.external_lint_crates.check_stmt(cx, stmt);
        ControlFlow::Continue(())
    }

    fn visit_expr<'ast>(&mut self, cx: &'ast MarkerContext<'ast>, expr: ExprKind<'ast>) -> ControlFlow<()> {
        self.external_lint_crates.check_expr(cx, expr);
        ControlFlow::Continue(())
    }
}
