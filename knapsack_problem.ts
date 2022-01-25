
export type Pair = {
    profit: number
    weight: number
}

export const findOptimalPairs = (allPairs: Pair[], maxBagWeight: number) => {
    allPairs.unshift({
        profit: 0,
        weight: 0
    })

    const table: number[][] = new Array(allPairs.length)
    const optimalPairs: Pair[] = []

    for (let i = 0; i < allPairs.length; i++) {
        if (!(table[i] instanceof Array)) {
            table[i] = []
        }

        for (let j = 0; j <= maxBagWeight; j++) {
            if (i === 0 || j === 0) {
                table[i][j] = 0

                continue
            }

            if (allPairs[i].weight > j) {
                table[i][j] = table[i-1][j]

                continue
            }

            table[i][j] = Math.max(
                allPairs[i].profit + table[i-1][j-allPairs[i].weight], 
                table[i-1][j])
        }
    }

    let i = allPairs.length - 1
    let j = maxBagWeight

    while (i > 0 && j > 0) {
        if (table[i][j] == table[i-1][j]) {
            i--

            continue
        } 

        optimalPairs.push(allPairs[i])

        
        j -= allPairs[i].weight
        i--
    }

    return optimalPairs
}

