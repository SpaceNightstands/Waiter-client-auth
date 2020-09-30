use wasm_bindgen::prelude::*;
use js_sys::Object;
use hmac::Hmac;
use sha2::Sha256 as SHA;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(not(debug_assertions))]
//Should scramble with const fn
const JWT_SECRET: &str = dotenv_codegen::dotenv!("JWT_SECRET");
#[cfg(debug_assertions)]
const JWT_SECRET: &str = "Test";

lazy_static::lazy_static! {
	static ref JWT_KEY: Result<Hmac<SHA>, String> = {
		use hmac::NewMac;
		Hmac::<SHA>::new_varkey(JWT_SECRET.as_bytes())
			.map_err(
				|err| {
					let error = format!("{:?}", err);
					wasm_bindgen::intern(&*error);
					error
				}
			)
	};
}

#[wasm_bindgen]
pub fn build_jwt(val: &Object) -> Result<String, JsValue> {
	use std::collections::HashMap;
	use jwt::token::signed::SignWithKey;

	use serde_json::Value as JsonValue;
	let map: HashMap<String, JsonValue> = Object::entries(val).iter()
    .filter_map(
			|kvpair| {
				use wasm_bindgen::JsCast;
				let kvpair = kvpair.dyn_ref::<js_sys::Array>()?;
				if kvpair.length() == 2 {
					kvpair.into_serde::<(String, JsonValue)>().ok()
				} else {
					None
				}
			}
		).collect();
	map.sign_with_key(
		JWT_KEY.as_ref()
			.map_err(
				|err| wasm_bindgen::throw_str(err)
			).unwrap()
	).map_err(
		|err| JsValue::from_str(
			&*format!("{}", err)
		)
	)
}

