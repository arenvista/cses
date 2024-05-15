def main(dna_seq):

    max = 1
    consecutive = 1
    char = dna_seq[0]
    for index, aa in enumerate(dna_seq[1:]):
        if aa == char:
            consecutive += 1
            if consecutive > max:
                max = consecutive
        else:
            if consecutive > max:
                max = consecutive
            consecutive = 1
            char = aa   
    print(max)

if __name__ == "__main__":
    i = input()
    i = [*i]
    main(i)
