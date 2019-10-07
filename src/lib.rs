extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "domUtils")]
extern {
    fn appendNumberToBody(x: u32);
    fn appendStringToBody(s: &str);
}

#[wasm_bindgen]
pub extern fn run() {
    appendNumberToBody(42);
    appendStringToBody("Hello World!");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
