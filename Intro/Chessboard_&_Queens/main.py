BOARD_LENGTH = 8 # 8x8 board
ClosedLeftDiag = [0]*15 # r + c 
ClosedRightDiag = [0]*15 # r - c + 7
ClosedCol = [0]*8 # c
ans = 0

def solve(board, row):
    if row == BOARD_LENGTH: #if all 8 queens are placed; can occupy only 1 queen per row
        global ans
        ans += 1
        return

    for col in range(BOARD_LENGTH):
        if board[row][col] == '*' or ClosedLeftDiag[row+col] or ClosedRightDiag[row-col+7] or ClosedCol[col]:
            continue
        ClosedCol[col] = ClosedRightDiag[row-col+7] = ClosedLeftDiag[row+col] = True
        solve(board, row+1)
        ClosedCol[col] = ClosedRightDiag[row-col+7] = ClosedLeftDiag[row+col] = False

if __name__ == "__main__":
    board = [input() for _ in range(BOARD_LENGTH)]
    solve(board, 0)
    print(ans)
