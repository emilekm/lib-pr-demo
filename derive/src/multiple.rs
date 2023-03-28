use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Error, Fields, Ident, Type, TypePath};

struct StructField {
    name: Ident,
    typ: TypePath,
    underlaying_type: String,
}

pub fn impl_my_derive(input: &DeriveInput) -> Result<TokenStream, Error> {
    let mut all_fields = Vec::new();

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_type = match &field.ty {
                        Type::Path(v) => v,
                        _ => unimplemented!(
                            "Deriving FromValue not possible for field: {}",
                            field_name
                        ),
                    };

                    let mut underlaying_type: String = String::new();
                    for attr in &field.attrs {
                        let (key, value): (String, String) = match attr.parse_meta().unwrap() {
                            syn::Meta::NameValue(syn::MetaNameValue {
                                path,
                                lit: syn::Lit::Str(s),
                                ..
                            }) => (path.segments[0].ident.to_string(), s.value()),
                            _ => panic!("malformed attribute syntax"),
                        };
                        if key == "many" {
                            underlaying_type = value;
                        }
                    }
                    all_fields.push(StructField {
                        name: field_name.clone(),
                        typ: field_type.clone(),
                        underlaying_type,
                    })
                }
            }
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    if all_fields.len() != 1 {
        unimplemented!("Only one field is supported, received {}", all_fields.len())
    }

    let field_name = &all_fields[0].name;
    let field_type = &all_fields[0].typ;
    let basic_type: proc_macro2::TokenStream = all_fields[0].underlaying_type.parse().unwrap();

    let name = &input.ident;
    let out = quote! {
        impl ::bincode::Decode for #name {
            fn decode<D: ::bincode::de::Decoder>(decoder: &mut D) -> Result<Self, ::bincode::error::DecodeError> {
                let mut #field_name: #field_type = Vec::new();
                loop {
                    let result: Result<Option<#basic_type>, ::bincode::error::DecodeError> = match #basic_type::decode(decoder) {
                        Ok(obj) => Ok(Some(obj)),
                        Err(error) => {
                            if let ::bincode::error::DecodeError::Io { inner, additional } = error {
                                if inner.kind() == std::io::ErrorKind::UnexpectedEof {
                                    Ok(None)
                                } else {
                                    Err(::bincode::error::DecodeError::Io { inner, additional })
                                }
                            } else {
                                Err(error)
                            }
                        }
                    };
                    let obj = match result {
                        Ok(obj) => obj,
                        Err(error) => return Err(error),
                    };
                    if obj.is_none() {
                        break;
                    };
                    #field_name.push(obj.unwrap());
                }
                Ok(Self { #field_name })
            }
        }
    };
    export_to_file(name.to_string(), out.to_string());
    Ok(out.to_token_stream().into())
}

fn export_to_file(name: String, stream: String) -> bool {
    use std::io::Write;

    let file_postfix = "DecodeMultiple";

    if let Ok(var) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(var);
        loop {
            {
                let mut path = path.clone();
                path.push("target");
                if path.exists() {
                    path.push(format!("{}_{}.rs", name, file_postfix));
                    if let Ok(mut file) = std::fs::File::create(path) {
                        let _ = file.write_all(stream.as_bytes());
                        return true;
                    }
                }
            }
            if let Some(parent) = path.parent() {
                path = parent.to_owned();
            } else {
                break;
            }
        }
    }
    false
}
