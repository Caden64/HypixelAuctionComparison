package hypixel

import (
	"net/http"
	"time"
)

func NewClient() *http.Client {
	return &http.Client{
		Transport: &http.Transport{
			MaxConnsPerHost: 0,
			MaxIdleConns:    20,
			IdleConnTimeout: 5 * time.Second,
		},
		Timeout: 10 * time.Second,
	}
}
