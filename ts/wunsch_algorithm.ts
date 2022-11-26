
export type InsertOp = {
    op: "insert"
    new: string
}

export type DeleteOp = {
    op: "delete"
    amount: number
}

export type ModifyOp = {
    op: "modify",
    new: string
    old: string
}

export type EditOp = (InsertOp | DeleteOp | ModifyOp) & {
    offset: number
}

export const wunschDistance = (str1: string, str2: string) => {
    const table = new Array(str1.length)

    for (let i = 0; i <= str2.length; i++) {
        if (!(table[i] instanceof Array)) {
            table[i] = new Array()
        }

        for (let j = 0; j <= str1.length; j++) {
            if (i === 0 && j === 0) {
                table[i][j] = 0

                continue
            }

            if (i === 0) {
                table[i][j] = table[i][j-1] + 1
            
                continue
            }

            if (j === 0) {
                table[i][j] = table[i-1][j] + 1

                continue
            }

            const diff = str1[j-1] !== str2[i-1] ? 1 : 0

            table[i][j] = Math.min(
                table[i-1][j-1] + diff,
                table[i-1][j] + 1,
                table[i][j-1] + 1            
            )
        }
    }

    const edits: EditOp[] = []

    let currEdit: any = null

    let i = str2.length
    let j = str1.length

    while(i > 0 && j > 0) {
        const diff = str1[j-1] !== str2[i-1] ? 1 : 0

        const a = table[i-1][j-1] + diff
        const b = table[i-1][j] + 1
        const c = table[i][j-1] + 1

        if (a <= b && a <= c) {
            if (currEdit && currEdit.op != "modify") {
                edits.push(currEdit)
                currEdit = null
            }
            
            const s1 = str1[j-1]
            const s2 = str2[i-1]

            if (s1 !== s2) {
                if (currEdit == null) {
                    currEdit = {
                        op: "modify",
                        offset: j-1,
                        new: "",
                        old: ""
                    }
                }

                currEdit.old = s1 + currEdit.old
                currEdit.new = s2 + currEdit.new
            }

            i--
            j--
        }

        if (b <= a && b <= c) {
            if (currEdit && currEdit.op != "insert") {
                edits.push(currEdit)
                currEdit = null
            }

            if (currEdit == null) {
                currEdit = {
                    op: "insert",
                    offset: j-1,
                    new: ""
                }
            }

            currEdit.new = str2[i-1] + currEdit.new
            
            i--
        }

        if (c <= a && c <= b) {
            if (currEdit && currEdit.op != "delete") {
                edits.push(currEdit)
                currEdit = null
            }
            
            if (currEdit == null) {
                currEdit = {
                    op: "delete",
                    offset: j-1,
                    amount: 1
                }
            } else {
                currEdit.amount += 1
            }

            j--
        }
    }

    if (currEdit != null) {
        edits.push(currEdit)
    }

    edits.sort((a, b) => a.offset - b.offset)

    return edits
}