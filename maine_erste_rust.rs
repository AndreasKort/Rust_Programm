// Zum Starten muss Rust installiert sein (rustc maine_erste_rust.rs) und dann ./maine_erste_rust
// `fn` ist die Funktion `one_to_ten()`, um die Zahlen 1-10 auszugeben. 
// `for i in 1..=10` erzeugt eine Range von Werten von der Startzahl (1) bis zur Endzahl (10).
// `println!("{}", i)` gibt den Wert von `i` auf dem Terminal aus. Die `{}`-Klammer ist ein Platzhalter, der durch den Wert von `i` ersetzt wird.
fn one_to_ten() {
    for i in 1..=10 {
        println!("{}", i);
    }
}

// `.step_by(2)` ist eine Methode, die auf die Range angewendet wird und eine neue Range erzeugt, die nur jede zweite Zahl enthält.
// Funktion, um die geraden Zahlen bis 20 auszugeben
fn evens_to_twenty() {
    for i in (2..=20).step_by(2) {
        println!("{}", i);
    }
}

// `fn from_to(start: i32, end: i32)` definiert eine Funktion "from_to" mit zwei Parametern: "start" und "end". 
// Diese Parameter sind vom Typ `i32` (32-Bit Integer), um ganze Zahlen darzustellen.
// Funktion, um Zahlen zwischen zwei gegebenen Punkten auszugeben (vorwärts oder rückwärts)
// `if start <= end` ist eine Bedingung, die überprüft, ob der "start"-Wert kleiner oder gleich dem "end"-Wert ist.
fn from_to(start: i32, end: i32) {
    if start <= end {
        for i in start..=end {
            println!("{}", i);
        }
    } else {
        for i in (end..=start).rev() {
            println!("{}", i);
        }
    }
}

// `if step == 0` überprüft, ob die Schrittgröße 0 ist. Wenn ja, wird ein Fehler ausgegeben.
// `let mut current = start` deklariert eine Variable "current" mit dem Wert "start". 
// Das "mut" steht für "mutable" und ermöglicht es, den Wert später zu ändern.
// `fn from_to_by(start: i32, end: i32, step: i32)` definiert eine Funktion "from_to_by" mit drei Parametern: "start", "end" und "step". 
// Diese Parameter sind ebenfalls vom Typ `i32`.
// Funktion, um Zahlen zwischen zwei gegebenen Punkten in einer bestimmten Schrittgröße auszugeben
// Die Bedingung in der Schleife überprüft, ob "start" kleiner als "end" ist und die aktuelle Zahl "current" kleiner oder gleich "end" ist, ODER ob "start" größer als "end" ist und "current" größer oder gleich "end" ist. In beiden Fällen wird die Schleife fortgesetzt und die aktuelle Zahl "current" ausgegeben. 
// Die Zahl wird dann um "step" erhöht bzw. verringert, je nachdem, ob die Schrittgröße positiv oder negativ ist.
fn from_to_by(start: i32, end: i32, step: i32) {
    if step == 0 {
        println!("Schrittgröße darf nicht 0 sein.");
        return;
    }

    if (start < end && step > 0) || (start > end && step < 0) {
        let mut current = start;
        while (start < end && current <= end) || (start > end && current >= end) {
            println!("{}", current);
            current += step;
        }
    } else {
        println!("Ungültige Kombination von Startpunkt, Endpunkt und Schrittgröße.");
    }
}

fn main() {
    println!("Zahlen von 1 bis 10:");
    one_to_ten();

    println!("\nGerade Zahlen bis 20:");
    evens_to_twenty();

    println!("\nZahlen von 6 bis 11:");
    from_to(6, 11);

    println!("\nZahlen von 1 bis 10 in 3er Schritten:");
    from_to_by(1, 10, 3);

    println!("\nZahlen von 10 bis 1 in -2er Schritten:");
    from_to_by(10, 1, -2);
}
