#[derive(Default)]
pub struct LamportClock {
    pub time: u64,
}

impl LamportClock {
    pub fn tick(&mut self) {
        self.time = self.time.saturating_add(1);
    }

    pub fn update(&mut self, other_time: u64) {
        self.time = std::cmp::max(self.time, other_time).saturating_add(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tick() {
        let mut clock = LamportClock::default();

        clock.tick();

        assert_eq!(clock.time, 1);
    }

    #[test]
    fn test_update() {
        let mut clock = LamportClock::default();

        clock.update(10);

        assert_eq!(clock.time, 11);
    }

    #[test]
    fn test_update_equal_time() {
        let mut clock = LamportClock::default();

        clock.update(0);

        assert_eq!(clock.time, 1);
    }
}
