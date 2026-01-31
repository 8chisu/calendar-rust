use std::convert::From;

trait Calendarize {}
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarDate {
    year: CalendarYear,
    month: CalendarMonth,
    day: CalendarDay,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarYear(u16);

/// CalendarMonth型は英語での月の名前ではなく、数字で月を表すもの
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarMonth {
    num: u8,
    name: MonthName,
}

/// CalendarDay型は曜日ではなく、数字の日付を表すもの
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarDay {
    num: u16,
    name: WeekDayName,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MonthName {
    #[default]
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

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeekDayName {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    #[default]
    Saturday,
}


impl CalendarDate {
    pub fn new() -> Self {
        CalendarDate {
            year: CalendarYear::default(),
            month: CalendarMonth::default(),
            day: CalendarDay::default(),
        }
    }
    
    pub fn year(&mut self, year: CalendarYear) -> &mut Self {
        self.year = year;
        self
    }
    
    pub fn month(&mut self, month: CalendarMonth) -> &mut Self {
        self.month = month;
        self
    }
    
    pub fn day(&mut self, day: CalendarDay) -> &mut Self {
        self.day = day;
        self
    }
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

fn main() {
    let mut calender= CalendarDate::new();
    let year = CalendarYear(2002);
    let month = CalendarMonth{num: 8, name: MonthName::October};
    let day = CalendarDay{num: 10, name: WeekDayName::Sunday};
    
    let date = calender
        .year(year)
        .month(month)
        .day(day);
    println!("{:?}", date);
}

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

        assert_eq!(month_name, MonthName::April);

        let month_num: u8 = month_name.into();
        assert_eq!(month_num, 4);
    }
}
