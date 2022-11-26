import { wunschDistance } from "./wunsch_algorithm.ts"

Deno.test("test", () => {
    const str1 = "TCGACGTCA"
    const str2 = "TGACGTGC"

    wunschDistance(str1, str2)
})