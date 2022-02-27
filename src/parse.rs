use proc_macro2::Span;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    Attribute, Error, ItemFn, ItemImpl, ItemTrait,
};

pub enum Item {
    Trait(ItemTrait),
    Impl(ItemImpl),
    Fn(ItemFn),
}

macro_rules! fork {
    ($fork:ident = $input:ident) => {{
        $fork = $input.fork();
        &$fork
    }};
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mut fork;
        let item = if let Some(mut item) = fork!(fork = input).parse::<ItemImpl>().ok() {
            if item.trait_.is_none() {
                return Err(Error::new(Span::call_site(), "expected a trait impl"));
            }
            item.attrs = attrs;
            Item::Impl(item)
        } else if let Some(mut item) = fork!(fork = input).parse::<ItemTrait>().ok() {
            item.attrs = attrs;
            Item::Trait(item)
        } else if let Some(mut item) = fork!(fork = input).parse::<ItemFn>().ok() {
            item.attrs = attrs;
            Item::Fn(item)
        } else {
            return Err(Error::new(
                Span::call_site(),
                "expected trait impl, trait or fn",
            ));
        };
        input.advance_to(&fork);
        Ok(item)
    }
}
