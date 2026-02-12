struct User {
    fname: String,
    lname: String,
}

fn main() {
    //Immutable-Degistirilemez
    //Ilk basta hangi deger atanmis ise o deger sonradan degistirilemez.
    //Rust'ta değişkenler varsayılan olarak immutable'dır.
    println!("Immutable-Degistirilemez");
    let gym_reps=10; //artik sonradan degistirilemez.
    println!("gym-Reps: {} sonradan degistirilemez",gym_reps );
    println!("-----------------------------------");
    println!("gym_day degeri degistirilebilir.");
    let mut gym_day=3;
    println!("gym-Day 1.deger : {}", gym_day);
    gym_day=4;
    println!("gym-Day 2.deger : {}", gym_day);
    println!("################################################");
    shadowing();

    // shadowing_parametreli_fonksiyon testi
    println!("________________________________________________");
    println!("----Shadowing Parametreli Fonksiyon Testi----");
    let mut user = User {
        fname: "Ali".to_string(),
        lname: "Veli".to_string(),
    };
    println!("Önce: {} {}", user.fname, user.lname);
    user.shadowing_parametreli_fonksiyon("Ahmet Yılmaz");
    println!("Sonra: {} {}", user.fname, user.lname);
}

fn shadowing(){
    println!("----Variable shadowing----"); //https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let x  = 5;                    // İlk x (i32)
    let x = x + 1;                // Yeni x (i32), eski x gizlendi → 6

    {
        let x = x * 2;            // İç scope'da yeni x → 12
        println!("İç scope: {}", x);  // Çıktı: 12
    }                             // İç scope'daki x burada ölür

    println!("Dış scope: {}", x); // Çıktı: 6 (dış scope'daki x)
    println!("________________________________________________");
    println!("-Tip Degistirme - Shadowing in gücü");
    let input = "42";                    // &str tipi

    // String'i sayıya çevir, aynı ismi kullan
    let input: u32 = input.parse().expect("Sayı değil!");  // u32 tipi

    // Artık aritmetik yapabiliriz
    let input = input * 2;               // Yine u32, değer: 84

    println!("Sonuç: {}", input);        // Çıktı: 84
}

impl User {
    fn shadowing_parametreli_fonksiyon(&mut self, new_name: impl Into<String>){
        // String, &str, vs. her şeyi kabul et
        let new_name = new_name.into();  // Shadowing ile tipi standartlaştır

        let (fname, lname) = new_name.split_once(" ")
            .expect("Geçersiz format");

        self.fname = fname.into();
        self.lname = lname.into();
    }
}