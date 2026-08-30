import { defineCollection } from 'astro:content';
import { glob } from 'astro/loaders';
import { z } from 'astro/zod';

/** Her rehber sayfasinin tasidigi kanit alanlari. */
const kanit = {
  verified_at_commit: z.string().optional(),
  verified_at: z.coerce.date().optional(),
  owner: z.string().optional(),
  needs_reverify: z.boolean().default(false),
};

const ajanlar = defineCollection({
  loader: glob({ base: './src/content/ajanlar', pattern: '**/*.md' }),
  schema: z.object({
    title: z.string(),
    /** Backend'in `manipulation_type` alaninda gonderdigi tam deger. */
    manipulation_type: z.string(),
    /** extension/background.js icindeki getProgressBarColor haritasi ile ayni olmali. */
    color: z.string().regex(/^#[0-9a-fA-F]{6}$/),
    order: z.number(),
    source_fn: z.string(),
    kisaca: z.string(),
    /** Teknik mod: ajanin prompt sinirlarini kaynak kodla birlikte ogretir. */
    teknik_odak: z.array(z.string().min(10)).min(3).max(5),
    prompt_karari: z.string().min(40),
    yanlis_pozitif: z.string().min(30),
    ...kanit,
  }),
});

const guncellemeler = defineCollection({
  loader: glob({ base: './src/content/guncellemeler', pattern: '**/*.md' }),
  schema: z.object({
    ...kanit,
    title: z.string(),
    date: z.coerce.date(),
    version: z.string().optional(),
    component: z.enum(['backend', 'extension', 'agents', 'devops', 'docs', 'site']),
    owner: z.string(),
    status: z.enum(['draft', 'released', 'reverted']).default('draft'),
    visibility: z.enum(['public', 'team']).default('team'),
    simple_summary: z.string(),
    technical_summary: z.string(),
    why_changed: z.string().optional(),
    /** Kullanicinin eline ne geciyor - teknik degil, fayda cumlesi. */
    ne_ise_yarar: z.string().optional(),
    impact: z.string().optional(),
    /**
     * Once/sonra karsilastirmasi. Her satir tek bir alanin eski ve yeni halini
     * tasir; "iyilestirildi" gibi olculemez ifade yerine iki somut durum yazilir.
     * Bos birakilabilir ama bos birakilan kayit degisimin ne oldugunu gostermez.
     */
    karsilastirma: z
      .array(
        z.object({
          alan: z.string().min(3),
          once: z.string().min(3),
          sonra: z.string().min(3),
        }),
      )
      .default([]),
    tests: z.string().optional(),
    known_issues: z.array(z.string()).default([]),
    trello_card: z.url().optional(),
    commit_or_pr: z.url().optional(),
  }),
});

/**
 * Sozluk - ekibin ortak dil sozlesmesi.
 * Yazim kurallari (detay planı Bolum C.2): 1-3 cumle, ilk cumle tanim ikincisi yanlis
 * anlamayi oldurur, somut benzetme, terimi terimle tanimlama, tanim bir tavir tasir.
 */
const sozluk = defineCollection({
  loader: glob({ base: './src/content/sozluk', pattern: '**/*.md' }),
  schema: z
    .object({
      terim: z.string(),
      /** Sozluk iki yariya ayrilir: her yerde gecerli genel kavramlar ve
       *  yalniz ManipuLens'te anlami olan proje terimleri. */
      alan: z.enum(['genel', 'manipulens']),
      kume: z.enum(['manipulasyon', 'yapay-zeka', 'sistem', 'web']),
      /** Sade tanim. Teknik terim kullanmaz. */
      kisaca: z.string().max(280),
      /** Manipulasyon kumesinde zorunlu: gercek bir ornek cumle. */
      ornek: z.string().optional(),
      /** Kod karsiligi olan terim capasini tasir: dosya veya fonksiyon. */
      kod_capasi: z.string().optional(),
      /** Diger terimlerin slug'lari. */
      ilgili: z.array(z.string()).default([]),
      /** Metin icinde otomatik baglanacak ek yazimlar. */
      esanlam: z.array(z.string()).default([]),
      ...kanit,
    })
    .refine((d) => d.kume !== 'manipulasyon' || !!d.ornek, {
      message: 'Manipulasyon kumesindeki her terim ornek cumle tasimak zorunda.',
      path: ['ornek'],
    }),
});

/**
 * Zincir - metin seciminden rapora uzanan yedi halka.
 * Icerik JS icinde gomulu degil; her halka ayri Markdown, sema dogrulamali.
 */
const zincir = defineCollection({
  loader: glob({ base: './src/content/zincir', pattern: '**/*.md' }),
  schema: z.object({
    title: z.string(),
    order: z.number().int().min(1).max(9),
    /** Halkanin ust etiketi. */
    kicker: z.string(),
    /** Teknik terim kullanmayan tek paragraf. */
    sade: z.string(),
    /** Dosya veya fonksiyon capasi. */
    kod: z.string(),
    /** Bu adim ne kadar suruyor. */
    sure: z.string(),
    /** Bu adim bozulursa ne oluyor - gizlenmez. */
    hata: z.string(),
    ...kanit,
  }),
});

/**
 * Parcalar - sistem envanterinin alt sayfalari.
 * Govde bes katmani izler: Bir ornekle / Teknik detay / Neden boyle / Bilinen sinir.
 * "Bilinen sinir" zorunlu degil ama bos birakilan sayfa taslak sayilir: calismayan
 * ozelligin calisiyormus gibi anlatilmamasi bu alanla saglaniyor.
 */
const parcalar = defineCollection({
  loader: glob({ base: './src/content/parcalar', pattern: '**/*.md' }),
  schema: z.object({
    title: z.string(),
    order: z.number().int(),
    /** Teknik terim kullanmayan tek cumle. */
    kisaca: z.string(),
    /** Bu parca neyden sorumlu, neyden degil. */
    sorumluluk: z.string(),
    /** Gercek dosya adlari ve buyuklukleri. */
    dosyalar: z.string(),
    ...kanit,
  }),
});

/**
 * Kod okuma katmani.
 * Uc soru kurali sema seviyesinde zorunlu: ne_yapiyor / neden_boyle / kaldirirsak.
 * Ucunden biri bos olan parca derlemede durdurulur - "boyle alisilmis" cevabi
 * olan parca siteye konmaz.
 */
const kod = defineCollection({
  loader: glob({ base: './src/content/kod', pattern: '**/*.md' }),
  schema: z.object({
    title: z.string(),
    order: z.number().int(),
    /** Hangi /parcalar sayfasina ait. */
    parca: z.enum(['uzanti', 'sunucu', 'orkestrator', 'model', 'hesap', 'veri']),
    zorluk: z.enum(['baslangic', 'orta', 'ileri']),
    dosya: z.string(),
    aralik: z.tuple([z.number().int(), z.number().int()]),
    dil: z.enum(['rust', 'js', 'json', 'sql', 'text']),
    ne_yapiyor: z.string().min(10),
    neden_boyle: z.string().min(20),
    kaldirirsak: z.string().min(20),
    notlar: z
      .array(z.object({ satirlar: z.tuple([z.number().int(), z.number().int()]), metin: z.string() }))
      .default([]),
    sina: z.array(z.object({ soru: z.string(), cevap: z.string() })).default([]),
    ...kanit,
  }),
});

export const collections = { ajanlar, guncellemeler, sozluk, zincir, parcalar, kod };
