use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub fn impl_to_string(name: &Ident, fields: &Vec<Ident>) -> TokenStream {
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

pub fn impl_from_string(name: &Ident, fields: &Vec<Ident>, ty: &Vec<Type>) -> TokenStream {
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
