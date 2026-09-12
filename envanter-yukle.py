#!/usr/bin/env python3
"""Reklam envanterini backend'e yükler.

Kullanim:
    ADS_ADMIN_TOKEN=... python envanter-yukle.py ornek-reklam-envanteri.json

Uc, ADS_ADMIN_TOKEN ortam degiskeni tanimli degilse kapalidir (403). Ayni id ile
tekrar yuklemek kampanyayi gunceller (upsert), kopya olusturmaz.
"""
import json
import os
import sys
import urllib.error
import urllib.request

BASE = os.environ.get("MANIPULENS_URL", "http://127.0.0.1:3000")
TOKEN = os.environ.get("ADS_ADMIN_TOKEN")


def main() -> int:
    if not TOKEN:
        print("ADS_ADMIN_TOKEN tanimli degil; envanter ucu kapali.", file=sys.stderr)
        return 2
    if len(sys.argv) < 2:
        print(__doc__, file=sys.stderr)
        return 2

    with open(sys.argv[1], encoding="utf-8") as f:
        ads = json.load(f)

    ok = 0
    for ad in ads:
        body = json.dumps(ad, ensure_ascii=False).encode("utf-8")
        req = urllib.request.Request(
            f"{BASE}/v1/ads/inventory",
            data=body,
            headers={"Content-Type": "application/json", "x-ads-admin-token": TOKEN},
        )
        try:
            with urllib.request.urlopen(req, timeout=15) as resp:
                print(ad["id"], "->", resp.read().decode("utf-8"))
                ok += 1
        except urllib.error.HTTPError as e:
            print(ad["id"], "-> HATA", e.code, e.read().decode("utf-8", "replace"), file=sys.stderr)

    print(f"{ok}/{len(ads)} kampanya yazildi.")
    return 0 if ok == len(ads) else 1


if __name__ == "__main__":
    raise SystemExit(main())
