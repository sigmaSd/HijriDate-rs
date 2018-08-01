// In the name of Allah

//! # HijriDate-rs 0.1.0
//!
//! Convert between hijri and gregorian date.
//!
//! The algorithm used to convert between dates is limited to:
//!
//! ```text     
//! minimum handled hijri year = 1356
//! maximum handled hijri year = 1500
//!
//! minimum handled gregorian year = 1938
//! maximum handled gregorian year = 2076
//! ```
//!
//! ## Usage
//!
//! *convert to gregorian*
//!
//! ```rust
//! extern crate hijri_date;
//! use hijri_date::HijriDate;
//!
//! let hd = HijriDate::from_hijri(1439,11,19);
//! assert_eq!((2018,8,1),(hd.year_gr,hd.month_gr,hd.day_gr));
//! ```
//!
//! *convert to hijri*
//!
//! ```rust
//! extern crate hijri_date;
//! use hijri_date::HijriDate;
//!
//! let hd = HijriDate::from_gr(2000,07,31);
//! assert_eq!((1421,4,29),(hd.year,hd.month,hd.day));
//! ```
//!
//! *hijri day and month name*
//!
//! ```rust
//! extern crate hijri_date;
//! use hijri_date::HijriDate;
//!
//! let hd = HijriDate::from_hijri(1439,11,18);
//! println!("{} {} {}",hd.year,hd.month_name,hd.day_name);
//! ```
//!
//! *compare dates*
//!
//! ```rust
//! extern crate hijri_date;
//! use hijri_date::HijriDate;
//!
//! let hd_1 = HijriDate::from_hijri(1500, 12, 30);
//! let hd_2 = HijriDate::from_hijri(1356, 1, 1);
//! assert!(hd_1 > hd_2);
//! ```
//!
//!  *substract duration from a day*
//!
//! ```rust
//! extern crate hijri_date;
//! use hijri_date::{Duration,HijriDate};
//!
//! let hd_1 = HijriDate::from_hijri(1420, 06, 15);
//! let hd_2 = HijriDate::from_hijri(1420, 05, 29);
//! assert_eq!(hd_1 - Duration::days(16), hd_2);
//! ```
//!
//!  *substract a day from an other to get a duration*
//!
//! ```rust
//! extern crate hijri_date;    
//! use hijri_date::{Duration,HijriDate};
//!
//! let hd_1 = HijriDate::from_hijri(1356, 06, 15);
//! let hd_2 = HijriDate::from_hijri(1356, 06, 7);
//! assert_eq!(hd_1-hd_2,Duration::days(8));
//! ```
//!

mod umalqura;
use umalqura::*;
mod umalqura_array;

#[macro_use]
extern crate lazy_static;

extern crate chrono;
pub use chrono::Duration;
use chrono::{Date, NaiveDate, Utc};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Sub};

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
        .into_iter()
                            //hack to correct letters order ; need to be handled
        .map(|(n, s)| (*n, s.chars().rev().collect()))
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
                            //hack to correct letters order ; need to be handled
        .map(|(e, a)| (e.to_string(), a.chars().rev().collect()))
        .collect();
}

///Main structure.
///  - Contains numeric value of hijri and gregorian dates plus hijri month and day names.
///  - Hijri names dosent have suffix, example (day,month,year,..)
///  - Gregorian names are denoted with `gr` or `en` suffix.
#[derive(Debug, PartialEq)]
pub struct HijriDate {
    //hijri
    pub day: usize,
    pub month: usize,
    pub month_len: usize,
    pub year: usize,
    pub day_name: String,
    pub month_name: String,

    //gregorian
    pub day_gr: usize,
    pub month_gr: usize,
    pub year_gr: usize,
    pub day_name_en: String,
    pub month_name_gr: String,
    // needed to ease trait impl(add,sub,partialeq..)
    date_gr: Date<Utc>,
}

impl Add<Duration> for HijriDate {
    type Output = HijriDate;

    fn add(self, other: Duration) -> HijriDate {
        HijriDate::chrno_to_hijri(self.date_gr + other)
    }
}

impl Sub<Duration> for HijriDate {
    type Output = HijriDate;

    fn sub(self, other: Duration) -> HijriDate {
        HijriDate::chrno_to_hijri(self.date_gr - other)
    }
}

impl Sub<HijriDate> for HijriDate {
    type Output = Duration;

    fn sub(self, other: HijriDate) -> Duration {
        self.date_gr - other.date_gr
    }
}

impl PartialOrd for HijriDate {
    //use chrono to implement cmp
    fn partial_cmp(&self, other: &HijriDate) -> Option<Ordering> {
        Some(self.date_gr.cmp(&other.date_gr))
    }
}

impl HijriDate {
    /// get data from hijri date
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
            date_gr,
        }
    }
    /// get data from gregorian date.
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
            date_gr,
        }
    }
    /// get data from today's date.
    pub fn today() -> Self {
        let today = Utc::today();

        Self::chrno_to_hijri(today)
    }

    //helper method
    fn chrno_to_hijri(date: Date<Utc>) -> Self {
        let (year_gr, month_gr, day_gr): (usize, usize, usize) = (
            date.format("%Y").to_string().parse().unwrap(),
            date.format("%m").to_string().parse().unwrap(),
            date.format("%d").to_string().parse().unwrap(),
        );
        HijriDate::from_gr(year_gr, month_gr, day_gr)
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
