import { lcs } from "./longest_common_subsequence.ts"
import { assertEquals } from "https://deno.land/std@0.122.0/testing/asserts.ts";

Deno.test("Test with empty strings", () => {
    const r = lcs("", "")

    assertEquals(r, 0)
})

Deno.test("Test string 1 is empty", () => {
    const r = lcs("", "asdf")

    assertEquals(r, 0)
})

Deno.test("Test string 2 is empty", () => {
    const r = lcs("asdf", "")

    assertEquals(r, 0)
})

Deno.test("Test with same strings", () => {
    const r = lcs("asdf", "asdf")

    assertEquals(r, 4)
})

Deno.test("Test subsecuences", () => {
    const r = lcs("asqwedf", "asdf")

    assertEquals(r, 4)  
})