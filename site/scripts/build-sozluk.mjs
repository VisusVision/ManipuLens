/**
 * docs/sozluk.md uretici.
 *
 * Sozlugun tek dogru kaynagi site/src/content/sozluk/*.md dosyalaridir.
 * Referans sitede sozluk iki yerde elle tutuluyordu ve ayrisiyordu; biz ayni
 * sorunu senkron disipliniyle degil, uretimle cozuyoruz.
 *
 * Kullanim: npm run sozluk
 */
import { readdir, readFile, writeFile, mkdir } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const buradan = dirname(fileURLToPath(import.meta.url));
const KAYNAK = resolve(buradan, '../src/content/sozluk');
const HEDEF = resolve(buradan, '../../docs/sozluk.md');

const KUME_ADI = {
  manipulasyon: 'Manipülasyon dili',
  'yapay-zeka': 'Yapay zekâ',
  sistem: 'ManipuLens sistemi',
  web: 'Web, uzantı, altyapı',
};
const KUME_SIRA = ['manipulasyon', 'yapay-zeka', 'sistem', 'web'];
const ALAN_ADI = {
  genel: 'Genel terimler',
  manipulens: "ManipuLens'e özel terimler",
};
const ALAN_ACIKLAMA = {
  genel: "ManipuLens'ten bağımsız olarak her yerde geçerli kavramlar.",
  manipulens: 'Yalnız bu projede anlamı olan terimler; her birinin kodda karşılığı var.',
};
const ALAN_SIRA = ['genel', 'manipulens'];

/** Minik frontmatter okuyucu: string, bool ve JSON dizi degerleri yeter. */
function frontmatterAyristir(ham) {
  const m = /^---\r?\n([\s\S]*?)\r?\n---\r?\n?([\s\S]*)$/.exec(ham);
  if (!m) throw new Error('frontmatter bulunamadi');
  const alanlar = {};
  for (const satir of m[1].split(/\r?\n/)) {
    const eslesme = /^([a-z_]+):\s*(.*)$/.exec(satir);
    if (!eslesme) continue;
    const [, anahtar, hamDeger] = eslesme;
    let deger = hamDeger.trim();
    if (deger.startsWith('[')) deger = JSON.parse(deger);
    else if (deger === 'true' || deger === 'false') deger = deger === 'true';
    else deger = deger.replace(/^"([\s\S]*)"$/, '$1').replace(/\\"/g, '"');
    alanlar[anahtar] = deger;
  }
  return { data: alanlar, govde: m[2].trim() };
}

const dosyalar = (await readdir(KAYNAK)).filter((f) => f.endsWith('.md')).sort();

const terimler = [];
for (const dosya of dosyalar) {
  const ham = await readFile(join(KAYNAK, dosya), 'utf8');
  const { data, govde } = frontmatterAyristir(ham);
  terimler.push({ slug: dosya.replace(/\.md$/, ''), govde, ...data });
}

// Sema sozlesmesi burada da kontrol edilir: uretim sessizce bozuk cikmasin.
const eksik = terimler.filter((t) => t.kume === 'manipulasyon' && !t.ornek);
if (eksik.length) {
  console.error(
    'HATA: manipulasyon kumesinde ornek cumlesi olmayan terim var:',
    eksik.map((t) => t.slug).join(', '),
  );
  process.exit(1);
}

const bugun = new Date().toISOString().slice(0, 10);
const satirlar = [
  '<!-- URETILMIS DOSYA - ELLE DUZENLEME.',
  '     Kaynak: site/src/content/sozluk/*.md · Uretici: site/scripts/build-sozluk.mjs',
  '     Yeniden uretmek icin: cd site && npm run sozluk -->',
  '',
  '# ManipuLens Sözlüğü',
  '',
  `${terimler.length} terim · son üretim ${bugun}`,
  '',
  'Ekibin ortak dil sözleşmesi. Sitedeki karşılığı: `/sozluk`.',
  '',
];

for (const alan of ALAN_SIRA) {
  const yari = terimler.filter((t) => t.alan === alan);
  if (!yari.length) continue;
  satirlar.push(`## ${ALAN_ADI[alan]} (${yari.length})`, '', ALAN_ACIKLAMA[alan], '');

  for (const kume of KUME_SIRA) {
  const grup = yari
    .filter((t) => t.kume === kume)
    .sort((a, b) => a.terim.localeCompare(b.terim, 'tr'));
  if (!grup.length) continue;

  satirlar.push(`### ${KUME_ADI[kume]} (${grup.length})`, '');
  for (const t of grup) {
    satirlar.push(`#### ${t.terim}`, '', t.kisaca, '');
    if (t.ornek) satirlar.push(`> **Örnek:** ${t.ornek}`, '');
    if (t.kod_capasi) satirlar.push(`Kodda: \`${t.kod_capasi}\``, '');
    if (t.govde) satirlar.push(t.govde, '');
    if (Array.isArray(t.ilgili) && t.ilgili.length) {
      satirlar.push(`İlgili: ${t.ilgili.map((s) => `\`${s}\``).join(', ')}`, '');
    }
  }
  }
}

await mkdir(dirname(HEDEF), { recursive: true });
await writeFile(HEDEF, satirlar.join('\n'), 'utf8');
console.log(`docs/sozluk.md yazildi - ${terimler.length} terim`);
