from main import main

class UnitTests:
    def test_main():
        input = 3
        expected = [3,10,5,16,8,4,2,1]
        output =main(input)
        if output == expected:
            print("Test Passed")
        else:
            print(f"Test Failed, expected {expected} but got {output}")

        

if __name__ == "__main__":
    UnitTests.test_main()
