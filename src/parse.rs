use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream, Result},
    Attribute, Error, ItemFn, ItemImpl, ItemTrait, Token,
};

pub enum Item {
    Trait(ItemTrait),
    Impl(ItemImpl),
    Fn(ItemFn),
}
impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mut lookahead = input.lookahead1();
        if lookahead.peek(Token![unsafe]) {
            let ahead = input.fork();
            ahead.parse::<Token![unsafe]>()?;
            lookahead = ahead.lookahead1();
        }
        if lookahead.peek(Token![impl]) {
            let mut item: ItemImpl = input.parse()?;
            if item.trait_.is_none() {
                return Err(Error::new(Span::call_site(), "expected a trait impl"));
            }
            item.attrs = attrs;
            Ok(Item::Impl(item))
        } else if lookahead.peek(Token![pub])
            || lookahead.peek(Token![trait])
            || lookahead.peek(Token![fn])
            || lookahead.peek(Token![async])
        {
            if lookahead.peek(Token![pub]) {
                let ahead = input.fork();
                ahead.parse::<Token![pub]>()?;
                lookahead = ahead.lookahead1();
            }
            if lookahead.peek(Token![trait]) {
                let mut item: ItemTrait = input.parse()?;
                item.attrs = attrs;
                Ok(Item::Trait(item))
            } else {
                let mut item: ItemFn = input.parse()?;
                item.attrs = attrs;
                Ok(Item::Fn(item))
            }
        } else {
            Err(lookahead.error())
        }
    }
}
