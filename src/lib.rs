mod umalqura;
use umalqura::*;
pub mod umalqura_array;

#[macro_use]
extern crate lazy_static;

extern crate chrono;
use chrono::{Date, Utc, NaiveDate};
use std::collections::HashMap;


lazy_static! {
    static ref month_dict: HashMap<usize,String> = [   
                (1,"محرم"),
                (2,"صفر"),
                (3,"ربيع الأول"),
                (4,"ربيع الثاني"),
                (5,"جمادي الأولى"),
                (6,"جمادي الآخرة"),
                (7,"رجب"),
                (8,"شعبان"),
                (9,"رمضان"),
                (10,"شوال"),
                (11,"ذو القعدة"),
                (12,"ذو الحجة")].iter().cloned()
                                .map(|(n,s)|(n,s.to_string()))
                                .collect();
}

lazy_static! {
    static ref day_dict: HashMap<String,String> = 
                [("Saturday","السبت"),
                ("Sunday","الاحد"),
                ("Monday","الاثنين"),
                ("Tuesday","الثلاثاء"),
                ("Wednesday","الاربعاء"),
                ("Thursday","الخميس"),
                ("Friday","الجمعة")].iter().cloned()
                                    .map(|(e,a)|(e.to_string(),a.to_string()))
                                    .collect();
}


#[derive(Debug)]
struct HijriDate {
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

impl HijriDate {
    pub fn from_hijri(year:usize, month:usize, day:usize) -> Self {
        
        let month_name = month_dict[&month].clone();
        let (year_gr, month_gr, day_gr) = hijri_to_gregorian(year, month, day);
        let date_gr = format!("{}-{}-{}",year_gr,month_gr,day_gr);
        let date_gr = if let Ok(date_gr) = NaiveDate::parse_from_str(&date_gr, "%Y-%m-%d") {
            Date::<Utc>::from_utc(date_gr,Utc)
        } else {
            panic!("Wrong gegorean date foramt")
        };
        let day_name_en = date_gr.format("%A").to_string();
        let day_name = day_dict[&day_name_en].clone();
        let month_name_gr = date_gr.format("%B").to_string();
        let (_,_,_,month_len) = gegorean_to_hijri(year_gr as usize, month_gr as usize , day_gr as usize);
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
    pub fn from_gr(year_gr:usize, month_gr:usize, day_gr:usize) -> Self {
        
        let date_gr = format!("{}-{}-{}",year_gr,month_gr,day_gr);
        let date_gr = if let Ok(date_gr) = NaiveDate::parse_from_str(&date_gr, "%Y-%m-%d") {
            Date::<Utc>::from_utc(date_gr,Utc)
        } else {
            panic!("Wrong gegorean date foramt")
        };
        
        let (year, month, day, month_len) = gegorean_to_hijri(year_gr, month_gr, day_gr);
        let month_name = month_dict[&month].clone();
        
        let day_name_en = date_gr.format("%A").to_string();
        println!("{}",&day_name_en);
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
        let (year_gr,month_gr,day_gr) : (usize,usize,usize) = 
                                (today.format("%Y")
                                    .to_string().parse().unwrap(),
                                today.format("%m")
                                    .to_string().parse().unwrap(),
                                today.format("%d")
                                    .to_string().parse().unwrap());
        println!("{}-{}-{}",year_gr,month_gr,day_gr);
        Self::from_gr(year_gr, month_gr, day_gr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates() {
        let hd_g = HijriDate::from_gr(2000, 07, 31);
        let date =format!("{}-{}-{}",hd_g.year,hd_g.month,hd_g.day);
        assert_eq!("1421-4-29",date);

        let hd  = HijriDate::from_hijri(1400, 11, 19);
        let date =format!("{}-{}-{}",hd.year_gr ,hd.month_gr,hd.day_gr);
        assert_eq!("1980-9-28",date);
    }
    #[test]
    fn today() {
        let hd = HijriDate::today();
        println!("{:?}",hd);
    }
}
