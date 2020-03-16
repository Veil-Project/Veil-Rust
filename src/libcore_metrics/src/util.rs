// Copyright 2020 Veil Rust Developers
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

pub trait VecMathSigned {
    fn largest(&self) -> i64;
    fn smallest(&self) -> i64;
    fn average(&self) -> f64;
    fn median(&self) -> f64;
}

pub trait VecMathUnsigned {
    fn largest(&self) -> u64;
    fn smallest(&self) -> u64;
    fn average(&self) -> f64;
    fn median(&self) -> f64;
}

impl<'a, T> VecMathSigned for Vec<T>
where
    T: Into<i64> + Ord + Copy + 'a,
{
    fn largest(&self) -> i64 {
        let mut largest = &self[0];

        for num in self.iter() {
            if num > largest {
                largest = num;
            }
        }

        Into::<i64>::into(*largest)
    }

    fn smallest(&self) -> i64 {
        let mut smallest = &self[0];

        for n in self.iter() {
            if n < smallest {
                smallest = n;
            }
        }

        Into::<i64>::into(*smallest)
    }

    fn average(&self) -> f64 {
        let mut sum: i64 = 0;
        let iter = self.iter();
        let count = iter.len();
        for num in iter {
            sum += Into::<i64>::into(*num);
        }
        sum as f64 / count as f64
    }

    fn median(&self) -> f64 {
        let mut v = self.clone();
        let n = self.len() + 1;
        let mid = n / 2;

        v.sort();
        let mid_val = Into::<i64>::into(v[mid]);
        let mid_val2 = Into::<i64>::into(v[mid + 1]);

        if self.len() % 2 == 0 {
            mid_val as f64
        } else {
            let mid_sum = mid_val + mid_val2;
            mid_sum as f64 / 2f64
        }
    }
}

impl<'a, T> VecMathUnsigned for Vec<T>
where
    T: Into<u64> + Ord + Copy + 'a,
{
    fn largest(&self) -> u64 {
        let mut largest = &self[0];

        for n in self.iter() {
            if n > largest {
                largest = n;
            }
        }

        Into::<u64>::into(*largest)
    }

    fn smallest(&self) -> u64 {
        let mut smallest = &self[0];

        for n in self.iter() {
            if n < smallest {
                smallest = n;
            }
        }

        Into::<u64>::into(*smallest)
    }

    fn average(&self) -> f64 {
        let mut sum: u64 = 0;
        let iter = self.into_iter();
        let count = iter.len();
        for num in iter {
            sum += Into::<u64>::into(*num);
        }
        sum as f64 / count as f64
    }

    fn median(&self) -> f64 {
        let mut v = self.clone();
        let n = self.len() + 1;
        let mid = n / 2;

        v.sort();
        let mid_val = Into::<u64>::into(self[mid]);
        let mid_val2 = Into::<u64>::into(self[mid + 1]);

        if self.len() % 2 == 0 {
            mid_val as f64
        } else {
            let mid_sum = mid_val + mid_val2;
            mid_sum as f64 / 2f64
        }
    }
}
