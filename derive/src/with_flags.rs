use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Error, Field, Fields, Type};

#[derive(Default)]
struct FieldAttributes {
    pub flag: u32,
    pub flags_field: bool,
}

impl FieldAttributes {
    fn from_field(field: &Field) -> Self {
        let mut flag: u32 = 0;
        let mut flags_field = false;
        for attr in &field.attrs {
            let (key, value): (String, u32) = match attr.parse_meta().unwrap() {
                syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    lit: syn::Lit::Int(s),
                    ..
                }) => (
                    path.segments[0].ident.to_string(),
                    s.base10_parse().unwrap(),
                ),
                syn::Meta::Path(syn::Path {
                    leading_colon: _,
                    segments,
                }) => (segments[0].ident.to_string(), 0),
                _ => panic!("malformed attribute syntax"),
            };
            if key == "flags_field" {
                flags_field = true;
            } else if key == "flag" {
                flag = value;
            }
        }
        Self { flag, flags_field }
    }
}

pub fn impl_my_derive(input: &DeriveInput) -> Result<TokenStream, Error> {
    let mut pre_unpacking = Vec::new();
    let mut struct_fields = Vec::new();

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let mut has_flags_field = false;
                let mut flags_field: Option<&syn::Ident> = None;
                for field in &fields.named {
                    let ident = field.ident.as_ref().unwrap();
                    let field_attributes = FieldAttributes::from_field(field);
                    let stype = match &field.ty {
                        Type::Path(v) => v,
                        _ => unimplemented!("Deriving FromValue not possible for field: {}", ident),
                    };
                    if field_attributes.flag > 0 && !has_flags_field {
                        panic!("flags_field has to be set before flag")
                    }
                    if !has_flags_field {
                        pre_unpacking.push(quote! {
                            let #ident: #stype = ::bincode::Decode::decode(decoder)?;
                        });
                        struct_fields.push(quote! {
                            #ident
                        });
                        if field_attributes.flags_field {
                            has_flags_field = true;
                            flags_field = Some(ident);
                        }
                    } else {
                        if field_attributes.flag > 0 {
                            let flag = field_attributes.flag;
                            struct_fields.push(quote! {
                                #ident: {
                                    if #flag & #flags_field as u32 == #flag {
                                        Some(::bincode::Decode::decode(decoder)?)
                                    } else {
                                        None
                                    }
                                }
                            });
                        } else {
                            struct_fields.push(quote! {
                                #ident: ::bincode::Decode::decode(decoder)?
                            })
                        }
                    }
                }
            }
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    let name = &input.ident;
    let out = quote! {
        impl ::bincode::Decode for #name {
            fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
                #( #pre_unpacking )*
                Ok(Self{
                    #( #struct_fields ),*
                })
            }
        }
    };
    export_to_file(name.to_string(), out.to_string());
    Ok(out.to_token_stream().into())
}

fn export_to_file(name: String, stream: String) -> bool {
    use std::io::Write;

    let file_postfix = "DecodeWithFlags";

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
