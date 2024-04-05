pub mod complex;

use core::{panic, time};
use std::{iter, str::FromStr};

use dashu_float::DBig;

use complex::{test_complexes, Complex};
use rust_mandelbrot::{get_frame_values, mandelbrot_floats, xy_mandelbrot_floats, FrameValues};

const START_X: f64 = -0.7451581436739864000000000000000;
const END_X: f64 = -  0.7451581436740010000000000000000;
const START_Y: f64 =  0.1239776004148912500000000000000;
const END_Y: f64 =    0.1239776004148765400000000000000;

// const START_X: f32 = -2.5;
// const START_Y: f32 = -2.5;
// const END_X: f32 = 2.5;
// const END_Y: f64 = 2.5;

const WIDTH: f64 = 600.0;
const MAX_ITER: f64 = 1000.0;

fn main() {
    let reference = Complex::new(-0.7451581436739960000000000000, 0.123977600414880970000000000000);
    let first_pxiel = Complex::new(-0.74515814367398640000000000, 0.123977600414891250000000000000);

    let x_0 =  Complex::new( -0.745158143673996, 0.12397760041488097);
    // let x_n_1 = low_precision_series((x_0.real, x_0.imag));
    // let x_n = x_0.get_series(1000);
    let x_n_high = x_0.high_precision_series(1000, 50);

    // let screen_size = WIDTH * WIDTH;
    // let end_x = float_to_dbig(END_X).with_precision(30).unwrap();
    // let start_x = float_to_dbig(START_X).with_precision(30).unwrap();
    // let start_y = float_to_dbig(START_Y).with_precision(30).unwrap();
    // let width = float_to_dbig(WIDTH).with_precision(30).unwrap();
    // let slope = (&end_x - &start_x) / &width;
    // let slope2 = (&END_X - &START_X) / WIDTH;
    // let mut pixels: Vec<Complex> = Vec::new();
    // let mut pixels2: Vec<Complex> = Vec::new();

    // for i in 0..WIDTH as i32 {
    //     for j in 0..WIDTH as i32{
    //         let x = &START_X +slope2 * i as f64;
    //         let y = &START_Y +slope2 * j as f64;
    //         pixels2.push(Complex::new(x, y));
    //     }
    // }


    // for i in 0..WIDTH as i32 {
    //     for j in 0..WIDTH as i32{
    //         let x = &start_x + (&slope * float_to_dbig(i as f64));
    //         let y = &start_y + (&slope * float_to_dbig(j as f64));
    //         pixels.push(Complex::new(dbig_to_float(x.clone()), dbig_to_float(y.clone())));
    //     }
    // }

    //print first 10 pixels
    // for i in 0..100 {
    //     print!("Pixel:  {}, {}\n", pixels[i].real, pixels[i].imag);
    //     print!("Pixel2: {}, {}\n", pixels2[i].real, pixels2[i].imag);
    // }

    //let n = modified_xy_mandelbrot_perturbation(x_n_high.clone(), first_pxiel, 1000.0);
    //print!("Iterations: {}\n", n);

    let pixels = get_perturbed_pixels(START_X, START_Y, END_X, WIDTH, MAX_ITER);

    // //print all the values
    let values = pixels.values().clone();
    for i in 0..10 {
        print!("{} \n", values[i]);
    }

    
    // // //print last 10 values of x_n, x_n_1, x_n_high
    // for i in 0..10 {
    //     print!("x_n:      {}, {}\n", x_n[i].real, x_n[i].imag);
    //     print!("x_n_1:    {}, {}\n", x_n_1[i].real, x_n_1[i].imag);
    //     print!("x_n_high: {}, {}\n", x_n_high[i+1].real, x_n_high[i+1].imag);
    // }


}

//and more correctly than the current function

