use syn::Type;

pub fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(typepath) if typepath.qself.is_none() => {
            let idents_of_path = typepath
                .path
                .segments
                .iter()
                .fold(String::new(), |mut acc, v| {
                    acc.push_str(&v.ident.to_string());
                    acc.push(':');
                    acc
                });
            vec!["Option:", "std:option:Option:", "core:option:Option:"]
                .into_iter()
                .find(|s| idents_of_path == *s)
                .and_then(|_| typepath.path.segments.last())
                .is_some()
        }
        _ => false,
    }
}
