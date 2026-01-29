#[derive(Default)]
pub struct LamportClock {
    pub time: u64,
}

impl LamportClock {
    pub fn tick(&mut self) {
        self.time = self.time.saturating_add(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ticket() {
        let mut clock = LamportClock::default();

        clock.tick();

        assert_eq!(clock.time, 1);
    }
}
