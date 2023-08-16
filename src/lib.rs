// In the name of Allah

//! # HijriDate-rs
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
//! use hijri_date::HijriDate;
//!
//! let hd = HijriDate::from_hijri(1439,11,19).unwrap();
//! assert_eq!((2018,8,1),(hd.year_gr(),hd.month_gr(),hd.day_gr()));
//! ```
//!
//! *convert to hijri*
//!
//! ```rust
//! use hijri_date::HijriDate;
//!
//! let hd = HijriDate::from_gr(2000,07,31).unwrap();
//! assert_eq!((1421,4,29),(hd.year(),hd.month(),hd.day()));
//! ```
//!
//! *hijri day and month name*
//!
//! ```rust
//! use hijri_date::HijriDate;
//!
//! let hd = HijriDate::from_hijri(1439,11,18).unwrap();
//! println!("{}",hd.format("%Y %M %D"));
//! ```
//!
//! *compare dates*
//!
//! ```rust
//! use hijri_date::HijriDate;
//!
//! let hd_1 = HijriDate::from_hijri(1400, 12, 30);
//! let hd_2 = HijriDate::from_hijri(1357, 1, 1);
//! assert!(hd_1 > hd_2);
//! ```
//!
//!  *substract duration from a day*
//!
//! ```rust
//! use hijri_date::{Duration,HijriDate};
//!
//! let hd_1 = HijriDate::from_hijri(1420, 06, 15).unwrap();
//! let hd_2 = HijriDate::from_hijri(1420, 05, 29).unwrap();
//! assert_eq!(hd_1 - Duration::days(16), hd_2);
//! ```
//!
//!  *substract a day from an other to get a duration*
//!
//! ```rust
//! use hijri_date::{Duration,HijriDate};
//!
//! let hd_1 = HijriDate::from_hijri(1358, 06, 15).unwrap();
//! let hd_2 = HijriDate::from_hijri(1358, 06, 7).unwrap();
//! assert_eq!(hd_1-hd_2,Duration::days(8));
//! ```
//!

#[macro_use]
mod utils;

use umalqura::*;
mod umalqura;
mod umalqura_array;

pub use chrono::Duration;
use chrono::{NaiveDate, Utc};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use std::cmp::Ordering;
use std::fmt;
use std::ops::Index;
use std::ops::{Add, Sub};

struct Map<T, U, const N: usize>([(T, U); N]);

impl<T: PartialEq, U, const N: usize> Index<T> for Map<T, U, N> {
    type Output = U;

    fn index(&self, t: T) -> &Self::Output {
        &self.0.iter().find(|elem| elem.0 == t).unwrap().1
    }
}

static MONTH_DICT: &Map<usize, &str, 12> = &Map([
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
    (12, "ذو الحجة"),
]);

static DAY_DICT: &Map<&str, &str, 7> = &Map([
    ("Saturday", "السبت"),
    ("Sunday", "الاحد"),
    ("Monday", "الاثنين"),
    ("Tuesday", "الثلاثاء"),
    ("Wednesday", "الاربعاء"),
    ("Thursday", "الخميس"),
    ("Friday", "الجمعة"),
]);
///Main structure.
///  - Contains numeric value of hijri and gregorian dates plus hijri month and day names.
///  - Hijri names dosent have suffix, example (day,month,year,..)
///  - Gregorian names are denoted with `gr` or `en` suffix.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, PartialEq, Eq)]
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
    month_name_en: String,
    // needed to ease trait impl(add,sub,partialeq..)
    date_gr: NaiveDate,
}

impl fmt::Display for HijriDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", &self.format("%Y %M %D"))?;
        writeln!(f, "{}", &self.format("%gD %gM %gY"))?;

        Ok(())
    }
}

impl Add<Duration> for HijriDate {
    type Output = HijriDate;

    fn add(self, other: Duration) -> HijriDate {
        // shouldn't fail
        HijriDate::chrno_to_hijri(self.date_gr + other).unwrap()
    }
}

impl Sub<Duration> for HijriDate {
    type Output = HijriDate;

