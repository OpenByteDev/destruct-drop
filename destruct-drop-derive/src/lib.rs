use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, DataEnum, DeriveInput, FieldsNamed, FieldsUnnamed, Ident,
    Index, Member,
};

#[proc_macro_derive(DestructDrop)]
pub fn derive_destruct_drop(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let drop_code = match data {
        syn::Data::Struct(s) => {
            let members = match s.fields {
                syn::Fields::Named(FieldsNamed { named, .. }) => named
                    .into_iter()
                    .map(|f| Member::Named(f.ident.unwrap()))
                    .collect(),
                syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed
                    .into_iter()
                    .enumerate()
                    .map(|(i, f)| {
                        Member::Unnamed(Index {
                            index: i as u32,
                            span: f.span(),
                        })
                    })
                    .collect(),
                syn::Fields::Unit => vec![],
            };

            quote! {
                #( unsafe { ::core::ptr::drop_in_place(&mut this.#members); } )*
            }
        }
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let variant_match_code = variants.into_iter().map(|v| {
                let v_ident = v.ident;
                match v.fields {
                    syn::Fields::Named(FieldsNamed { named, .. }) => {
                        let fields = named.into_iter()
                            .map(|f| f.ident.unwrap())
                            .collect::<Vec<_>>();
                        quote! {
                            #ident::#v_ident {#( ref mut #fields)*,} => { #( unsafe { ::core::ptr::drop_in_place(#fields); } )* }
                        }
                    }
                    syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let fields = unnamed.into_iter()
                            .enumerate()
                            .map(|(i, f)| Ident::new(format!("__f_{}", i).as_str(), f.span()))
                            .collect::<Vec<_>>();
                        quote! {
                            #ident::#v_ident (#( ref mut #fields)*,) => { #( unsafe { ::core::ptr::drop_in_place(#fields); } )* }
                        }
                    }
                    syn::Fields::Unit => quote! {
                        #ident::#v_ident => {}
                    },
                }
            });
            quote! {
                match *this {
                    #( #variant_match_code )*,
                }
            }
        }
        syn::Data::Union(_) => {
            panic!("DestructDrop cannot be derived for unions.");
        }
    };

    quote! {
        impl #ident {
            fn destruct_drop(self) {
                let mut this = ::core::mem::ManuallyDrop::new(self);
                #drop_code
            }
        }
    }
    .into()
}
