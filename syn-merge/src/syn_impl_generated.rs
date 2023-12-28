//! Automatically generated by `cargo run --bin=generate`.
impl_merge_enum! {
    syn::Expr { Array, Assign, Async, Await, Binary, Block, Break, Call, Cast, Closure,
    Const, Continue, Field, ForLoop, Group, If, Index, Infer, Let, Lit, Loop, Macro,
    Match, MethodCall, Paren, Path, Range, Reference, Repeat, Return, Struct, Try,
    TryBlock, Tuple, Unary, Unsafe, Verbatim, While, Yield, _, }
}
impl_merge_enum! {
    syn::FnArg { Receiver, Typed, }
}
impl_merge_enum! {
    syn::ForeignItem { Fn, Static, Type, Macro, Verbatim, _, }
}
impl_merge_enum! {
    syn::ImplItem { Const, Fn, Type, Macro, Verbatim, _, }
}
impl_merge_enum! {
    syn::ImplRestriction { _, }
}
impl_merge_enum! {
    syn::Item { Const, Enum, ExternCrate, Fn, ForeignMod, Impl, Macro, Mod, Static,
    Struct, Trait, TraitAlias, Type, Union, Use, Verbatim, _, }
}
impl_merge_enum! {
    syn::Meta { Path, List, NameValue, }
}
impl_merge_enum! {
    syn::Pat { Const, Ident, Lit, Macro, Or, Paren, Path, Range, Reference, Rest, Slice,
    Struct, Tuple, TupleStruct, Type, Verbatim, Wild, _, }
}
impl_merge_enum! {
    syn::TraitItem { Const, Fn, Type, Macro, Verbatim, _, }
}
impl_merge_struct! {
    syn::Abi { extern_token, name, }
}
impl_merge_struct! {
    #[attrs] syn::Arm { attrs, pat, guard, fat_arrow_token, body, comma, }
}
impl_merge_struct! {
    syn::Block { brace_token, stmts, }
}
impl_merge_struct! {
    #[attrs] syn::ExprArray { attrs, bracket_token, elems, }
}
impl_merge_struct! {
    #[attrs] syn::ExprAssign { attrs, left, eq_token, right, }
}
impl_merge_struct! {
    #[attrs] syn::ExprAsync { attrs, async_token, capture, block, }
}
impl_merge_struct! {
    #[attrs] syn::ExprAwait { attrs, base, dot_token, await_token, }
}
impl_merge_struct! {
    #[attrs] syn::ExprBinary { attrs, left, op, right, }
}
impl_merge_struct! {
    #[attrs] syn::ExprBlock { attrs, label, block, }
}
impl_merge_struct! {
    #[attrs] syn::ExprBreak { attrs, break_token, label, expr, }
}
impl_merge_struct! {
    #[attrs] syn::ExprCall { attrs, func, paren_token, args, }
}
impl_merge_struct! {
    #[attrs] syn::ExprCast { attrs, expr, as_token, ty, }
}
impl_merge_struct! {
    #[attrs] syn::ExprClosure { attrs, lifetimes, constness, movability, asyncness,
    capture, or1_token, inputs, or2_token, output, body, }
}
impl_merge_struct! {
    #[attrs] syn::ExprConst { attrs, const_token, block, }
}
impl_merge_struct! {
    #[attrs] syn::ExprContinue { attrs, continue_token, label, }
}
impl_merge_struct! {
    #[attrs] syn::ExprField { attrs, base, dot_token, member, }
}
impl_merge_struct! {
    #[attrs] syn::ExprForLoop { attrs, label, for_token, pat, in_token, expr, body, }
}
impl_merge_struct! {
    #[attrs] syn::ExprGroup { attrs, group_token, expr, }
}
impl_merge_struct! {
    #[attrs] syn::ExprIf { attrs, if_token, cond, then_branch, else_branch, }
}
impl_merge_struct! {
    #[attrs] syn::ExprIndex { attrs, expr, bracket_token, index, }
}
impl_merge_struct! {
    #[attrs] syn::ExprInfer { attrs, underscore_token, }
}
impl_merge_struct! {
    #[attrs] syn::ExprLet { attrs, let_token, pat, eq_token, expr, }
}
impl_merge_struct! {
    #[attrs] syn::ExprLit { attrs, lit, }
}
impl_merge_struct! {
    #[attrs] syn::ExprLoop { attrs, label, loop_token, body, }
}
impl_merge_struct! {
    #[attrs] syn::ExprMacro { attrs, mac, }
}
impl_merge_struct! {
    #[attrs] syn::ExprMatch { attrs, match_token, expr, brace_token, arms, }
}
impl_merge_struct! {
    #[attrs] syn::ExprMethodCall { attrs, receiver, dot_token, method, turbofish,
    paren_token, args, }
}
impl_merge_struct! {
    #[attrs] syn::ExprParen { attrs, paren_token, expr, }
}
impl_merge_struct! {
    #[attrs] syn::ExprPath { attrs, qself, path, }
}
impl_merge_struct! {
    #[attrs] syn::ExprRange { attrs, start, limits, end, }
}
impl_merge_struct! {
    #[attrs] syn::ExprReference { attrs, and_token, mutability, expr, }
}
impl_merge_struct! {
    #[attrs] syn::ExprRepeat { attrs, bracket_token, expr, semi_token, len, }
}
impl_merge_struct! {
    #[attrs] syn::ExprReturn { attrs, return_token, expr, }
}
impl_merge_struct! {
    #[attrs] syn::ExprStruct { attrs, qself, path, brace_token, fields, dot2_token, rest,
    }
}
impl_merge_struct! {
    #[attrs] syn::ExprTry { attrs, expr, question_token, }
}
impl_merge_struct! {
    #[attrs] syn::ExprTryBlock { attrs, try_token, block, }
}
impl_merge_struct! {
    #[attrs] syn::ExprTuple { attrs, paren_token, elems, }
}
impl_merge_struct! {
    #[attrs] syn::ExprUnary { attrs, op, expr, }
}
impl_merge_struct! {
    #[attrs] syn::ExprUnsafe { attrs, unsafe_token, block, }
}
impl_merge_struct! {
    #[attrs] syn::ExprWhile { attrs, label, while_token, cond, body, }
}
impl_merge_struct! {
    #[attrs] syn::ExprYield { attrs, yield_token, expr, }
}
impl_merge_struct! {
    #[attrs] syn::Field { attrs, vis, mutability, ident, colon_token, ty, }
}
impl_merge_struct! {
    #[attrs] syn::FieldPat { attrs, member, colon_token, pat, }
}
impl_merge_struct! {
    #[attrs] syn::FieldValue { attrs, member, colon_token, expr, }
}
impl_merge_struct! {
    syn::FieldsNamed { brace_token, named, }
}
impl_merge_struct! {
    syn::FieldsUnnamed { paren_token, unnamed, }
}
impl_merge_struct! {
    #[attrs] syn::ForeignItemFn { attrs, vis, sig, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ForeignItemMacro { attrs, mac, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ForeignItemStatic { attrs, vis, static_token, mutability, ident,
    colon_token, ty, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ForeignItemType { attrs, vis, type_token, ident, generics, semi_token,
    }
}
impl_merge_struct! {
    #[attrs] syn::ImplItemConst { attrs, vis, defaultness, const_token, ident, generics,
    colon_token, ty, eq_token, expr, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ImplItemFn { attrs, vis, defaultness, sig, block, }
}
impl_merge_struct! {
    #[attrs] syn::ImplItemMacro { attrs, mac, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ImplItemType { attrs, vis, defaultness, type_token, ident, generics,
    eq_token, ty, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ItemConst { attrs, vis, const_token, ident, generics, colon_token, ty,
    eq_token, expr, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ItemEnum { attrs, vis, enum_token, ident, generics, brace_token,
    variants, }
}
impl_merge_struct! {
    #[attrs] syn::ItemExternCrate { attrs, vis, extern_token, crate_token, ident, rename,
    semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ItemFn { attrs, vis, sig, block, }
}
impl_merge_struct! {
    #[attrs] syn::ItemForeignMod { attrs, unsafety, abi, brace_token, items, }
}
impl_merge_struct! {
    #[attrs] syn::ItemImpl { attrs, defaultness, unsafety, impl_token, generics, trait_,
    self_ty, brace_token, items, }
}
impl_merge_struct! {
    #[attrs] syn::ItemMacro { attrs, ident, mac, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ItemMod { attrs, vis, unsafety, mod_token, ident, content, semi, }
}
impl_merge_struct! {
    #[attrs] syn::ItemStatic { attrs, vis, static_token, mutability, ident, colon_token,
    ty, eq_token, expr, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ItemStruct { attrs, vis, struct_token, ident, generics, fields,
    semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ItemTrait { attrs, vis, unsafety, auto_token, restriction, trait_token,
    ident, generics, colon_token, supertraits, brace_token, items, }
}
impl_merge_struct! {
    #[attrs] syn::ItemTraitAlias { attrs, vis, trait_token, ident, generics, eq_token,
    bounds, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ItemType { attrs, vis, type_token, ident, generics, eq_token, ty,
    semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::ItemUnion { attrs, vis, union_token, ident, generics, fields, }
}
impl_merge_struct! {
    #[attrs] syn::ItemUse { attrs, vis, use_token, leading_colon, tree, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::Local { attrs, let_token, pat, init, semi_token, }
}
impl_merge_struct! {
    syn::LocalInit { eq_token, expr, diverge, }
}
impl_merge_struct! {
    syn::MetaList { path, delimiter, tokens, }
}
impl_merge_struct! {
    syn::MetaNameValue { path, eq_token, value, }
}
impl_merge_struct! {
    #[attrs] syn::PatIdent { attrs, by_ref, mutability, ident, subpat, }
}
impl_merge_struct! {
    #[attrs] syn::PatOr { attrs, leading_vert, cases, }
}
impl_merge_struct! {
    #[attrs] syn::PatParen { attrs, paren_token, pat, }
}
impl_merge_struct! {
    #[attrs] syn::PatReference { attrs, and_token, mutability, pat, }
}
impl_merge_struct! {
    #[attrs] syn::PatRest { attrs, dot2_token, }
}
impl_merge_struct! {
    #[attrs] syn::PatSlice { attrs, bracket_token, elems, }
}
impl_merge_struct! {
    #[attrs] syn::PatStruct { attrs, qself, path, brace_token, fields, rest, }
}
impl_merge_struct! {
    #[attrs] syn::PatTuple { attrs, paren_token, elems, }
}
impl_merge_struct! {
    #[attrs] syn::PatTupleStruct { attrs, qself, path, paren_token, elems, }
}
impl_merge_struct! {
    #[attrs] syn::PatType { attrs, pat, colon_token, ty, }
}
impl_merge_struct! {
    #[attrs] syn::PatWild { attrs, underscore_token, }
}
impl_merge_struct! {
    #[attrs] syn::Receiver { attrs, reference, mutability, self_token, colon_token, ty, }
}
impl_merge_struct! {
    syn::Signature { constness, asyncness, unsafety, abi, fn_token, ident, generics,
    paren_token, inputs, variadic, output, }
}
impl_merge_struct! {
    #[attrs] syn::StmtMacro { attrs, mac, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::TraitItemConst { attrs, const_token, ident, generics, colon_token, ty,
    default, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::TraitItemFn { attrs, sig, default, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::TraitItemMacro { attrs, mac, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::TraitItemType { attrs, type_token, ident, generics, colon_token,
    bounds, default, semi_token, }
}
impl_merge_struct! {
    #[attrs] syn::Variadic { attrs, pat, dots, comma, }
}
impl_merge_struct! {
    #[attrs] syn::Variant { attrs, ident, fields, discriminant, }
}
impl_merge_eq!(syn::token::Abstract);
impl_merge_eq!(syn::token::And);
impl_merge_eq!(syn::token::AndAnd);
impl_merge_eq!(syn::token::AndEq);
impl_merge_eq!(syn::token::As);
impl_merge_eq!(syn::token::Async);
impl_merge_eq!(syn::token::At);
impl_merge_eq!(syn::token::Auto);
impl_merge_eq!(syn::token::Await);
impl_merge_eq!(syn::token::Become);
impl_merge_eq!(syn::token::Box);
impl_merge_eq!(syn::token::Brace);
impl_merge_eq!(syn::token::Bracket);
impl_merge_eq!(syn::token::Break);
impl_merge_eq!(syn::token::Caret);
impl_merge_eq!(syn::token::CaretEq);
impl_merge_eq!(syn::token::Colon);
impl_merge_eq!(syn::token::Comma);
impl_merge_eq!(syn::token::Const);
impl_merge_eq!(syn::token::Continue);
impl_merge_eq!(syn::token::Crate);
impl_merge_eq!(syn::token::Default);
impl_merge_eq!(syn::token::Do);
impl_merge_eq!(syn::token::Dollar);
impl_merge_eq!(syn::token::Dot);
impl_merge_eq!(syn::token::DotDot);
impl_merge_eq!(syn::token::DotDotDot);
impl_merge_eq!(syn::token::DotDotEq);
impl_merge_eq!(syn::token::Dyn);
impl_merge_eq!(syn::token::Else);
impl_merge_eq!(syn::token::Enum);
impl_merge_eq!(syn::token::Eq);
impl_merge_eq!(syn::token::EqEq);
impl_merge_eq!(syn::token::Extern);
impl_merge_eq!(syn::token::FatArrow);
impl_merge_eq!(syn::token::Final);
impl_merge_eq!(syn::token::Fn);
impl_merge_eq!(syn::token::For);
impl_merge_eq!(syn::token::Ge);
impl_merge_eq!(syn::token::Group);
impl_merge_eq!(syn::token::Gt);
impl_merge_eq!(syn::token::If);
impl_merge_eq!(syn::token::Impl);
impl_merge_eq!(syn::token::In);
impl_merge_eq!(syn::token::LArrow);
impl_merge_eq!(syn::token::Le);
impl_merge_eq!(syn::token::Let);
impl_merge_eq!(syn::token::Loop);
impl_merge_eq!(syn::token::Lt);
impl_merge_eq!(syn::token::Macro);
impl_merge_eq!(syn::token::Match);
impl_merge_eq!(syn::token::Minus);
impl_merge_eq!(syn::token::MinusEq);
impl_merge_eq!(syn::token::Mod);
impl_merge_eq!(syn::token::Move);
impl_merge_eq!(syn::token::Mut);
impl_merge_eq!(syn::token::Ne);
impl_merge_eq!(syn::token::Not);
impl_merge_eq!(syn::token::Or);
impl_merge_eq!(syn::token::OrEq);
impl_merge_eq!(syn::token::OrOr);
impl_merge_eq!(syn::token::Override);
impl_merge_eq!(syn::token::Paren);
impl_merge_eq!(syn::token::PathSep);
impl_merge_eq!(syn::token::Percent);
impl_merge_eq!(syn::token::PercentEq);
impl_merge_eq!(syn::token::Plus);
impl_merge_eq!(syn::token::PlusEq);
impl_merge_eq!(syn::token::Pound);
impl_merge_eq!(syn::token::Priv);
impl_merge_eq!(syn::token::Pub);
impl_merge_eq!(syn::token::Question);
impl_merge_eq!(syn::token::RArrow);
impl_merge_eq!(syn::token::Ref);
impl_merge_eq!(syn::token::Return);
impl_merge_eq!(syn::token::SelfType);
impl_merge_eq!(syn::token::SelfValue);
impl_merge_eq!(syn::token::Semi);
impl_merge_eq!(syn::token::Shl);
impl_merge_eq!(syn::token::ShlEq);
impl_merge_eq!(syn::token::Shr);
impl_merge_eq!(syn::token::ShrEq);
impl_merge_eq!(syn::token::Slash);
impl_merge_eq!(syn::token::SlashEq);
impl_merge_eq!(syn::token::Star);
impl_merge_eq!(syn::token::StarEq);
impl_merge_eq!(syn::token::Static);
impl_merge_eq!(syn::token::Struct);
impl_merge_eq!(syn::token::Super);
impl_merge_eq!(syn::token::Tilde);
impl_merge_eq!(syn::token::Trait);
impl_merge_eq!(syn::token::Try);
impl_merge_eq!(syn::token::Type);
impl_merge_eq!(syn::token::Typeof);
impl_merge_eq!(syn::token::Underscore);
impl_merge_eq!(syn::token::Union);
impl_merge_eq!(syn::token::Unsafe);
impl_merge_eq!(syn::token::Unsized);
impl_merge_eq!(syn::token::Use);
impl_merge_eq!(syn::token::Virtual);
impl_merge_eq!(syn::token::Where);
impl_merge_eq!(syn::token::While);
impl_merge_eq!(syn::token::Yield);
