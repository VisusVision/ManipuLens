/**
 * Arayuz ikonlarinin cizim govdeleri - viewBox 0 0 24 24, tek renk.
 *
 * Neden ayri modul: Icon.astro bunlari sunucuda bileseneb dondurur, SearchModal
 * ise ayni cizimleri tarayicida `innerHTML` ile uretir. Tek kaynak olmazsa iki
 * kopya zamanla ayrisir.
 */
export const ICON_PATHS: Record<string, string> = {
  // Buyutec - arama tetikleyicisi ve arama modali basligi.
  ara: '<circle cx="11" cy="11" r="6.5"/><path d="M16 16l4.5 4.5"/>',

  // Satirlar - icindekiler cekmecesi.
  icindekiler:
    '<path d="M4 6h16M4 12h16M4 18h10"/><circle cx="20" cy="18" r="1.2" fill="currentColor" stroke="none"/>',

  // Klavye - kisayol yardimcisi.
  klavye:
    '<rect x="2.5" y="6" width="19" height="12" rx="2"/><path d="M6 10h.01M9.5 10h.01M13 10h.01M16.5 10h.01M6 13.5h.01M9.5 13.5h.01M13 13.5h.01M16.5 13.5h.01M8 16.5h8"/>',

  // Goz - sade mod (okuyan goz).
  sade: '<path d="M2 12s3.6-6 10-6 10 6 10 6-3.6 6-10 6-10-6-10-6z"/><circle cx="12" cy="12" r="2.6"/>',

  // Kaydiraklar - teknik mod (ayarlar acildi).
  teknik:
    '<path d="M4 7h9M17 7h3M4 17h3M11 17h9"/><circle cx="15" cy="7" r="2.2"/><circle cx="9" cy="17" r="2.2"/>',

  // Carpi - kapatma dugmeleri.
  kapat: '<path d="M6 6l12 12M18 6L6 18"/>',

  // Sayfa - arama sonuclarinda "Sayfa" kumesi.
  sayfa: '<path d="M6 3h8l4 4v14H6z"/><path d="M14 3v4h4"/><path d="M9 12h6M9 16h6"/>',

  // Simsek - "Ajan" kumesi (hizli karar veren birim).
  ajan: '<path d="M13 2.5L5.5 13.5H11l-.5 8L18 10.5h-5.5z"/>',

  // Bloklar - "Parca" kumesi (mimari bilesen).
  parca:
    '<rect x="3" y="3" width="8" height="8" rx="1.5"/><rect x="13" y="3" width="8" height="8" rx="1.5"/><rect x="3" y="13" width="8" height="8" rx="1.5"/><path d="M13 17h8M17 13v8"/>',

  // Acili parantez - "Kod" kumesi.
  kod: '<path d="M8.5 7.5L4 12l4.5 4.5M15.5 7.5L20 12l-4.5 4.5"/><path d="M13.5 4.5l-3 15"/>',

  // Etiket - "Sozluk" kumesi.
  sozluk: '<path d="M3.5 11.5V4.5h7l9.5 9.5-7 7z"/><circle cx="7.5" cy="8.5" r="1.4"/>',

  // Ust uste iki sayfa - kod kesiti kopyalama dugmesi.
  kopyala: '<rect x="9" y="9" width="11" height="11" rx="2"/><path d="M5.5 15H5a1 1 0 01-1-1V5a1 1 0 011-1h9a1 1 0 011 1v.5"/>',

  // Onay - kopyalama basarili geri bildirimi.
  onay: '<path d="M4.5 12.5l5 5 10-11"/>',

  // Ayrac - bolum degisimi bildirimi.
  bolum: '<path d="M6.5 3.5h11v17l-5.5-4-5.5 4z"/>',
};

/** Bilinmeyen ad geldiginde sessizce dusmesin diye tek yedek. */
export const ICON_FALLBACK = 'sayfa';

/**
 * Tarayicida `innerHTML` ile basilacak tam SVG dizesi.
 * Icon.astro ile ayni oznitelikleri tasir; ikisi ayni gorunmeli.
 */
export function iconSvg(ad: string, boyut = 18): string {
  const cizim = ICON_PATHS[ad] ?? ICON_PATHS[ICON_FALLBACK];
  return (
    `<svg class="ml-icon" width="${boyut}" height="${boyut}" viewBox="0 0 24 24" fill="none" ` +
    `stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" ` +
    `aria-hidden="true">${cizim}</svg>`
  );
}
