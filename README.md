 # HijriDate-rs 0.1.3

[Documentation](https://docs.rs/hijri_date/0.1.1/hijri_date/)

Convert between hijri and gregorian date.

 The algorithm used to convert between dates is limited to:

 ```text     
 minimum handled hijri year = 1356
 maximum handled hijri year = 1500

 minimum handled gregorian year = 1938
 maximum handled gregorian year = 2076
 ```

 ## Usage

 *convert to gregorian*

 ```rust
 extern crate hijri_date;
 use hijri_date::HijriDate;

 let hd = HijriDate::from_hijri(1439,11,19);
 assert_eq!((2018,8,1),(hd.year_gr,hd.month_gr,hd.day_gr));
 ```

 *convert to hijri*

 ```rust
 extern crate hijri_date;
 use hijri_date::HijriDate;

 let hd = HijriDate::from_gr(2000,07,31);
 assert_eq!((1421,4,29),(hd.year,hd.month,hd.day));
 ```

 *hijri day and month name*

 ```rust
 extern crate hijri_date;
 use hijri_date::HijriDate;

 let hd = HijriDate::from_hijri(1439,11,18);
 println!("{}",hd.format("%Y %M %D"));
 ```

 *compare dates*

 ```rust
 extern crate hijri_date;
 use hijri_date::HijriDate;

 let hd_1 = HijriDate::from_hijri(1500, 12, 30);
 let hd_2 = HijriDate::from_hijri(1356, 1, 1);
 assert!(hd_1 > hd_2);
 ```

  *substract duration from a day*

 ```rust
 extern crate hijri_date;
 use hijri_date::{Duration,HijriDate};

 let hd_1 = HijriDate::from_hijri(1420, 06, 15);
 let hd_2 = HijriDate::from_hijri(1420, 05, 29);
 assert_eq!(hd_1 - Duration::days(16), hd_2);
 ```

  *substract a day from an other to get a duration*

 ```rust
 extern crate hijri_date;    
 use hijri_date::{Duration,HijriDate};

 let hd_1 = HijriDate::from_hijri(1356, 06, 15);
 let hd_2 = HijriDate::from_hijri(1356, 06, 7);
 assert_eq!(hd_1-hd_2,Duration::days(8));
 ```


# Credits
I translated the python version https://github.com/tytkal/python-hijiri-ummalqura to rust.

**Original algorithm author**

Suhail Alkowaileet 

**Python version author**

Khalid Al-hussayen
