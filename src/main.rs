use std::convert::From;

trait Calendarize {}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarDate {
    year: CalendarYear,
    month: CalendarMonth,
    day: CalendarDay,
}

impl CalendarDate {}
/// CalendarYear型は数字で年を表すもの
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CalendarYear(u16);

/// CalendarMonth型は英語での月の名前ではなく、数字で月を表すもの
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CalendarMonth {
    num: u8,
    name: MonthName,
}

/// CalendarDay型は曜日ではなく、数字の日付を表すもの
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CalendarDay {
    num: u16,
    name: WeekDayName,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MonthName {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeekDayName {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl From<CalendarMonth> for u8 {
    fn from(month: CalendarMonth) -> Self {
        month.num
    }
}

impl From<CalendarMonth> for MonthName {
    fn from(month: CalendarMonth) -> Self {
        month.name
    }
}

impl CalendarMonth {
    fn new(month_name: MonthName) -> Self {
        CalendarMonth {
            num: u8::from(month_name),
            name: month_name,
        }
    }
}

impl From<MonthName> for u8 {
    fn from(m: MonthName) -> Self {
        match m {
            MonthName::January => 1,
            MonthName::February => 2,
            MonthName::March => 3,
            MonthName::April => 4,
            MonthName::May => 5,
            MonthName::June => 6,
            MonthName::July => 7,
            MonthName::August => 8,
            MonthName::September => 9,
            MonthName::October => 10,
            MonthName::November => 11,
            MonthName::December => 12,
        }
    }
}

fn main() {}

/// printのために'''cargo test -- --nocapture'''でテストを開始してください。
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn newtypeから月名に変換する() {
        const JANUARY: MonthName = MonthName::January;
        let calender_month: CalendarMonth = CalendarMonth::new(JANUARY);

        let month_name: MonthName = calender_month.into();
        assert_eq!(month_name, JANUARY);

        let month_num: u8 = month_name.into();
        assert_eq!(month_num, 1);

        let month: CalendarMonth = CalendarMonth::new(MonthName::April);
        let month_name: MonthName = month.into();
        assert_eq!(month_name, MonthName::April);

        let month_num: u8 = month_name.into();
        assert_eq!(month_num, 4);
    }
}
