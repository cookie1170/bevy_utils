use unsynn::*;

keyword! {
    KStruct = "struct";
    KEnum = "enum";
}

unsynn! {
    enum DeclKind {
        Struct(KStruct),
        Enum(KEnum)
    }

    struct UntilDecl {
        until_attrs: Vec<Cons<Except<Either<Cons<Pound, BracketGroup>, DeclKind>>, TokenTree>>,
        attrs: Many<Cons<Pound, BracketGroup>>,
        until_decl: Vec<Cons<Except<DeclKind>, TokenTree>>,
    }

    struct Decl {
        until: UntilDecl,
        decl: DeclKind,
        name: Ident,
        rest: Vec<TokenTree>,
    }

    struct AttrInput {
        group: Ident
    }
}

fn output_err(
    err: impl std::error::Error,
    item: impl Into<TokenStream>,
) -> proc_macro::TokenStream {
    let err = err.to_string();
    let mut out = item.into();
    out.extend([
        TokenTree::Ident(Ident::new("compile_error", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenTree::Literal(Literal::string(&err)).into(),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);

    out.into()
}

/// Make this component part of the given group
/// Only one component in a group may exist on a given entity
#[proc_macro_attribute]
pub fn group(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr2 = TokenStream::from(attr);
    let mut attr = attr2.to_token_iter();
    let item2 = TokenStream::from(item);
    let mut item = item2.clone().to_token_iter();

    let Decl {
        until:
            UntilDecl {
                until_attrs,
                attrs,
                until_decl,
            },
        decl,
        name,
        rest,
    } = match Decl::parse(&mut item) {
        Ok(r) => r,
        Err(e) => {
            return output_err(e, item2);
        }
    };

    let AttrInput { group } = match AttrInput::parse(&mut attr) {
        Ok(r) => r,
        Err(e) => {
            return output_err(e, item2);
        }
    };

    let hooks = quote! {
        #[component(on_insert = ::bevy_utils::group::component_group_on_insert::<#name, #group>)]
        #[component(on_remove = ::bevy_utils::group::component_group_on_remove::<#name, #group>)]
    };

    let mut out = TokenStream::new();
    out.extend([
        until_attrs.into_token_stream(),
        attrs.into_token_stream(),
        hooks.into_token_stream(),
        until_decl.into_token_stream(),
        decl.into_token_stream(),
        name.into_token_stream(),
        rest.into_token_stream(),
    ]);

    out.into()
}
