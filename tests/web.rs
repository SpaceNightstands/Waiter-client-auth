//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use waiter_client_auth::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn object_1() {
	console_error_panic_hook::set_once();
	use wasm_bindgen::{JsCast, JsValue};
	let test = js_sys::Function::new_no_args("return {test: \"ABC\"}").call0(&JsValue::NULL).unwrap()
		.dyn_into::<js_sys::Object>()
		.unwrap();
	assert_eq!(build_jwt(&test), "eyJhbGciOiJIUzI1NiJ9.eyJ0ZXN0IjoiQUJDIn0.i1J1hhBpHZT70_gZZipdE_cWsZJxAoK3p03I7FjcfsA");
}

fn object_2() {
	console_error_panic_hook::set_once();
	use wasm_bindgen::{JsCast, JsValue};
	let test = js_sys::Function::new_no_args("return {test: 123}").call0(&JsValue::NULL).unwrap()
		.dyn_into::<js_sys::Object>()
		.unwrap();
	assert_eq!(build_jwt(&test), "eyJhbGciOiJIUzI1NiJ9.eyJ0ZXN0IjoxMjN9.yFfpIQpxtxzaXgLENVd9yDOfq9rA9l17E2aPLqmRmiA");
}
