use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Error, Fields, Ident, Type, TypePath};

fn extract_type_from_vec(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    fn extract_vec_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Vec|"]
            .into_iter()
            .find(|s| &idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_vec_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

struct StructField {
    name: Ident,
    typ: TypePath,
    basic_type: Type,
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

                    let basic_type = extract_type_from_vec(&field.ty).unwrap();

                    all_fields.push(StructField {
                        name: field_name.clone(),
                        typ: field_type.clone(),
                        basic_type: basic_type.clone(),
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
    let basic_type = &all_fields[0].basic_type;

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
