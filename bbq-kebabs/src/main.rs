fn main() {
    let mut restuls = bbq_kebab();
    println!("{}", restuls);
}

fn bbq_kebab() -> String {
    let kebabs1 = [ // 1 vegan 4 meat
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];
    let kebabs2 =  [ // 2 vegan 3 meat
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];
    let kebabs3 = [ // 3 vegan 2 meat
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"
    ];
    let mut vegan = 0; // how many vegan ones
    let mut meat = 0; // how many meat ones

    for kebab in kebabs2 {
        if kebab.contains("x") {
            meat += 1;
        } else {
            vegan += 1;
        }
    }
    return format!("[{}, {}]", vegan, meat);
}
