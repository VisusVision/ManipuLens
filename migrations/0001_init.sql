-- ManipuLens kalıcı şema (PostgreSQL).
--
-- SQLite sürümünden taşınırken tipler gerçek karşılıklarına çekildi:
-- INTEGER bayraklar boolean, JSON metin sütunları jsonb, id'ler bigserial.
-- Kimlik sütunları (users.id, history.user_id, sessions.user_id) bilerek
-- `text` kaldı: uygulama bunları UUID metni olarak üretiyor ama eski
-- kayıtlarda e-posta da geçebiliyor; tipi daraltmak veri kaybı riski olurdu.

CREATE TABLE IF NOT EXISTS users (
    id            text PRIMARY KEY,
    email         text NOT NULL UNIQUE,
    password_hash text NOT NULL,
    -- Uygulamanın ürettiği RFC3339 metni; olduğu gibi korunur.
    created_at    text NOT NULL,
    verified      boolean NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS history (
    id                    bigserial PRIMARY KEY,
    -- Uygulama katmanının yazdığı zaman metni (RFC3339, yerel saat).
    timestamp             text NOT NULL,
    -- Sunucunun kaydettiği gerçek an: zaman aralığı sorguları bunu kullanır
    -- (metin sütunu sıralamak için güvenilir değil, zaman dilimi taşımıyor).
    created_at            timestamptz NOT NULL DEFAULT now(),
    client_id             text NOT NULL,
    text_preview          text NOT NULL,
    is_manipulated        boolean NOT NULL,
    dominant_manipulation text NOT NULL,
    genel_sonuc           text NOT NULL,
    lang                  text,
    user_id               text,
    agents_json           jsonb,
    predicted_product     text,
    text_len              bigint
);

CREATE INDEX IF NOT EXISTS idx_history_client ON history (client_id);
CREATE INDEX IF NOT EXISTS idx_history_user ON history (user_id);
CREATE INDEX IF NOT EXISTS idx_history_created_at ON history (created_at);

CREATE TABLE IF NOT EXISTS sessions (
    token      text PRIMARY KEY,
    user_id    text NOT NULL,
    email      text NOT NULL,
    -- Unix epoch saniye; auth katmanı zaten epoch ile çalışıyor.
    created_at bigint NOT NULL,
    expires_at bigint NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_email ON sessions (email);
CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions (expires_at);

CREATE TABLE IF NOT EXISTS user_profiles (
    user_id        text PRIMARY KEY,
    profile_json   jsonb NOT NULL,
    analyzed_count bigint NOT NULL,
    model_version  text NOT NULL,
    updated_at     text NOT NULL
);

-- Hedefleme sorgularının temeli: "ilgi alanı X olan kullanıcılar" gibi
-- aramalar profile_json içinde döner; GIN indeksi olmadan tam tarama olur.
CREATE INDEX IF NOT EXISTS idx_user_profiles_json ON user_profiles USING gin (profile_json);
