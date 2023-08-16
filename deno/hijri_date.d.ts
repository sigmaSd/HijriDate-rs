/* tslint:disable */
/* eslint-disable */
/**
*Main structure.
*  - Contains numeric value of hijri and gregorian dates plus hijri month and day names.
*  - Hijri names dosent have suffix, example (day,month,year,..)
*  - Gregorian names are denoted with `gr` or `en` suffix.
*/
export class HijriDate {
  free(): void;
/**
* Get the day of the Hijri month
* @returns {number}
*/
  day(): number;
/**
* Get the month of the Hijri year
* @returns {number}
*/
  month(): number;
/**
* Get the length of the current Hijri month
* @returns {number}
*/
  month_len(): number;
/**
* Get the year of the Hijri date
* @returns {number}
*/
  year(): number;
/**
* Get the name of the day in the Hijri calendar
* @returns {string}
*/
  day_name(): string;
/**
* Get the name of the month in the Hijri calendar
* @returns {string}
*/
  month_name(): string;
/**
* Get the day of the week (Gregorian) corresponding to the Hijri date
* @returns {number}
*/
  day_gr(): number;
/**
* Get the month of the year (Gregorian) corresponding to the Hijri date
* @returns {number}
*/
  month_gr(): number;
/**
* Get the year (Gregorian) corresponding to the Hijri date
* @returns {number}
*/
  year_gr(): number;
/**
* Get the English name of the day in the Hijri calendar
* @returns {string}
*/
  day_name_en(): string;
/**
* Get the English name of the month in the Hijri calendar
* @returns {string}
*/
  month_name_en(): string;
/**
* @returns {string}
*/
  toString(): string;
/**
* get data from hijri date
* @param {number} year
* @param {number} month
* @param {number} day
* @returns {HijriDate}
*/
  static from_hijri(year: number, month: number, day: number): HijriDate;
/**
* get data from gregorian date.
* @param {number} year_gr
* @param {number} month_gr
* @param {number} day_gr
* @returns {HijriDate}
*/
  static from_gr(year_gr: number, month_gr: number, day_gr: number): HijriDate;
/**
* get data from today's date.
* @returns {HijriDate}
*/
  static today(): HijriDate;
/**
* Returns a representation of HijriDate defined by the given formatter
*
* ```text
*        hijri
*
*     %Y              hijri_year
*     %m              hijri_month
*     %d              hijri_day
*     %D              hijri_day_name
*     %M              hijri_month_name
*     %l              hijri_month_len
*
*        gregorian
*
*     %gY             gregorian_year
*     %gm             gregorian_month
*     %gd             gregorian_day
*     %gD             gregorian_day_name
*     %gM             gregorian_month_name
* ```
* @param {string} f
* @returns {string}
*/
  format(f: string): string;
}
