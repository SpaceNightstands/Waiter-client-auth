use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use rand::Rng;

#[proc_macro]
pub fn scramble(input: TS) -> TS {
	scramble_impl(input.into()).into()
}

fn scramble_impl(input: TokenStream) -> TokenStream {
	let name = match syn::parse2::<syn::LitStr>(input) {
		Ok(lit) => lit,
		Err(error) => {
			return quote_spanned!(
				error.span() => compile_error!("Expected 1 string literal argument")
			)
		}
	};
	let secret = match dotenv::var(name.value()) {
		Ok(string) => string,
		Err(_) => {
			let error_messsage = format!("Environment Variable {:?} not found", name.value());
			return quote_spanned!(
				name.span() => compile_error!(#error_messsage)
			);
		}
	};
	let secret = secret.as_bytes();
	let mask = rand::thread_rng()
		.sample_iter(rand::distributions::Standard)
		.take(secret.len())
		.collect::<Vec<u8>>();
	let complementary = secret
		.iter()
		.zip(mask.iter())
		.map(|(src, mask): (&u8, &u8)| mask ^ src)
		.collect::<Vec<u8>>();

	let length = secret.len();
	let ident = syn::Ident::new(&*name.value(), name.span());
	quote::quote!(
		const #ident: [(u8, u8); #length] = [#((#mask, #complementary)),*];
	)
	.into()
}

#[cfg(test)]
mod tests {
	#[test]
	fn scramble_impl() {
		let array = super::scramble_impl(quote::quote!("JWT_SECRET"));
		println!("{}", array);
	}
}
