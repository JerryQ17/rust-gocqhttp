use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub fn impl_to_string(name: &Ident, fields: &Vec<Ident>) -> TokenStream {
    quote! {
        fn to_string(&self) -> String {
            let mut result = format!("[CQ:{}", stringify!(#name).to_lowercase());
            #({
                let mut f_name = stringify!(#fields);
                if f_name == "type_" {
                    f_name = "type";
                }
                if let Some(ref fields) = self.#fields {
                    result += &format!(
                        ",{}={}",
                        f_name,
                        Self::escape(fields.to_string())
                    );
                }
            })*
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
            if name != stringify!(#name).to_lowercase() {
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
                    _ => return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "CQCode字段不匹配"
                    ))),
                }
            }
            Ok(result)
        }
    }
}

pub fn impl_to_json(name: &Ident) -> TokenStream {
    quote! {
        fn to_json(&self) -> crate::Result<String> {
            let data = serde_json::to_string(self)?;
            Ok(format!("{{\"type\":\"{}\",\"data\":{}}}", stringify!(#name).to_lowercase(), data))
        }
    }
}

pub fn impl_from_json(name: &Ident) -> TokenStream {
    quote! {
        fn from_json(s: &str) -> crate::Result<Self> {
            let v: serde_json::Value = serde_json::from_str(s)?;
            let name = v.get("type").ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "没有找到type字段")
            })?;
            if name.as_str().unwrap() != stringify!(#name).to_lowercase() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "CQCode类型不匹配",
                )));
            }
            let data = v.get("data").ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "没有找到data字段")
            })?;
            Ok(serde_json::from_value(data.to_owned())?)
        }
    }
}
