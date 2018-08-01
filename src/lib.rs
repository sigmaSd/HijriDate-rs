mod umalqura;
use umalqura::*;
pub mod umalqura_array;

#[macro_use]
extern crate lazy_static;

extern crate chrono;
use chrono::{Date, NaiveDate, Utc};
use std::cmp::Ordering;
use std::collections::HashMap;

lazy_static! {
    static ref month_dict: HashMap<usize, String> = [
        (1, "محرم"),
        (2, "صفر"),
        (3, "ربيع الأول"),
        (4, "ربيع الثاني"),
        (5, "جمادي الأولى"),
        (6, "جمادي الآخرة"),
        (7, "رجب"),
        (8, "شعبان"),
        (9, "رمضان"),
        (10, "شوال"),
        (11, "ذو القعدة"),
        (12, "ذو الحجة")
    ]
        .iter()
        .cloned()
        .map(|(n, s)| (n, s.to_string()))
        .collect();
    static ref day_dict: HashMap<String, String> = [
        ("Saturday", "السبت"),
        ("Sunday", "الاحد"),
        ("Monday", "الاثنين"),
        ("Tuesday", "الثلاثاء"),
        ("Wednesday", "الاربعاء"),
        ("Thursday", "الخميس"),
        ("Friday", "الجمعة")
    ]
        .iter()
        .cloned()
        .map(|(e, a)| (e.to_string(), a.to_string()))
        .collect();
}

#[derive(Debug, PartialEq)]
pub struct HijriDate {
    //hijri
    day: usize,
    month: usize,
    month_len: usize,
    year: usize,
    day_name: String,
    month_name: String,

    //gregorian
    day_gr: usize,
    month_gr: usize,
    year_gr: usize,
    day_name_en: String,
    month_name_gr: String,
}
impl PartialOrd for HijriDate {
    //use chrono to implement cmp
    fn partial_cmp(&self, other: &HijriDate) -> Option<Ordering> {
        //self
        let date_gr = format!("{}-{}-{}", self.year_gr, self.month_gr, self.day_gr);
        let date_gr = Date::<Utc>::from_utc(
            NaiveDate::parse_from_str(&date_gr, "%Y-%m-%d").unwrap(),
            Utc,
        );

        //other
        let other_date_gr = format!("{}-{}-{}", other.year_gr, other.month_gr, other.day_gr);
        let other_date_gr = Date::<Utc>::from_utc(
            NaiveDate::parse_from_str(&other_date_gr, "%Y-%m-%d").unwrap(),
            Utc,
        );

        //cmp
        Some(date_gr.cmp(&other_date_gr))
    }
}

impl HijriDate {
    pub fn from_hijri(year: usize, month: usize, day: usize) -> Self {
        valid_hijri_date(year, month, day);

        let month_name = month_dict[&month].clone();
        let (year_gr, month_gr, day_gr) = hijri_to_gregorian(year, month, day);
        let date_gr = format!("{}-{}-{}", year_gr, month_gr, day_gr);
        let date_gr = if let Ok(date_gr) = NaiveDate::parse_from_str(&date_gr, "%Y-%m-%d") {
            Date::<Utc>::from_utc(date_gr, Utc)
        } else {
            panic!("Wrong gegorean date foramt")
        };
        let day_name_en = date_gr.format("%A").to_string();
        let day_name = day_dict[&day_name_en].clone();
        let month_name_gr = date_gr.format("%B").to_string();
        let (_, _, _, month_len) = gegorean_to_hijri(year_gr, month_gr, day_gr);

        Self {
            day,
            month,
            month_len,
            year,
            day_name,
            month_name,

            //gregorian
            day_gr,
            month_gr,
            year_gr,
            day_name_en,
            month_name_gr,
        }
    }
    pub fn from_gr(year_gr: usize, month_gr: usize, day_gr: usize) -> Self {
        valid_greorian_date(year_gr, month_gr, day_gr);

        let date_gr = format!("{}-{}-{}", year_gr, month_gr, day_gr);
        let date_gr = if let Ok(date_gr) = NaiveDate::parse_from_str(&date_gr, "%Y-%m-%d") {
            Date::<Utc>::from_utc(date_gr, Utc)
        } else {
            panic!("Wrong gegorean date foramt")
        };

        let (year, month, day, month_len) = gegorean_to_hijri(year_gr, month_gr, day_gr);
        let month_name = month_dict[&month].clone();

        let day_name_en = date_gr.format("%A").to_string();
        //println!("{}", &day_name_en);
        let day_name = day_dict[&day_name_en].clone();
        let month_name_gr = date_gr.format("%B").to_string();

        Self {
            day,
            month,
            month_len,
            year,
            day_name,
            month_name,

            //gregorian
            day_gr,
            month_gr,
            year_gr,
            day_name_en,
            month_name_gr,
        }
    }

