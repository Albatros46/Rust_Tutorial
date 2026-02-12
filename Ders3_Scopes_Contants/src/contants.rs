//Eğer bir değer asla değişmeyecekse ve derleme zamanında biliniyorsa, const kullan! 🦀
/*
| Kullanım Alanı                     | Örnek                          |
| ---------------------------------- | ------------------------------ |
| **Fiziksel/Matematiksel sabitler** | `PI`, `E`, `G`                 |
| **Sistem ayarları**                | `MAX_CONNECTIONS`, `TIMEOUT`   |
| **Boyutlar/Limitler**              | `BUFFER_SIZE`, `MAX_RETRIES`   |
| **Versiyon bilgisi**               | `API_VERSION`, `APP_VERSION`   |
| **Birim dönüşümleri**              | `SECONDS_PER_HOUR`, `KB`, `MB` |
*/

// Genel sabitler (İLK PI TANIMI - BUNU KULLAN)
const PI: f64 = 3.14159;
const MAX_USERS: u32 = 100;
const APP_NAME: &str = "Rust Tutorial";

// Oyun sabitleri
const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const FPS: u32 = 60;
const PLAYER_SPEED: f32 = 5.5;
const MAX_HEALTH: i32 = 100;

// Matematiksel sabitler (İKİNCİ PI Yİ SİLDİK - ZATEN YUKARIDA VAR)
// const PI: f64 = 3.14159265359;  // ❌ SİLİNDİ - Çakışma vardı
const E: f64 = 2.71828182846;
const GOLDEN_RATIO: f64 = 1.61803398875;

// Network
const SERVER_PORT: u16 = 8080;
const MAX_CONNECTIONS: usize = 1000;
const TIMEOUT_SECONDS: u64 = 30;
const API_VERSION: &str = "v1";
const DEFAULT_PAGE_SIZE: usize = 20;

// Zaman birimleri (saniye cinsinden)
const SECONDS_PER_MINUTE: u32 = 60;
const SECONDS_PER_HOUR: u32 = 60 * SECONDS_PER_MINUTE;  // 3600
const SECONDS_PER_DAY: u32 = 24 * SECONDS_PER_HOUR;      // 86400

// Byte birimleri
const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;

// ═══════════════════════════════════════════════════
// FONKSİYONLAR - HEPSİ pub OLMALI!
// ═══════════════════════════════════════════════════

pub fn global_constant(){
    println!("Uygulama: {}", APP_NAME);
    println!("PI değeri: {}", PI);
    println!("Maksimum kullanıcı: {}", MAX_USERS);
    // PI = 3.14;  // ❌ HATA! const değiştirilemez
}

pub fn game_constants(){
    println!("Ekran: {}x{} @ {}FPS", SCREEN_WIDTH, SCREEN_HEIGHT, FPS);
    println!("Oyuncu hızı: {}", PLAYER_SPEED);
    println!("Maksimum can: {}", MAX_HEALTH);

    // Hesaplama
    let screen_area = SCREEN_WIDTH * SCREEN_HEIGHT;
    println!("Piksel sayısı: {}", screen_area);
}

pub fn matematiksel(){
    // Daire çevresi: 2 * PI * r
    let radius = 10.0;
    let circumference = 2.0 * PI * radius;
    println!("Çevre: {}", circumference);

    // Küre hacmi: (4/3) * PI * r^3
    let volume = (4.0 / 3.0) * PI * radius.powi(3);
    println!("Hacim: {}", volume);
}

pub fn network(){
    let url = build_api_url("users");
    println!("API URL: {}", url);
    // Çıktı: http://localhost:8080/api/v1/users

    println!("Timeout: {} saniye", TIMEOUT_SECONDS);
    println!("Sayfa başına kayıt: {}", DEFAULT_PAGE_SIZE);

    // Yardımcı fonksiyon - sadece bu modül içinde kullanılabilir
    fn build_api_url(endpoint: &str) -> String {
        format!("http://localhost:{}/api/{}",
                SERVER_PORT, API_VERSION, )
    }
}

pub fn birim_donusumu(){
    let file_size_bytes: u64 = 5_500_000_000;

    // GB'ye çevir
    let file_size_gb = file_size_bytes as f64 / GB as f64;
    println!("Dosya boyutu: {:.2} GB", file_size_gb);

    // Zaman dönüşümü
    let total_seconds = 3665;
    let hours = total_seconds / SECONDS_PER_HOUR;
    let minutes = (total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let seconds = total_seconds % SECONDS_PER_MINUTE;

    println!("{} saat {} dakika {} saniye", hours, minutes, seconds);
    // Çıktı: 1 saat 1 dakika 5 saniye
}