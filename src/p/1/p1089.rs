/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let mut budgets = Vec::with_capacity(12);
    for _ in 0..12 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let budget: i32 = line.trim().parse().unwrap();
        budgets.push(budget);
    }

    let mut cash = 0;
    let mut savings = 0;

    for (i, &budget) in budgets.iter().enumerate() {
        let month = i + 1;
        cash += 300;
        if cash < budget {
            println!("-{}", month);
            return;
        }
        cash -= budget;
        let save = (cash / 100) * 100;
        savings += save;
        cash -= save;
    }

    let total = cash + (savings * 6) / 5;
    println!("{}", total);
}
