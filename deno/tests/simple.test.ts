import { assertEquals } from "https://deno.land/std@0.198.0/assert/assert_equals.ts";
import { HijriDate } from "../pkg/hijri_date.js";

Deno.test("simple", () => {
  const hijriDate = HijriDate.from_gr(2010, 10, 5);
  assertEquals(hijriDate.format("%Y/%m/%d"), "1431/10/26");
});
