use std::time::Duration;

use rand::{distributions::Standard, thread_rng, Rng};

pub fn get_random_string(len: usize) -> String {
    thread_rng()
        .sample_iter::<char, _>(&Standard)
        .take(len)
        .collect()
}

pub fn num_milliseconds(duration: &Duration) -> u64 {
    let secs_part = duration.as_secs() * 1000;
    let nano_part = duration.subsec_nanos() / 1_000_000;

    secs_part + nano_part as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_string() {
        assert_eq!(get_random_string(32).chars().count(), 32);
    }

    #[test]
    fn test_num_milliseconds() {
        assert_eq!(num_milliseconds(&Duration::from_millis(5010)), 5010);
        assert_eq!(num_milliseconds(&Duration::from_millis(0)), 0);
    }
}
