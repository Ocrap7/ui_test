use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, visit_mut::VisitMut, Token};

struct MultiInput {
    name: syn::Ident,
    _comma: Token![,],
    count: syn::Expr,
}

impl Parse for MultiInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(MultiInput {
            name: input.parse()?,
            _comma: input.parse()?,
            count: input.parse()?,
        })
    }
}

struct BinFolder(u32);

// impl Fold for BinFolder {
//     fn fold_expr(&mut self, i: syn::Expr) -> syn::Expr {
//         match i {
//             syn::Expr::Lit(ExprLit {lit: Lit::Int(i), ..}) =>  {}
//             syn::Expr::Binary(ExprBinary { attrs, left, op, right }) => {

//             }
//             _ => panic!("Unexpected expr")
//         }
//     }
// }

impl VisitMut for BinFolder {
    fn visit_lit_int_mut(&mut self, i: &mut syn::LitInt) {
        self.0 += i.base10_parse::<u32>().unwrap();
    }
}

#[proc_macro]
pub fn multi(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MultiInput {
        name, mut count, ..
    } = parse_macro_input!(tokens as MultiInput);

    let mut folder = BinFolder(0);
    folder.visit_expr_mut(&mut count);

    let ucount = folder.0;

    let struct_name = syn::Ident::new(&format!("{}{}", name, ucount), name.span());

    let val_list: Vec<String> = (0..ucount).map(|i| format!("E{}", i)).collect();
    let ty_list: Vec<String> = (0..ucount).map(|i| format!("E{}: Element", i)).collect();

    let layouts: Vec<TokenStream> = (0..ucount)
        .map(|i| format!("{i} => self.{i}.body().layout(available_rect)"))
        .map(|s| TokenStream::from_str(&s).unwrap())
        .collect();

    let draws: Vec<TokenStream> = (0..ucount)
        .map(|i| format!("{i} => self.{i}.body().draw(dctx)"))
        .map(|s| TokenStream::from_str(&s).unwrap())
        .collect();

    let from_vals: Vec<TokenStream> = (0..ucount)
        .map(|i| format!("value.{i}"))
        .map(|s| TokenStream::from_str(&s).unwrap())
        .collect();

    let val = TokenStream::from_str(&val_list.join(", ")).unwrap();
    let ty = TokenStream::from_str(&ty_list.join(", ")).unwrap();

    let tokens = quote! {
        struct #struct_name<#ty>(#val);

        impl <#ty> ElementIterator for #struct_name<#val> {
            fn len(&self) -> usize {
                #count
            }

            fn layout_at(&self, available_rect: Rect, index: usize) -> Rect {
                match index {
                    #(#layouts),*,
                    _ => panic!("This Element only has 2 children!")
                }
            }

            fn draw_at(&self, dctx: &mut DrawingContext, index: usize) {
                match index {
                    #(#draws),*,
                    _ => panic!("This Element only has 2 children!")
                }
            }
        }

        impl <#ty> From<(#val)> for #struct_name<#val> {
            fn from(value: (#val)) -> #struct_name<#val> {
                #struct_name(#(#from_vals),*)
            }
        }
    };

    tokens.into()
}
