def find_ndigits(k_index: int) -> str:
    num_digits = 1
    num_chars = 9 * 10 ** (num_digits - 1) * num_digits
    while k_index > num_chars: #While we haven't found the upper bound of the number of digits
        k_index -= num_chars #Keep subtracting the number of characters in the total index position of the target index: k_index
        num_digits += 1
        num_chars = 9 * 10 ** (num_digits - 1) * num_digits
    quotient, remainder = divmod(k_index-1, num_digits)
    result = str(10 ** (num_digits - 1) + quotient)[remainder]
    return result

def test_case() -> None:
    input = [7, 19, 12]
    expected_output = ["7", "4", "1"]
    for i in range(len(input)):
        assert find_ndigits(input[i]) == expected_output[i]
    print("All test cases pass")

def main() -> None:
    num_inputs = int(input())
    ans = []
    for _ in range(num_inputs):
        k_index = int(input())
        ans.append(find_ndigits(k_index))
    print("\n".join(ans))

if __name__ == "__main__":
    main()
