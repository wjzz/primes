pub fn format_time(milis: u128) -> String {
    let mut v = milis;

    let mil = v % 1000;
    v /= 1000;
    let sec = v % 60;
    v /= 60;
    let min = v % 60;
    v /= 60;
    let hours = v % 24;
    v /= 24;
    let days = v;

    format!("{}d {:02}:{:02}:{:02}.{:03}", days, hours, min, sec, mil)
}

#[cfg(test)]
mod tests {
    use super::format_time;

    #[test]
    fn test_secs() {
        assert_eq!(format_time(1000), "0d 00:00:01.000");
    }

    #[test]
    fn test_secs2() {
        assert_eq!(format_time(25500), "0d 00:00:25.500");
    }

    #[test]
    fn test_mins() {
        assert_eq!(format_time(60 * 1000), "0d 00:01:00.000");
    }

    #[test]
    fn test_mins2() {
        assert_eq!(format_time(25 * 6000), "0d 00:02:30.000");
    }

    #[test]
    fn test_days2() {
        assert_eq!(format_time(36 * 3600 * 1000), "1d 12:00:00.000");
    }
}
