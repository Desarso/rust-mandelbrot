pub mod complex;

use core::{panic, time};
use std::str::FromStr;

use dashu_float::DBig;

use complex::{test_complexes, Complex};
use rust_mandelbrot::{get_frame_values, mandelbrot_floats, xy_mandelbrot_floats, FrameValues};

const START_X: f64 = -0.7451581436739864;
const END_X: f64 = -0.745158143674001;
const START_Y: f64 = 0.12397760041489125;
const END_Y: f64 = 0.12397760041487654;

// const START_X: f32 = -2.5;
// const START_Y: f32 = -2.5;
// const END_X: f32 = 2.5;
// const END_Y: f64 = 2.5;

const WIDTH: f64 = 600.0;
const MAX_ITER: f64 = 1000.0;

fn main() {
    let reference =   Complex::new(-0.745158143673996,  0.12397760041488097);
    let first_pixel = Complex::new(-0.7451581436739864, 0.12397760041489125);

    let mut pixels = Vec::new();
    for i in 0..600 {
        for j in 0..600 {
            let x = START_X + (i as f64 * (END_X - START_X) / WIDTH);
            let y = START_Y + (j as f64 * (END_Y - START_Y) / WIDTH);
            pixels.push(Complex::new(x, y));
        }
    }


    let n = mandelbrot_floats(first_pixel.real, first_pixel.imag, 1000);
    // let high_res = high_precision_series((
    //     float_to_dbig(-0.7451581436739864),
    //     float_to_dbig(0.12397760041489125),
    // ));
    let low_res = low_precision_series((reference.real, reference.imag));

    let perturbation = xy_mandelbrot_perturbation(
        low_res.0.clone(),
        low_res.1.clone(),
        pixels[0].clone(),
        1000 as f64,
    );
    // let perturbation2 = xy_mandelbrot_perturbation(
    //     low_res.0.clone(),
    //     low_res.1.clone(),
    //     pixels[1].clone(),
    //     1000 as f64,
    // );

    print!("Iterations: {}\n", n);
    print!("low res: {}\n", low_res.1.len() -2);
    print!("Perturbation: {}\n", perturbation);
    // print!("Perturbation2: {}\n", perturbation2);


    // let values = get_frame_values(START_X, START_Y, END_X, WIDTH, MAX_ITER);
    // let pertTurbedVals = get_perturbed_pixels2(START_X, START_Y, END_X, WIDTH, MAX_ITER);
    // //print the first 10 values
    // // let vals = values.values();
    // // for i in 0..10 {
    // //     print!("Value: {}\n", vals[i]);
    // // }
    // let perVals = pertTurbedVals.values();
    // for i in 0..10 {
    //     print!("Perts: {}\n", perVals[i])
    // }
    



}

//create a function that can accurately decipher the correct points faster
//and more correctly than the current function

pub fn xy_mandelbrot_perturbation(
    series: Vec<(Complex, Complex, Complex)>,
    x_n: Vec<Complex>,
    y0: Complex,
    max_iterations: f64,
) -> u32 {
    let delta_0: Complex = &y0 - x_n[0].clone();
    //check if delta^3 is significantly smaller than delta^2
    if !significanly_smaller(delta_0.clone()) {
        print!("delta^3 is not significantly smaller than delta^2\n");
        //priny y0 and x0
        print!("y0: {}, x0: {}\n", y0, x_n[0]);
        print!(
            "delta^3: {}, delta^2: {}\n",
            delta_0.pow(3).magnitude(),
            delta_0.square().magnitude()
        );
        panic!("delta^3 is not significantly smaller than delta^2");
    }
    for n in 0..max_iterations as usize {
        let delta_n = (&series[n+1].0 * &delta_0)
            + (&series[n+1].1 * delta_0.square())
            + (&series[n+1].2 * delta_0.pow(3));
        // let yn = &x_n[n] + &delta_n;
        let yn = &x_n[n+1] + &delta_n;

       
        if  n < 1{
            print!("n: {} - {}\n", delta_n.real, delta_n.imag);
            print!("n: {} - {}\n", &x_n[n+1].real, &x_n[n+1].imag);
            print!("n: {} - {}\n", yn.real, yn.imag);
            // print!("c: {},   d: {}\n", yn.real, yn.imag);
            //print magnitude
    
        }
        if  &yn.real * &yn.real + &yn.imag * &yn.imag > 4.0{
            //print the number of iterations it took to escape
            return n as u32;
        }       
    }
    return max_iterations as u32;
}

