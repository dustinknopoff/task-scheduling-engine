use chrono::{DateTime, Datelike, Duration, FixedOffset, Weekday};

fn is_saturday(date: DateTime<FixedOffset>) -> bool {
    date.weekday() == Weekday::Sat
}

fn is_sunday(date: DateTime<FixedOffset>) -> bool {
    date.weekday() == Weekday::Sun
}

pub fn shift_to_first_next_business_day(date: DateTime<FixedOffset>) -> DateTime<FixedOffset> {
    if is_saturday(date) {
        return date.checked_add_signed(Duration::days(2)).unwrap();
    } else if is_sunday(date) {
        return date.checked_add_signed(Duration::days(1)).unwrap();
    }
    date
}

pub fn shift_to_first_prev_business_day(date: DateTime<FixedOffset>) -> DateTime<FixedOffset> {
    if is_saturday(date) {
        return date.checked_sub_signed(Duration::days(1)).unwrap();
    } else if is_sunday(date) {
        return date.checked_sub_signed(Duration::days(2)).unwrap();
    }
    date
}

pub fn add_business_days(date: DateTime<FixedOffset>, duration: usize) -> DateTime<FixedOffset> {
    (1..=duration).fold(date, |acc, _| {
        shift_to_first_next_business_day(acc.checked_add_signed(Duration::days(1)).unwrap())
    })
}

pub fn sub_business_days(date: DateTime<FixedOffset>, duration: usize) -> DateTime<FixedOffset> {
    (1..=duration).fold(date, |acc, _| {
        shift_to_first_prev_business_day(acc.checked_sub_signed(Duration::days(1)).unwrap())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_3_days() {
        let now =
            DateTime::parse_from_str("2020-03-12 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        assert_eq!(
            now.checked_add_signed(Duration::days(5)).unwrap(),
            add_business_days(now, 3)
        )
    }

    #[test]
    fn test_add_3_days_normal() {
        let now =
            DateTime::parse_from_str("2020-03-10 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        assert_eq!(
            now.checked_add_signed(Duration::days(3)).unwrap(),
            add_business_days(now, 3)
        )
    }

    #[test]
    fn test_sub_3_days_normal() {
        let now =
            DateTime::parse_from_str("2020-03-12 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        assert_eq!(
            now.checked_sub_signed(Duration::days(3)).unwrap(),
            sub_business_days(now, 3)
        )
    }

    #[test]
    fn test_sub_3_days() {
        let now =
            DateTime::parse_from_str("2020-03-10 12:29:03.274 +0000", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap();
        assert_eq!(
            now.checked_sub_signed(Duration::days(5)).unwrap(),
            sub_business_days(now, 3)
        )
    }
}
