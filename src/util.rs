pub fn dec_format(x: i128) -> String {
    fn d(s: String, i: i128) -> String {
        if i < 1000 {
            return format!("{i}");
        }
        return format!("{}.{:03}", d(s, i/1000), i%1000);
    }
    let res = d("".to_string(), x.abs());
    if x < 0 {
        return format!("-{res}")
    }
    return res;
}