//high precision function
pub fn high_precision_series(
    mut c: (DBig, DBig),
) -> (Vec<(Complex, Complex, Complex)>, Vec<Complex>) {
    let max_iter = 1000;
    //first we compute the high precision series for the given c0
    let ac = c.0.clone();
    let bc = c.1.clone();
    let four: DBig = DBig::from(4);
    let two: DBig = DBig::from(2);
    let mut series: Vec<(Complex, Complex, Complex)> = Vec::new();
    let mut Xn: Vec<Complex> = Vec::new();

    Xn.push(Complex::new(
        c.0.to_string().parse::<f64>().unwrap(),
        c.1.to_string().parse::<f64>().unwrap(),
    ));
    print!("Xn: {}\n", Xn[0]);
    let mut A = Complex::new(1.0, 0.0);
    let mut B = Complex::new(0.0, 0.0);
    let mut C = Complex::new(0.0, 0.0);
    let two_complex = Complex::new(2.0, 0.0);
    let one_complex = Complex::new(1.0, 0.0);
    series.push((A.clone(), B.clone(), C.clone()));

    for n in 0..max_iter {
        let aa = (&c.0 * &c.0) - (&c.1 * &c.1);
        let bb = &two * &c.0 * &c.1;
        c.0 = aa + &ac;
        c.1 = bb + &bc;

        //series logic
        let temp_a = A.clone();
        let temp_b = B.clone();
        let temp_c = C.clone();
        let x_n = Complex::new(
            c.0.to_string().parse::<f64>().unwrap(),
            c.1.to_string().parse::<f64>().unwrap(),
        );

        Xn.push(x_n.clone());

        A = &two_complex * &x_n * &temp_a + &one_complex;
        B = &two_complex * &x_n * &temp_b + temp_a.square();
        C = &two_complex * &x_n * temp_c + &two_complex * &temp_a * &temp_b;
        series.push((A.clone(), B.clone(), C.clone()));

        if n >= 0 && n < 10 {
            print!("a: {}, b: {}\n", &c.0, &c.1);
            //print magnitude
        }

        if &c.0 * &c.0 + &c.1 * &c.1 > four {
            // print!("Iterations high res: {}\n", n);
            // print!("c: {}, {}\n", &c.0, &c.1);
            return (series, Xn);
        }
    }
    return (series, Xn);
}

