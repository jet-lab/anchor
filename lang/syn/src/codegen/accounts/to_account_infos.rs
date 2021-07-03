use crate::codegen::accounts::generics;
use crate::{AccountField, AccountsStruct};
use quote::quote;

// Generates the `ToAccountInfos` trait implementation.
pub fn generate(accs: &AccountsStruct) -> proc_macro2::TokenStream {
    let name = &accs.ident;
    let (combined_generics, trait_generics, strct_generics) = generics(accs);

    let to_acc_infos: Vec<proc_macro2::TokenStream> = accs
        .fields
        .iter()
        .map(|f: &AccountField| {
            let name = match f {
                AccountField::CompositeField(s) => &s.ident,
                AccountField::Field(f) => &f.ident,
            };
            quote! {
                account_infos.extend(self.#name.to_account_infos());
            }
        })
        .collect();
    quote! {
        impl#combined_generics anchor_lang::ToAccountInfos#trait_generics for #name#strct_generics {
            fn to_account_infos(&self) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = vec![];

                #(#to_acc_infos)*

                account_infos
            }
        }
    }
}