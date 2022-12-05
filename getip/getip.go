package main

import (
	"os"
	"fmt"
	"strings"
	"net/http"
	"io/ioutil"
)

const url = "https://icanhazip.com"

func die(function string, err error) {
	fmt.Printf("[Erorr] %s - %s\n", function, err.Error())
	os.Exit(1)
}

func get_ip() string {
	resp, err := http.Get(url)
	if err != nil {
		die("Get", err)
	}

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		die("ReadAll", err)
	}

	ip := strings.Trim(string(body), "\r\n")

	return ip
}

func main() {
	fmt.Printf("%s\n", get_ip())
}
