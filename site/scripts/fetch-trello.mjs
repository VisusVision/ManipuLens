/**
 * Trello panosunu derleme zamaninda ceker, src/data/pano.json olarak yazar.
 *
 * Tarayiciya HICBIR sir gitmez: anahtar yalniz bu betigin ortaminda durur,
 * cikti sadece temizlenmis JSON'dur.
 *
 * Ortam degiskenleri (yerelde .env, CI'da GitHub Secrets):
 *   TRELLO_BOARD   pano kisa kodu (varsayilan: 5KCEsJ51)
 *   TRELLO_KEY     API anahtari
 *   TRELLO_TOKEN   okuma yetkili token
 *   TRELLO_FILTRE  'beyaz' (varsayilan: yalniz `site` etiketli kartlar) | 'kara' (yalniz
 *                  `gizli` etiketliler haric) | 'hepsi'
 *
 * Anahtar yoksa betik HATA VERMEZ: bos ama gecerli bir pano.json yazar ve site
 * "veri yok" durumunu durustce gosterir. Boylece derleme sirsiz makinede de calisir.
 *
 * Kullanim: npm run pano
 */
import { writeFile, mkdir, readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const buradan = dirname(fileURLToPath(import.meta.url));
const HEDEF = resolve(buradan, '../src/data/pano.json');
const ENV_DOSYASI = resolve(buradan, '../.env');

/** .env varsa oku - kutuphane kullanmadan, sadece KEY=deger satirlari. */
async function envYukle() {
  try {
    const ham = await readFile(ENV_DOSYASI, 'utf8');
    for (const satir of ham.split(/\r?\n/)) {
      const m = /^\s*([A-Z_]+)\s*=\s*(.*)\s*$/.exec(satir);
      if (m && !process.env[m[1]]) process.env[m[1]] = m[2].replace(/^["']|["']$/g, '');
    }
  } catch {
    /* .env yoksa sorun degil */
  }
}
await envYukle();

const BOARD = process.env.TRELLO_BOARD || '5KCEsJ51';
const KEY = process.env.TRELLO_KEY;
const TOKEN = process.env.TRELLO_TOKEN;
const FILTRE = process.env.TRELLO_FILTRE || 'beyaz';

/** Liste adindan durum cikar - Trello'da liste adlari degisebilir, esnek eslesme. */
function durumCikar(ad) {
  const a = ad.toLocaleLowerCase('tr');
  if (/(bitti|tamam|done|bitmiş|bitmis|yapıldı|yapildi)/.test(a)) return 'bitti';
  if (/(yapılıyor|yapiliyor|devam|progress|sürüyor|surüyor|suruyor)/.test(a)) return 'yapiliyor';
  if (/(bekle|blok|blocked|park)/.test(a)) return 'bekliyor';
  return 'yapilacak';
}

/** Aciklamayi kirp ve gizli isaretinden sonrasini at. */
function aciklamaTemizle(desc) {
  if (!desc) return '';
  const kesik = desc.split('<!-- gizli -->')[0].trim();
  return kesik.length > 240 ? kesik.slice(0, 237).trimEnd() + '…' : kesik;
}

function basHarfler(adSoyad) {
  return adSoyad
    .split(/\s+/)
    .slice(0, 2)
    .map((p) => p[0] || '')
    .join('')
    .toLocaleUpperCase('tr');
}

async function cek(yol, ekParam = {}) {
  const url = new URL('https://api.trello.com/1' + yol);
  url.searchParams.set('key', KEY);
  url.searchParams.set('token', TOKEN);
  for (const [k, v] of Object.entries(ekParam)) url.searchParams.set(k, v);

  const cevap = await fetch(url);
  if (!cevap.ok) {
    // URL'i basmiyoruz: icinde anahtar var.
    throw new Error(`Trello ${yol} -> HTTP ${cevap.status}`);
  }
  return cevap.json();
}

function bosPano(sebep) {
  return {
    durum: 'veri-yok',
    sebep,
    pano: { kod: BOARD, ad: null, url: `https://trello.com/b/${BOARD}` },
    cekildi: new Date().toISOString(),
    sutunlar: [],
    uyeler: [],
    sayilar: { kart: 0, gosterilen: 0 },
  };
}

async function panoyuCek() {
  const [pano, listeler, kartlar, uyeler] = await Promise.all([
    cek(`/boards/${BOARD}`, { fields: 'name,url,shortUrl' }),
    cek(`/boards/${BOARD}/lists`, { fields: 'name,pos', filter: 'open' }),
    cek(`/boards/${BOARD}/cards`, {
      fields: 'name,desc,idList,idMembers,labels,due,dueComplete,shortUrl,dateLastActivity',
      filter: 'open',
    }),
    cek(`/boards/${BOARD}/members`, { fields: 'fullName,username' }),
  ]);

  const uyeAdi = Object.fromEntries(uyeler.map((u) => [u.id, u.fullName || u.username]));

  const gorunur = kartlar.filter((k) => {
    const etiketler = (k.labels || []).map((e) => (e.name || '').toLocaleLowerCase('tr'));
    if (FILTRE === 'hepsi') return true;
    if (FILTRE === 'kara') return !etiketler.includes('gizli');
    return etiketler.includes('site'); // beyaz liste - varsayilan
  });

  const sutunlar = listeler
    .sort((a, b) => a.pos - b.pos)
    .map((l) => ({
      id: l.id,
      ad: l.name,
      durum: durumCikar(l.name),
      kartlar: gorunur
        .filter((k) => k.idList === l.id)
        .map((k) => ({
          id: k.id,
          ad: k.name,
          aciklama: aciklamaTemizle(k.desc),
          kimde: (k.idMembers || []).map((id) => ({
            ad: uyeAdi[id] || 'Bilinmeyen',
            harfler: basHarfler(uyeAdi[id] || '??'),
          })),
          etiketler: (k.labels || [])
            .filter((e) => e.name)
            .map((e) => ({ ad: e.name, renk: e.color || 'gri' })),
          bitis: k.due || null,
          bitisTamam: !!k.dueComplete,
          url: k.shortUrl,
          sonHareket: k.dateLastActivity || null,
        })),
    }));

  return {
    durum: 'tamam',
    pano: { kod: BOARD, ad: pano.name, url: pano.shortUrl || pano.url },
    filtre: FILTRE,
    cekildi: new Date().toISOString(),
    sutunlar,
    uyeler: uyeler.map((u) => ({
      ad: u.fullName || u.username,
      harfler: basHarfler(u.fullName || u.username),
    })),
    sayilar: { kart: kartlar.length, gosterilen: gorunur.length },
  };
}

let cikti;
if (!KEY || !TOKEN) {
  cikti = bosPano('TRELLO_KEY / TRELLO_TOKEN tanimli degil');
  console.warn('UYARI: Trello anahtari yok, bos pano.json yazildi. Site "veri yok" gosterecek.');
} else {
  try {
    cikti = await panoyuCek();
    console.log(
      `pano.json yazildi - ${cikti.sayilar.gosterilen}/${cikti.sayilar.kart} kart, ` +
        `${cikti.sutunlar.length} sutun, filtre: ${FILTRE}`,
    );
  } catch (hata) {
    cikti = bosPano(String(hata.message || hata));
    console.error('HATA:', hata.message);
    if (process.env.CI) process.exit(1); // CI'da sessiz bozulma olmasin
  }
}

// Son savunma: cikti icinde anahtar sizmis mi?
const metin = JSON.stringify(cikti, null, 2);
if (KEY && metin.includes(KEY)) {
  console.error('HATA: cikti API anahtari iceriyor, yazilmadi.');
  process.exit(1);
}

await mkdir(dirname(HEDEF), { recursive: true });
await writeFile(HEDEF, metin + '\n', 'utf8');
