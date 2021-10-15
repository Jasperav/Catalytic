use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn json(derive_input: DeriveInput) -> TokenStream {
    let name = derive_input.ident;

    quote! {
        impl scylla::frame::value::Value for #name {
            fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), scylla::frame::value::ValueTooBig> {
                let serialized: String = serde_json::to_string(&self).unwrap().into();

                serialized.serialize(buf)
            }
        }

        impl scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue> for #name {
            fn from_cql(cql_val: scylla::frame::response::result::CqlValue) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
                let t = match cql_val.as_text() {
                    None => return Err(scylla::cql_to_rust::FromCqlValError::BadCqlType),
                    Some(t) => t
                };

                serde_json::from_str(&t).map_err(|_| scylla::cql_to_rust::FromCqlValError::BadCqlType)
            }
        }
    }
}
