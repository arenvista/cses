Want: # of valid posititions
- Position is valid if knights are not engaged

## Plan

valid_pos = total_pos - invalid_pos
- Where total_pos = to the total number of ways we can place 2 knights on a k x k board

### How many ways can we place 2 knights on a k x k board?
We can place the first knight anywhere on the board since it's empty
- Since the board is square, there are k^2 positions we can place the first knight

The second knight would have every position on the board except the one the first knight is on
- So there are k^2 - 1 positions we can place the second knight

Since these are two independent events we want to do together we multiply the number of ways we can place the first knight by the number of ways we can place the second knight
- So the total number of ways we can place 2 knights on a k x k board is `k^2 * (k^2 - 1)`

But since the board is a square, half the possitions are just reflections of the other half we can forget about those, we only care about unique positions:
- So the total number of ways we can place 2 knights on a k x k board is `k^2 * (k^2 - 1) / 2`

### How many invalid positions are there?
Well there's outright no way for knights to attack each other when k < 3 since knight need at least 2 spaces between them to attack each other, so we can just return 0 for those cases

Consider when knights are engaged, there are 4 possible scenarios:
```
[k][ ][ ]
[ ][ ][k]

[ ][ ][k]
[k][ ][ ]

[k][ ]
[ ][ ]
[ ][k]

[ ][k]
[ ][ ]
[k][ ]
```

These grids represent all the ways knights can attack each other.

If we know how many of each grid can fit on a k x k board, we can calculate the number of invalid positions - since each position ties every possible way knights can attack each other
- We have 4 different ways knights can attack each other

How many ways can we fit a 3x2 grid on a k x k board?

```
[x][x][x][ ][ ]
[x][x][x][ ][ ]

[ ][x][x][x][ ]
[ ][x][x][x][ ]

[ ][x][x][x][ ]
[ ][x][x][x][ ]

[ ][ ][x][x][x]
[ ][ ][x][x][x]

[ ][ ][ ][x][x] x
[ ][ ][ ][x][x] x

[ ][ ][ ][ ][x] x x <-|
                      | Notice the two x's, we won't be able to fit some of the 2x3 if we start the index at the end; where is the last avaliable index?
[ ][ ][ ][ ][x] x x <-|

```
That's right it's at k - 2
- Note how we can place a grid on nearly every tile on the board `k` posititons except the last 2 columns `-2` and rows this gives us a total of `k - 2` positions

Likewise we can do this for the other dimension

```
[x][x][x][ ][ ]
[x][x][x][ ][ ]

 |--|--|---These are off by 1
 x  x  x
[x][x][x][ ][ ]
[ ][ ][ ][ ][ ]

```
So for a 3x2 grid we can fit it on `k - 2` rows and `k - 1` columns
Likewise for a 2x3 grid we can fit it on `k - 1` rows and `k - 2` columns

so we can say:
```
- total_num_3x2 = (k - 2) * (k - 1)  = total_num_2x3 = (k - 1) * (k - 2) = u

total_invalid_positions = (total_num_3x2 * invalid_3x2_configs) + (total_num_2x3 * invalid_2x3_configs)
= 2 * u + 2 * u
= 4 * u
```

So the number of invalid positions is `4 * (k - 2) * (k - 1)` or `4 * u` where `u = (k - 2) * (k - 1)`

Since we said `valid_pos = total_pos - invalid_pos` we can say:
```
valid_pos = (k^2 * (k^2 - 1) / 2)  - invalid_pos

valid_pos = (k^2 * (k^2 - 1) / 2)  - (4 * (k - 2) * (k - 1))

```
