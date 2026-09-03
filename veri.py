"""ManipuLens veri okuyucu.

Analiz gecmisini ve demografi ajaninin urettigi profili tek komutla okunur
hale getirir. Veritabanini SALT OKUNUR acar; sunucu calisirken guvenlidir.

Kullanim:
    python veri.py           # terminale ozet bas
    python veri.py --html    # veri-raporu.html uret ve yolunu yaz

Neden ayri bir arac: ham `history` tablosunda ajan kararlari tek bir JSON
metni olarak duruyor, SQLite tarayicisinda okunmuyor. Burada acilip tabloya
donusturuluyor.
"""

import html as html_mod
import json
import sqlite3
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")

KOK = Path(__file__).resolve().parent
DB = KOK / "manipulens.db"
CIKTI = KOK / "veri-raporu.html"

AJAN_SIRASI = ["Dilsel", "Psikolojik", "Davranışsal", "Algısal", "Sosyal", "Pazarlama"]


def veriyi_oku():
    con = sqlite3.connect(f"file:{DB}?mode=ro", uri=True)
    analizler = []
    for r in con.execute(
        "select id, timestamp, lang, text_len, is_manipulated, dominant_manipulation,"
        " agents_json, predicted_product, text_preview, genel_sonuc"
        " from history order by id desc"
    ):
        ajanlar = json.loads(r[6]) if r[6] else []
        analizler.append(
            {
                "id": r[0],
                "ts": r[1][:19].replace("T", " "),
                "dil": r[2],
                "uzunluk": r[3],
                "manipulatif": bool(r[4]),
                "dominant": r[5],
                "ajanlar": {a["t"]: (a["d"], a["c"]) for a in ajanlar},
                "ajan_var": bool(ajanlar),
                "urun": r[7],
                "onizleme": r[8],
                "ozet": r[9],
            }
        )

    profiller = []
    for r in con.execute(
        "select user_id, analyzed_count, model_version, updated_at, profile_json"
        " from user_profiles"
    ):
        profiller.append(
            {
                "user_id": r[0],
                "sayi": r[1],
                "model": r[2],
                "guncellendi": r[3][:19].replace("T", " "),
                "veri": json.loads(r[4]),
            }
        )
    con.close()
    return analizler, profiller


def cagri_basarisiz(a):
    """Ajan cagrisi coktugunde butun guvenler 0.0 kalir; gercek 'temiz' sonuc degildir."""
    return a["ajan_var"] and all(g == 0.0 for _, g in a["ajanlar"].values())


# ---------------------------------------------------------------- terminal


def terminale_bas(analizler, profiller):
    print(f"\nANALIZLER ({len(analizler)} kayit, yeniden eskiye)\n" + "-" * 76)
    for a in analizler:
        durum = "BOZUK CAGRI" if cagri_basarisiz(a) else ("MANIPULATIF" if a["manipulatif"] else "temiz")
        print(f"\n[{a['id']}] {a['ts']}  {a['uzunluk']} karakter  -> {durum}")
        print(f"     {a['onizleme'][:66]}")
        if not cagri_basarisiz(a):
            tespit = [f"{t} {g:.2f}" for t, (d, g) in a["ajanlar"].items() if d]
            print(f"     dominant={a['dominant']}   tespit: {', '.join(tespit) or 'yok'}")
        if a["urun"]:
            print(f"     urun tahmini: {a['urun'][:70]}")

    gecerli = [a for a in analizler if not cagri_basarisiz(a)]
    manip = sum(1 for a in gecerli if a["manipulatif"])
    print("\n" + "-" * 76)
    print(f"gecerli analiz: {len(gecerli)}   manipulatif: {manip}   temiz: {len(gecerli) - manip}")
    if gecerli:
        oran = 100 * manip / len(gecerli)
        uyari = "  <-- her metne 'var' diyor, kalibrasyon suphesi" if oran > 90 else ""
        print(f"manipulasyon orani: %{oran:.0f}{uyari}")
    bozuk = len(analizler) - len(gecerli)
    if bozuk:
        print(f"bozuk cagri (LLM'e ulasilamamis, sayilmadi): {bozuk}")

    print("\nDEMOGRAFI AJANI\n" + "-" * 76)
    if not profiller:
        print("profil yok. Ajan 5 analiz esigine gelince kosar (PROFILE_MIN_ANALYSES=5).")
        print(f"su anki analiz sayisi: {len(analizler)}")
    for p in profiller:
        print(f"durum: CALISIYOR   {p['sayi']} analiz islendi   son kosu: {p['guncellendi']}")
        print(f"model surumu: {p['model']}")
        c = p["veri"].get("inference") or {}
        if not c:
            print("  cikarim bos - ajan cagrisi basarisiz olmus, eski profil korunmus")
        for alan in ("yas_araligi", "cinsiyet", "egitim_seviyesi", "tuketici_egilimi"):
            d = c.get(alan)
            if isinstance(d, dict):
                print(f"  {alan:18s} {str(d.get('deger')):24s} guven {d.get('guven', 0):.2f}")
                print(f"  {'':18s} dayanak: {str(d.get('dayanak'))[:70]}")
        if c.get("ilgi_alanlari"):
            print(f"  {'ilgi_alanlari':18s} {', '.join(c['ilgi_alanlari'])}")