//low precision function
pub fn low_precision_series(mut c: (f64, f64)) -> (Vec<(Complex, Complex, Complex)>, Vec<Complex>) {
    let max_iter = 1000;
    //first we compute the high precision series for the given c0
    let ac = c.0.clone();
    let bc = c.1.clone();
    let four: f64 = 4.0;
    let two: f64 = 2.0;
    let mut series: Vec<(Complex, Complex, Complex)> = Vec::new();
    let mut Xn: Vec<Complex> = Vec::new();

    Xn.push(Complex::new(c.0, c.1));
    // print!("Xn: {}\n", Xn[0]);
    let mut A = Complex::new(1.0, 0.0);
    let mut B = Complex::new(0.0, 0.0);
    let mut C = Complex::new(0.0, 0.0);
    let two_complex = Complex::new(2.0, 0.0);
    let one_complex = Complex::new(1.0, 0.0);
    series.push((A.clone(), B.clone(), C.clone()));

    for n in 0..max_iter {
        let aa = (c.0 * c.0) - (c.1 * c.1);
        let bb = two * c.0 * c.1;
        c.0 = aa + ac;
        c.1 = bb + bc;

        let temp_a = A.clone();
        let temp_b = B.clone();
        let temp_c = C.clone();
        let x_n = Complex::new(c.0, c.1);

        Xn.push(x_n.clone());

        A = &two_complex * &x_n * &temp_a + &one_complex;
        B = &two_complex * &x_n * &temp_b + &temp_a.square();
        C = &two_complex * &x_n * &temp_c + &two_complex * temp_a * &temp_b;
        series.push((A.clone(), B.clone(), C.clone()));

        // if n >= 0 && n < 10 {
        //     print!("a: {}, b: {}\n", c.0, c.1);
        //     //print magnitude
        // }
        if c.0 * c.0 + c.1 * c.1 > four {
            // print!("Iterations low res: {}\n", n);
            // print!("c: {}, {}\n", c.0, c.1);
            return (series, Xn);
        }
    }
    return (series, Xn);
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

    ////print the first 10 pixels
    // for i in 0..10 {
    //     print!("Pixel: {}\n", pixesl[i]);
    // }
    // return FrameValues::new(Vec::new(), 0.0, 0.0);

    let mut high_res_found = false;
    let mut pixesl_clone = pixesl.clone();
    let mut loops = 0;
    let mut high_res = (Vec::new(), Vec::new());
    //let time = std::time::Instant::now();
    let mut max = 0;
    let mut max_res: (Vec<(Complex, Complex, Complex)>, Vec<Complex>) = (Vec::new(), Vec::new());
    while !high_res_found {
        let random_index = rand::random::<usize>() % pixesl_clone.len();
        let y0 = Complex::new(
            pixesl_clone[random_index].real,
            pixesl_clone[random_index].imag,
        );
        //remove the random index from the list
        pixesl_clone.remove(random_index);
        high_res = high_precision_series((float_to_dbig(y0.real), float_to_dbig(y0.imag)));
        let n = high_res.0.len();

        if high_res.0.len() > max as usize {
            max = n;
            max_res = high_res.clone();
            print!("Max: {}\n", max);
            //need to remove later;
            // if high_res.0.len()>=3{
            //     break;
            // }
        }

        if n >= max_iter as usize {
            high_res_found = true;
            break;
        }
        loops += 1;
        if loops == pixesl.len() {
            //throw an error
            high_res = max_res.clone();
            break;
            // panic!("Could not find a high precision series that could generate the right amount of iterations max: {}", n)
        }
    }

    // println!(
    //     "Time elapsed to get high_res is: {:?}",
    //     time.elapsed()
    // );

    // print!("High res: {:?}\n", high_res.1[0]);
    // print!("High res length: {}\n", high_res.0.len());

    // for i in 0..10 {
    //     print!("High_res: {}\n", high_res.0[i].0);
    // }

    // return FrameValues::new(Vec::new(), 0.0, 0.0);

    // return FrameValues::new(Vec::new(), 0.0, 0.0);

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


fn get_perturbed_pixels2(
    start_x: f64,
    start_y: f64,
    end_x: f64,
    width: f64,
    max_iter: f64,
)-> FrameValues{
    let reference =   Complex::new(-0.745158143673996,  0.12397760041488097);
    let slope = (&end_x - &start_x) / &width;
    let mut arr : Vec<(i32, i32)> = Vec::new();
    for i in 0..width as i32{
        for j in 0..width as i32{
            arr.push((i, j));
        }
    }

    let mut high_res_found = false;
    let mut high_res = (Vec::new(), Vec::new());
    // let mut loops = 0;
    // let mut max = 0;
    // let mut max_res: (Vec<(Complex, Complex, Complex)>, Vec<Complex>) = (Vec::new(), Vec::new());
    while !&high_res_found{
        let random_index = rand::random::<usize>() % arr.len();
        let y0 = Complex::new(
            start_x + (slope * arr[random_index].0 as f64),
            start_y + (slope * arr[random_index].1 as f64),
        );
        high_res = low_precision_series(((y0.real),(y0.imag)));
        break;
        // let n = high_res.0.len();
        // if high_res.0.len() > max as usize{
        //     max = n;
        //     max_res = high_res.clone();
        // }
        // if n >= max_iter as usize{
        //     high_res_found = true;
        //     break;
        // }
        // loops += 1;
        // if loops == arr.len(){
        //     high_res = max_res.clone();
        //     break;
        // }
    }

    let mut res = Vec::new();
    let mut max = 0;
    let mut min = 100000;

    for i in 0..width as usize{
        let y0 = Complex::new(
            start_x + (slope * i as f64),
            start_y + (slope * i as f64),
        );

        let size = high_res.1.len();
        let n = xy_mandelbrot_perturbation(high_res.0.clone(), high_res.1.clone(), y0, size as f64);
        res.push(n as f64);
        if n as f64 > max as f64{
            max = n;
        }
        if n  < min{
            min = n;
        }
    }

    return FrameValues::new(res, min as f64, max as f64);


}   