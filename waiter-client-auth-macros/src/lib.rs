use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;

#[proc_macro]
pub fn scramble(input: TS) -> TS {
	scramble_impl(input.into()).into()
}

fn scramble_impl(input: TokenStream) -> TokenStream {
	let name = syn::parse2::<syn::LitStr>(input)
    .unwrap();
	let secret = dotenv::var(name.value()).unwrap();
	let secret = secret.as_bytes();
	let mask = rand::thread_rng().sample_iter(rand::distributions::Standard)
    .take(secret.len())
    .collect::<Vec<u8>>();
	let complementary = secret.iter()
    .zip(mask.iter())
    .map(
			|(src, mask): (&u8, &u8)| mask ^ src
		).collect::<Vec<u8>>();

	let length = secret.len();
	let ident = syn::Ident::new(&*name.value(), name.span());
	quote!(
		const #ident: [(u8, u8); #length] = [#((#mask, #complementary)),*];
	).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn scramble_impl() {
			let array = super::scramble_impl(quote::quote!("JWT_SECRET"));
			println!("{}", array);
    }
}
