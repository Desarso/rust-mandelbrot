pub struct Complex{
    pub real: f64,
    pub imag: f64
}

pub trait Add{
    fn add(&self, other: Complex) -> Complex;
}

//clone 
pub trait Clone{
    fn clone(&self) -> Complex;
}

pub trait Sub{
    fn sub(&self, other: Complex) -> Complex;
}

pub trait Mul{
    fn mul(&self, other: &Complex) -> Complex;
}

pub trait Div{
    fn div(&self, other: Complex) -> Complex;
}

pub trait Power{
    fn pow(&self, n: i32) -> Complex;
}

pub trait Magnitude{
    fn magnitude(&self) -> f64;
}


//implement Debug
impl std::fmt::Debug for Complex{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Complex {{ real: {}, imag: {} }}", self.real, self.imag)
    }
}

impl Power for Complex{
    fn pow(&self, n: i32) -> Complex{
        let mut result = Complex::new(1.0, 0.0);
        for _ in 0..n{
            result = result.mul(self);
        }
        result
    }
}



impl Sub for Complex{
    fn sub(&self, other: Complex) -> Complex{
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}



//implement std::clone::sub for Complex
impl std::ops::Sub for Complex{
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex{
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}


//implement std::ops::Sub for  references
impl std::ops::Sub<&Complex> for Complex{
    type Output = Complex;
    fn sub(self, other: &Complex) -> Complex{
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

//implement std::ops::Sub for  references
impl std::ops::Sub<&Complex> for &Complex{
    type Output = Complex;
    fn sub(self, other: &Complex) -> Complex{
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

//implement std::ops::Sub for  references
impl std::ops::Sub<Complex> for &Complex{
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex{
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}


impl Clone for Complex{
    fn clone(&self) -> Complex{
        Complex{real: self.real, imag: self.imag}
    }
}

//implement std::clone::clone for Complex
impl std::clone::Clone for Complex{
    fn clone(&self) -> Complex{
        Complex{real: self.real, imag: self.imag}
    }
}

//std::fmt::Display
impl std::fmt::Display for Complex{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{


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
impl std::ops::Mul for Complex{
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex{
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

//oveload mul to work with references
impl std::ops::Mul<&Complex> for Complex{
    type Output = Complex;
    fn mul(self, other: &Complex) -> Complex{
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

//make both of them be references
impl std::ops::Mul<&Complex> for &Complex{
    type Output = Complex;
    fn mul(self, other: &Complex) -> Complex{
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

//now only the last one is a reference
impl std::ops::Mul<Complex> for &Complex{
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex{
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}


impl Div for Complex{
    fn div(&self, other: Complex) -> Complex{
        let denom = other.real * other.real + other.imag * other.imag;
        Complex::new((self.real * other.real + self.imag * other.imag) / denom, (self.imag * other.real - self.real * other.imag) / denom)
    }
}


//implement std::ops::Div for Complex
impl std::ops::Div for Complex{
    type Output = Complex;
    fn div(self, other: Complex) -> Complex{
        let denom = other.real * other.real + other.imag * other.imag;
        Complex::new((self.real * other.real + self.imag * other.imag) / denom, (self.imag * other.real - self.real * other.imag) / denom)
    }
}


impl Add for Complex{
    fn add(&self, other: Complex) -> Complex{
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

//implement std::ops::Add for Complex
impl std::ops::Add for Complex{
    type Output = Complex;
    fn add(self, other: Complex) -> Complex{
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}


//implement std::ops::Add for references
impl std::ops::Add<&Complex> for Complex{
    type Output = Complex;
    fn add(self, other: &Complex) -> Complex{
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

//implement std::ops::Add for references
impl std::ops::Add<&Complex> for &Complex{
    type Output = Complex;
    fn add(self, other: &Complex) -> Complex{
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

//implement std::ops::Add for references
impl std::ops::Add<Complex> for &Complex{
    type Output = Complex;
    fn add(self, other: Complex) -> Complex{
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

impl Complex{
    pub fn new(real: f64, imag: f64) -> Complex{
        Complex{real, imag}
    }

    // pub fn sub(&self, other: Complex) -> Complex{
    //     Complex::new(self.real - other.real, self.imag - other.imag)
    // }

    //multiply two complex numbers
    pub fn mul(&self, other: &Complex) -> Complex{
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }

    pub fn clone(&self) -> Complex{
        Complex{real: self.real, imag: self.imag}
    }

    //divide two complex numbers
    // pub fn div(&self, other: Complex) -> Complex{
    //     let denom = other.real * other.real + other.imag * other.imag;
    //     Complex::new((self.real * other.real + self.imag * other.imag) / denom, (self.imag * other.real - self.real * other.imag) / denom)
    // }


    // pub fn add(&self, other: Complex) -> Complex{
    //     Complex::new(self.real + other.real, self.imag + other.imag)
    // }   

    pub fn square(&self) -> Complex{
        Complex::new(self.real * self.real - self.imag * self.imag, 2.0 * self.real * self.imag)
    }

    //power of a complex number
    pub fn pow(&self, n: i32) -> Complex{
        let mut result = Complex::new(1.0, 0.0);
        for _ in 0..n{
            result = result.mul(self);
        }
        result
    }

    //magnitude of a complex number
    pub fn magnitude(&self) -> f64{
        (self.real * self.real + self.imag * self.imag).sqrt()
    }


}


pub fn test_complexes(){
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

    //assert magnitude works
    let a = Complex::new(1.0, 2.0);
    let c = a.magnitude();
    assert_eq!(c, 2.236068);


}