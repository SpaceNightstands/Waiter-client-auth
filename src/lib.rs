mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
	fn alert(s: &str);
}

//Should scramble with const fn
const JWT_SECRET: &str = dotenv_codegen::dotenv!("JWT_SECRET");

#[wasm_bindgen]
pub fn build_jwt() {
	utils::set_panic_hook();
	alert(JWT_SECRET);
}
