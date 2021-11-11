use crate::ast;
use crate::{
    Id, OptionSpanned as _, Parse, ParseError, Parser, Peek, Peeker, Span, Spanned, ToTokens,
};

/// A module item.
///
/// # Examples
///
/// ```rust
/// use rune::{testing, ast};
///
/// testing::roundtrip::<ast::ItemMod>("mod ruins {}");
///
/// let item = testing::roundtrip::<ast::ItemMod>("#[cfg(test)] mod tests {}");
/// assert_eq!(item.attributes.len(), 1);
///
/// let item = testing::roundtrip::<ast::ItemMod>("mod whiskey_bravo { #![allow(dead_code)] fn x() {} }");
/// assert_eq!(item.attributes.len(), 0);
/// assert!(matches!(item.body, ast::ItemModBody::InlineBody(..)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Parse, ToTokens, Spanned)]
#[rune(parse = "meta_only")]
pub struct ItemMod {
    /// The id of the module item.
    #[rune(id)]
    pub id: Option<Id>,
    /// The *inner* attributes are applied to the module  `#[cfg(test)] mod tests {  }`
    #[rune(iter, meta)]
    pub attributes: Vec<ast::Attribute>,
    /// The visibility of the `mod` item
    #[rune(optional, meta)]
    pub visibility: ast::Visibility,
    /// The `mod` keyword.
    pub mod_token: T![mod],
    /// The name of the mod.
    pub name: ast::Ident,
    /// The optional body of the module declaration.
    pub body: ItemModBody,
}

impl ItemMod {
    /// Get the span of the mod name.
    pub fn name_span(&self) -> Span {
        if let Some(span) = self.visibility.option_span() {
            span.join(self.name.span())
        } else {
            self.mod_token.span().join(self.name.span())
        }
    }
}

item_parse!(Mod, ItemMod, "mod item");

/// An item body.
#[derive(Debug, Clone, PartialEq, Eq, ToTokens, Spanned)]
pub enum ItemModBody {
    /// An empty body terminated by a semicolon.
    EmptyBody(T![;]),
    /// An inline body.
    InlineBody(ItemInlineBody),
}

impl Parse for ItemModBody {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        Ok(match p.nth(0)? {
            K!['{'] => Self::InlineBody(p.parse()?),
            _ => Self::EmptyBody(p.parse()?),
        })
    }
}

/// A module declaration.
#[derive(Debug, Clone, PartialEq, Eq, ToTokens, Parse, Spanned)]
pub struct ItemInlineBody {
    /// The open brace.
    pub open: T!['{'],
    /// A nested "file" declaration.
    pub file: Box<ast::File>,
    /// The close brace.
    pub close: T!['}'],
}

impl Peek for ItemInlineBody {
    fn peek(p: &mut Peeker<'_>) -> bool {
        <T!['{']>::peek(p)
    }
}
