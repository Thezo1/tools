use std::time::SystemTime;
use std::u32;

const MATH_N: i32 = 624;
const MATH_M: i32 = 397;
const MATH_MATRIX_A: u64 = 0x9908b0df;
const MATH_UPPER_MASK: u64 = 0x80000000;
const MATH_LOWER_MASK: u64 = 0x7fffffff;
const MATH_TEMPRING_MASK_B: u64 = 0x9d2c5680;
const MATH_TEMPRING_MASK_C: u64 = 0xefc60000;

pub struct Random
{
    rseed: u32,
    rseed_sp: u32,
    mt: [u64;(MATH_N as usize)],
    mti: i32,
}

impl Random
{
    pub fn new() -> Self
    {
        Self
        {
            rseed: 1, 
            rseed_sp: 0,
            mt: [0;MATH_N as usize],
            mti: MATH_N + 1,
        }
    }

    //return number from 0..n excluding n
    pub fn random(&mut self, n: u32) -> u32
    {
        let mut y = 0;
        let mag01 = [0x0, MATH_MATRIX_A];
        let mut kk = 0;
        if n == 0
        {
            return 0;
        }
        if self.mti >= MATH_N
        {
            if self.mti == MATH_N+1
            {
                self.set_random_seed(4357);
            }

            while kk<(MATH_N - MATH_M)
            {
                y = (self.mt[kk as usize]&MATH_UPPER_MASK)|(self.mt[(kk+1) as usize]&MATH_LOWER_MASK);
                self.mt[kk as usize] = self.mt[(kk+(MATH_M - MATH_N)) as usize] ^ mag01[(y & 0x1) as usize];
                kk+=1;
            }
            while kk<(MATH_N-1)
            {
                y = (self.mt[kk as usize]&MATH_UPPER_MASK)|(self.mt[(kk + 1) as usize]&MATH_LOWER_MASK);
                self.mt[kk as usize] = self.mt[(kk+(MATH_M-MATH_N)) as usize]^(y>>1)^mag01[(y&0x1) as usize];
                kk+=1;
            }
            y = (self.mt[(MATH_N-1) as usize]&MATH_UPPER_MASK)|(self.mt[0]&MATH_LOWER_MASK);
            self.mt[(MATH_N-1) as usize] = self.mt[(MATH_M-1) as usize]^(y>>1)^mag01[(y&0x1) as usize];
            self.mti = 0;
        }
        y = self.mt[(self.mti+1) as usize];
        y^= y>>11;
        y^= (y<<7)&MATH_TEMPRING_MASK_B;
        y^= (y<<15)&MATH_TEMPRING_MASK_C;
        y^= y>>18;

        return y as u32%n;
    }

    fn set_random_seed(&mut self, n: u32)
    {
        self.mt[0] = n as u64 & 0xffffffff;
        self.mti = 1;
        while self.mti<MATH_N
        {
            self.mt[self.mti as usize] = (69069 * self.mt[(self.mti-1) as usize]) & 0xffffffff;
            self.mti += 1;
        }

        self.rseed = n;
    }

    pub fn get_random_seed(&self) -> u32
    {
        return self.rseed;
    }

    pub fn randomize(&mut self)
    {
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Something went wrong retrieving system time");

        self.set_random_seed(d.as_secs() as u32);
    }

    pub fn random_float(&mut self) -> f32
    {
        let r = self.random(u32::MAX) as f32;
        let divisor = u32::MAX as f32;

        return r/divisor;
    }
}

