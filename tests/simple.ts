import { HijriDate } from "../pkg/hijri_date.js";

Deno.test("simple", () => {
  const hijriDate = HijriDate.today();
  console.log(hijriDate.toString());
});
