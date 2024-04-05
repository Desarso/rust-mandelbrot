use std::str::FromStr;

use dashu_float::DBig;
use dashu_macros::dbig;

pub struct Complex {
    pub real: f64,
    pub imag: f64,
    pub high_prec_real: DBig,
    pub high_prec_imag: DBig,
}

pub trait Add {
    fn add(&self, other: Complex) -> Complex;
}

//clone
pub trait Clone {
    fn clone(&self) -> Complex;
}

pub trait Sub {
    fn sub(&self, other: Complex) -> Complex;
}

pub trait Mul {
    fn mul(&self, other: &Complex) -> Complex;
}

pub trait Div {
    fn div(&self, other: Complex) -> Complex;
}

pub trait Power {
    fn pow(&self, n: i32) -> Complex;
}

pub trait Magnitude {
    fn magnitude(&self) -> f64;
}

//implement Debug
impl std::fmt::Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Complex {{ real: {}, imag: {} }}", self.real, self.imag)
    }
}

impl Power for Complex {
    fn pow(&self, n: i32) -> Complex {
        let mut result = Complex::new(1.0, 0.0);
        for _ in 0..n {
            result = result.mul(self);
        }
        result
    }
}

impl Clone for Complex {
    fn clone(&self) -> Complex {
        Complex {
            real: self.real,
            imag: self.imag,
            high_prec_real: self.high_prec_real.clone(),
            high_prec_imag: self.high_prec_imag.clone(),
        }
    }
}

//implement std::clone::clone for Complex
impl std::clone::Clone for Complex {
    fn clone(&self) -> Complex {
        Complex {
            real: self.real,
            imag: self.imag,
            high_prec_imag: self.high_prec_imag.clone(),
            high_prec_real: self.high_prec_real.clone(),
        }
    }
}

//std::fmt::Display
impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Complex {{ real: {}, imag: {} }}", self.real, self.imag)
    }
}

