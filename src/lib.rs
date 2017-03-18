#![recursion_limit = "256"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


fn impl_glium_vertex(ast: &syn::MacroInput) -> quote::Tokens {
    let field_name = match ast.body {
        syn::Body::Struct(ref data) => data.fields(),
        syn::Body::Enum(_) => panic!("#[derive(NumFields)] can only be used with structs"),
    };
    let field_name: Vec<syn::Ident> = field_name.iter()
        .cloned()
        .map(|f| {
                 f.ident
             .expect("#[derive(GliumVertex)] can only be used with structs with named fields")
             })
        .collect();
    let field_name1 = field_name.as_slice();
    let field_name2 = field_name.as_slice();
    let field_name3 = field_name.as_slice();

    let struct_name = &ast.ident;
    let struct_name1 = std::iter::repeat(&struct_name);
    let struct_name2 = std::iter::repeat(&struct_name);

    quote! {
        // The generated impl
        impl ::glium::vertex::Vertex for #struct_name {
            #[inline]
            fn build_bindings() -> ::glium::vertex::VertexFormat {
                use std::borrow::Cow;

                // TODO: use a &'static [] if possible
                Cow::Owned(vec![
                    #(
                        (
                            Cow::Borrowed(stringify!(#field_name1)),
                            {
                                let dummy: &#struct_name1 = unsafe {
                                    ::std::mem::transmute(0usize)
                                };
                                let dummy_field = &dummy.#field_name2;
                                let dummy_field: usize = unsafe {
                                    ::std::mem::transmute(dummy_field)
                                };
                                dummy_field
                            },
                            {
                                fn attr_type_of_val<T: ::glium::vertex::Attribute>(_: &T)
                                    -> ::glium::vertex::AttributeType
                                {
                                    <T as ::glium::vertex::Attribute>::get_type()
                                }
                                let dummy: &#struct_name2 = unsafe {
                                    ::std::mem::transmute(0usize)
                                };
                                attr_type_of_val(&dummy.#field_name3)
                            },
                        )
                    ),*
                ])
            }
        }
    }
}


#[proc_macro_derive(GliumVertex)]
pub fn glium_vertex(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_glium_vertex(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
