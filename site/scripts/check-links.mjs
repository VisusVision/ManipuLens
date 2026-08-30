import { readdir, readFile, stat } from 'node:fs/promises';
import { dirname, extname, join, normalize, relative, resolve } from 'node:path';

const dist = resolve('dist');
const htmlDosyalari = [];
const tumDosyalar = new Set();

async function tara(dizin) {
  for (const ad of await readdir(dizin)) {
    const yol = join(dizin, ad);
    const bilgi = await stat(yol);
    if (bilgi.isDirectory()) await tara(yol);
    else {
      tumDosyalar.add(yol);
      if (extname(yol) === '.html') htmlDosyalari.push(yol);
    }
  }
}

await tara(dist);

const htmlHaritasi = new Map(
  await Promise.all(htmlDosyalari.map(async (yol) => [yol, await readFile(yol, 'utf8')])),
);
const hatalar = [];

function hedefDosya(kaynak, pathname) {
  const temiz = decodeURIComponent(pathname).replace(/^\/+/, '');
  const mutlak = pathname.startsWith('/')
    ? join(dist, temiz)
    : normalize(join(dirname(kaynak), temiz));
  if (extname(mutlak)) return tumDosyalar.has(mutlak) ? mutlak : null;
  for (const aday of [mutlak, `${mutlak}.html`, join(mutlak, 'index.html')]) {
    if (htmlHaritasi.has(aday)) return aday;
  }
  return null;
}

for (const [kaynak, html] of htmlHaritasi) {
  for (const eslesme of html.matchAll(/href=["']([^"']+)["']/g)) {
    const ham = eslesme[1].replaceAll('&amp;', '&');
    if (/^(?:https?:|mailto:|tel:|javascript:|data:)/i.test(ham)) continue;

    const [yolBolumu, parca = ''] = ham.split('#', 2);
    const hedef = yolBolumu ? hedefDosya(kaynak, yolBolumu.split('?')[0]) : kaynak;
    if (!hedef) {
      hatalar.push(`${relative(dist, kaynak)} -> ${ham} (rota yok)`);
      continue;
    }
    if (parca && htmlHaritasi.has(hedef)) {
      const id = decodeURIComponent(parca);
      const hedefHtml = htmlHaritasi.get(hedef) ?? '';
      const guvenliId = id.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      if (!new RegExp(`\\sid=["']${guvenliId}["']`).test(hedefHtml)) {
        hatalar.push(`${relative(dist, kaynak)} -> ${ham} (çapa yok)`);
      }
    }
  }
}

if (hatalar.length) {
  console.error(`Bozuk iç bağlantı: ${hatalar.length}`);
  hatalar.forEach((hata) => console.error(`- ${hata}`));
  process.exit(1);
}

console.log(`${htmlDosyalari.length} sayfada iç bağlantılar ve çapalar geçerli.`);
