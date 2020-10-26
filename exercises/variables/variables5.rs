// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E";
    println!("Spell a Number : {}", number);
    let number = 3; // shadowing; although can't reuse x without mut
                    // we can redeclare x
                    // we can also redeclare x with a different type
    println!("Number plus two is : {}", number + 2);
}
