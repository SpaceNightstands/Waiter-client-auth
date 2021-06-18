use hmac::Hmac;
use js_sys::Object;
use jwt::token::signed::SignWithKey;
use serde_json::Value as JsonValue;
use sha2::Sha256 as SHA;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

waiter_client_auth_macros::scramble!("JWT_SECRET");

lazy_static::lazy_static! {
	static ref JWT_KEY: Result<Hmac<SHA>, String> = {
		use hmac::NewMac;
		let key = JWT_SECRET.iter()
			.map(
				|(m, c)| (m^c)
			).collect::<Vec<u8>>();
		Hmac::<SHA>::new_varkey(&*key)
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
	let key = JWT_KEY
		.as_ref()
		.map_err(|err| wasm_bindgen::throw_str(err))
		.unwrap();
	let map: HashMap<String, JsonValue> = Object::entries(val)
		.iter()
		.filter_map(|kvpair| {
			use wasm_bindgen::JsCast;
			let kvpair = kvpair.dyn_ref::<js_sys::Array>()?;
			if kvpair.length() == 2 {
				kvpair.into_serde::<(String, JsonValue)>().ok()
			} else {
				None
			}
		})
		.collect();
	map.sign_with_key(key)
		.map_err(|err| JsValue::from_str(&*format!("{}", err)))
}
