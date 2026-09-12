-- Reklam hedefleme katmanı.
--
-- Hedefleme kararı kullanıcının çıkarılmış profiline bakar; bu KVKK/GDPR'da
-- açık rıza ister. Rıza varsayılan olarak KAPALI (`users.ads_consent = false`)
-- ve rıza yokken profil hiç okunmaz — karar da üretilmez.

ALTER TABLE users ADD COLUMN IF NOT EXISTS ads_consent boolean NOT NULL DEFAULT false;

-- Kampanya envanteri. Dış reklam ağı yok: satırları biz doldururuz, böylece
-- profil verisi hiçbir zaman dışarı çıkmaz.
CREATE TABLE IF NOT EXISTS ad_inventory (
    id          text PRIMARY KEY,
    brand       text NOT NULL,
    category    text NOT NULL,
    title       text NOT NULL,
    body        text NOT NULL,
    -- Hedef kriterleri: ilgi_alanlari, tuketici_egilimi, yas_araliklari,
    -- egitim_seviyeleri, diller dizileri. Boş bırakılan kriter "fark etmez".
    target      jsonb NOT NULL DEFAULT '{}'::jsonb,
    -- Yaş doğrulaması isteyen kategoriler (kumar, alkol, kredi/finansal ürün).
    sensitive   boolean NOT NULL DEFAULT false,
    -- Aciliyet/kıtlık kurgusu kullanan kampanya. ManipuLens'in kendi duruşu
    -- gereği, davranışsal manipülasyona açık kullanıcıya gösterilmez.
    urgency     boolean NOT NULL DEFAULT false,
    active      boolean NOT NULL DEFAULT true,
    created_at  timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ad_inventory_active ON ad_inventory (active);

-- Üretilen kararlar: hangi kullanıcıya hangi reklam, hangi skorla ve hangi
-- gerekçeyle gösterildi. Gerekçe saklanır çünkü kullanıcıya "neden bu reklam?"
-- diye gösteriliyor ve sonradan denetlenebilmesi gerekiyor.
CREATE TABLE IF NOT EXISTS ad_decisions (
    id            bigserial PRIMARY KEY,
    user_id       text NOT NULL,
    ad_id         text NOT NULL REFERENCES ad_inventory (id) ON DELETE CASCADE,
    score         real NOT NULL,
    reason        text NOT NULL,
    model_version text NOT NULL,
    decided_at    timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ad_decisions_user ON ad_decisions (user_id);

-- Gösterim/tıklama/gizleme olayları.
CREATE TABLE IF NOT EXISTS ad_events (
    id          bigserial PRIMARY KEY,
    decision_id bigint NOT NULL REFERENCES ad_decisions (id) ON DELETE CASCADE,
    kind        text NOT NULL CHECK (kind IN ('impression', 'click', 'dismiss')),
    at          timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ad_events_decision ON ad_events (decision_id);
