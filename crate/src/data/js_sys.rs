use js_sys::{ArrayBuffer, Object,Array };
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

pub trait ArrayBufferExt {
    fn to_vec_f32(&self) -> Vec<f32>;

    fn to_vec_f64(&self) -> Vec<f64>;

    fn to_vec_u8(&self) -> Vec<u8>;
    
    fn to_vec_u16(&self) -> Vec<u16>;

    fn to_vec_u32(&self) -> Vec<u32>;

    fn to_vec_i8(&self) -> Vec<i8>;

    fn to_vec_i16(&self) -> Vec<i16>;

    fn to_vec_i32(&self) -> Vec<i32>;
}

impl ArrayBufferExt for ArrayBuffer {
    fn to_vec_f32(&self) -> Vec<f32> {
        js_sys::Float32Array::new(&self).to_vec()
    }
    fn to_vec_f64(&self) -> Vec<f64> {
        js_sys::Float64Array::new(&self).to_vec()
    }
    fn to_vec_u8(&self) -> Vec<u8> {
        js_sys::Uint8Array::new(&self).to_vec()
    }
    fn to_vec_u16(&self) -> Vec<u16> {
        js_sys::Uint16Array::new(&self).to_vec()
    }
    fn to_vec_u32(&self) -> Vec<u32> {
        js_sys::Uint32Array::new(&self).to_vec()
    }
    fn to_vec_i8(&self) -> Vec<i8> {
        js_sys::Int8Array::new(&self).to_vec()
    }
    fn to_vec_i16(&self) -> Vec<i16> {
        js_sys::Int16Array::new(&self).to_vec()
    }
    fn to_vec_i32(&self) -> Vec<i32> {
        js_sys::Int32Array::new(&self).to_vec()
    }
}

///newtype wrapper for typed arrays
///
///The main idea is to make the wildly unsafe view() easier
///So use with caution! See the various TypedArray docs on js_sys for details
pub struct TypedData<T, U>(T, PhantomData<U>);
impl<T: AsRef<[U]>, U> TypedData<T, U> {
    pub fn new(values: T) -> Self {
        Self(values, PhantomData)
    }
}

impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for js_sys::Int8Array {
    fn from(data: TypedData<T, i8>) -> Self {
        unsafe { js_sys::Int8Array::view(data.0.as_ref()) }
    }
}

impl<T: AsRef<[u8]>> From<TypedData<T, u8>> for js_sys::Uint8Array {
    fn from(data: TypedData<T, u8>) -> Self {
        unsafe { js_sys::Uint8Array::view(data.0.as_ref()) }
    }
}
impl<T: AsRef<[i16]>> From<TypedData<T, i16>> for js_sys::Int16Array {
    fn from(data: TypedData<T, i16>) -> Self {
        unsafe { js_sys::Int16Array::view(data.0.as_ref()) }
    }
}
impl<T: AsRef<[u16]>> From<TypedData<T, u16>> for js_sys::Uint16Array {
    fn from(data: TypedData<T, u16>) -> Self {
        unsafe { js_sys::Uint16Array::view(data.0.as_ref()) }
    }
}
impl<T: AsRef<[i32]>> From<TypedData<T, i32>> for js_sys::Int32Array {
    fn from(data: TypedData<T, i32>) -> Self {
        unsafe { js_sys::Int32Array::view(data.0.as_ref()) }
    }
}
impl<T: AsRef<[u32]>> From<TypedData<T, u32>> for js_sys::Uint32Array {
    fn from(data: TypedData<T, u32>) -> Self {
        unsafe { js_sys::Uint32Array::view(data.0.as_ref()) }
    }
}
impl<T: AsRef<[f32]>> From<TypedData<T, f32>> for js_sys::Float32Array {
    fn from(data: TypedData<T, f32>) -> Self {
        unsafe { js_sys::Float32Array::view(data.0.as_ref()) }
    }
}
impl<T: AsRef<[f64]>> From<TypedData<T, f64>> for js_sys::Float64Array {
    fn from(data: TypedData<T, f64>) -> Self {
        unsafe { js_sys::Float64Array::view(data.0.as_ref()) }
    }
}

//implementations for different data types as Array
impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for Array {
    fn from(data: TypedData<T, i8>) -> Self {
        unsafe { Array::from(&js_sys::Int8Array::view(data.0.as_ref())) }
    }
}

impl<T: AsRef<[u8]>> From<TypedData<T, u8>> for Array {
    fn from(data: TypedData<T, u8>) -> Self {
        unsafe { Array::from(&js_sys::Uint8Array::view(data.0.as_ref())) }
    }
}
impl<T: AsRef<[i16]>> From<TypedData<T, i16>> for Array {
    fn from(data: TypedData<T, i16>) -> Self {
        unsafe { Array::from(&js_sys::Int16Array::view(data.0.as_ref())) }
    }
}
impl<T: AsRef<[u16]>> From<TypedData<T, u16>> for Array {
    fn from(data: TypedData<T, u16>) -> Self {
        unsafe { Array::from(&js_sys::Uint16Array::view(data.0.as_ref())) }
    }
}
impl<T: AsRef<[i32]>> From<TypedData<T, i32>> for Array {
    fn from(data: TypedData<T, i32>) -> Self {
        unsafe { Array::from(&js_sys::Int32Array::view(data.0.as_ref())) }
    }
}
impl<T: AsRef<[u32]>> From<TypedData<T, u32>> for Array {
    fn from(data: TypedData<T, u32>) -> Self {
        unsafe { Array::from(&js_sys::Uint32Array::view(data.0.as_ref())) }
    }
}
impl<T: AsRef<[f32]>> From<TypedData<T, f32>> for Array {
    fn from(data: TypedData<T, f32>) -> Self {
        unsafe { Array::from(&js_sys::Float32Array::view(data.0.as_ref())) }
    }
}
impl<T: AsRef<[f64]>> From<TypedData<T, f64>> for Array {
    fn from(data: TypedData<T, f64>) -> Self {
        unsafe { Array::from(&js_sys::Float64Array::view(data.0.as_ref())) }
    }
}

