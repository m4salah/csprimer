import csv
import math
import time
import array

def average_age(ages):
    total = 0
    for age in ages:
        total += age
    return total / len(ages)


def average_payment_amount(payments):
    amount = 0
    for dollers, centes in payments:
        amount += dollers + centes / 100
    return amount / len(payments)


def stddev_payment_amount(payments, mean):
    squared_diffs = 0
    for dollers, centes in payments:
        amount = dollers + centes / 100
        diff = amount - mean
        squared_diffs += diff * diff
    return math.sqrt(squared_diffs / len(payments))


def load_data():
    ages = array.array('I', [])
    payments = []
    with open('users.csv') as f:
        for line in csv.reader(f):
            uid, name, age, address_line, zip_code = line
            ages.append(int(age))
    with open('payments.csv') as f:
        for line in csv.reader(f):
            amount, timestamp, uid = line
            payments.append((float(int(amount)//100), float(int(amount) % 100)))
    return ages, payments


if __name__ == '__main__':
    t = time.perf_counter()
    ages, payments = load_data()
    print(f'Data loading: {time.perf_counter() - t:.3f}s')
    t = time.perf_counter()
    mean = average_payment_amount(payments)
    assert abs(average_age(ages) - 59.626) < 0.01
    assert abs(mean - 499850.559) < 0.01
    assert abs(stddev_payment_amount(payments, mean) - 288684.849) < 0.01
    print(f'Computation {time.perf_counter() - t:.3f}s')
