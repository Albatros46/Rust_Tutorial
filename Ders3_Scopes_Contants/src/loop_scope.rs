pub fn loop_example() {
    println!("______Loop Scope______");

    let mut count = 0;

    'outer: loop {
        let inner_var = 10;
        println!("inner_var: {}", inner_var);

        loop {
            if count > 5 {
                break 'outer;
            }
            count += 1;
        }
    }
    println!("Döngü bitti, count: {}\n", count);
}

pub fn match_scope() {
    println!("______Match Scope______");

    let num = Some(5);

    match num {
        Some(n) => {
            println!("Match içindeki n: {}", n);
        },
        None => {}
    }
    println!("Match scope'u bitti\n");
}

pub fn raii() {
    println!("______RAII Scope______");
    println!("Resource Acquisition Is Initialization");

    {
        // Gerçek bir dosya açmayı dene, yoksa dummy kullan
        match std::fs::File::open("data.txt") {
            Ok(_file) => println!("Dosya açıldı ve scope sonunda otomatik kapanacak"),
            Err(e) => println!("Dosya bulunamadı (normal): {}", e),
        }
    }
    println!("Dosya scope'u kapandı, otomatik temizlendi\n");
}