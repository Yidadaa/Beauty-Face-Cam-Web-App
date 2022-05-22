mod utils;

extern crate serde_json;
extern crate wasm_bindgen;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    fn compare(a: f64, b: f64) -> i32;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log("Hello, wasm-rust!");
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_distance_to_zero(&self) -> f64 {
        let d2 = (self.x.pow(2) + self.y.pow(2)) as f64;
        d2.sqrt()
    }
}

#[wasm_bindgen(js_name = "printObject")]
pub fn print_object(point: &Point) {
    set_panic_hook();
    log(&format!("{:?}", point));
}

#[wasm_bindgen(js_name = "quickSort")]
pub fn quick_sort(arr: &mut [f64]) {
    set_panic_hook();

    let n = arr.len();
    quick_sort_(arr, 0, n - 1);
}

#[wasm_bindgen(js_name = "quickSortCmp")]
pub fn quick_sort_cmp(arr: &mut [f64]) {
    set_panic_hook();

    let n = arr.len();
    quick_sort_cmp_(arr, 0, n - 1);
}

fn quick_sort_cmp_(arr: &mut [f64], left: usize, right: usize) {
    if left < right {
        let pivot = arr[(left + right) / 2];
        let mut i = left;
        let mut j = right;

        while i <= j {
            while compare(arr[i as usize], pivot) < 0 {
                i += 1;
            }

            while compare(arr[j as usize], pivot) > 0 {
                j -= 1;
            }

            if i <= j {
                arr.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        if left < j {
            quick_sort_(arr, left, j);
        }

        if i < right {
            quick_sort_(arr, i, right);
        }
    }
}

fn quick_sort_(arr: &mut [f64], left: usize, right: usize) {
    if left < right {
        let pivot = arr[(left + right) / 2];
        let mut i = left;
        let mut j = right;

        while i <= j {
            while arr[i as usize] < pivot {
                i += 1;
            }

            while arr[j as usize] > pivot {
                j -= 1;
            }

            if i <= j {
                arr.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        if left < j {
            quick_sort_(arr, left, j);
        }

        if i < right {
            quick_sort_(arr, i, right);
        }
    }
}
