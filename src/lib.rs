extern crate proc_macro;
extern crate proc_macro2;
extern crate latte_verify;
extern crate latte_lib;

use latte_verify::*;
extern crate quote;
extern crate syn;
use proc_macro2::{Ident, Span};
// use regex::Regex;
use proc_macro::TokenStream;
mod utils;
use utils::*;
use quote::quote;
use syn::DeriveInput;

use latte_lib::*;

use latte_verify::Verify;



fn getAttrStr(attrs: &std::vec::Vec<syn::Attribute>, name: String) -> String {
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident.to_string() == name {
            return attr.tts.to_string()
        }
    }
    return "".to_string()
}
fn parseAttr(name: &syn::Ident, field: &syn::Field) -> String {
    let ty = field.ty.clone();
    let ident = field.ident.clone();
    let attrs = field.attrs.clone();
    let attrStr = getAttrStr(&attrs, "verify".to_string());
    let identStr = ident.unwrap().to_string();
    let mut v: Vec<char> = identStr.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    let mut fnameStr: String =  v.into_iter().collect();
    let setStr = "set".to_string() + &fnameStr;
    let set = Ident::new(&setStr, Span::call_site());
    let getStr = "get".to_string() + &fnameStr;
    let get = Ident::new(&getStr, Span::call_site());
    let mut result = r#"
    impl {{name}} {
        fn {{set}}(mut self,value: {{ty}}) -> Result<bool, &'static str> {
            let str = {{r#}}"{{attrStr}}"{{#}};
            let copy = value.clone();
            if (str.len() == 0) {
                self.{{ident}} = copy;
                return Ok(true);
            }
            let result = value.verify(latte_verify::VerifyConfig::String(str.to_string()));
            match result {
                Ok(v) => {
                    if v {
                        self.{{ident}} = copy;
                    }
                    return Ok(v);
                }
                Err(_) => {
                    return result;
                }
            }
        }
        fn {{get}}(self) -> {{ty}} {
            self.{{ident}}
        }
    }"#.to_string();
    // result = str::replace(&result, "{{name}}", &quote!(#name).to_string());
    // result = str::replace(&result, "{{ty}}", &quote!(#ty).to_string());
    // result = str::replace(&result, "{{set}}", &setStr);
    // result = str::replace(&result, "{{get}}", &getStr);
    // result = str::replace(&result, "{{ident}}", &identStr);
    // result = str::replace(&result, "{{attrStr}}", &toEscape(attrStr));
    result = replace!(result, {
        "{{name}}" => &quote!(#name).to_string(),
        "{{ty}}" => &quote!(#ty).to_string(),
        "{{set}}" => &setStr,
        "{{get}}" => &getStr,
        "{{ident}}" => &identStr,
        "{{attrStr}}" =>  &toEscape(attrStr),
        "{{r#}}" => "r#",
        "{{#}}" => "#"
    });
    // println!("hello {:?}",result);
    result
}

#[proc_macro_derive(Set, attributes(verify))]
pub fn set_derive(input: TokenStream) -> TokenStream {
    //转换成解析对象
    let input: DeriveInput = syn::parse(input).unwrap();
    let mut name = input.ident;
    let output:proc_macro2::TokenStream  = if let syn::Data::Struct(data) = input.data {
        let mut fs = vec!();
        match data.fields {
            syn::Fields::Named(ref fields) => {
                fs = fields.named.iter().enumerate().map(|(i, f)| {
                    parseAttr(&name, &f)
                }).collect();
            },
            syn::Fields::Unit => {
                println!("unit");
            },
            syn::Fields::Unnamed(ref fields) => {
                println!("unamed");
            },
        }
        let ss = fs.join(" ");
        println!("over: {:?}", ss);
        (&ss).parse().unwrap()
    }else{
        panic!("Only impl to struct")
    };
    output.into()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_class() {
        #[derive(Set)]
        struct Struct {
            #[verify{"min":10, "max":100}]  
            x: i32,
            #[verify{"len":3}] 
            y: String
        }
        let s = Struct {
            x: 1,
            y: "200".to_string(),
        };
        println!("Hello, world! {:?}", s.setX(2));
    }
}