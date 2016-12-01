use rustc::lint::*;
use rustc::hir::*;
use utils::{is_automatically_derived, span_lint};

/// **What it does:** Checks for manual re-implementations of `PartialEq::ne`.
///
/// **Why is this bad?** `PartialEq::ne` is required to always return the
/// negated result of `PartialEq::eq`, which is exactly what the default
/// implementation does. Therefore, there should never be any need to
/// re-implement it.
///
/// **Known problems:** None.
///
/// **Example:**
/// ```rust
/// struct Foo;
///
/// impl PartialEq for Foo {
///    fn eq(&self, other: &Foo) -> bool { ... }
///    fn ne(&self, other: &Foo) -> bool { !(self == other) }
/// }
/// ```
declare_lint! {
    pub PARTIALEQ_NE_IMPL,
    Warn,
    "re-implementing `PartialEq::ne`"
}

#[derive(Clone, Copy)]
pub struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(PARTIALEQ_NE_IMPL)
    }
}

impl LateLintPass for Pass {
    fn check_item(&mut self, cx: &LateContext, item: &Item) {
        if_let_chain! {[
            let ItemImpl(_, _, _, Some(ref trait_ref), _, ref impl_items) = item.node,
            !is_automatically_derived(&*item.attrs),
            trait_ref.path.def.def_id() == cx.tcx.lang_items.eq_trait().unwrap(),
        ], {
            for impl_item in impl_items {
                if &*impl_item.name.as_str() == "ne" {
                    span_lint(cx,
                              PARTIALEQ_NE_IMPL,
                              impl_item.span,
                              "re-implementing `PartialEq::ne` is unnecessary")
                }
            }
        }};
    }
}
