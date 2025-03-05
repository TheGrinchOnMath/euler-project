import math


def find_amicable_numbers(floor, ceil):
    amicable_list = []

    for n in range(floor, ceil+1):      
        amicable = divisor_sum(n)
        m = divisor_sum(amicable)

        if n == m and n != amicable:
            bigger = max(n, amicable)
            smaller = min(n, amicable)
            amicable_list.append((smaller, bigger))
    print(amicable_list)
    amicable_list = list(dict.fromkeys(amicable_list))
    
    print(amicable_list)

def divisor_sum(num):
    sum = 0
    divisors = []
    ## ceil of root, since need int for range expr
    root = int(math.sqrt(num)) + 1
    for n in range(1, root):
        if num % n == 0:
            divisors.append(num // n)
            divisors.append(n)
    
    divisors = list(set(divisors))
    # print(divisors)
    try:
        divisors.pop()
        divisors.sort()
    except ValueError:
        print(num)
    for div in divisors:
        sum += div
    return sum
# safe to iter from 2, since 1 is its own amicable number, 
# and will for now be ignored

find_amicable_numbers(2, 10000)
