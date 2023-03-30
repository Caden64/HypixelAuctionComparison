package main

import (
	"HypixelAPIGo/hypixel"
	"fmt"
	"time"
)

func main() {
	now := time.Now()
	client := hypixel.NewClient()
	initialData, err := hypixel.Request(client, 0)
	if err != nil {
		fmt.Println("error:", err)
		return
	}
	restOfData := hypixel.MultipleRequest(client, 1, initialData.TotalPages)
	if restOfData.Err != nil {
		fmt.Println("error:", err)
	}
	fmt.Println(time.Since(now).Seconds())
	fmt.Println(len(restOfData.Auctions), initialData.TotalAuctions)
}
