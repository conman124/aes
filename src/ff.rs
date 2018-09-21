use std::ops;
use std::cmp;

#[derive(Debug, Clone, Copy)]
pub struct FF {
    val: u8
}

impl FF {
    pub fn new(val: u8) -> FF {
        FF{val}
    }

	pub fn value(self) -> u8 {
		self.val
	}

    fn xtime(&self) -> FF {
        let mut val = self.val;
        let do_mod = if val >= 0b10000000 {true} else {false};

        val <<= 1;

        FF::new(if do_mod {val ^ 0x1b} else {val})
    }
}

impl ops::Add for FF {
    type Output = FF;

    fn add(self, rhs: FF) -> FF {
        FF::new(self.val ^ rhs.val)
    }
}

// TODO figure out how to make this work
/*impl<'a> ops::AddAssign for &'a FF {
    fn add_assign(&mut self, rhs: &'a FF) {
        *self = &(*self + rhs);
    }
}*/

impl ops::Mul for FF {
    type Output = FF;

    fn mul(self, rhs: FF) -> FF {
		if self.val == 1 { return FF::new(rhs.val); }
        if rhs.val == 1 { return FF::new(self.val); }

        let mut a = FF::new(self.val);
        let mut res = FF::new(0);
        
        for i in 0..8 {
            if rhs.val & (1 << i) > 0 {
                res = res + a;
            }
            a = a.xtime();
        }

        res
	}
}

impl cmp::PartialEq for FF {
    fn eq(&self, other: &FF) -> bool {
        self.val == other.val
    }
}

impl cmp::Eq for FF {}

#[cfg(test)]
mod tests {
    use ff::FF;

    #[test]
    fn add() {
        assert_eq!(FF::new(0xd4), FF::new(0x57) + FF::new(0x83));
        assert_eq!(FF::new(0xd4), FF::new(0x83) + FF::new(0x57));
    }

    #[test]
    fn xtime() {
        assert_eq!(FF::new(0xae), FF::new(0x57).xtime());
        assert_eq!(FF::new(0x47), FF::new(0xae).xtime());
        assert_eq!(FF::new(0x8e), FF::new(0x47).xtime());
        assert_eq!(FF::new(0x07), FF::new(0x8e).xtime());
    }

    #[test]
    fn mult() {
        assert_eq!(FF::new(0xfe), FF::new(0x57) * FF::new(0x13));
    }
}