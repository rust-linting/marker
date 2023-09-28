use marker_api::ast::stmt::{CommonStmtData, ExprStmt, ItemStmt, LetStmt, StmtKind};
use rustc_hir as hir;

use super::MarkerConverterInner;

impl<'ast, 'tcx> MarkerConverterInner<'ast, 'tcx> {
    #[must_use]
    pub fn to_stmt(&self, stmt: &hir::Stmt<'tcx>) -> Option<StmtKind<'ast>> {
        let id = self.to_stmt_id(stmt.hir_id);
        if let Some(stmt) = self.stmts.borrow().get(&id) {
            return Some(*stmt);
        }

        let data = CommonStmtData::builder()
            .id(self.to_stmt_id(stmt.hir_id))
            .span(self.to_span_id(stmt.span))
            .build();
        let stmt = match &stmt.kind {
            hir::StmtKind::Local(local) => match local.source {
                hir::LocalSource::Normal => StmtKind::Let(
                    self.alloc(
                        LetStmt::builder()
                            .data(data)
                            .pat(self.to_pat(local.pat))
                            .ty(local.ty.map(|ty| self.to_syn_ty(ty)))
                            .init(local.init.map(|init| self.to_expr(init)))
                            .els(local.els.map(|els| self.to_expr_from_block(els)))
                            .build(),
                    ),
                ),
                hir::LocalSource::AssignDesugar(_) => {
                    unreachable!("this will be handled by the block expr wrapping the desugar")
                },
                hir::LocalSource::AsyncFn | hir::LocalSource::AwaitDesugar => {
                    eprintln!("skipping not implemented statement at: {:?}", stmt.span);
                    return None;
                },
            },
            hir::StmtKind::Item(item) => {
                let item = self.to_item_from_id(*item)?;
                StmtKind::Item(self.alloc(ItemStmt::builder().data(data).item(item).build()))
            },
            hir::StmtKind::Expr(expr) | hir::StmtKind::Semi(expr) => {
                StmtKind::Expr(self.alloc(ExprStmt::builder().data(data).expr(self.to_expr(expr)).build()))
            },
        };

        self.stmts.borrow_mut().insert(id, stmt);
        Some(stmt)
    }
}
