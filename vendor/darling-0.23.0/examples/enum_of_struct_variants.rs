use darling::{FromDeriveInput, FromMeta};

/// This enum will parse either of the following:
///
/// ```rust,ignore
/// #[parent_field(casual(recipient = "Alice"))]
/// #[parent_field(formal(recipient_surname = "Smith", title = "Dr."))]
/// ```
#[derive(Debug, Clone, FromMeta, PartialEq, Eq)]
enum Greeting {
    Casual {
        recipient: String,
    },
    Formal {
        recipient_surname: String,
        title: String,
    },
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(letter))]
pub struct Letter {
    greeting: Greeting,
}

fn main() {
    // Success case
    let letter = Letter::from_derive_input(&syn::parse_quote! {
        #[letter(greeting(casual(recipient = "Alice")))]
        struct MyLetter;
    })
    .unwrap();
    assert_eq!(
        letter.greeting,
        Greeting::Casual {
            recipient: "Alice".into()
        }
    );
    println!("{:#?}", letter);

    // Failure case - variant does not match fields
    let error = Letter::from_derive_input(&syn::parse_quote! {
        #[letter(greeting(casual(recipient_surname = "Smith", title = "Dr.")))]
        struct MyLetter;
    })
    .unwrap_err();

    println!("{}", error);

    // Failure case - variant format is wrong
    let error = Letter::from_derive_input(&syn::parse_quote! {
        #[letter(greeting = "casual")]
        struct MyLetter;
    })
    .unwrap_err();

    println!("{}", error);
}
