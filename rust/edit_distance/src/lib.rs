use std::{cmp::{max, min}, fmt::Debug};

// fn edit_distance<T: PartialEq + Clone>(a: &[T], b: &[T]) -> usize {
//     let n = a.len();
//     let m = b.len();

//     let mut d: Vec<Vec<usize>> = Vec::with_capacity(n + 1);
//     for i in 0..=n {
//         d.push(Vec::with_capacity(m + 1));
//         for j in 0..=m {
//             d[i].push(0);
//         }
//     }

//     for i in 0..=n {
//         d[i][0] = i;
//     }
//     for j in 0..=m {
//         d[0][j] = j;
//     }

//     for i in 1..=n {
//         for j in 1..=m {
//             let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
//             d[i][j] = min(
//                 min(d[i - 1][j] + 1, d[i][j - 1] + 1),
//                 d[i - 1][j - 1] + cost,
//             );
//         }
//     }

//     d[n][m]
// }

// use std::cmp::{max, min};

#[derive(Debug, Clone, PartialEq)]
enum Edit<T> {
    InsertFirst(T),
    InsertAt(usize, T),
    RemoveAt(usize),
    ReplaceAt(usize, T),
}

// fn get_minimum_edits<T: PartialEq + Clone + Debug>(old: &[T], new: &[T]) -> Vec<Edit<T>> {
//     println!("a: {:?}", old);
//     println!("b: {:?}", new);

//     let old_len = old.len();
//     let new_len = new.len();

//     println!("old_len: {}", old_len);
//     println!("new_len: {}", new_len);

//     // let n = a.len();
//     // let m = b.len();

//     // let mut d: Vec<Vec<usize>> = Vec::with_capacity(n + 1);
//     // for i in 0..=n {
//     //     d.push(Vec::with_capacity(m + 1));
//     //     for j in 0..=m {
//     //         d[i].push(0);
//     //     }
//     // }

//     // let mut path: Vec<Vec<Option<(usize, usize)>>> = Vec::with_capacity(n + 1);
//     // for i in 0..=n {
//     //     path.push(Vec::with_capacity(m + 1));
//     //     for j in 0..=m {
//     //         path[i].push(None);
//     //     }
//     // }

//     let mut dp = vec![vec![0; new_len + 1]; old_len + 1];

//     println!("dp len: {}", dp.len());

//     for x in 0..=new_len {
//         dp[0][x] = x;
//     }
//     for y in 0..=old_len {
//         dp[y][0] = y;
//     }

//     // for i in 1..=n {
//     //     for j in 1..=m {
//     //         let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
//     //         let (x, y) = if d[i - 1][j - 1] + cost < d[i - 1][j] && d[i - 1][j - 1] + cost < d[i][j - 1] {
//     //             path[i][j] = Some((i - 1, j - 1));
//     //             (i - 1, j - 1)
//     //         } else if d[i - 1][j] < d[i][j - 1] {
//     //             path[i][j] = Some((i - 1, j));
//     //             (i - 1, j)
//     //         } else {
//     //             path[i][j] = Some((i, j - 1));
//     //             (i, j - 1)
//     //         };
//     //         d[i][j] = min(d[x][y] + cost, min(d[i - 1][j] + 1, d[i][j - 1] + 1));
//     //     }
//     // }

//     let mut results = Vec::new();

//     for y in 1..=old_len {
//         for x in 1..=new_len {
//             let cost = if new[x-1] == old[y-1] { 0 } else { 1 };

//             let del = dp[y - 1][x] + 1;
//             let ins = dp[y][x - 1] + 1;
//             let sub = dp[y - 1][x - 1] + cost;

//             if del < ins && del < sub {
//                 results.push(EditOperation::RemoveAt(y - 1))
//             } else if ins < del && ins < sub {
//                 results.push(EditOperation::InsertAt(x - 1, new[x - 1].clone()))
//             } else if sub < del && sub < ins {
//                 results.push(EditOperation::ReplaceAt(x - 1, new[x - 1].clone()))
//             } else {
//                 results.push(EditOperation::InsertFirst(new[x - 1].clone()))
//             }

//             // dp[y][x] = min(
//             //     min(dp[y - 1][x] + 1, dp[y][x - 1] + 1),
//             //     dp[y - 1][x - 1] + cost,
//             // );

//             // if a[i - 1] == b[j - 1] {
//             //     dp[i][j] = dp[i - 1][j - 1];
//             // } else {
//             //     dp[i][j] = min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1])) + 1;
//             // }
//         }
//     } 

//     for i in 0..=old_len {
//         for j in 0..=new_len {
//             print!("{:3}", dp[i][j]);
//         }
//         println!();
//     }


