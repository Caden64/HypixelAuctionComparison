import threading
import time

import requests

URL = "https://api.hypixel.net/skyblock/auctions"


def fetch_data(url, page_number, results):
    params = {"page": page_number}
    res = requests.get(url, params=params)
    results.append(res)


def main():
    now = time.time()
    res = requests.get(URL).json()
    tp = res["totalPages"]
    total = [res]

    threads = []
    results = []
    for i in range(1, tp):
        t = threading.Thread(target=fetch_data, args=(URL, i, results))
        t.start()
        threads.append(t)

    for t in threads:
        t.join()
    total.extend(results)
    print(time.time() - now)
    print(len(total))


if __name__ == '__main__':
    main()
