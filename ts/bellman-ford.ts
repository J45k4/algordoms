
export type Edge = {
    from: number
    to: number
    cost: number
}

export type Vertex = {
    id: number
    distance: number
}

const relaxEdges = (edges: Edge[], vertexes: Vertex[]): boolean => {
    let edgeRelaxed = false
    
    for (const edge of edges) {
        const from = vertexes[edge.from]
        const to = vertexes[edge.to]

        if (from.distance + edge.cost < to.distance) {
            to.distance = from.distance + edge.cost

            edgeRelaxed = true
        }
    }

    return edgeRelaxed
}

const buildVertexes = (edges: Edge[]) => {
    const vertexes: Vertex[] = []

    for (const edge of edges) {
        if (!vertexes[edge.from]) {
            vertexes[edge.from] = {
                id: edge.from,
                distance: Infinity
            }
        }

        if (!vertexes[edge.to]) {
            vertexes[edge.to] = {
                id: edge.to,
                distance: Infinity
            }
        }
    }

    vertexes[0].distance = 0

    return vertexes
}


export const bellmanFord = (edges: Edge[]): {
    vertexes: Vertex[]
    hasNegativeCycle: boolean
} => {
    const vertexes = buildVertexes(edges)

    const n = vertexes.length - 1

    for (let i = 0; i < n; i++) {
        relaxEdges(edges, vertexes)
    }

    const hasNegativeCycle = relaxEdges(edges, vertexes)

    return {
        vertexes: vertexes,
        hasNegativeCycle: hasNegativeCycle
    }
}