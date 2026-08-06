//! Oturum token'ı, istek hız limiti (rate limit) ve brute-force korumaları.
//!
//! Tasarım notu: sayaçlar bellek-içi tutulur (sunucu yeniden başlarsa
//! sıfırlanır — kabul edilebilir); oturumlar ise SQLite'ta kalıcıdır.

use uuid::Uuid;

/// Oturum ömrü: 30 gün
pub const SESSION_TTL_SECS: i64 = 30 * 24 * 60 * 60;

/// 64 hex karakterlik rastgele oturum token'ı (2 × UUIDv4 ≈ 244 bit entropi).
pub fn new_token() -> String {
    format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple())
}

// ===================== ANALİZ HIZ LİMİTİ =====================

/// Sabit pencereli sayaç: `max_per_window` istek / `window_secs` saniye.
/// Aşılırsa Err(kalan_saniye) döner.
pub struct RateWindow {
    window_start: i64,
    count: u32,
}

pub const ANALYZE_WINDOW_SECS: i64 = 60;
pub const ANALYZE_MAX_PER_WINDOW: u32 = 10;

impl RateWindow {
    pub fn new() -> Self {
        RateWindow { window_start: 0, count: 0 }
    }

    pub fn allow(&mut self, now: i64, window_secs: i64, max_per_window: u32) -> Result<(), i64> {
        if now - self.window_start >= window_secs {
            self.window_start = now;
            self.count = 0;
        }
        if self.count >= max_per_window {
            return Err(self.window_start + window_secs - now);
        }
        self.count += 1;
        Ok(())
    }
}

// ===================== LOGIN BRUTE-FORCE KİLİDİ =====================

/// Aynı e-postaya art arda başarısız girişte hesabı geçici kilitler.
/// 8 hata / 15 dk pencere → 15 dk kilit. Başarılı giriş sayacı sıfırlar.
pub struct LoginGuard {
    window_start: i64,
    fails: u32,
    locked_until: i64,
}

pub const LOGIN_WINDOW_SECS: i64 = 15 * 60;
pub const LOGIN_MAX_FAILS: u32 = 8;
pub const LOGIN_LOCK_SECS: i64 = 15 * 60;

impl LoginGuard {
    pub fn new() -> Self {
        LoginGuard { window_start: 0, fails: 0, locked_until: 0 }
    }

    /// Giriş denemesine izin var mı? Kilitliyse Err(kalan_saniye).
    pub fn check(&self, now: i64) -> Result<(), i64> {
        if now < self.locked_until {
            Err(self.locked_until - now)
        } else {
            Ok(())
        }
    }

    pub fn record_failure(&mut self, now: i64) {
        if now - self.window_start >= LOGIN_WINDOW_SECS {
            self.window_start = now;
            self.fails = 0;
        }
        self.fails += 1;
        if self.fails >= LOGIN_MAX_FAILS {
            self.locked_until = now + LOGIN_LOCK_SECS;
            self.fails = 0;
        }
    }

    pub fn record_success(&mut self) {
        self.fails = 0;
        self.locked_until = 0;
        self.window_start = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_is_long_and_unique() {
        let a = new_token();
        let b = new_token();
        assert_eq!(a.len(), 64);
        assert_ne!(a, b);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn rate_window_blocks_after_limit_and_resets() {
        let mut rw = RateWindow::new();
        let now = 1000;
        for _ in 0..ANALYZE_MAX_PER_WINDOW {
            assert!(rw.allow(now, ANALYZE_WINDOW_SECS, ANALYZE_MAX_PER_WINDOW).is_ok());
        }
        // 11. istek reddedilir, kalan süre pozitif
        let err = rw.allow(now + 5, ANALYZE_WINDOW_SECS, ANALYZE_MAX_PER_WINDOW).unwrap_err();
        assert!(err > 0 && err <= ANALYZE_WINDOW_SECS);
        // Pencere geçince tekrar izin verilir
        assert!(rw
            .allow(now + ANALYZE_WINDOW_SECS, ANALYZE_WINDOW_SECS, ANALYZE_MAX_PER_WINDOW)
            .is_ok());
    }

    #[test]
    fn login_guard_locks_after_max_fails() {
        let mut g = LoginGuard::new();
        let now = 5000;
        assert!(g.check(now).is_ok());
        for _ in 0..LOGIN_MAX_FAILS {
            g.record_failure(now);
        }
        let remaining = g.check(now).unwrap_err();
        assert!(remaining > 0 && remaining <= LOGIN_LOCK_SECS);
        // Kilit süresi dolunca açılır
        assert!(g.check(now + LOGIN_LOCK_SECS).is_ok());
    }

    #[test]
    fn login_guard_success_resets() {
        let mut g = LoginGuard::new();
        let now = 5000;
        for _ in 0..(LOGIN_MAX_FAILS - 1) {
            g.record_failure(now);
        }
        g.record_success();
        for _ in 0..(LOGIN_MAX_FAILS - 1) {
            g.record_failure(now);
        }
        // Sıfırlandığı için hâlâ kilitlenmemiş olmalı
        assert!(g.check(now).is_ok());
    }

    #[test]
    fn login_guard_old_window_expires() {
        let mut g = LoginGuard::new();
        let now = 5000;
        for _ in 0..(LOGIN_MAX_FAILS - 1) {
            g.record_failure(now);
        }
        // 15 dk sonra tek hata kilitlememeli (pencere sıfırlanır)
        g.record_failure(now + LOGIN_WINDOW_SECS + 1);
        assert!(g.check(now + LOGIN_WINDOW_SECS + 2).is_ok());
    }
}