pub fn xy_mandelbrot_perturbation(
    x_n: Vec<Complex>,
    y0: Complex,
    max_iterations: f64,
) -> u32 {
    let delta_0: Complex = &y0 - x_n[1].clone();
    let mut delta_n = Vec::new();
    let two_complex = Complex::new(2.0, 0.0);
    delta_n.push(delta_0.clone());
    //check if delta^3 is significantly smaller than delta^2
    if !significanly_smaller(delta_0.clone()) {
        print!("delta^3 is not significantly smaller than delta^2\n");
        //priny y0 and x0
        print!("y0: {}, x0: {}\n", y0, x_n[1]);
        print!(
            "delta^3: {}, delta^2: {}\n",
            delta_0.pow(3).magnitude(),
            delta_0.square().magnitude()
        );
        panic!("delta^3 is not significantly smaller than delta^2");
    }
    for n in 0..x_n.len() {
        let delta = &two_complex * &x_n[n] * &delta_n[n] + &delta_n[n].square() + &delta_0;
        let yn = &x_n[n] + &delta;
        delta_n.push(delta);
        if yn.magnitude() > 2.0 {
            return n as u32;
        }
    }
    
    return max_iterations as u32;
}

fn modified_xy_mandelbrot_perturbation(
    x_n: Vec<Complex>,
    y0: Complex,
    max_iterations: f64,
) -> u32 {
    let delta_0: Complex = y0.high_prec_sub(x_n[1].clone(), 100);
    delta_0.print_high_precision();
    print!("real: {}, imag: {}\n", delta_0.real, delta_0.imag);
    let mut delta_z = Complex::new(0.0, 0.0);
    let two_complex = Complex::new(2.0, 0.0);
    let mut ref_iter = 0;
    let mut iter = 0;
    let max_iter = x_n.len() as f64-2.0;
    //check if delta^3 is significantly smaller than delta^2
    if !significanly_smaller(delta_0.clone()) {
        print!("delta^3 is not significantly smaller than delta^2\n");
        //priny y0 and x0
        print!("y0: {}\nx0: {}\n", y0, x_n[1]);
        //print high press parts of y0 and x0
        y0.print_high_precision();
        x_n[1].print_high_precision();
        print!(
            "delta^3: {}, delta^2: {}\n",
            delta_0.pow(3).magnitude(),
            delta_0.square().magnitude()
        );
        print!("delta_0: {}\n", delta_0);
        panic!("delta^3 is not significantly smaller than delta^2");
    }
    while iter < max_iter as usize{
        delta_z = &two_complex  * &delta_z * &x_n[ref_iter] + &delta_z.square() + &delta_0;
        // print!("delta_z: {}\n", delta_z);
        ref_iter += 1;
        let yn = &x_n[ref_iter] + &delta_z;
        if yn.magnitude() > 2.0 {
            return iter as u32;
        }
        if yn.magnitude() < delta_z.magnitude() || ref_iter == max_iter as usize {
            delta_z = yn.clone();
            ref_iter = 0;
        }
        iter += 1;
    }
    
    return max_iterations as u32;
}

fn modified_xy_mandelbrot_perturbation_high_prec(
    x_n: Vec<Complex>,
    y0: Complex,
    max_iterations: f64,
) -> u32 {
    let precision = 100;
    let delta_0: Complex = y0.high_prec_sub(x_n[1].clone(), 100);
    // delta_0.print_high_precision();
    // print!("real: {}, imag: {}\n", delta_0.real, delta_0.imag);
    let mut delta_z = Complex::new_with_high_precision(float_to_dbig(0.0), float_to_dbig(0.0), precision);
    let two_complex = Complex::new_with_high_precision(float_to_dbig(2.0), float_to_dbig(0.0), precision);
    let mut ref_iter = 0;
    let mut iter = 0;
    let max_iter = x_n.len() as f64-2.0;
    //check if delta^3 is significantly smaller than delta^2
    if !significanly_smaller(delta_0.clone()) {
        print!("delta^3 is not significantly smaller than delta^2\n");
        //priny y0 and x0
        print!("y0: {}\nx0: {}\n", y0, x_n[1]);
        //print high press parts of y0 and x0
        y0.print_high_precision();
        x_n[1].print_high_precision();
        print!(
            "delta^3: {}, delta^2: {}\n",
            delta_0.pow(3).magnitude(),
            delta_0.square().magnitude()
        );
        print!("delta_0: {}\n", delta_0);
        panic!("delta^3 is not significantly smaller than delta^2");
    }
    while iter < max_iter as usize{
        delta_z = two_complex.high_prec_mul(&delta_z,precision).high_prec_mul(&x_n[ref_iter], precision).high_prec_add(delta_z.high_prec_square(precision),precision).high_prec_add(delta_0.clone(),precision);
        // print!("delta_z: {}\n", delta_z);
        ref_iter += 1;
        let yn = x_n[ref_iter].high_prec_add(delta_z.clone(), precision);
        if yn.magnitude() > 2.0 {
            return iter as u32;
        }
        if yn.magnitude() < delta_z.magnitude() || ref_iter == max_iter as usize {
            delta_z = yn.clone();
            ref_iter = 0;
        }
        iter += 1;
    }
    
    return max_iterations as u32;
}