//     // for i in 1..=s1_len {
//     //     for j in 1..=s2_len {
//     //         if s1[i - 1] == s2[j - 1] {
//     //             dp[i][j] = dp[i - 1][j - 1];
//     //         } else {
//     //             dp[i][j] = min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1])) + 1;
//     //         }
//     //     }
//     // }

//     // for i in 1..=s1_len {
//     //     for j in 1..=s2_len {
//     //         if s1[i - 1] == s2[j - 1] {
//     //             dp[i][j] = dp[i - 1][j - 1];
//     //         } else {
//     //             dp[i][j] = min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1])) + 1;
//     //             if dp[i][j] == dp[i - 1][j - 1] + 1 {
//     //                 // Replace
//     //                 results.push(EditOperation::Replace(i - 1, j - 1));
//     //             } else if dp[i][j] == dp[i - 1][j] + 1 {
//     //                 // Insert
//     //                 results.push(EditOperation::Insert(j));
//     //             } else {
//     //                 // Remove
//     //                 results.push(EditOperation::Remove(i - 1));
//     //             }
//     //         }
//     //     }
//     // }

//     // println!("d {:?}", d);
//     // println!("path {:?}", path);

    


//     // let mut x = 0;
//     // let mut y = 0;

//     // while x < b.len() || y < a.len() {
//     //     if x == b_len {
//     //         results.push(EditOperation::RemoveAt(y));
//     //         y += 1;
//     //     } else if y == a_len {
//     //         results.push(EditOperation::InsertAt(x, b[y].clone()));
//     //         x += 1;
//     //     }  else {
//     //         if dp[y+1][x] <= dp[y+1][x+1] && dp[y+1][x+1] <= dp[y][x+1] {
//     //             results.push(EditOperation::RemoveAt(x));
//     //             y += 1;
//     //         } else if dp[y+1][x+1] <= dp[y][x+1] {
//     //             if x == 1 {
//     //                 results.push(EditOperation::InsertFirst(b[y].clone()))
//     //             } else {
//     //                 results.push(EditOperation::InsertAt(x, b[y].clone()));
//     //             } 
//     //             x += 1;
//     //         } else {
//     //             if a[y] != b[x] {
//     //                 results.push(EditOperation::ReplaceAt(x, b[y].clone()));
//     //             }
//     //             x += 1;
//     //             y += 1;
//     //         }

//     //         // if dp[y][x+1] <= dp[y+1][x+1] && dp[y+1][x+1] <= dp[y+1][x] {
//     //         //     if x == 1 {
//     //         //         results.push(EditOperation::InsertFirst(b[y-1].clone()))
//     //         //     } else {
//     //         //         results.push(EditOperation::InsertAt(x-1, b[y].clone()));
//     //         //     }                
//     //         //     x += 1;
//     //         // } else if dp[y+1][x+1] <= dp[y+1][x] {
                
//     //         // } else {
                
//     //         // }
//     //     }

//     //     // if y == a.len() {
//     //     //     results.push(EditOperation::InsertAt(x, b[y].clone()));
//     //     //     y += 1;
//     //     // } else if y == b.len() {
//     //     //     results.push(EditOperation::RemoveAt(x));
//     //     //     x += 1;
//     //     // } else if a[x] == b[y] {
//     //     //     x += 1;
//     //     //     y += 1;
//     //     // } else {
//     //     //     if dp[x + 1][y + 1] == dp[x][y] + 1 {
//     //     //         results.push(EditOperation::ReplaceAt(x, b[y].clone()));
//     //     //         x += 1;
//     //     //         y += 1;
//     //     //     } else if dp[x + 1][y] == dp[x][y] + 1 {
//     //     //         results.push(EditOperation::RemoveAt(x));
//     //     //         x += 1;
//     //     //     } else if dp[x][y + 1] == dp[x][y] + 1 {
//     //     //         results.push(EditOperation::InsertAt(x, b[y].clone()));
//     //     //         y += 1;
//     //     //     }
//     //     // }
//     // }


//     // let (mut i, mut j) = (n, m);  
//     // while i > 0 || j > 0 {
//     //     println!("\n\n iteration");

//     //     println!("i {} j {}", i, j);

//     //     let (x, y) = match path[i][j] {
//     //         Some((x, y)) => (x, y),
//     //         None => {
//     //             if i == 0 {
//     //                 result.push(EditOperation::InsertAt(0, b[j - 1].clone()));
//     //                 j -= 1;
//     //             } else if j == 0 {
//     //                 result.push(EditOperation::RemoveAt(i - 1));
//     //                 i -= 1;
//     //             }
//     //             continue;
//     //         },
//     //     };

