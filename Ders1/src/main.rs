fn main() {
     println!("Hello, world!");
     println!("------------------------");
     variable();
    /*
    Komut,Açıklama
    cargo new proje_adi,Yeni binary proje
    cargo new --lib proje_adi,Yeni kütüphane projesi
    cd proje_adi,Klasöre gir
    cargo run,Derle + çalıştır
    cargo build,Sadece derle
    cargo build --release,Optimize edilmiş derleme
     */
     let apples=50;
     let oranges=14+6;
     let fruits= apples+oranges;
     println!("apples = {}, oranges = {}", apples, oranges);
     println!("My garden!");
     println!("--------------------");
     
}
fn variable(){
     print!("______Değişkenler - Variable\n ______" );
     let x     = 5;
     println!("x değeri: {}", x);
     /*
     Tip,Açıklama,Boyut,Örnek
     "i8, i16, i32, i64, i128",İşaretli tam sayı (negatif olabilir),8-128 bit,let x: i32 = -42;
     "u8, u16, u32, u64, u128",İşaretsiz tam sayı (sadece pozitif),8-128 bit,let y: u8 = 255;
     "isize, usize",İşletim sistemine göre pointer boyutu,64-bit sistemde 64 bit,let z: usize = 100;
     "f32, f64",Ondalıklı sayı (floating point),32/64 bit,let pi: f64 = 3.14159;
     bool,Doğru / Yanlış,1 byte,let aktif: bool = true;
     char,Tek bir Unicode karakter,4 byte,let harf: char = 'A'; veya 'ç' veya '笑'
     -------------------
     Tip,Açıklama,Örnek
     String,"UTF-8, büyüyüp küçülebilen metin","let s = String::from(""merhaba"");"
     &str,String dilimi (en çok kullanılan),"let s = ""merhaba"";"
     Vec<T>,Dinamik dizi (ArrayList gibi),"let v: Vec<i32> = vec![1,2,3];"
     "HashMap<K,V>",Anahtar-değer deposu,let mut map = HashMap::new();
     Option<T>,Null yerine güvenli alternatif,Some(5) veya None
     "Result<T,E>",Hata yönetimi,"Ok(42) veya Err(""hata"")"
      */
}