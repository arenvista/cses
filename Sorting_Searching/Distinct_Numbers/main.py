if __name__ == '__main__':
    _ = int(input())
    nums = list(map(int, input().strip().split()))
    print(len(set(nums)))
