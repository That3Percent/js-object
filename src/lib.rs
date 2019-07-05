#![feature(specialization, proc_macro_hygiene)]

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

#[doc(hidden)]
pub trait JsObjectValue__ {
	fn js_object_set__(self, obj: &Object, key: &JsValue);
}

impl<T: Into<JsValue>> JsObjectValue__ for T {
	#[inline(always)]
	default fn js_object_set__(self, obj: &Object, key: &JsValue) {
		let value = self.into();
		Reflect::set(obj, key, &value).unwrap();
	}
}


impl JsObjectValue__ for &JsValue {
	#[inline(always)]
	fn js_object_set__(self, obj: &Object, key: &JsValue) {
		Reflect::set(obj, key, self).unwrap();
	}
}


/// Helper for creating Object instances
#[macro_export]
macro_rules! js_object {
	($($key:expr, $value:expr),+) => {
		{
			use js_intern::{js_intern, try_js_intern};

			let o = js_sys::Object::new();
			$(
				{
					let k = js_intern!($key);
					let v = try_js_intern!($value);
					$crate::JsObjectValue__::js_object_set__(v, &o, k);
				}
			)*
			o
		}
	};
}

