import { bellmanFord, Edge } from "./bellman-ford.ts"
import { assertEquals } from "https://deno.land/std@0.122.0/testing/asserts.ts";


Deno.test("test good graph", () => {
    const edges: Edge[] = [
        { from: 0, to: 1, cost: 6 },
        { from: 0, to: 2, cost: 5 },
        { from: 0, to: 3, cost: 5 },
        { from: 1, to: 4, cost: -1 },
        { from: 2, to: 1, cost: -2 },
        { from: 2, to: 4, cost: 1 },
        { from: 3, to: 2, cost: -2 },
        { from: 3, to: 5, cost: -1 },
        { from: 4, to: 6, cost: 3 },
        { from: 5, to: 6, cost: 3 },
    ]

    const { vertexes, hasNegativeCycle } = bellmanFord(edges)

    assertEquals(hasNegativeCycle, false)
    assertEquals(vertexes[0].distance, 0)
    assertEquals(vertexes[1].distance, 1)
    assertEquals(vertexes[2].distance, 3)
    assertEquals(vertexes[3].distance, 5)
    assertEquals(vertexes[4].distance, 0)
    assertEquals(vertexes[5].distance, 4)
    assertEquals(vertexes[6].distance, 3)
})

Deno.test("Test bad graph", () => {
    const edges: Edge[] = [
        { from: 0, to: 1, cost: 4 },
        { from: 0, to: 3, cost: 5 },
        { from: 1, to: 3, cost: 5 },
        { from: 3, to: 2, cost: 3 },
        { from: 2, to: 1, cost: -10 },
    ]
    
    const { vertexes, hasNegativeCycle } = bellmanFord(edges)

    assertEquals(hasNegativeCycle, true)

    assertEquals(vertexes[0].distance, 0)
    assertEquals(vertexes[1].distance, -8)
    assertEquals(vertexes[2].distance, 2)
    assertEquals(vertexes[3].distance, -1)
})