pub fn get_perturbed_pixels(
    start_x: f64,
    start_y: f64,
    end_x: f64,
    width: f64,
    max_iter: f64,
) -> FrameValues {
    // let slope = (&end_x - &start_x) / &width;
    //print screen size
    // print!("Screen size: {}\n", width * width);
    let screen_size = width * width;
    let slopef64 = (&end_x - &start_x) / width;
    let slope = float_to_dbig(slopef64);
    let start_x = float_to_dbig(start_x);
    let start_y = float_to_dbig(start_y);
    let mut pixels: Vec<Complex> = Vec::new();

    for i in 0..WIDTH as i32 {
        for j in 0..WIDTH as i32{
            let x = &start_x + (&slope * float_to_dbig(i as f64));
            let y = &start_y + (&slope * float_to_dbig(j as f64));
            pixels.push(Complex::new_with_high_precision(x, y, 100));
        }
    }

    let mut high_res_found = false;
    let mut pixels_clone = pixels.clone();
    let mut loops = 0;
    let mut high_res = Vec::new();
    //let time = std::time::Instant::now();
    let mut max = 0;
    let mut max_res: Vec<Complex> = Vec::new();

    while !high_res_found {
        let random_index = rand::random::<usize>() % &pixels_clone.len();
        let mut y0 = pixels_clone[random_index].clone();
        //remove the random index from the list
        pixels_clone.remove(random_index);
        high_res = y0.high_precision_series(1000, 100);
        // high_res = low_precision_series((y0.real, y0.imag));
        let n = high_res.len();

        if n > max as usize {
            max = n;
            max_res = high_res.clone();
            print!("Max: {}\n", max);
        }

        if n >= max_iter as usize {
            high_res_found = true;
            break;
        }
        loops += 1;
        if loops == pixels.len() {
            //throw an error
            high_res = max_res.clone();
            break;
            // panic!("Could not find a high precision series that could generate the right amount of iterations max: {}", n)
        }
    }
    // let y0 = Complex::new(0.01, 0.01);
    // let high_res = y0.high_precision_series(1000, 50);

    let mut max = 0.0;
    let mut min = 100000.0;
    let mut res: Vec<f64> = Vec::new();
    for i in 0..screen_size as usize {
        let y0 = pixels[i].clone();
        let n = modified_xy_mandelbrot_perturbation_high_prec( high_res.clone(), y0, max_iter);
        if i % 1000 == 0 {
            print!("i is {} out of {}\n", i, screen_size);
        }

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

//LETS MAKE SURE delta^3 is significanly smaller than delta^2
pub fn significanly_smaller(delta: Complex) -> bool {
    let delta_square = delta.square();
    let delta_cube = delta.pow(3);
    if delta_cube.magnitude() == 0.0 {
        return true;
    }

    if delta_cube.magnitude() < delta_square.magnitude() * 0.01 {
        return true;
    }
    return false;
}

pub fn float_to_dbig(f: f64) -> DBig {
    return DBig::from_str(&f.to_string()).unwrap();
}

pub fn dbig_to_float(d: DBig) -> f64 {
    //turn to string
    return d.to_string().parse::<f64>().unwrap();
}