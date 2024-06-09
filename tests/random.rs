use wasm_bindgen_test::wasm_bindgen_test;
use vintergatan::random::random_bool;

#[wasm_bindgen_test]
fn test_random_bool() {
    let bool = random_bool();
    assert!(bool || !bool);
}


