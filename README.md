# HijriDate-rs

[![Released API docs](https://docs.rs/hijri_date/badge.svg)](https://docs.rs/hijri_date)

Convert between hijri and gregorian date. (with wasm target)

## Limits

The algorithm used has the following limits:

|           | Minimum | Maximum |
|-----------|---------|---------|
| Hijri     | 1356    | 1499    |
| Gregorian | 1938    | 2076    |

## Minimum Rust version
From version `0.4.0` and onwards the MSRV is **1.51**

## Usage

### Conversion

* Hijri → Gregorian

 ```rust
 use hijri_date::HijriDate;

 let hd = HijriDate::from_hijri(1439,11,19);
 assert_eq!((2018,8,1), (hd.year_gr, hd.month_gr, hd.day_gr));
 ```

* Gregorian → Hijri

 ```rust
 use hijri_date::HijriDate;

 let hd = HijriDate::from_gr(2000,07,31);
 assert_eq!((1421,4,29), (hd.year, hd.month, hd.day));
 ```

### Comparison

 ```rust
 use hijri_date::HijriDate;

 let hd_1 = HijriDate::from_hijri(1500, 12, 30);
 let hd_2 = HijriDate::from_hijri(1356, 1, 1);
 assert!(hd_1 > hd_2);
 ```

### Misc

* Subtract duration from a day

 ```rust
 use hijri_date::{Duration,HijriDate};

 let hd_1 = HijriDate::from_hijri(1420, 06, 15);
 let hd_2 = HijriDate::from_hijri(1420, 05, 29);
 assert_eq!(hd_1 - Duration::days(16), hd_2);
 ```

* Subtract a day from another to get a duration

 ```rust
 use hijri_date::{Duration,HijriDate};

 let hd_1 = HijriDate::from_hijri(1356, 06, 15);
 let hd_2 = HijriDate::from_hijri(1356, 06, 7);
 assert_eq!(hd_1 - hd_2, Duration::days(8));
 ```

### Formatting 

 * Hijri day and month name

 ```rust
 use hijri_date::HijriDate;

 let hd = HijriDate::from_hijri(1439,11,18);
 println!("{}", hd.format("%Y %M %D"));
 ```

* Formatting guide

```
hijri

%Y              hijri_year
%m              hijri_month
%d              hijri_day
%D              hijri_day_name
%M              hijri_month_name
%l              hijri_month_len

gregorian

%gY             gregorian_year
%gm             gregorian_month
%gd             gregorian_day
%gD             gregorian_day_name
%gM             gregorian_month_name
```

## Wasm

To compile to Wasm run:

```
 cargo build --release --target wasm32-unknown-unknown
```

Deno bindings are exposed at `deno`

- to test run `deno run deno_mod.ts`
- to build the bindings, compile to wasm then run `wasm-bindgen --target deno $CARGO_TARGET_DIR/wasm32-unknown-unknown/release/hijri_date.wasm --out-dir deno`


## Credits

I translated [Tytkal's Python library](https://github.com/tytkal/python-hijiri-ummalqura) to Rust.

* **Original algorithm author**

  Suhail Alkowaileet

* **Python version author**

  Khalid Al-hussayen

[Chrono](https://github.com/chronotope/chrono)
