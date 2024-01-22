use crc::{Crc, Digest};

pub type CheckSum = u64;

pub struct CheckSummer {
    pub context: Digest<'static, CheckSum>,
}

// 64-bit CRC has a 1% chance of collision in a sample size
// of 600.000.000, which should be sufficient for this purpose.
const CRC64: Crc<u64> = Crc::<u64>::new(&crc::CRC_64_REDIS);

impl CheckSummer {
    pub fn new() -> CheckSummer {
        return CheckSummer {
            context: CRC64.digest(),
        };
    }

    pub fn consume(&mut self, bs: &[u8]) -> &mut Self {
        self.context.update(bs);
        return self;
    }

    pub fn finalize(&mut self) -> CheckSum {
        return std::mem::replace(&mut self.context, CRC64.digest()).finalize();
    }
}
