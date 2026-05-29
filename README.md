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
2. Word and result is 1-7 chars (`1 <= words[i].length, result.length <= 7`) 
3. Only uppercase English letters
4. The number of different characters used in the expression is at most 10

## Examples

### Example 1

```js
test({ words: ["SEND","MORE"], result: "MONEY" }) = true;
```

```
"SEND" + "MORE" = "MONEY"
 9567  +  1085  =  10652
```

| S | E | N | D | M | O | R | Y |
| - | - | - | - | - | - | - | - |
| 9 | 5 | 6 | 7 | 1 | 0 | 8 | 2 |

### Example 2
```js
test({words: ["SIX","SEVEN","SEVEN"], result: "TWENTY"}) = true;
```

```
"SIX" + "SEVEN" + "SEVEN" = "TWENTY"
 650  +  68782  +  68782  =  138214
```

| S | I | X | E | V | N | T | W | Y |
| - | - | - | - | - | - | - | - | - |
| 6 | 5 | 0 | 8 | 7 | 2 | 1 | 3 | 4 | 

### Example 3

```js
test({words: ["LEET","CODE"], result: "POINT"}) = false;
```

There is no possible mapping to satisfy the equation, so we return false.
 - Note: Two different characters cannot map to the same digit.
