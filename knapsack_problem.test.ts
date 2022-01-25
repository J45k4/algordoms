import { Pair, findOptimalPairs } from "./knapsack_problem.ts"
import { assertEquals } from "https://deno.land/std@0.122.0/testing/asserts.ts";

Deno.test("Test sack", () => {
    const pairs: Pair[] = [
        {
            profit: 1,
            weight: 2
        },
        {
            profit: 2,
            weight: 3
        },
        {
            profit: 5,
            weight: 4
        },
        {
            profit: 6,
            weight: 5
        }
    ]

    const maxBagWeight = 8

    const optimalPairs = findOptimalPairs(pairs, maxBagWeight)

    console.log(optimalPairs)

    assertEquals(optimalPairs.some(p => p.profit === 6 && p.weight === 5), true)
    assertEquals(optimalPairs.some(p => p.profit === 2 && p.weight === 3), true)
})