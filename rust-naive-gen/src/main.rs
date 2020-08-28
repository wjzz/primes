fn main() {
    let mut count = 1;
    let mut current = 3;
    let limit = 1_000_000;
    while current <= limit {
        let mut divisor = 3;
        let mut prime = true;
        while prime && divisor * divisor <= current {
            if current % divisor == 0 {
                prime = false;
            }
            divisor += 2;
        }
        if prime {
            count += 1;
        }
        current += 2;
    }

    print!("Count = {}\n", count)
}