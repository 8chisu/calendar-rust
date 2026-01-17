/// CalendarYear型は数字で年を表すもの
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CalendarYear(u16);

/// CalendarMonth型は英語での月の名前ではなく、数字で月を表すもの
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CalendarMonth(u8);

/// CalendarDay型は曜日ではなく、数字の日付を表すもの
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CalendarDay(u16);

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// enum MonthName {
//     January,
//     February,
//     March,
//     April,
//     May,
//     June,
//     July,
//     August,
//     September,
//     October,
//     November,
//     December,
//     InvalidMonth,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// enum WeekDayName {
//     Sunday,
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Saturday,
//     InvalidDay,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarDate {
    year: CalendarYear,
    month: CalendarMonth,
    day: CalendarDay,
    // day_of_week: Days,
}


fn main() {
}

/// printのために'''cargo test -- --nocapture'''でテストを開始してください。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 数字からnewtype型に変換する() {
        println!("Year型：{:?}", CalendarYear(2026));
        println!("Month型：{:?}", CalendarMonth(1));
        println!("Day型：{:?}", CalendarDay(17));
    }
}
