// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    naming::{property::QPropertyName, qobject::QObjectName},
    rust::fragment::RustFragmentPair,
};
use quote::quote;
use syn::Type;

pub fn generate(
    idents: &QPropertyName,
    qobject_idents: &QObjectName,
    ty: &Type,
) -> RustFragmentPair {
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
    let rust_struct_name_rust = &qobject_idents.rust_struct.rust;
    let getter_cpp = idents.getter.cpp.to_string();
    let getter_rust = &idents.getter.rust;
    let getter_mutable_rust = &idents.getter_mutable.rust;
    let ident = &idents.name.rust;
    let ident_str = ident.to_string();
    let notify_ident_str = idents.notify.rust.to_string();

    RustFragmentPair {
        cxx_bridge: vec![quote! {
            extern "Rust" {
                #[cxx_name = #getter_cpp]
                unsafe fn #getter_rust<'a>(self: &'a #rust_struct_name_rust, cpp: &'a #cpp_class_name_rust) -> &'a #ty;
            }
        }],
        implementation: vec![
            quote! {
                impl #rust_struct_name_rust {
                    #[doc(hidden)]
                    pub fn #getter_rust<'a>(&'a self, cpp: &'a #cpp_class_name_rust) -> &'a #ty {
                        cpp.#getter_rust()
                    }
                }
            },
            quote! {
                impl #cpp_class_name_rust {
                    #[doc = "Getter for the Q_PROPERTY "]
                    #[doc = #ident_str]
                    pub fn #getter_rust(&self) -> &#ty {
                        &self.rust().#ident
                    }
                }
            },
            quote! {
                impl #cpp_class_name_rust {
                    #[doc = "unsafe getter for the Q_PROPERTY "]
                    #[doc = #ident_str]
                    #[doc = "\n"]
                    #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
                    #[doc = "\n"]
                    #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
                    #[doc = #notify_ident_str]
                    pub unsafe fn #getter_mutable_rust<'a>(self: Pin<&'a mut Self>) -> &'a mut #ty {
                        &mut self.rust_mut().get_unchecked_mut().#ident
                    }
                }
            },
        ],
    }
}