impl Mul for Complex {
    fn mul(&self, other: &Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

//implement std::ops::Mul for Complex
impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

//oveload mul to work with references
impl std::ops::Mul<&Complex> for Complex {
    type Output = Complex;
    fn mul(self, other: &Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

//make both of them be references
impl std::ops::Mul<&Complex> for &Complex {
    type Output = Complex;
    fn mul(self, other: &Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

//now only the last one is a reference
impl std::ops::Mul<Complex> for &Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

impl Div for Complex {
    fn div(&self, other: Complex) -> Complex {
        let denom = other.real * other.real + other.imag * other.imag;
        Complex::new(
            (self.real * other.real + self.imag * other.imag) / denom,
            (self.imag * other.real - self.real * other.imag) / denom,
        )
    }
}

//implement std::ops::Div for Complex
impl std::ops::Div for Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        let denom = other.real * other.real + other.imag * other.imag;
        Complex::new(
            (self.real * other.real + self.imag * other.imag) / denom,
            (self.imag * other.real - self.real * other.imag) / denom,
        )
    }
}

//implement std::ops::Div for references
impl std::ops::Div<&Complex> for Complex {
    type Output = Complex;
    fn div(self, other: &Complex) -> Complex {
        let denom = other.real * other.real + other.imag * other.imag;
        Complex::new(
            (self.real * other.real + self.imag * other.imag) / denom,
            (self.imag * other.real - self.real * other.imag) / denom,
        )
    }
}

//implement std::ops::Div for references
impl std::ops::Div<&Complex> for &Complex {
    type Output = Complex;
    fn div(self, other: &Complex) -> Complex {
        let denom = other.real * other.real + other.imag * other.imag;
        Complex::new(
            (self.real * other.real + self.imag * other.imag) / denom,
            (self.imag * other.real - self.real * other.imag) / denom,
        )
    }
}

//implement std::ops::Div for references
impl std::ops::Div<Complex> for &Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        let denom = other.real * other.real + other.imag * other.imag;
        Complex::new(
            (self.real * other.real + self.imag * other.imag) / denom,
            (self.imag * other.real - self.real * other.imag) / denom,
        )
    }
}

impl Add for Complex {
    fn add(&self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

//implement std::ops::Add for Complex
impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

//implement std::ops::Add for references
impl std::ops::Add<&Complex> for Complex {
    type Output = Complex;
    fn add(self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

//implement std::ops::Add for references
impl std::ops::Add<&Complex> for &Complex {
    type Output = Complex;
    fn add(self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

//implement std::ops::Add for references
impl std::ops::Add<Complex> for &Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

impl Sub for Complex {
    fn sub(&self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

//implement std::clone::sub for Complex
impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

//implement std::ops::Sub for  references
impl std::ops::Sub<&Complex> for Complex {
    type Output = Complex;
    fn sub(self, other: &Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

//implement std::ops::Sub for  references
impl std::ops::Sub<&Complex> for &Complex {
    type Output = Complex;
    fn sub(self, other: &Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

//implement std::ops::Sub for  references
impl std::ops::Sub<Complex> for &Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex {
            real,
            imag,
            high_prec_real: dbig!(0),
            high_prec_imag: dbig!(0),
        }
    }

    pub fn new_with_high_precision(real: DBig, imag: DBig, precision: usize) -> Complex {
        Complex {
            real: Complex::dbig_to_float(real.clone()),
            imag: Complex::dbig_to_float(imag.clone()),
            high_prec_real: real.with_precision(precision).unwrap(),
            high_prec_imag: imag.with_precision(precision).unwrap(),
        }
    }

    pub fn high_prec_add(&self, other: Complex, precision: usize) -> Complex {
        Complex::new_with_high_precision(
            &self.high_prec_real + &other.high_prec_real,
            &self.high_prec_imag + &other.high_prec_imag,
            precision,
        )
    }

    pub fn high_prec_sub(&self, other: Complex, precision: usize) -> Complex {
        Complex::new_with_high_precision(
            &self.high_prec_real - &other.high_prec_real,
            &self.high_prec_imag - &other.high_prec_imag,
            precision,
        )
    }

    pub fn high_prec_mul(&self, other: &Complex, precision: usize) -> Complex {
        Complex::new_with_high_precision(
            &self.high_prec_real * &other.high_prec_real
                - &self.high_prec_imag * &other.high_prec_imag,
            &self.high_prec_real * &other.high_prec_imag
                + &self.high_prec_imag * &other.high_prec_real,
            precision,
        )
    }

    pub fn high_prec_div(&self, other: Complex, precision: usize) -> Complex {
        let denom = &other.high_prec_real * &other.high_prec_real
            + &other.high_prec_imag * &other.high_prec_imag;
        Complex::new_with_high_precision(
            (&self.high_prec_real * &other.high_prec_real
                + &self.high_prec_imag * &other.high_prec_imag)
                / &denom,
            (&self.high_prec_imag * &other.high_prec_real
                - &self.high_prec_real * &other.high_prec_imag)
                / &denom,
            precision,
        )
    }

    pub fn clone(&self) -> Complex {
        Complex {
            real: self.real,
            imag: self.imag,
            high_prec_real: self.high_prec_real.clone(),
            high_prec_imag: self.high_prec_imag.clone(),
        }
    }

    pub fn square(&self) -> Complex {
        Complex::new(
            self.real * self.real - self.imag * self.imag,
            2.0 * self.real * self.imag,
        )
    }

    //power of a complex number
    pub fn pow(&self, n: i32) -> Complex {
        let mut result = Complex::new(1.0, 0.0);
        for _ in 0..n {
            result = result.mul(self);
        }
        result
    }

    //magnitude of a complex number
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn get_series(&self, n: i32) -> Vec<Complex> {
        let mut series = Vec::new();
        let mut z = self.clone();
        for _ in 0..n {
            series.push(z.clone());
            z = z.square().add(self.clone());
            if z.magnitude() > 2.0 {
                break;
            }
        }
        series
    }

    pub fn high_precision_series(&self, n: i32, precision: usize) -> Vec<Complex> {
        let mut series = Vec::new();
        let mut z = self.clone();
        //add 0 to the series
        let zero = Complex::new(0.0, 0.0);
        series.push(zero);
        for _ in 0..n {
            series.push(z.clone());
            z = z.high_prec_mul(&z, precision).high_prec_add(self.clone(), precision);
            if z.magnitude() > 2.0 {
                break;
            }
        }
        series
    }

    pub fn high_prec_square(&self, precision: usize) -> Complex {
        Complex::new_with_high_precision(
            &self.high_prec_real * &self.high_prec_real
                - &self.high_prec_imag * &self.high_prec_imag,
            dbig!(2) * &self.high_prec_real * &self.high_prec_imag,
            precision,
        )
    }

    pub fn print_high_precision(&self) {
        //print high precision parts
        let real = self.high_prec_real.clone();
        let imag = self.high_prec_imag.clone();
        println!("Real: {}, Imag: {}", real, imag);
    }

    pub fn float_to_dbig(f: f64) -> DBig {
        return DBig::from_str(&f.to_string()).unwrap();
    }

    pub fn dbig_to_float(d: DBig) -> f64 {
        //turn to string
        return d.to_string().parse::<f64>().unwrap();
    }
}

pub fn test_complexes() {
    //assert addition works
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a.add(b);
    assert_eq!(c.real, 4.0);
    assert_eq!(c.imag, 6.0);

    //assert subtraction works
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a.sub(b);
    assert_eq!(c.real, -2.0);
    assert_eq!(c.imag, -2.0);

    //assert multiplication works
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a.mul(&b);
    assert_eq!(c.real, -5.0);
    assert_eq!(c.imag, 10.0);

    //assert division works
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a.div(b);
    assert_eq!(c.real, 0.44);
    assert_eq!(c.imag, 0.08);

    //assert power works
    let a = Complex::new(1.0, 2.0);
    let c = a.pow(2);
    assert_eq!(c.real, -3.0);
    assert_eq!(c.imag, 4.0);

    //assert power of 3 works
    let a = Complex::new(1.0, 2.0);
    let c = a.pow(3);
    assert_eq!(c.real, -11.0);
    assert_eq!(c.imag, -2.0);

    //assert magnitude works
    let a = Complex::new(1.0, 2.0);
    let c = a.magnitude();
    assert_eq!(c, 2.23606797749979);

    //assert square works
    let a = Complex::new(1.0, 2.0);
    let c = a.square();
    assert_eq!(c.real, -3.0);
    assert_eq!(c.imag, 4.0);
}
