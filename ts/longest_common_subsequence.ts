
const _lcs = (str1: string, str2: string, i: number, j: number): number => {
    if (i === str1.length || j === str2.length) {
        return 0
    }
    
    if (str1[i] === str2[j]) {
        return 1 + _lcs(str1, str2, i + 1, j + 1)
    }

    return Math.max(_lcs(str1, str2, i+1, j), _lcs(str1, str2, i, j+1))    
}

export const lcs = (str1: string, str2: string): number => {
    return _lcs(str1, str2, 0, 0)
}


export const lcsDyn = (str1: string, str2: string) => {
    const table = new Array(str1.length)
}