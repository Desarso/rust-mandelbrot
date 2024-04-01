
use wasm_bindgen::prelude::*;

use rug::ops::CompleteRound;
use rug::Float;


// const PRECISION: i64= 100;
// const PRECISION2: u32= 100;
// const MAX_ITER: u32 = 100;

// fn main() {



//     main_code_run();

// }

// fn main_code_run(){
//     let mut arr = Vec::new();
//     for i in 0..800 {
//         for j in 0..800 {
//             arr.push((i, j));
//         }
//     }
//     let num_threads = 100;
//     let workload_per_thread = arr.len() / num_threads;

//     let mut handles = vec![];
//     let results: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));

//     let start = std::time::Instant::now();

//     for k in 0..num_threads {
//         let arr = arr.clone();
//         let results = results.clone();
//         let handle = thread::spawn(move|| {
//         let mut size = 0;
            
//             for i in k * workload_per_thread..(k + 1) * workload_per_thread {
//                 let (i, j) = arr[i];
//                 size = size + 1;
//                 let n = xy_mandelbrot(i, j, 800, 800, 100, 100);
//                 let mut results = results.lock().unwrap();
//                 results.insert(format!("{}-{}", i, j), n);
//             }
//         });
          
//             handles.push(handle);
//         };
     
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // for (key, value) in results.lock().unwrap().iter() {
//     //     println!("{}: {}\n", key, value);
//     // }

//     let duration = start.elapsed();
//     println!("Time elapsed in expensive_function() is: {:?}", duration);
// }

//gmp-mpfr-sys = {version = "~1.6", features = ["c-no-tests"]}
//rug = "1.24.1"



fn map_range(value: Float, from_low: Float, from_high: Float, to_low: Float, to_high: Float, precision2: u32) -> Float {
    let value_scaled = (value - &from_low)/(&from_high - &from_low).complete(precision2);
    let result = ((&to_low + &to_high).complete(precision2) - &to_low) * value_scaled;
    result
}



fn mandelbrot(c: (Float, Float), precision2: u32, max_iter: u32) -> u32 {
    let zero = Float::with_val(precision2, 0);
    let two = Float::with_val(precision2, 2);

    let mut z = (zero.clone(), zero.clone());
    let mut n = 0;
    while n < max_iter {
       let result = z.0.clone().mul_add_mul(&z.0, &z.1, &z.1);

        if result > two.clone() {
            break;
        }
        z.0 = z.0.clone().mul_sub_mul(&z.0, &z.1, &z.1) + &c.0;
        z.1 = two.clone() * z.0.clone() * z.1.clone() + &c.1;
        n += 1;
    }
    n
}


#[wasm_bindgen]
pub fn xy_mandelbrot(x: i32, y: i32, width: u32, height: u32, precision2: u32, max_iter: u32 ) -> u32 {
    let a = map_range(Float::with_val(precision2, x), Float::with_val(precision2, 0), Float::with_val(precision2, width), Float::with_val(precision2, -2), Float::with_val(precision2, 1), precision2);
    let b = map_range(Float::with_val(precision2, y), Float::with_val(precision2, 0), Float::with_val(precision2, height), Float::with_val(precision2, -1), Float::with_val(precision2, 1), precision2);
    return mandelbrot((a, b), precision2, max_iter);
}