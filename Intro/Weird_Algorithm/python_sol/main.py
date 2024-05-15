def check_odd(n):
    if n % 2 == 1:
        return True
    return False


def main(n):
    values = [n]
    while n != 1:
        if check_odd(n):
            n = int((n * 3) + 1)
        else:
            n = int(n/2)
        values.append(n)
    values = str(values).replace("[", "").replace(",","").replace("]","")
    print(values)

if __name__ == "__main__":
    i = int(input())
    main(i)
