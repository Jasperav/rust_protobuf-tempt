use crate::calculators::{ValueCalculator, Assign, add_ampersand, RepeatedComputer, wire_check};
use proc_macro2::{Ident, TokenStream};
use crate::parser::OneOfMapping;

pub struct Repeated {
    pub inner_calculator: Box<dyn ValueCalculator>,
    pub tag_size: u32,
}


impl ValueCalculator for Repeated {
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, type_without_opt: &TokenStream) -> (Assign, TokenStream) {
        let (_, reader) = self.inner_calculator.read_repeated();

        (Assign::Mutate, quote! {
            ::protobuf::rt::#reader(wire_type, is, &mut #ident)?
        })
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let computer = self.inner_calculator.read_repeated().0;
        let tag_size = self.tag_size;

        match computer {
            RepeatedComputer::Reuse => {
                let loop_variable = quote! { temp_loop };
                let size = self.inner_calculator.size(&loop_variable, size_ident, field_number, type_without_opt, true);

                quote! {
                    for #loop_variable in &#ident {
                        let len = #loop_variable.compute_size() as u32;
                        #size_ident += #tag_size + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                    };
                }
            }
            RepeatedComputer::UseLen => quote! {
                #size_ident += #tag_size * #ident.len() as u32;
            },
        }
    }

    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let loop_ident = quote! { v };
        let writer = self.inner_calculator.write(&loop_ident, os_ident, field_number, type_without_opt, true);

        quote! {
            for #loop_ident in &#ident {
                #writer
            };
        }
    }

    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            vec![]
        })
    }

    fn read_repeated(&self) -> (RepeatedComputer, TokenStream) {
        unreachable!()
    }
}