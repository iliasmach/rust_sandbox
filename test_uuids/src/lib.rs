#[cfg(test)]
mod tests {
    use std::time::Instant;
    use fastuuid::Generator;

    #[test]
    fn test_speed_nanoid() {
        let now = Instant::now();
        let count = 100000;
        for i in 0..count {
            let guid = nanoid::rngs::non_secure(21);

        }

        let since = Instant::now().duration_since(now).as_millis();

        println!("{} requests generated in {:?} milliseconds", count, since);
    }

    #[test]
    fn test_speed_uuid() {
        let now = Instant::now();
        let count = 100000;
        for i in 0..count {
            let guid = uuid::Uuid::new_v4();

        }

        let since = Instant::now().duration_since(now).as_millis();

        println!("{} requests generated in {:?} milliseconds", count, since);
    }

    #[test]
    fn test_speed_nano_id() {
        let now = Instant::now();
        let count = 100000;

        for i in 0..count {
            let guid = nano_id::base64(16);

        }

        let since = Instant::now().duration_since(now).as_millis();

        println!("{} requests generated in {:?} milliseconds", count, since);
    }
}
