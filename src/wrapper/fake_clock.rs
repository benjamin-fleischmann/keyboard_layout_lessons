use chrono::Duration;
use chrono::{DateTime, Utc};
use std::cell::RefCell;
use std::ops::Add;

pub struct FakeClock {
    current_time: DateTime<Utc>,
}

thread_local! {
     static FAKE_CLOCK : RefCell<FakeClock> = RefCell::new(FakeClock {
        current_time: Utc::now(),
    })
}
impl FakeClock {
    fn get_current_time(&self) -> DateTime<Utc> {
        self.current_time
    }
    fn set_current_time(&mut self, time: DateTime<Utc>) {
        self.current_time = time;
    }
    fn advance_current_time(&mut self, duration: Duration) {
        self.current_time = self.current_time.add(duration);
    }

    pub fn now() -> DateTime<Utc> {
        FAKE_CLOCK.with(|clock| clock.borrow_mut().get_current_time())
    }

    pub fn set(time: DateTime<Utc>) {
        FAKE_CLOCK.with(|clock| clock.borrow_mut().set_current_time(time))
    }

    pub fn advance(duration: Duration) {
        FAKE_CLOCK.with(|clock| clock.borrow_mut().advance_current_time(duration))
    }
}
#[cfg(test)]
mod test_fake_clock {
    use crate::wrapper::fake_clock::FakeClock;
    use chrono::{DateTime, Duration, Utc};
    use std::ops::Add;
    use std::thread;

    #[test]
    fn fake_clock_does_not_advance_by_itself() {
        let time1 = FakeClock::now();
        thread::sleep(std::time::Duration::from_millis(10));
        let time2 = FakeClock::now();
        assert_eq!(time1, time2)
    }
    #[test]
    fn fake_clock_returns_set_time() {
        let expected_datetime: DateTime<Utc> = DateTime::parse_from_rfc3339("2000-01-01T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        FakeClock::set(expected_datetime);
        assert_eq!(expected_datetime, FakeClock::now())
    }
    #[test]
    fn fake_advances() {
        let time1 = FakeClock::now();
        let expected_duration = Duration::days(1);
        FakeClock::advance(expected_duration);
        assert_eq!(time1.add(expected_duration), FakeClock::now())
    }
}
