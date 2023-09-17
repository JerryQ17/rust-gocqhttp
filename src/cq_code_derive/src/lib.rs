use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(CQCode)]
pub fn cq_code_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_cq_code(&ast)
}

fn get_field(ast: &DeriveInput) -> Vec<&Ident> {
    let data = &ast.data;
    if let Data::Struct(s) = data {
        if let Fields::Named(n) = &s.fields {
            Vec::from_iter(n.named.iter().map(|f| f.ident.as_ref().unwrap()))
        } else {
            panic!("CQCode can only be implemented for structs with named fields")
        }
    } else {
        panic!("CQCode can only be implemented for structs")
    }
}

fn impl_cq_code(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = get_field(ast);
    let body = quote! {
        let mut result = format!("[CQ:{}", stringify!(#name));
        #(
            if let Some(ref #fields) = self.#fields {
                result += &format!(",{}={}", stringify!(#fields), #fields);
            }
        )*
        result += "]";
        result
    };
    let gen = quote! {
        impl CQCode for #name {
            fn to_string(&self) -> String {
                #body
            }
        }
    };
    gen.into()
}
