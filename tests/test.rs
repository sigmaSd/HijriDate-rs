extern crate hijri_date;
use hijri_date::{Duration, HijriDate};

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
fn add_sub() {
    let hd_1 = HijriDate::from_hijri(1420, 06, 15);
    let hd_2 = HijriDate::from_hijri(1420, 05, 29);
    assert_eq!(hd_1 - Duration::days(16), hd_2);

    let hd_1 = HijriDate::from_gr(2015, 06, 8);
    let hd_2 = HijriDate::from_gr(2015, 06, 20);
    assert_eq!(hd_1 + Duration::days(12), hd_2);
}

#[test]
fn sub_day() {
    let hd_1 = HijriDate::from_hijri(1356, 06, 15);
    let hd_2 = HijriDate::from_hijri(1356, 06, 7);
    assert_eq!(hd_1-hd_2,Duration::days(8));
}

#[test]
fn fmt() {
    let hd_2 = HijriDate::from_hijri(1356, 06, 7);
    assert_eq!("1356-6-7",hd_2.format("%Y-%m-%d") );
}

#[test]
#[should_panic]
fn invalid() {
    //let hd_1 = HijriDate::from_hijri(1301, 06, 15);
    //let hd_1 = HijriDate::from_hijri(1401, 06, 1500);
    //let hd_g = HijriDate::from_gr(2077, 11 ,16);
    let hd_g = HijriDate::from_gr(1935, 11, 16);
}
