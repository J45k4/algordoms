
type Job = {
    weight: number
    start: number
    end: number
}

const OPT = (jobs: Job[], index: number): number => {
    if (index === 0) {
        return 0
    }

    const r = Math.max(jobs[index - 1].weight + OPT(jobs, P(jobs, index)), OPT(jobs, index - 1))

    return r
}

const P = (jobs: Job[], index: number) => {
    for (let i = index - 1; i > 0; i--) {
        if (!jobs[i - 1]) {
            throw new Error("Job not found i:" + i)
        }

        if (!jobs[index - 1]) {
            throw new Error("Job not found index:" + index)
        }

        if (jobs[i - 1].end < jobs[index - 1].start) {
            
            return i
        }
    }

    return 0
}

type Result = {
    index: number
    weight: number
    pVal: number
    optVal: number
}

const findOptimalResults = (results: Result[]): Result[] => {
    const optimal: Result[] = []
    
    for (let i = results.length - 1; i > 0; i--) {
        const leftR = results[i]
        const rightR = results[i-1]
        const optVal = results[leftR.pVal]?.optVal || 0

        if (!leftR || !rightR) {
            continue
        }

        if (leftR.weight + optVal >= rightR.optVal) {
            optimal.push(leftR)
            
            i = leftR.pVal - 1
        }
    }

    return optimal
}

const resultTable: Result[] = []

const jobs: Job[] = [
    {
        start: 0,
        end: 2,
        weight: 6,
    },
    {
        start: 1,
        end: 3,
        weight: 2
    },
    {
        start: 0,
        end: 4,
        weight: 4
    },
    {
        start: 2,
        end: 7,
        weight: 10
    },
    {
        start: 2,
        end: 8,
        weight: 9
    },
    {
        start: 5,
        end: 9,
        weight: 8
    },
    {
        start: 8,
        end: 9,
        weight: 7
    },
    {
        start: 9,
        end: 11,
        weight: 2
    },
    {
        start: 12,
        end: 13,
        weight: 12
    }
]

for (let i = 1; i <= jobs.length; i++) {
    const r: Result = {
        index: i,
        weight: jobs[i-1].weight,
        optVal: OPT(jobs, i),
        pVal: P(jobs, i)
    }

    resultTable.push(r)
}

console.log(resultTable)

const optimal = findOptimalResults(resultTable)

console.log(optimal)

const totalWeight = optimal.reduce((p, c) => c.weight + p, 0)

console.log(totalWeight)