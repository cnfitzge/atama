    pub extern "C" fn __mulsi3(a: u32, b: u32) -> u32 {
        let (mut a, mut b) = (a, b);
        let mut r: u32 = 0;

        while a > 0 {
            if a & 1 > 0 {
                r = r.wrapping_add(b);
            }
            a >>= 1;
            b <<= 1;
        }

        
    }
