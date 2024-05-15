def main(n, Xa):
    summation_n = sum(range(1, n+1))
    sum_Xa = sum(Xa)
    b = summation_n - sum_Xa
    print(b)

if __name__ == "__main__":
    n = int(input())
    Xa = input().split(" ")
    Xa = [int(x) for x in Xa]
    main(n, Xa)
