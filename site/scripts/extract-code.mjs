/**
 * Kod okuma katmaninin cikarma betigi.
 *
 * Gercek kaynaktan (../src/*.rs, ../extension/*.js) satir araligi keser,
 * hash'ler ve src/data/kod/<id>.json olarak yazar.
 *
 * Neden elle kopyalanmiyor: elle kopyalanan kod sessizce eskiyor ve site yalan
 * soylemeye baslar. JSON, kesildigi andaki DOSYANIN TAMAMININ hash'ini tutar;
 * kaynak degisirse hash tutmaz ve sayfa "kod degismis, aciklama dogrulanmadi"
 * rozetiyle cikar. Sessiz eskimeyi imkansiz kilan mekanizma bu.
 *
 * Kullanim:
 *   npm run kod          cikar ve yaz
 *   npm run kod -- --kontrol   yalniz kontrol et, yazma (CI icin)
 */
import { readFile, writeFile, mkdir, readdir } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const buradan = dirname(fileURLToPath(import.meta.url));
const KAYNAK_KOK = resolve(buradan, '../..'); // ManipuLens deposunun koku
const ICERIK = resolve(buradan, '../src/content/kod');
const HEDEF = resolve(buradan, '../src/data/kod');

const yalnizKontrol = process.argv.includes('--kontrol');

/** Minik frontmatter okuyucu: string, sayi, dizi. */
function frontmatterAyristir(ham, dosyaAdi) {
  const m = /^---\r?\n([\s\S]*?)\r?\n---\r?\n?([\s\S]*)$/.exec(ham);
  if (!m) throw new Error(`${dosyaAdi}: frontmatter bulunamadi`);
  const alanlar = {};
  for (const satir of m[1].split(/\r?\n/)) {
    const e = /^([a-z_]+):\s*(.*)$/.exec(satir);
    if (!e) continue;
    let [, anahtar, deger] = e;
    deger = deger.trim();
    if (deger.startsWith('[')) deger = JSON.parse(deger);
    else if (/^-?\d+$/.test(deger)) deger = Number(deger);
    else deger = deger.replace(/^"([\s\S]*)"$/, '$1').replace(/\\"/g, '"');
    alanlar[anahtar] = deger;
  }
  return alanlar;
}

const dosyalar = (await readdir(ICERIK)).filter((f) => f.endsWith('.md')).sort();
if (!dosyalar.length) {
  console.warn('UYARI: src/content/kod altinda hic parca yok.');
  process.exit(0);
}

const kaynakOnbellek = new Map();
async function kaynagiOku(gorecelYol) {
  if (!kaynakOnbellek.has(gorecelYol)) {
    kaynakOnbellek.set(gorecelYol, await readFile(join(KAYNAK_KOK, gorecelYol), 'utf8'));
  }
  return kaynakOnbellek.get(gorecelYol);
}

await mkdir(HEDEF, { recursive: true });

let degisen = 0;
let bozuk = 0;

for (const dosya of dosyalar) {
  const id = dosya.replace(/\.md$/, '');
  const ham = await readFile(join(ICERIK, dosya), 'utf8');
  const fm = frontmatterAyristir(ham, dosya);

  // Uc soru kurali: uculden biri eksikse parca yayina girmez.
  for (const alan of ['ne_yapiyor', 'neden_boyle', 'kaldirirsak']) {
    if (!fm[alan] || String(fm[alan]).trim() === '') {
      console.error(`HATA ${dosya}: "${alan}" bos. Uc sorunun ucu de dolu olmali.`);
      bozuk++;
    }
  }
  if (!Array.isArray(fm.aralik) || fm.aralik.length !== 2) {
    console.error(`HATA ${dosya}: "aralik" [basla, bitir] olmali.`);
    bozuk++;
    continue;
  }

  let kaynak;
  try {
    kaynak = await kaynagiOku(fm.dosya);
  } catch {
    console.error(`HATA ${dosya}: kaynak bulunamadi -> ${fm.dosya}`);
    bozuk++;
    continue;
  }

  const satirlar = kaynak.split(/\r?\n/);
  const [bas, son] = fm.aralik;
  if (bas < 1 || son > satirlar.length || bas > son) {
    console.error(
      `HATA ${dosya}: aralik disarida (${bas}-${son}), dosyada ${satirlar.length} satir var.`,
    );
    bozuk++;
    continue;
  }

  const kesit = satirlar.slice(bas - 1, son);

  // Ortak girintiyi kirp: kod bloklari sayfada gereksiz bosluk tasimasin.
  const doluSatirlar = kesit.filter((s) => s.trim() !== '');
  const enAzGirinti = doluSatirlar.length
    ? Math.min(...doluSatirlar.map((s) => s.match(/^\s*/)[0].length))
    : 0;
  const kirpilmis = kesit.map((s) => s.slice(enAzGirinti));

  const dosyaHash = createHash('sha256').update(kaynak).digest('hex').slice(0, 12);
  const kesitHash = createHash('sha256').update(kesit.join('\n')).digest('hex').slice(0, 12);

  const cikti = {
    id,
    dosya: fm.dosya,
    dil: fm.dil ?? 'text',
    baslangic_satiri: bas,
    bitis_satiri: son,
    satirlar: kirpilmis,
    dosya_hash: dosyaHash,
    kesit_hash: kesitHash,
    cekildi: new Date().toISOString(),
  };

  const yol = join(HEDEF, `${id}.json`);
  let onceki = null;
  try {
    onceki = JSON.parse(await readFile(yol, 'utf8'));
  } catch {
    /* ilk cekim */
  }

  if (onceki && onceki.kesit_hash === kesitHash) continue; // degismemis

  if (yalnizKontrol) {
    console.error(
      `DEGISMIS ${id}: ${fm.dosya}:${bas}-${son} kaynakta degisti, aciklama gozden gecirilmeli.`,
    );
    degisen++;
    continue;
  }

  await writeFile(yol, JSON.stringify(cikti, null, 2) + '\n', 'utf8');
  degisen++;
}

if (bozuk) {
  console.error(`\n${bozuk} parca hatali. Derleme durduruldu.`);
  process.exit(1);
}

if (yalnizKontrol) {
  if (degisen) {
    console.error(`\n${degisen} parcanin kaynagi degismis.`);
    process.exit(1);
  }
  console.log(`Kod parcalari guncel (${dosyalar.length} parca).`);
} else {
  console.log(
    `Kod cikarildi - ${dosyalar.length} parca, ${degisen} tanesi yeniden yazildi.`,
  );
}
