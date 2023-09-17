use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident, Type};

#[proc_macro_derive(CQCode)]
pub fn cq_code_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_cq_code(&ast)
}

fn mapping_input<T>(ast: &DeriveInput, f: impl FnMut(&Field) -> T) -> Vec<T> {
    let data = &ast.data;
    if let Data::Struct(s) = data {
        if let Fields::Named(n) = &s.fields {
            Vec::from_iter(n.named.iter().map(f))
        } else {
            panic!("CQCode can only be implemented for structs with named fields")
        }
    } else {
        panic!("CQCode can only be implemented for structs")
    }
}

fn get_field(ast: &DeriveInput) -> Vec<Ident> {
    mapping_input(ast, |f| f.ident.clone().unwrap())
}

fn get_type(ast: &DeriveInput) -> Vec<Type> {
    mapping_input(ast, |f| f.ty.clone())
}

fn impl_to_string(name: &Ident, fields: &Vec<Ident>) -> TokenStream {
    quote! {
        fn to_string(&self) -> String {
            let mut result = format!("[CQ:{}", stringify!(#name));
            #(
                if let Some(ref fields) = self.#fields {
                    result += &format!(
                        ",{}={}",
                        stringify!(#fields),
                        Self::escape(fields.to_string())
                    );
                }
            )*
            result += "]";
            result
        }
    }
}

fn impl_from_string(name: &Ident, fields: &Vec<Ident>, ty: &Vec<Type>) -> TokenStream {
    quote! {
        fn from_string(s: String) -> crate::Result<Self> {
            let re = regex::Regex::new(r"\[CQ:(?P<name>\w+)(?P<fields>(,\w+=[^,\]]+)*)\]").unwrap();
            let caps = re.captures(&s).unwrap();
            let name = caps.name("name").unwrap().as_str();
            if name != stringify!(#name) {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "CQCode类型不匹配"
                )));
            }
            let fields = caps.name("fields").unwrap().as_str();
            let mut result = #name {
                #(
                    #fields: None,
                )*
            };
            let re = regex::Regex::new(r"(?P<field>\w+)=(?P<value>[^,\]]+)")?;
            for cap in re.captures_iter(fields) {
                let field = cap.name("field").unwrap().as_str();
                let value = Self::anti_escape(cap.name("value").unwrap().as_str());
                match field {
                    #(
                        stringify!(#fields) => {
                            let field: #ty = Some(value.parse().unwrap());
                            result.#fields = field;
                        }
                    )*
                    _ => {}
                }
            }
            Ok(result)
        }
    }
}

fn impl_cq_code(ast: &DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let ty = get_type(ast);
    let fields = get_field(ast);
    let fn_to_string = impl_to_string(name, &fields);
    let fn_from_string = impl_from_string(name, &fields, &ty);
    let gen = quote! {
        impl CQCode for #name {
            #fn_to_string

            #fn_from_string
        }
    };
    gen.into()
}