# -------------------------------------------------------------------- html


def hucre(a, ajan):
    if cagri_basarisiz(a):
        return '<td class="bos">-</td>'
    d, g = a["ajanlar"].get(ajan, (False, 0.0))
    if not d:
        return '<td class="bos">-</td>'
    return f'<td class="var">{g:.2f}</td>'


def html_uret(analizler, profiller):
    gecerli = [a for a in analizler if not cagri_basarisiz(a)]
    manip = sum(1 for a in gecerli if a["manipulatif"])
    oran = (100 * manip / len(gecerli)) if gecerli else 0

    satirlar = []
    for a in analizler:
        if cagri_basarisiz(a):
            rozet = '<span class="rozet bozuk">bozuk cagri</span>'
        elif a["manipulatif"]:
            rozet = '<span class="rozet manip">manipulatif</span>'
        else:
            rozet = '<span class="rozet temiz">temiz</span>'
        hucreler = "".join(hucre(a, ajan) for ajan in AJAN_SIRASI)
        satirlar.append(
            f"<tr><td class='no'>{a['id']}</td>"
            f"<td class='ts'>{a['ts']}</td>"
            f"<td class='metin'>{html_mod.escape(a['onizleme'][:110])}"
            f"<div class='alt'>{a['uzunluk']} karakter &middot; {rozet}"
            f"{' &middot; dominant: ' + html_mod.escape(a['dominant']) if a['dominant'] and a['dominant'] != 'Yok' else ''}</div>"
            f"{'<div class=urun>urun tahmini: ' + html_mod.escape(str(a['urun'])[:110]) + '</div>' if a['urun'] else ''}"
            f"</td>{hucreler}</tr>"
        )

    prof_html = "<p class='bos-not'>Henuz profil yok. Demografi ajani 5 analiz esiginde kosar.</p>"
    if profiller:
        p = profiller[0]
        c = p["veri"].get("inference") or {}
        st = p["veri"].get("stats") or {}
        alanlar = []
        for anahtar, etiket in [
            ("yas_araligi", "Yas araligi"),
            ("cinsiyet", "Cinsiyet"),
            ("egitim_seviyesi", "Egitim"),
            ("tuketici_egilimi", "Tuketici egilimi"),
        ]:
            d = c.get(anahtar)
            if isinstance(d, dict):
                alanlar.append(
                    f"<tr><td>{etiket}</td><td class='deger'>{html_mod.escape(str(d.get('deger')))}</td>"
                    f"<td class='guven'>{d.get('guven', 0):.2f}</td>"
                    f"<td class='dayanak'>{html_mod.escape(str(d.get('dayanak', ''))[:150])}</td></tr>"
                )
        prof_html = f"""
        <p class="calisiyor">Durum: CALISIYOR &mdash; {p['sayi']} analiz islendi, son kosu {p['guncellendi']},
           model surumu <code>{p['model']}</code></p>
        <h3>Sayac katmani (LLM yok, dogrudan veriden)</h3>
        <p class="sayac">toplam {st.get('total')} analiz &middot; {st.get('manipulated')} manipulatif &middot;
           ortalama {st.get('avg_text_len')} karakter &middot; diller: {', '.join(st.get('lang_counts', {}))}</p>
        <h3>Cikarim katmani (demografi ajani, llama3)</h3>
        <table class="profil"><thead><tr><th>Alan</th><th>Deger</th><th>Guven</th><th>Ajanin dayanagi</th></tr></thead>
        <tbody>{''.join(alanlar)}</tbody></table>
        <p class="ozet">{html_mod.escape(str(c.get('ozet', '')))}</p>
        """

    return f"""<!doctype html>
<html lang="tr"><head><meta charset="utf-8">
<title>ManipuLens veri raporu</title>
<style>
 body{{font:14px/1.55 system-ui,Segoe UI,sans-serif;margin:0;background:#f6f7f9;color:#1a1d23}}
 .sar{{max-width:1180px;margin:0 auto;padding:32px 24px 64px}}
 h1{{font-size:22px;margin:0 0 4px}} h2{{font-size:16px;margin:36px 0 12px}}
 h3{{font-size:13px;text-transform:uppercase;letter-spacing:.06em;color:#5b6472;margin:20px 0 8px}}
 .ust{{color:#5b6472;margin:0 0 24px;font-size:13px}}
 .kutular{{display:flex;gap:12px;flex-wrap:wrap;margin-bottom:8px}}
 .kutu{{background:#fff;border:1px solid #e2e5ea;border-radius:10px;padding:14px 18px;min-width:130px}}
 .kutu b{{display:block;font-size:24px;font-weight:600}}
 .kutu span{{font-size:12px;color:#5b6472}}
 .uyari{{background:#fff4e5;border:1px solid #f0c48a;border-radius:10px;padding:12px 16px;margin:16px 0;font-size:13px}}
 table{{width:100%;border-collapse:collapse;background:#fff;border:1px solid #e2e5ea;border-radius:10px;overflow:hidden}}
 th,td{{padding:9px 11px;text-align:left;border-bottom:1px solid #eef0f3;vertical-align:top}}
 th{{background:#fafbfc;font-size:11px;text-transform:uppercase;letter-spacing:.05em;color:#5b6472}}
 th.ajan{{writing-mode:horizontal-tb;font-size:10px;text-align:center;width:64px}}
 td.var{{text-align:center;background:#fdecec;color:#b42318;font-weight:600;font-variant-numeric:tabular-nums}}
 td.bos{{text-align:center;color:#c3c8d0}}
 .no{{color:#98a0ac;font-variant-numeric:tabular-nums;width:34px}}
 .ts{{color:#5b6472;font-size:12px;white-space:nowrap;font-variant-numeric:tabular-nums}}
 .metin{{max-width:460px}}
 .alt{{color:#5b6472;font-size:12px;margin-top:3px}}
 .urun{{color:#8a5a00;font-size:12px;margin-top:3px}}
 .rozet{{display:inline-block;padding:1px 7px;border-radius:20px;font-size:11px}}
 .manip{{background:#fdecec;color:#b42318}} .temiz{{background:#e8f5ec;color:#116b34}}
 .bozuk{{background:#eceff3;color:#5b6472}}
 .calisiyor{{background:#e8f5ec;border:1px solid #b5dfc4;border-radius:10px;padding:12px 16px;color:#116b34}}
 .profil td.deger{{font-weight:600}} .profil td.guven{{font-variant-numeric:tabular-nums;width:60px}}
 .profil td.dayanak{{color:#5b6472;font-size:12px}}
 .ozet{{background:#fff;border:1px solid #e2e5ea;border-radius:10px;padding:12px 16px;margin-top:12px}}
 .sayac,.bos-not{{color:#5b6472}}
 code{{background:#eef0f3;padding:1px 5px;border-radius:4px;font-size:12px}}
</style></head><body><div class="sar">
<h1>ManipuLens veri raporu</h1>
<p class="ust">Kaynak: <code>manipulens.db</code> &rarr; <code>history</code> + <code>user_profiles</code>.
Uretim: <code>python veri.py --html</code></p>

<div class="kutular">
 <div class="kutu"><b>{len(gecerli)}</b><span>gecerli analiz</span></div>
 <div class="kutu"><b>{manip}</b><span>manipulatif</span></div>
 <div class="kutu"><b>{len(gecerli) - manip}</b><span>temiz</span></div>
 <div class="kutu"><b>%{oran:.0f}</b><span>manipulasyon orani</span></div>
 <div class="kutu"><b>{len(analizler) - len(gecerli)}</b><span>bozuk cagri</span></div>
</div>
{'<div class="uyari"><b>Kalibrasyon suphesi:</b> gecerli analizlerin %' + f'{oran:.0f}' + "'i manipulatif isaretlendi. Bir siniflandirici her metne ayni etiketi veriyorsa girdiye bakmiyor olabilir. Notr kontrol metniyle (ansiklopedi paragrafi, tarif) dogrula.</div>" if oran > 90 and gecerli else ''}

<h2>Analizler</h2>
<table><thead><tr><th>#</th><th>Zaman</th><th>Metin</th>
{''.join(f'<th class="ajan">{a}</th>' for a in AJAN_SIRASI)}
</tr></thead><tbody>{''.join(satirlar)}</tbody></table>
<p class="ust">Ajan sutunlarindaki sayi guven degeridir; bos hucre o ajanin tespit bulmadigini gosterir.</p>

<h2>Demografi ajani</h2>
{prof_html}
</div></body></html>"""


def main():
    if not DB.exists():
        print(f"Veritabani bulunamadi: {DB}")
        return 1
    analizler, profiller = veriyi_oku()
    if "--html" in sys.argv:
        CIKTI.write_text(html_uret(analizler, profiller), encoding="utf-8")
        print(f"Rapor yazildi: {CIKTI}")
    else:
        terminale_bas(analizler, profiller)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