//     //     println!("x {} y {}", x, y);

//     //     if x == i - 1 && y == j - 1 {
//     //         if a[i - 1] != b[j - 1] {
//     //             result.push(EditOperation::ReplaceAt(i - 1, b[j - 1].clone()));
//     //         }
//     //         i -= 1;
//     //         j -= 1;
//     //     } else if x == i - 1 {
//     //         result.push(EditOperation::RemoveAt(i - 1));
//     //         i -= 1;
//     //     } else if y == j - 1 {
//     //         result.push(EditOperation::InsertAt(i, b[j - 1].clone()));
//     //         j -= 1;
//     //     }
//     // }

//     results
// }

#[derive(Debug, PartialEq)]
enum EditOperation<T> {
    InsertFirst(T),
    InsertAt(usize, T),
    RemoveAt(usize),
    ReplaceAt(usize, T),
}

fn get_minimum_edits<T: PartialEq + std::fmt::Display + Clone>(s: &Vec<T>, t: &Vec<T>) -> Vec<EditOperation<T>> {
    let m = s.len();
    let n = t.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the base cases
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Fill in the DP table
    for i in 1..=m {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                let mut cost = 1;

                // if i == 1 && j == 1 {
                //     cost = 0;
                // } else if i > 1 && j > 1 && s[i - 2] == t[j - 2] {
                //     cost = 0;
                // }

                println!("i {} j {} cost {}", i, j, cost);

                dp[i][j] = min(dp[i - 1][j], dp[i][j - 1]) + cost;
            }
        }
    }

    // Here we print the table
    for i in 0..=m {
        for j in 0..=n {
            print!("{} ", dp[i][j]);
        }
        println!();
    }

    // Initialize the list of edit operations
    let mut edits = Vec::new();

    // Backtrack to generate the list of edit operations
    let mut i = m;
    let mut j = n;
    while i > 0 || j > 0 {
        if i == 0 {
            // Insert t[j - 1]
            edits.push(EditOperation::InsertFirst(t[j - 1].clone()));
            j -= 1;

            continue;
        } else if j == 0 {
            // Delete s[i - 1]
            edits.push(EditOperation::RemoveAt(i - 1));
            i -= 1;

            continue;
        } else if s[i - 1] == t[j - 1] {
            // No operation required
            i -= 1;
            j -= 1;

            continue;
        } 
        
        let top = dp[i - 1][j];
        let left = dp[i][j - 1];
        let diag = dp[i - 1][j - 1];

        if diag < top && diag < left {
            // Replace s[i - 1] with t[j - 1]
            edits.push(EditOperation::ReplaceAt(i - 1, t[j - 1].clone()));
            i -= 1;
            j -= 1;
        } else if top < left {
            // Delete s[i - 1]
            edits.push(EditOperation::RemoveAt(i - 1));
            i -= 1;
        } else {
            // Insert t[j - 1]
            edits.push(EditOperation::InsertAt(i - 1, t[j - 1].clone()));
            j -= 1;
        }
        
        // else if dp[i][j] == dp[i - 1][j] + 1 {
        //     // Delete s[i - 1]
        //     edits.push(EditOperation::RemoveAt(i - 1));
        //     i -= 1;
        // } else if dp[i][j] == dp[i][j - 1] + 1 {
        //     // Insert t[j - 1]
        //     edits.push(EditOperation::InsertAt(i - 1, t[j - 1].clone()));
        //     j -= 1;
        // } else if dp[i][j] > dp[i - 1][j - 1] {
        //     // Replace s[i - 1] with t[j - 1]
        //     edits.push(EditOperation::ReplaceAt(i - 1, t[j - 1].clone()));
        //     i -= 1;
        //     j -= 1;
        // }
    }

    // Reverse the list of edit operations to get the correct order
    edits.reverse();

    edits
}

// fn edit_distance<T: PartialEq>(s1: &[T], s2: &[T]) -> usize {
//     let s1_len = s1.len();
//     let s2_len = s2.len();

//     let mut dp = vec![vec![0; s2_len + 1]; s1_len + 1];

//     for i in 0..=s1_len {
//         dp[i][0] = i;
//     }
//     for j in 0..=s2_len {
//         dp[0][j] = j;
//     }

//     for i in 1..=s1_len {
//         for j in 1..=s2_len {
//             if s1[i - 1] == s2[j - 1] {
//                 dp[i][j] = dp[i - 1][j - 1];
//             } else {
//                 dp[i][j] = min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1])) + 1;
//             }
//         }
//     }

