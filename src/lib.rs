use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields, Path};


#[derive(FromMeta, Clone)]
struct UpdateWithOpt {
    ty: Path,
    func: Path,
}

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(update_struct), supports(struct_named))]
struct StructUpdateArgs {
    #[darling(default, multiple, rename = "with")]
    pub models: Vec<UpdateWithOpt>,
}

fn derive_struct_update_impl(input: TokenStream) -> TokenStream {
    let original_struct = parse_macro_input!(input as DeriveInput);

    let DeriveInput {
        data, ident, vis, ..
    } = original_struct.clone();

    if let Struct(data_struct) = data {
        let DataStruct { fields, .. } = data_struct;

        let args = match StructUpdateArgs::from_derive_input(&original_struct) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };

        let StructUpdateArgs { models } = args;

        let mut output = quote!();

        if models.is_empty() {
            panic!("Please specify at least 1 model using the `model` attribute")
        }

        for model in models {
            let generated_model = generate_update_func(&fields, &model);

            output.extend(quote!(#generated_model));
        }

        quote! {
            impl #ident {
                #vis fn update_struct(&mut self){
                    #output
                }
            }
        }
        .into()
    } else {
        panic!("DeriveCustomModel can only be used with named structs")
    }
}

fn generate_update_func(fields: &Fields, model: &UpdateWithOpt) -> proc_macro2::TokenStream {
    let UpdateWithOpt { ty, func } = model;

    let mut new_fields = quote!();

    for Field {
        ident: field_ident,
        ty: field_ty,
        ..
    } in fields
    {
        let Some(ident) = field_ident else {
            panic!("Failed to get struct field identifier")
        };

        if let syn::Type::Path(field_ty) = field_ty {
            let field_ty_path = field_ty.path.clone();
            if field_ty_path.get_ident() != ty.get_ident() {
                continue;
            }

            new_fields.extend(quote! {
                self.#ident = #func(self.#ident.clone());
            });
        }
    }
    new_fields
}

#[proc_macro_derive(StructUpdate, attributes(update_struct))]
pub fn derive_struct_update(item: TokenStream) -> TokenStream {
    derive_struct_update_impl(item)
}
