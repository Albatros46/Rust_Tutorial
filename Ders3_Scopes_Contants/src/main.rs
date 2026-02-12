mod loop_scope;
mod contants;  // ⭐ EKLENDİ

use std::sync::Mutex;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║     MAIN.RS METODLARI BAŞLIYOR       ║");
    println!("╚══════════════════════════════════════╝\n");

    // 1. Önce main.rs içindeki scope örnekleri
    basic_scope();
    parameter_scope();
    ownership_scope();
    mutex_guard_scope();

    println!("\n╔══════════════════════════════════════╗");
    println!("║   LOOP_SCOPE.RS METODLARI BAŞLIYOR   ║");
    println!("╚══════════════════════════════════════╝\n");

    // 2. Sonra loop_scope.rs modülündeki metodlar
    loop_scope::loop_example();
    loop_scope::match_scope();
    loop_scope::raii();

    println!("\n╔══════════════════════════════════════╗");
    println!("║  CONTANTS.RS METODLARI BAŞLIYOR      ║");
    println!("╚══════════════════════════════════════╝\n");

    // 3. EN SON contants.rs modülündeki metodlar
    contants::global_constant();
    contants::game_constants();
    contants::matematiksel();
    contants::network();
    contants::birim_donusumu();

    println!("\n✅ Tüm metodlar başarıyla tamamlandı!");
}

// ═══════════════════════════════════════════════════
// MAIN.RS İÇİNDEKİ METODLAR (Değişmedi)
// ═════════════════════════════════════════════════==

fn basic_scope() {
    println!("______Basic Scope______");
    let x = 10;
    {
        let y = 20;
        println!("x: {}, y: {}", x, y);
    }
    println!("x: {}", x);
    println!();
}

fn parameter_scope() {
    println!("-----Parameter Scope-----");
    let s = String::from("test");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    println!("x hala yaşar: {}", x);
    println!();
}

fn ownership_scope() {
    println!("-----Ownership Scope-----");
    {
        let s = String::from("merhaba");
        println!("{}", s);
    }
    println!("String scope dışında otomatik silindi\n");
}

fn takes_ownership(s: String) {
    println!("Ownership alındı: {}", s);
}

fn makes_copy(x: i32) {
    println!("Copy yapıldı: {}", x);
}

fn mutex_guard_scope() {
    println!("-----Mutex Guard Scope-----");
    let data = Mutex::new(0);
    {
        let mut guard = data.lock().unwrap();
        *guard += 1;
        println!("Kilitli veri: {}", *guard);
    }
    println!("Mutex otomatik açıldı\n");
}