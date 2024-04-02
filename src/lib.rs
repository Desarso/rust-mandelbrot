use wasm_bindgen::prelude::*;

use dashu_float::DBig;
use dashu_int::IBig;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};



// fn main() {
//     draw_frame(
//         "-25".to_owned(),
//         -1,
//         "-25".to_owned(),
//         -1,
//         "25".to_owned(),
//         -1,
//         600,
//         100,
//     );
// }


#[wasm_bindgen]
pub fn draw_frame(
    start_x_significand: String,
    start_x_exp: isize,
    start_y_significand: String,
    start_y_exp: isize,
    end_x_significand: String,
    end_x_exp: isize,
    width: i32,
    max_iter: u32,
) {
    let start_x = DBig::from_parts(
        IBig::from_str_radix(&start_x_significand, 10).unwrap(),
        start_x_exp,
    );
    let start_y = DBig::from_parts(
        IBig::from_str_radix(&start_y_significand, 10).unwrap(),
        start_y_exp,
    );
    let end_x = DBig::from_parts(
        IBig::from_str_radix(&end_x_significand, 10).unwrap(),
        end_x_exp,
    );
    let slope = (&end_x - &start_x) / IBig::from(width);
    print!("start_x: {}\n", start_x);
    print!("start_y: {}\n", start_y);
    print!("end_x: {}\n", end_x);
    print!("slope: {}\n", slope);

    let mut arr: Vec<(DBig, DBig)> = Vec::new();
    for i in 0..width {
        for j in 0..width {
            arr.push((DBig::from(i), DBig::from(j)));
        }
    }

    let num_threads = 40;
    let workload_per_thread = arr.len() / num_threads;

    let mut handles = vec![];
    let results: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));

    let start = std::time::Instant::now();

    for k in 0..num_threads {
        let results = Arc::clone(&results);
        let arr = arr.clone();
        let start_x = start_x.clone();
        let start_y = start_y.clone();
        let slope = slope.clone();
        let handle = thread::spawn(move || {
            let mut size = 0;
            for i in k * workload_per_thread..(k + 1) * workload_per_thread {
                let (i, j) = &arr[i];
                size = size + 1;
                let n = xy_mandelbrot2(&start_x, &start_y, &i, &j, &slope, max_iter);
                let mut results = results.lock().unwrap();
                results.insert(format!("{}-{}", i, j), n);
                //print!("Doing pixel ({}, {}) with n = {}\n", i, j, n);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // for (key, value) in results.lock().unwrap().iter() {
    //     println!("{}: {}\n", key, value);
    // }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

// //big decimal
fn xy_mandelbrot2(
    start_x: &DBig,
    start_y: &DBig,
    x: &DBig,
    y: &DBig,
    slope: &DBig,
    max_iter: u32,
) -> u32 {
    let a = start_x + (slope * x);
    let b = start_y + (slope * y);
    return mandelbrot(a, b, max_iter);
}




fn mandelbrot(mut a: DBig, mut b: DBig, max_iter: u32) -> u32 {
    let ac = a.clone();
    let bc = b.clone();
    let two: DBig = DBig::from(2);
    // println!("a: {}\n b: {}\n", a, b);

    for n in 0..max_iter {
        a = (&a * &a - &b * &b) + &ac;
        b = (&two * &a * &b) + &bc;
        if &a * &a + &b * &b > two {
            return n;
        }
    }
    return max_iter;
}

#[wasm_bindgen]
pub fn xy_mandelbrot_floats(
    start_x: f64,
    start_y: f64,
    x: f64,
    y: f64,
    slope: f64,
    max_iter: u32,
) -> u32 {
    let a = start_x + slope * x;
    let b = start_y + slope * y;
    return mandelbrot_floats(a, b, max_iter);
}

#[wasm_bindgen]
pub fn mandelbrot_floats(mut a: f64,mut b: f64, max_iter: u32) -> u32 {
    let ac = a;
    let bc = b;
    let two: f64 = 2.0;
    for n in 0..max_iter {
        a = (&a * &a - &b * &b) + &ac;
        b = (&two * &a * &b) + &bc;
        if a * a + b * b > two {
            return n;
        }
    }
    return max_iter;
}