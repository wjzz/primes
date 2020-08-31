use std::env;

fn strip_characters(original: &str, to_strip: &str) -> String {
    original
        .chars()
        .filter(|&c| !to_strip.contains(c))
        .collect()
}

fn get_and_parse(arg: Option<&String>, default: u32) -> u32 {

    match arg {
        Some(s) => strip_characters(s, "_").parse().unwrap_or(default),
        None => default
    }
}

pub fn parse_cmd_args() -> (u32, u32) {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let lower_bound = get_and_parse(args.get(1), 2);
        let upper_bound = get_and_parse(args.get(2), 1000);
        (lower_bound, upper_bound)
    } else {
        let lower_bound = 2;
        let upper_bound = get_and_parse(args.get(1), 1000);
        (lower_bound, upper_bound)
    }
}