    pub fn today() -> Self {
        let today = Utc::today();
        let (year_gr, month_gr, day_gr): (usize, usize, usize) = (
            today.format("%Y").to_string().parse().unwrap(),
            today.format("%m").to_string().parse().unwrap(),
            today.format("%d").to_string().parse().unwrap(),
        );
        //println!("{}-{}-{}", year_gr, month_gr, day_gr);
        Self::from_gr(year_gr, month_gr, day_gr)
    }
}

fn valid_hijri_date(year: usize, month: usize, day: usize) {
    if month > 12 {
        panic!("enter a valid month, Err m = {}", month);
    }
    if day > 30 {
        panic!("enter a valid day, Err d = {}", day);
    }
    //hack to cmp to max min ; should be replaced by a proper way
    if year < 1356 {
        panic!("minumum handled hijri year is 1356");
    }
    if year > 1500 {
        panic!("maximum handled hijri year is 1500");
    }
}

fn valid_greorian_date(year_gr: usize, month_gr: usize, day_gr: usize) {
    if month_gr > 12 {
        panic!("enter a valid month, Err m = {}", month_gr);
    }
    if day_gr > 31 {
        panic!("enter a valid day, Err d = {}", day_gr);
    }
    //hack to cmp to max min ; should be replaced by a proper way
    if year_gr < 1938 {
        panic!("minumum handled gregorian year is 1938");
    }
    if year_gr > 2076 {
        panic!("maximum handled gregorian year is 2076");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate reqwest;

    #[test]
    fn dates() {
        let hd_g = HijriDate::from_gr(2000, 07, 31);
        let date = format!("{}-{}-{}", hd_g.year, hd_g.month, hd_g.day);
        assert_eq!("1421-4-29", date);

        let hd = HijriDate::from_hijri(1400, 11, 19);
        let date = format!("{}-{}-{}", hd.year_gr, hd.month_gr, hd.day_gr);
        assert_eq!("1980-9-28", date);
    }
    #[test]
    fn max_min() {
        //min value //to be precise 1937,03,14
        let hd_g = HijriDate::from_gr(1938, 01, 01);
        //println!("{}-{}-{}",hd_g.year,hd_g.month,hd_g.day);
        let hd = HijriDate::from_hijri(1356, 1, 1);
        //assert_eq!(hd,hd_g);

        //max value //to be precise 2077,11,16
        let hd_g = HijriDate::from_gr(2076, 12, 31);
        //println!("{:?}",hd_g);
        let hd = HijriDate::from_hijri(1500, 12, 30);
        //assert_eq!(hd,hd_g);
    }

    #[test]
    fn cmp() {
        let hd_1 = HijriDate::from_hijri(1500, 12, 30);
        let hd_2 = HijriDate::from_hijri(1356, 1, 1);
        assert!(hd_1 > hd_2);

        let hd_1 = HijriDate::from_hijri(1420, 06, 15);
        let hd_2 = HijriDate::from_hijri(1410, 12, 01);
        assert!(hd_1 > hd_2);
    }
    #[test]
    #[should_panic]
    fn invalid() {
        //let hd_1 = HijriDate::from_hijri(1301, 06, 15);
        //let hd_1 = HijriDate::from_hijri(1401, 06, 1500);
        //let hd_g = HijriDate::from_gr(2077, 11 ,16);
        let hd_g = HijriDate::from_gr(1935, 11, 16);
    }
    #[test]
    fn today() {}

}
