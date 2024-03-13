def is_prime(n):
    """判斷一個數字是否是質數"""
    if n <= 1:
        return False
    for i in range(2, int(n**0.5) + 1):
        if n % i == 0:
            return False
    return True

def print_primes(start, end):
    """打印指定範圍內的所有質數"""
    print(f"在範圍 {start} 到 {end} 內的質數有：")
    for num in range(start, end + 1):
        if is_prime(num):
            print(num, end=" ")

# 範例用法
start = 10
end = 50
print_primes(start, end)
