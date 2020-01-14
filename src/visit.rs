use syn::{
    parse_quote, token,
    visit_mut::{self, VisitMut},
    Expr, ExprBlock, ExprParen,
};

pub struct AsyncAwaitRemoval;

impl VisitMut for AsyncAwaitRemoval {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        match node {
            Expr::Await(expr) => {
                let inner = &expr.base;
                let block = ExprParen {
                    attrs: expr.attrs.clone(),
                    expr: parse_quote!(#inner),
                    paren_token: token::Paren(expr.await_token.span),
                };
                *node = Expr::Paren(block)
            }
            Expr::Async(expr) => {
                let inner = &expr.block;
                let block = ExprBlock {
                    attrs: expr.attrs.clone(),
                    block: parse_quote!(#inner),
                    label: None,
                };
                *node = Expr::Block(block);
            }
            _ => {}
        }

        // Delegate to the default impl to visit nested expressions.
        visit_mut::visit_expr_mut(self, node);
    }
}