//     // for i in 1..=n {
//     //     for j in 1..=m {
//     //         let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
//     //         let (x, y) = if d[i - 1][j - 1] + cost < d[i - 1][j] && d[i - 1][j - 1] + cost < d[i][j - 1] {
//     //             path[i][j] = Some((i - 1, j - 1));
//     //             (i - 1, j - 1)
//     //         } else if d[i - 1][j] < d[i][j - 1] {
//     //             path[i][j] = Some((i - 1, j));
//     //             (i - 1, j)
//     //         } else {
//     //             path[i][j] = Some((i, j - 1));
//     //             (i, j - 1)
//     //         };
//     //         d[i][j] = min(d[x][y] + cost, min(d[i - 1][j] + 1, d[i][j - 1] + 1));
//     //     }
//     // }


//     println!("{:?}", dp);

//     dp[s1_len][s2_len]
// }

// #[derive(Debug)]
// enum EditOperation {
//     Replace(usize, usize),
//     Insert(usize),
//     Remove(usize),
// }

// fn edit_distance_with_results<T: PartialEq>(s1: &[T], s2: &[T]) -> (usize, Vec<EditOperation>) {
//     let s1_len = s1.len();
//     let s2_len = s2.len();

//     let mut dp = vec![vec![0; s2_len + 1]; s1_len + 1];
//     let mut results = Vec::new();

//     for i in 0..=s1_len {
//         dp[i][0] = i;
//     }
//     for j in 0..=s2_len {
//         dp[0][j] = j;
//     }

//     for i in 1..=s1_len {
//         for j in 1..=s2_len {
//             if s1[i - 1] == s2[j - 1] {
//                 dp[i][j] = dp[i - 1][j - 1];
//             } else {
//                 dp[i][j] = min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1])) + 1;
//                 if dp[i][j] == dp[i - 1][j - 1] + 1 {
//                     // Replace
//                     results.push(EditOperation::Replace(i - 1, j - 1));
//                 } else if dp[i][j] == dp[i - 1][j] + 1 {
//                     // Insert
//                     results.push(EditOperation::Insert(j));
//                 } else {
//                     // Remove
//                     results.push(EditOperation::Remove(i - 1));
//                 }
//             }
//         }
//     }

//     (dp[s1_len][s2_len], results)
// }

    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_front() {
        let a = vec![1, 2, 3];
        let b = vec![0, 1, 2, 3];

        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::InsertFirst(0)]);
    }
    
    #[test]
    fn test_insert_back() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3, 4];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::InsertAt(2, 4)]);
    }
    
    #[test]
    fn test_insert_middle() {
        /*    1  2  4  3
          [0, 1, 2, 3, 4], 
        1 [1, 0, 1, 2, 3], 
        2 [2, 1, 0, 1, 2], 
        3 [3, 2, 1, 1, 1]
        */

        let a = vec![1, 2, 3];
        let b = vec![1, 2, 4, 3];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::InsertAt(1, 4)]);
    }
    
    #[test]
    fn test_remove_first() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::RemoveAt(0)]);
    }
    
    #[test]
    fn test_remove_last() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::RemoveAt(2)]);
    }
    
    #[test]
    fn test_remove_middle() {
        let a = vec![1, 2, 3];
        let b = vec![1, 3];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::RemoveAt(1)]);
    }
    
    #[test]
    fn test_replace_first() {
        let a = vec![1, 2, 3];
        let b = vec![4, 2, 3];

        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::ReplaceAt(0, 4)]);
    }
    
    #[test]
    fn test_replace_last() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 4];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::ReplaceAt(2, 4)]);
    }
    
    #[test]
    fn test_replace_middle() {
        let a = vec![1, 2, 3];
        let b = vec![1, 4, 3];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::ReplaceAt(1, 4)]);
    }
    
    #[test]
    fn test_replace_all() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::ReplaceAt(0, 4), EditOperation::ReplaceAt(1, 5), EditOperation::ReplaceAt(2, 6)]);
    }

    #[test]
    fn test_move_from_back_to_front() {
        let a = vec![1, 2, 3];
        let b = vec![3, 1, 2];

        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::InsertFirst(3), EditOperation::RemoveAt(2)]);
    }

    #[test]
    fn test_string() {
        let old = "sitting";
        let new = "kitten";

        let edits = get_minimum_edits(&old.chars().collect::<Vec<_>>(), &new.chars().collect::<Vec<_>>());

        assert_eq!(edits, 
            vec![
                EditOperation::ReplaceAt(0, 'k'),
                EditOperation::ReplaceAt(4, 'e'),
                EditOperation::RemoveAt(6),
            ]
        );
    }
}
