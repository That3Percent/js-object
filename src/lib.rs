/// Helper for creating Object instances
#[macro_export]
macro_rules! js_object {
	($($key:expr, $value:expr),+) => {
		{
			use js_sys::{Object, Reflect};
			use wasm_bindgen::JsValue;

			let o = Object::new();
			$(
				Reflect::set(&o, &JsValue::from_str($key), &$value).unwrap();
			)*
			o
		}
	};
}

