from itertools import permutations

def main():
    input_string = input("")
    input_string = sorted(input_string)
    unique_string = set()
    for perm in permutations(input_string):
        unique_string.add("".join(perm))

    print(len(unique_string))
    for unique_string in sorted(unique_string):
        print(unique_string)
    return unique_string

if __name__ == '__main__':
    main()
