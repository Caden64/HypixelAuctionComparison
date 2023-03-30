package hypixel

import (
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"strconv"
	"sync"
)

const BaseURL = "https://api.hypixel.net/skyblock/auctions"

func Request(client *http.Client, page int) (Auctions, error) {
	// Creates the request
	req, err := http.NewRequest(http.MethodGet, BaseURL, nil)

	if err != nil {
		return Auctions{}, err
	}
	// Adds the page number if greater than zero
	if page > 0 {
		req.URL.Query().Add("page", strconv.Itoa(page))
	}
	// Makes the request
	res, err := client.Do(req)
	if err != nil {
		return Auctions{}, err
	}
	// Reads the body
	body, err := io.ReadAll(res.Body)
	if err != nil {
		return Auctions{}, err
	}
	// Turns the body into the Auctions type
	var data Auctions
	err = json.Unmarshal(body, &data)
	if err != nil {
		return Auctions{}, err
	}
	return data, nil
}

func MultipleRequest(client *http.Client, start, end int) *Holder {
	hold := Holder{}
	var wg sync.WaitGroup
	for i := start; i < end; i++ {
		wg.Add(1)
		i := i
		go func() {
			defer wg.Done()
			data, err := Request(client, i)
			if err != nil {
				hold.Mu.Lock()
				defer hold.Mu.Unlock()
				hold.Err = errors.Join(hold.Err, err)
				return
			}
			hold.Mu.Lock()
			defer hold.Mu.Unlock()
			hold.Auctions = append(hold.Auctions, data)
		}()
	}
	wg.Wait()
	return &hold
}
