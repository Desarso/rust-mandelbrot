
pub mod complex;

use complex::Complex;
use wasm_bindgen::prelude::*;

use dashu_float::DBig;
use dashu_int::IBig;
use std::{
    collections::HashMap, str::FromStr, sync::{Arc, Mutex}, thread
};

// fn main() {
//     // draw_frame(
//     //     "-25".to_owned(),
//     //     -1,
//     //     "-25".to_owned(),
//     //     -1,
//     //     "25".to_owned(),
//     //     -1,
//     //     600,
//     //     100,
//     // );

//    
// }

// #[wasm_bindgen]
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
    x: i32,
    y: i32,
    slope: f64,
    max_iter: u32,
) -> u32 {
    let a = start_x + (slope * x as f64);
    let b = start_y + (slope * y as f64);
    return mandelbrot_floats(a, b, max_iter);
}

#[wasm_bindgen]
pub fn mandelbrot_floats(mut a: f64, mut b: f64, max_iter: u32) -> u32 {
    let ac = a;
    let bc = b;
    let four = 4;
    for n in 0..max_iter {
        let aa = a * a - b * b;
        let bb = 2.0 * a * b;
        a = aa + ac;
        b = bb + bc;
        if n >= 390 && n < 400{
            // print!("a: {},    b: {}\n", a, b);
            //print magnitude
        }
        if a * a + b * b > four as f64 {
            return n;
        }
    }
    return max_iter;
}

#[wasm_bindgen]
pub struct FrameValues {
    values: Vec<f64>,
    min: f64,
    max: f64,
}



