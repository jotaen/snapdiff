use crate::file::SizeBytes;

const DECIMAL_SEPARATOR: &str = ",";
const THOUSANDS_SEPARATOR: &str = ".";

pub mod term {
    pub const ___: &str = "";
    pub const RST: &str = "\x1b[0m"; // Reset
    pub const BLD: &str = "\x1b[1m"; // Bold
    pub const LGT: &str = "\x1b[38;5;253m"; // Light
    pub const GRY: &str = "\x1b[38;5;246m"; // Gray
    pub const BLU: &str = "\x1b[38;5;039m"; // Blue
    pub const GRN: &str = "\x1b[38;5;082m"; // Green
    pub const RED: &str = "\x1b[38;5;202m"; // Red
    pub const YLW: &str = "\x1b[38;5;011m"; // Yellow
}

pub fn dec(x: i128) -> String {
    fn d(s: String, i: i128) -> String {
        if i < 1000 {
            return format!("{i}");
        }
        return format!("{}{}{:03}", d(s, i/1000), DECIMAL_SEPARATOR, i%1000);
    }
    let res = d("".to_string(), x.abs());
    if x < 0 {
        return format!("-{res}")
    }
    return res;
}

pub fn size_human(x: SizeBytes) -> String {
    let mut mantissa = x;
    let mut decimal = 0;
    let mut suffix = " B";
    for s in vec![" K", " M", " G", " T", " P"] {
        if mantissa < 1000 {
            break;
        }
        decimal = (mantissa/100) % 10;
        mantissa = mantissa / 1000;
        suffix = s;
    }
    let mut decimal_suffix = "".to_string();
    if x >= 1000 {
        decimal_suffix = format!("{}{}", THOUSANDS_SEPARATOR, decimal)
    }
    return format!(
        "{}{}{}",
        dec(mantissa as i128),
        decimal_suffix,
        suffix,
    );
}

pub fn duration_human(seconds: u64) -> String {
    if seconds == 0 {
        return "0s".to_string();
    }
    let s = format!("{}s", seconds%60);
    let m = if seconds >= 60 {
        format!("{}m ", seconds/60%60)
    } else {
        "".to_string()
    };
    let h = if seconds >= 60*60 {
        format!("{}h ", seconds/60/60)
    } else {
        "".to_string()
    };
    return format!("{}{}{}", h, m, s);
}
#[cfg(test)]
mod tests {
    use crate::format;

    #[test]
    fn dec() {
        assert_eq!(format::dec(-123_456_789), "-123,456,789");
        assert_eq!(format::dec(-12_345), "-12,345");
        assert_eq!(format::dec(-1), "-1");
        assert_eq!(format::dec(0), "0");
        assert_eq!(format::dec(1), "1");
        assert_eq!(format::dec(543), "543");
        assert_eq!(format::dec(987), "987");
        assert_eq!(format::dec(1_234), "1,234");
        assert_eq!(format::dec(9_876), "9,876");
        assert_eq!(format::dec(12_345), "12,345");
        assert_eq!(format::dec(98_765), "98,765");
        assert_eq!(format::dec(123_456_789), "123,456,789");
    }

    #[test]
    fn dec_human() {
        assert_eq!(format::size_human(0), "0 B");
        assert_eq!(format::size_human(1), "1 B");
        assert_eq!(format::size_human(543), "543 B");
        assert_eq!(format::size_human(987), "987 B");
        assert_eq!(format::size_human(1_234), "1.2 K");
        assert_eq!(format::size_human(9_999), "9.9 K");
        assert_eq!(format::size_human(34_567), "34.5 K");
        assert_eq!(format::size_human(123_456), "123.4 K");
        assert_eq!(format::size_human(999_999), "999.9 K");
        assert_eq!(format::size_human(1_234_567), "1.2 M");
        assert_eq!(format::size_human(1_234_567_890), "1.2 G");
    }

    #[test]
    fn duration_human() {
        assert_eq!(format::duration_human(0), "0s");
        assert_eq!(format::duration_human(59), "59s");
        assert_eq!(format::duration_human(60), "1m 0s");
        assert_eq!(format::duration_human(61), "1m 1s");
        assert_eq!(format::duration_human(34*60), "34m 0s");
        assert_eq!(format::duration_human(34*60+12), "34m 12s");
        assert_eq!(format::duration_human(59*60+59), "59m 59s");
        assert_eq!(format::duration_human(60*60), "1h 0m 0s");
        assert_eq!(format::duration_human(1*60*60+1), "1h 0m 1s");
        assert_eq!(format::duration_human(1*60*60+44*60), "1h 44m 0s");
        assert_eq!(format::duration_human(7*60*60+32*60+54), "7h 32m 54s");
    }
}