pub mod solution
{
    use std::ops::{Add, AddAssign};
    use std::convert::{TryFrom, TryInto, From, Into};
    use std::cmp::Ordering;
    use std::hash::Hash;

    #[derive(Debug, Clone, Copy, Default, PartialEq)]
    pub struct ComplexNumber
    {
        real: f64,
        imag: f64
    }


    #[derive(Debug)]
    pub enum ComplexNumberError
    {
        NonZeroImagConvertion,
    }

    
    impl Add for ComplexNumber
    {
        type Output = Self;
        /// Performs + between two complex numbers
        fn add(self, rhs: Self) -> Self::Output 
        {
            Self {real: self.real + rhs.real, imag: self.imag + rhs.imag}
        }
    }


    impl Add<f64> for ComplexNumber
    {
        type Output = Self;
        /// Performs + between a complex number and a real number
        fn add(self, rhs: f64) -> Self::Output 
        {
            Self {real: self.real + rhs, imag: self.imag}
        }
    }


    impl Add<&Self> for ComplexNumber
    {
        type Output = Self;
        /// Performs + between two complex numbers
        fn add(self, rhs: &Self) -> Self::Output 
        {
            Self {real: self.real + rhs.real, imag: self.imag + rhs.imag}
        }
    }


    impl AddAssign for ComplexNumber
    {
        /// Performs += between two complex numbers
        fn add_assign(&mut self, rhs: Self) 
        {
            *self = Self {real: self.real + rhs.real, imag: self.imag + rhs.imag};
        }
    }


    impl AddAssign<f64> for ComplexNumber
    {
        /// Performs += between a complex number and a real number
        fn add_assign(&mut self, rhs: f64) 
        {
            *self = Self {real: self.real + rhs, imag: self.imag};
        }
    }


    impl Into<f64> for ComplexNumber
    {
        fn into(self) -> f64 
        {
            if self.imag == 0.0 as f64
            {
                self.real
            }
            else 
            {
                panic!("Cannot convert into real number with imag != 0");
            }
        }
    }


    impl From<f64> for ComplexNumber
    {
        fn from(value: f64) -> Self 
        {
            Self {real: value, imag: 0.0 as f64}
        }
    }


    // impl TryFrom<f64> for ComplexNumber
    // {
    //     type Error = ComplexNumberError;
    
    //     fn try_from(value: f64) -> Result<Self, Self::Error> 
    //     {
    //         Ok(Self { real: value, imag: 0.0 as f64 })
    //     }
    // }


    // impl TryInto<f64> for ComplexNumber
    // {
    //     type Error = ComplexNumberError;
    
    //     fn try_into(self) -> Result<f64, Self::Error> 
    //     {
    //         if self.imag != 0.0 as f64
    //         {
    //             Err(ComplexNumberError::NonZeroImagConvertion)
    //         }
    //         else 
    //         {
    //             Ok(self.real)    
    //         }
    //     }
    // }


    impl PartialOrd for ComplexNumber
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
        {
            let norm_self = self.real * self.real + self.imag * self.imag;
            let norm_oth = other.real * other.real + other.imag * other.imag;

            if norm_self < norm_oth
            {
                Some(Ordering::Less)
            }
            else if norm_self > norm_oth
            {
                Some(Ordering::Greater)
            }
            else 
            {
                Some(Ordering::Equal)    
            }
        }
    }


    impl Ord for ComplexNumber
    {
        fn cmp(&self, other: &Self) -> Ordering 
        {
            let norm_self = self.real * self.real + self.imag * self.imag;
            let norm_oth = other.real * other.real + other.imag * other.imag;

            norm_self.total_cmp(&norm_oth)
        }
    }


    impl Eq for ComplexNumber{}


    impl Hash for ComplexNumber
    {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) 
        {
            self.real.to_bits().hash(state);
            self.imag.to_bits().hash(state);
        }
    }


    impl AsRef<f64> for ComplexNumber
    {
        fn as_ref(&self) -> &f64 
        {
            &self.real
        }
    }


    impl AsMut<f64> for ComplexNumber
    {
        fn as_mut(&mut self) -> &mut f64 
        {
            &mut self.real
        }
    }




    
    impl ComplexNumber
    {
        /// Creates a new complex number from its real and imaginary parts
        pub fn new(real: f64, imag: f64) -> Self
        {
            ComplexNumber { real: real, imag: imag }
        }

        /// Retrieves the real part of the complex number
        pub fn real(&self) -> f64
        {
            self.real
        }

        /// Retrieves the imaginary part of the complex number
        pub fn imag(&self) -> f64
        {
            self.imag
        }

        /// Produces a complex number from a pure real number
        pub fn from_real(real: f64) -> Self
        {
            Self { real: real, imag: 0.0 }
        }

        /// Retrieve the real and imaginary parts as a tuple
        pub fn to_tuple(&self) -> (f64, f64)
        {
            (self.real, self.imag)
        }
    }
}