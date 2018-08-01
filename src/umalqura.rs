use umalqura_array::*;

pub fn gegorean_to_hijri(mut year_gr: usize,mut month_gr: usize, day_gr: usize) -> (usize,usize,usize,usize) {
    //This code the modified version of R.H. van Gent Code, it can be found at http://www.staff.science.uu.nl/~gent0113/islam/ummalqura.htm

    //append January and February to the previous year (i.e. regard March as
    // the first month of the year in order to simplify leapday corrections)

    if month_gr < 3 {
        year_gr -= 1;
        month_gr += 12;
    }

    //determine offset between Julian and Gregorian calendar
    let a = (year_gr/100) as f64;
    let jgc = a - a/4.0 -2.0;
    let (y,m,d) = (year_gr as f64, month_gr as f64, day_gr as f64) ;
    //compute Chronological Julian Day Number (CJDN)
    let cjdn = (365.25 * (y + 4716.0)).floor()
                + (30.6001 * (m + 1.0)).floor()
                + d - jgc -1524.0;

    // compute Modified Chronological Julian Day Number (MCJDN)
    let mcjdn = cjdn - 2_400_000.0;
    
    let index = umalqura_index(mcjdn);

    //compute and output the Umm al-Qura calendar date
    let iln = index + 16260;
    let ii = (iln - 1) / 12;
    let iy = ii + 1;
    let im = iln - 12 * ii;
    let id = mcjdn - UMALQURA_DAT[index - 1] as f64  + 1.0;
    let ml = UMALQURA_DAT[index] - UMALQURA_DAT[index -1];

    (iy, im, id as usize, ml)
}

pub fn hijri_to_gregorian(year:usize, month:usize, day:usize) -> (usize,usize,usize) {
    
    let ii = year -1;
    let iln = (ii*12) + 1 + (month - 1);
    let i :usize = iln - 16260;
    let mcjdn = day + UMALQURA_DAT[i - 1] - 1;
    let cjdn = mcjdn +  2_400_000;

    julian_to_gregorian(cjdn as f64)
}

fn julian_to_gregorian(cjdn: f64) -> (usize,usize,usize) {
    //source from: http://keith-wood.name/calendars.html

    let z = (cjdn + 0.5).floor();
    let a = ((z - 1_867_216.25) / 36524.25).floor();
    let a = z + 1.0 + a - (a / 4.0).floor();
    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d) / 30.6001).floor();
    let day = b - d - (e * 30.6001).floor();

    let month = if e > 13.5 {
        e - 13.0
    } else {
        e - 1.0
    };

    let mut year = if month > 2.5 {
        c - 4716.0
    } else {
        c - 4715.0
    };

    if year <= 0.0 {
        year -= 1.0;
    }

    (year as usize ,month as usize ,day as usize)

}