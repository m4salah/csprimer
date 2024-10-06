import urllib.request


def memoize(f):
    k = {}

    def inner(input):
        possibly_cached = k.get(input)
        if possibly_cached is not None:
            return possibly_cached
        result = f(input)
        k[input] = result
        return result
    return inner


def fetch(url):
    with urllib.request.urlopen(url) as response:
        content = response.read().decode('utf-8')
        return content


def fib(n):
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)


if __name__ == '__main__':
    cached_fetch = memoize(fetch)
    print(cached_fetch('http://google.com')[:80])
    print(cached_fetch('http://google.com')[80:160])
    print(cached_fetch('http://google.com')[160:240])
    print(cached_fetch('http://google.com')[240:320])