//implementations for different data types as ArrayBuffer
impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for ArrayBuffer {
    fn from(data: TypedData<T, i8>) -> Self {
        unsafe { js_sys::Int8Array::view(data.0.as_ref()).buffer() }
    }
}

impl<T: AsRef<[u8]>> From<TypedData<T, u8>> for ArrayBuffer {
    fn from(data: TypedData<T, u8>) -> Self {
        unsafe { js_sys::Uint8Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[i16]>> From<TypedData<T, i16>> for ArrayBuffer {
    fn from(data: TypedData<T, i16>) -> Self {
        unsafe { js_sys::Int16Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[u16]>> From<TypedData<T, u16>> for ArrayBuffer {
    fn from(data: TypedData<T, u16>) -> Self {
        unsafe { js_sys::Uint16Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[i32]>> From<TypedData<T, i32>> for ArrayBuffer {
    fn from(data: TypedData<T, i32>) -> Self {
        unsafe { js_sys::Int32Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[u32]>> From<TypedData<T, u32>> for ArrayBuffer {
    fn from(data: TypedData<T, u32>) -> Self {
        unsafe { js_sys::Uint32Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[f32]>> From<TypedData<T, f32>> for ArrayBuffer {
    fn from(data: TypedData<T, f32>) -> Self {
        unsafe { js_sys::Float32Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[f64]>> From<TypedData<T, f64>> for ArrayBuffer {
    fn from(data: TypedData<T, f64>) -> Self {
        unsafe { js_sys::Float64Array::view(data.0.as_ref()).buffer() }
    }
}

//implementations for different data types as JsValue
impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for JsValue {
    fn from(data: TypedData<T, i8>) -> Self {
        unsafe { js_sys::Int8Array::view(data.0.as_ref()).into() }
    }
}

impl<T: AsRef<[u8]>> From<TypedData<T, u8>> for JsValue {
    fn from(data: TypedData<T, u8>) -> Self {
        unsafe { js_sys::Uint8Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[i16]>> From<TypedData<T, i16>> for JsValue {
    fn from(data: TypedData<T, i16>) -> Self {
        unsafe { js_sys::Int16Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[u16]>> From<TypedData<T, u16>> for JsValue {
    fn from(data: TypedData<T, u16>) -> Self {
        unsafe { js_sys::Uint16Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[i32]>> From<TypedData<T, i32>> for JsValue {
    fn from(data: TypedData<T, i32>) -> Self {
        unsafe { js_sys::Int32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[u32]>> From<TypedData<T, u32>> for JsValue {
    fn from(data: TypedData<T, u32>) -> Self {
        unsafe { js_sys::Uint32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[f32]>> From<TypedData<T, f32>> for JsValue {
    fn from(data: TypedData<T, f32>) -> Self {
        unsafe { js_sys::Float32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[f64]>> From<TypedData<T, f64>> for JsValue {
    fn from(data: TypedData<T, f64>) -> Self {
        unsafe { js_sys::Float64Array::view(data.0.as_ref()).into() }
    }
}

//implementations for different data types as Object
impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for Object {
    fn from(data: TypedData<T, i8>) -> Self {
        unsafe { js_sys::Int8Array::view(data.0.as_ref()).into() }
    }
}

impl<T: AsRef<[u8]>> From<TypedData<T, u8>> for Object {
    fn from(data: TypedData<T, u8>) -> Self {
        unsafe { js_sys::Uint8Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[i16]>> From<TypedData<T, i16>> for Object {
    fn from(data: TypedData<T, i16>) -> Self {
        unsafe { js_sys::Int16Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[u16]>> From<TypedData<T, u16>> for Object {
    fn from(data: TypedData<T, u16>) -> Self {
        unsafe { js_sys::Uint16Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[i32]>> From<TypedData<T, i32>> for Object {
    fn from(data: TypedData<T, i32>) -> Self {
        unsafe { js_sys::Int32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[u32]>> From<TypedData<T, u32>> for Object {
    fn from(data: TypedData<T, u32>) -> Self {
        unsafe { js_sys::Uint32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[f32]>> From<TypedData<T, f32>> for Object {
    fn from(data: TypedData<T, f32>) -> Self {
        unsafe { js_sys::Float32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[f64]>> From<TypedData<T, f64>> for Object {
    fn from(data: TypedData<T, f64>) -> Self {
        unsafe { js_sys::Float64Array::view(data.0.as_ref()).into() }
    }
}
