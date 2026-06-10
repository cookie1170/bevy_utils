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

    struct Attribute {
        pound: Pound,
        bracket_group: BracketGroup
    }

    struct UntilDecl {
        until_attrs: Vec<Cons<Except<Either<Attribute, DeclKind>>, TokenTree>>,
        attrs: Many<Attribute>,
        until_decl: Vec<Cons<Except<DeclKind>, TokenTree>>,
    }

    struct Decl {
        until: UntilDecl,
        decl: DeclKind,
        name: Ident,
        rest: Vec<TokenTree>,
    }
}

fn output_err(err: impl ToString, item: impl Into<TokenStream>) -> proc_macro::TokenStream {
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

    let group = match Ident::parse(&mut attr) {
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

unsynn! {
    struct ComputedDecl {
        until_enum: Vec<Cons<Except<KEnum>, TokenTree>>,
        _enum: KEnum,
        name: Ident,
    }

    struct NamedAttribute {
        pound: Pound,
        bracket: BracketGroupContaining<Cons<Ident, ParenthesisGroupContaining<Vec<TokenTree>>>>
    }

    struct Variant {
        until_attr: Vec<Cons<Except<NamedAttribute>, TokenTree>>,
        attrs: Vec<NamedAttribute>,
        name: Ident,
    }

    struct ComputedInput {
        decl: ComputedDecl,
        variants: BraceGroupContaining<CommaDelimitedVec<Variant>>,
    }
}

/// Make a computed state
///
/// # Example
/// ```ignore
/// #[derive(States, Debug, Hash, Default, PartialEq, Eq, Clone, Copy)]
/// enum State {
///     Something,
///     SomethingElse,
///     AnotherThing,
/// }
///
/// #[computed(State)]
/// enum Computed {
///     #[pat(State::Something | State::SomethingElse)]
///     Something,
///     #[pat(State::AnotherThing)]
///     Another,
/// }
/// ```
#[proc_macro_attribute]
pub fn computed(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let orig_item = item.clone();
    let attr = TokenStream::from(attr);
    let mut item = TokenStream::from(item).into_token_iter();

    let ComputedInput {
        decl: ComputedDecl {
            until_enum,
            _enum,
            name,
        },
        variants: BraceGroupContaining { content: variants },
    } = match item.parse() {
        Ok(r) => r,
        Err(e) => return output_err(e, orig_item),
    };

    let mut out_variants = vec![];
    let mut out_matches = vec![];

    for v in variants {
        let mut pat = None;
        for attr in v.value.attrs {
            if attr.bracket.content.first != "pat" {
                out_variants.push(attr.into_token_stream());
                continue;
            }

            if pat.is_none() {
                pat = Some(attr.bracket.content.second.content.into_token_stream());
            } else {
                return output_err("multiple #[pat] attributes", orig_item);
            }
        }

        let Some(pat) = pat else {
            return output_err("expected #[pat] attribute with a pattern", orig_item);
        };

        let name = v.value.name.into_token_stream();
        let mut out_variant = v.value.until_attr.into_token_stream();
        out_variant.extend([name.clone(), v.delimiter.into_token_stream()]);
        let mut out_match = pat;
        out_match.extend(["=>".parse().unwrap(), quote! { Some(Self::#name), }]);
        out_variants.push(out_variant);
        out_matches.push(out_match);
    }

    quote! {
        #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
        #until_enum enum #name {
            #{out_variants}
        }

        impl ::bevy::state::state::ComputedStates for #name {
            type SourceStates = #attr;

            fn compute(sources: Self::SourceStates) -> Option<Self> {
                match sources {
                    #{out_matches}
                    _ => None,
                }
            }
        }
    }
    .into()
}
