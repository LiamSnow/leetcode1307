# Leetcode 1307

Given an equation:

```
┌────words────┐      ┌─result
"SEND" + "MORE" = "MONEY"
```

Find if it's solvable, where:
 - Each character is decoded as one digit (0 - 9)
 - No two characters can map to the same digit
 - Each `words[i]` and `result` are decoded as one number without leading zeros
 - Sum of numbers on the LHS (`words`) will equal to the number on the RHS (`result`)

## Constraints
1. 2-5 words (`2 <= words.length <= 5`)
2. At least one word (`words[i].length >= 1`)
3. `result` is 7 or less characters (`result.length <= 7`)
4. Only uppercase English letters
5. The number of different characters used in the expression is at most 10

## Example 1

```js
test({ words: ["SEND","MORE"], result: "MONEY" }) = true;
```

```
"SEND" + "MORE" = "MONEY"
 9567  +  1085  =  10652
```

| &nbsp; | &nbsp; |
| S | 9 |
| E | 5 |
| N | 6 |
| D | 7 |
| M | 1 |
| O | 0 |
| R | 8 |
| Y | 2 |

## Example 2
```js
test({words: ["SIX","SEVEN","SEVEN"], result: "TWENTY"}) = true;
```

```
"SIX" + "SEVEN" + "SEVEN" = "TWENTY"
 650  +  68782  +  68782  =  138214
```

| &nbsp; | &nbsp; |
| S | 6 |
| I | 5 |
| X | 0 |
| E | 8 |
| V | 7 |
| N | 2 |
| T | 1 |
| W | 3 |
| Y | 4 |

 
## Example 3

```js
test({words: ["LEET","CODE"], result: "POINT"}) = false;
```

There is no possible mapping to satisfy the equation, so we return false.
 - Note: Two different characters cannot map to the same digit.



