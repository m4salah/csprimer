package main

import (
	"fmt"
	"io"
	"net/http"
)

func memoize[K comparable, V any](f func(K) V) func(K) V {
	cache := map[K]V{}
	return func(k K) V {
		cached, ok := cache[k]
		if ok {
			return cached
		}
		result := f(k)
		cache[k] = result
		return result
	}
}

func fetch(url string) string {
	res, err := http.Get(url)
	if err != nil {
		panic(err)
	}
	defer res.Body.Close()
	b, err := io.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}
	return string(b)
}

func main() {
	memoizedFetch := memoize(fetch)
	fmt.Println(memoizedFetch("https://google.com")[0:80])
	fmt.Println(memoizedFetch("https://google.com")[0:80])
	fmt.Println(memoizedFetch("https://google.com")[0:80])
	fmt.Println(memoizedFetch("https://google.com")[0:80])
	fmt.Println(memoizedFetch("https://google.com")[0:80])
	fmt.Println(memoizedFetch("https://google.com")[0:80])
}
