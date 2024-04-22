link = "https://e4ftl01.cr.usgs.gov/ASTT/ASTGTM.003/2000.03.01/"

# check if page.html already exists

import os

if not os.path.exists("page.html"):
    # get all links of this page:
    import urllib.request

    url = link
    response = urllib.request.urlopen(url)
    html = response.read()

    # save html to file
    with open("page.html", "wb") as f:
        f.write(html)
    print("saved to page.html")
else:
    with open("page.html", "rb") as f:
        html = f.read()
    print("page.html already exists")

# parse html
# we want to get all of the links to the tiles (.zip form)
# these will be relative links

import bs4
import tqdm
from typing import List

soup = bs4.BeautifulSoup(html, "html.parser")

links = soup.find_all("a")

download_links: List[object] = []

# pass 1
for link in tqdm.tqdm(links):
    if link.get("href").endswith(".zip"):
        download_links.append({"zip": link.get("href"), "xml": link.get("href") + ".xml"})

# save links to file
import json

with open("download_links_img.json", "w") as f:
    json.dump(download_links, f)
   