#[wasm_bindgen]
impl FrameValues {
    #[wasm_bindgen(constructor)]
    pub fn new(values: Vec<f64>, min: f64, max: f64) -> FrameValues {
        FrameValues { values, min, max}
    }



    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Vec<f64> {
        self.values.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn min(&self) -> f64 {
        self.min
    }

    #[wasm_bindgen(getter)]
    pub fn max(&self) -> f64 {
        self.max
    }


}

#[wasm_bindgen]
pub fn get_frame_values(
    start_x: f64,
    start_y: f64,
    end_x: f64,
    width: f64,
    max_iter: f64,
) -> FrameValues {
    // let start = std::time::Instant::now();
    let slope = (&end_x - &start_x) / width;
    let mut arr: Vec<(i32, i32)> = Vec::new();
    for i in 0..width as i32 {
        for j in 0..width as i32 {
            arr.push((i, j));
        }
    }
    let mut res: Vec<f64> = Vec::new();
    let mut max = 0.0;
    let mut min = 100000.0;
    for (i, j) in arr {
        let n = xy_mandelbrot_floats(start_x, start_y, i, j, slope, max_iter as u32);
        res.push(n as f64);
        if n as f64 > max {
            max = n as f64;
        }
        if (n as f64) < min {
            min = n as f64;
        }
    }
    // let duration = start.elapsed().as_secs_f64();
    return FrameValues {
        values: res,
        min,
        max
    };
}


pub fn significanly_smaller(delta: Complex) -> bool {
    let delta_square = delta.square();
    let delta_cube = delta.pow(3);
    if delta_cube.magnitude() == 0.0{
        return true;
    }


    if delta_cube.magnitude() < delta_square.magnitude() * 0.1{
        return true;
    }
    return false;
}


pub fn float_to_dbig(f: f64) -> DBig {
    return DBig::from_str(&f.to_string()).unwrap();
}

pub fn xy_mandelbrot_perturbation(
    series: Vec<(Complex, Complex, Complex)>,
    x_n: Vec<Complex>,
    y0: Complex,
    max_iterations: u32,
) -> i32 {
    let delta_0: Complex = &y0 - x_n[0].clone();
    //check if delta^3 is significantly smaller than delta^2
    if !significanly_smaller(delta_0.clone()) {
        print!("delta^3 is not significantly smaller than delta^2\n");
        //priny y0 and x0
        print!("y0: {}, x0: {}\n", y0, x_n[0]);
        print!("delta^3: {}, delta^2: {}\n", delta_0.pow(3).magnitude(), delta_0.square().magnitude());
        panic!("delta^3 is not significantly smaller than delta^2");
    }
    for n in 0..max_iterations as usize {
        let delta_n = (&series[n].0 * &delta_0)
            + (&series[n].1 * delta_0.square())
            + (&series[n].2 * delta_0.pow(3));
        let yn = &x_n[n] + &delta_n;
        if yn.magnitude() > 2.0 {
            return n as i32;
        }
    }
    return max_iterations as i32;
}
//high precision function
pub fn high_precision_series(mut c: (DBig, DBig)) -> (Vec<(Complex, Complex, Complex)>, Vec<Complex>) {
    let max_iter = 1000;
    //first we compute the high precision series for the given c0
    let ac = c.0.clone();
    let bc = c.1.clone();
    let two: DBig = DBig::from(2);
    let mut series: Vec<(Complex, Complex, Complex)> = Vec::new();
    let mut Xn: Vec<Complex> = Vec::new();
    Xn.push(Complex::new(c.0.to_string().parse::<f64>().unwrap(), c.1.to_string().parse::<f64>().unwrap()));
    let mut A = Complex::new(1.0, 0.0);
    let mut B = Complex::new(0.0, 0.0);
    let mut C = Complex::new(0.0, 0.0);
    series.push((A.clone(), B.clone(), C.clone()));

    for n in 0..max_iter {
        let temp = c.0.clone();
        c.0 = (&c.0 * &c.0 - &c.1 * &c.1) + &ac;
        c.1 = (&two * temp * &c.1) + &bc;
        let temp_a = A.clone();
        let temp_b = B.clone();
        let temp_c = C.clone();
        let x_n = Complex::new(
            c.0.to_string().parse::<f64>().unwrap(),
            c.1.to_string().parse::<f64>().unwrap(),
        );
        Xn.push(x_n.clone());
        let two_complex = Complex::new(2.0, 0.0);
        let one_complex = Complex::new(1.0, 0.0);

        A = &two_complex * &x_n * &temp_a + one_complex;
        B = &two_complex * &x_n * &temp_b + temp_a.square();
        C = &two_complex * &x_n * temp_c + &two_complex * &temp_a * &temp_b;
        series.push((A.clone(), B.clone(), C.clone()));
        if x_n.magnitude() > 2.0 {
            break;
        }
    }
    return (series, Xn);
}




#[wasm_bindgen]
pub fn get_perturbed_pixels(
    start_x: f64,
    start_y: f64,
    end_x: f64,
    width: f64,
    max_iter: u32,
) -> FrameValues {
    let screen_size = width * width;
    let slope = (&end_x - &start_x) / width;
    let pixesl: Vec<Complex> = (0..screen_size as i32)
        .map(|i| {
            let x = i % width as i32;
            let y = i / width as i32;
            // pixels are not mapped to the complex plane correctly

            let x = start_x + (slope * x as f64);
            let y = start_y + (slope * y as f64);
            Complex::new(x, y)
        })
        .collect();
    
    let mut high_res_found = false;
    let mut pixesl_clone = pixesl.clone();
    let mut loops = 0;
    let mut high_res = (Vec::new(), Vec::new());
    //let time = std::time::Instant::now();
    let mut max = 0;
    let mut max_res: (Vec<(Complex,Complex,Complex)>,Vec<Complex>) = (Vec::new(), Vec::new());
    while !high_res_found {


        let random_index = rand::random::<usize>() % pixesl_clone.len();
        let y0 = Complex::new(pixesl_clone[random_index].real, pixesl_clone[random_index].imag);
        //remove the random index from the list
        pixesl_clone.remove(random_index);
        high_res = high_precision_series((float_to_dbig(y0.real),float_to_dbig(y0.imag)));
        let n = high_res.0.len();

        if high_res.0.len() > max as usize{
            max = n;
            max_res = high_res.clone();
            print!("Max: {}\n", max);
        }

        if n >= max_iter as usize{
            high_res_found = true;
            break;
        }
        loops +=1;
        if loops == pixesl.len(){
            //throw an error
            high_res = max_res.clone();
            break;
        }
    }

    // println!(
    //     "Time elapsed to get high_res is: {:?}",
    //     time.elapsed()
    // );
    //print the first 10 parts of the high_res





    let mut max = 0.0;
    let mut min = 100000.0;
    let mut res: Vec<f64> = Vec::new();
    for i in 0..screen_size as usize {
        let y0 = Complex::new(pixesl[i].real, pixesl[i].imag);
        let n = xy_mandelbrot_perturbation(high_res.0.clone(), high_res.1.clone(), y0, max_iter);
        res.push(n as f64);
        if n as f64 > max {
            max = n as f64;
        }
        if (n as f64) < min {
            min = n as f64;
        }
    }
    return FrameValues::new(res, min, max);
}

