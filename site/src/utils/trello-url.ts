/** Trello verisinden gelen baglantilari yalniz resmi HTTPS alan adina sinirlar. */
export function guvenliTrelloUrl(deger: unknown): string | null {
  if (typeof deger !== 'string') return null;
  try {
    const url = new URL(deger);
    const trelloAlani = url.hostname === 'trello.com' || url.hostname.endsWith('.trello.com');
    return url.protocol === 'https:' && trelloAlani ? url.toString() : null;
  } catch {
    return null;
  }
}