    fn sub(self, other: Duration) -> HijriDate {
        // shouldn't fail
        HijriDate::chrno_to_hijri(self.date_gr - other).unwrap()
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

// #[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[wasm_bindgen]
impl HijriDate {
    // Define getter methods for various attributes of HijriDate

    /// Get the day of the Hijri month
    pub fn day(&self) -> usize {
        self.day
    }

    /// Get the month of the Hijri year
    pub fn month(&self) -> usize {
        self.month
    }

    /// Get the length of the current Hijri month
    pub fn month_len(&self) -> usize {
        self.month_len
    }

    /// Get the year of the Hijri date
    pub fn year(&self) -> usize {
        self.year
    }

    /// Get the name of the day in the Hijri calendar
    pub fn day_name(&self) -> String {
        self.day_name.clone()
    }

    /// Get the name of the month in the Hijri calendar
    pub fn month_name(&self) -> String {
        self.month_name.clone()
    }

    /// Get the day of the week (Gregorian) corresponding to the Hijri date
    pub fn day_gr(&self) -> usize {
        self.day_gr
    }

    /// Get the month of the year (Gregorian) corresponding to the Hijri date
    pub fn month_gr(&self) -> usize {
        self.month_gr
    }

    /// Get the year (Gregorian) corresponding to the Hijri date
    pub fn year_gr(&self) -> usize {
        self.year_gr
    }

    /// Get the English name of the day in the Hijri calendar
    pub fn day_name_en(&self) -> String {
        self.day_name_en.clone()
    }

    /// Get the English name of the month in the Hijri calendar
    pub fn month_name_en(&self) -> String {
        self.month_name_en.clone()
    }

    // Define a JavaScript-compatible method to convert the HijriDate to a formatted string
    // #[wasm_bindgen(js_name = toString)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name=toString))]
    pub fn wasm_to_string(&self) -> String {
        // Use the format method to create a formatted string representation of the HijriDate
        self.format("%Y %m %d -> %gY %gm %gd")
    }

    /// get data from hijri date
    pub fn from_hijri(year: usize, month: usize, day: usize) -> Result<HijriDate, String> {
        valid_hijri_date(year, month, day)?;
        let month_name = MONTH_DICT[month].to_string();
        let (year_gr, month_gr, day_gr) = hijri_to_gregorian(year, month, day);
        let date_gr = format!("{}-{}-{}", year_gr, month_gr, day_gr);
        let date_gr = if let Ok(date_gr) = NaiveDate::parse_from_str(&date_gr, "%Y-%m-%d") {
            date_gr
        } else {
            bail!("Wrong gegorean date foramt")
        };
        let day_name_en = date_gr.format("%A").to_string();
        let day_name = DAY_DICT[&day_name_en].to_string();
        let month_name_en = date_gr.format("%B").to_string();
        let (_, _, _, month_len) = gegorean_to_hijri(year_gr, month_gr, day_gr);

        Ok(Self {
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
            month_name_en,
            date_gr,
        })
    }
    /// get data from gregorian date.
    pub fn from_gr(year_gr: usize, month_gr: usize, day_gr: usize) -> Result<HijriDate, String> {
        valid_greorian_date(year_gr, month_gr, day_gr)?;
        let date_gr = format!("{}-{}-{}", year_gr, month_gr, day_gr);
        let date_gr = if let Ok(date_gr) = NaiveDate::parse_from_str(&date_gr, "%Y-%m-%d") {
            date_gr
        } else {
            bail!("Wrong gegorean date foramt")
        };

        let (year, month, day, month_len) = gegorean_to_hijri(year_gr, month_gr, day_gr);
        let month_name = MONTH_DICT[month].to_string();

        let day_name_en = date_gr.format("%A").to_string();
        let day_name = DAY_DICT[day_name_en.as_str()].to_string();
        let month_name_en = date_gr.format("%B").to_string();

        Ok(Self {
            //hijri
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
            month_name_en,
            date_gr,
        })
    }
    /// get data from today's date.
    pub fn today() -> Self {
        let today = Utc::now().date_naive();

        // It shouldn't fail
        Self::chrno_to_hijri(today).unwrap()
    }

    //helper method
    fn chrno_to_hijri(date: NaiveDate) -> Result<Self, String> {
        let (year_gr, month_gr, day_gr): (usize, usize, usize) = (
            date.format("%Y")
                .to_string()
                .parse()
                .map_err(|_| "Error parsing date")?,
            date.format("%m")
                .to_string()
                .parse()
                .map_err(|_| "Error parsing date")?,
            date.format("%d")
                .to_string()
                .parse()
                .map_err(|_| "Error parsing date")?,
        );
        HijriDate::from_gr(year_gr, month_gr, day_gr)
    }

    /// Returns a representation of HijriDate defined by the given formatter
    ///
    /// ```text
    ///        hijri
    ///
    ///     %Y              hijri_year
    ///     %m              hijri_month
    ///     %d              hijri_day
    ///     %D              hijri_day_name
    ///     %M              hijri_month_name
    ///     %l              hijri_month_len
    ///
    ///        gregorian
    ///
    ///     %gY             gregorian_year
    ///     %gm             gregorian_month
    ///     %gd             gregorian_day
    ///     %gD             gregorian_day_name
    ///     %gM             gregorian_month_name
    /// ```
    pub fn format(&self, f: &str) -> String {
        f.replace("%Y", &self.year.to_string())
            .replace("%m", &self.month.to_string())
            .replace("%d", &self.day.to_string())
            .replace("%D", &self.day_name)
            .replace("%M", &self.month_name)
            .replace("%l", &self.month_len.to_string())
            .replace("%gY", &self.year_gr.to_string())
            .replace("%gm", &self.month_gr.to_string())
            .replace("%gd", &self.day_gr.to_string())
            .replace("%gD", &self.day_name_en)
            .replace("%gM", &self.month_name_en)
    }
}

fn valid_hijri_date(year: usize, month: usize, day: usize) -> Result<(), String> {
    if month > 12 {
        bail!("enter a valid month, Err m = {}", month);
    }
    if day > 30 {
        bail!("enter a valid day, Err d = {}", day);
    }
    if year < 1357 {
        bail!("minumum handled hijri year is 1357");
    }
    if year > 1499 {
        bail!("maximum handled hijri year is 1499");
    }
    Ok(())
}

fn valid_greorian_date(year_gr: usize, month_gr: usize, day_gr: usize) -> Result<(), String> {
    if month_gr > 12 {
        bail!("enter a valid month, Err m = {}", month_gr);
    }
    if day_gr > 31 {
        bail!("enter a valid day, Err d = {}", day_gr);
    }
    if year_gr < 1938 {
        bail!(
            "minumum handled gregorian year is 1938, input year: {}",
            year_gr
        );
    }
    if year_gr > 2076 {
        bail!("maximum handled gregorian year is 2076");
    }
    Ok(())
}
