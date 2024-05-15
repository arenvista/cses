def solve(index, apples, sum_1, sum_2, n):
    if index == n: #if we hit the end
        return abs(sum_1 - sum_2)

    g1 = solve(index+1, apples, sum_1 + apples[index], sum_2, n)
    g2 = solve(index+1, apples, sum_1, sum_2+apples[index], n)
    return min(g1, g2)

if __name__ == '__main__':
    n = int(input(""))
    apples = [int(a) for a in input("").split(" ")]
    ans = solve(0, apples, 0, 0, n)
    print(ans)
