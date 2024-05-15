def main(n, mountain):
    counter = 0
    for index, height in enumerate(mountain[0:-1]):
        while mountain[index] > mountain[index+1]:
            mountain[index+1] += 1
            counter += 1

    print(counter)
if __name__ == "__main__":
    l1 = int(input())
    l2 = input().split(" ")
    l2 = [int(i) for i in l2]
    main(l1, l2)
