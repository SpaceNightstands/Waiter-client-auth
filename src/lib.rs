use wasm_bindgen::prelude::*;
use js_sys::Object;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
	fn alert(s: &str);
}

//Should scramble with const fn
const JWT_SECRET: &str = dotenv_codegen::dotenv!("JWT_SECRET");

#[wasm_bindgen]
pub fn build_jwt(val: &Object) -> String {
	use hmac::NewMac;
	use std::collections::HashMap;
	use jwt::token::signed::SignWithKey;

	let key = hmac::Hmac::<sha2::Sha256>::new_varkey(JWT_SECRET.as_bytes())
    .unwrap();

	use serde_json::Value;
	let map: HashMap<String, Value> = Object::entries(val).iter()
    .filter_map(
			|kvpair| {
				use wasm_bindgen::JsCast;
				let kvpair = kvpair.dyn_ref::<js_sys::Array>()?;
				if kvpair.length() == 2 {
					Some(kvpair.into_serde::<(String, Value)>().unwrap())
				} else {
					None
				}
			}
		).collect();
	map.sign_with_key(&key).unwrap()
}

