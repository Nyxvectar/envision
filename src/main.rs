/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 08/13/2025
 */

fn main() {
    let mut birth_year = 2000;
    let mut birth_month = 1;
    let mut birth_day = 1;
    
    birth_year = 2020;
    birth_month = 8;
    birth_day = 28;
    
    if birth_month == 8 { 
        let birth_month = "Aug";
        let name = "Nyxvectar Yan";
        print!("{} Was Born on ",name);
        println!("{} {}, {}", birth_month, birth_day, birth_year)
    }
    
    let (year_diff, month_diff) = test_function(birth_year,birth_month);
    println!("The Year Diff is {0}, again {0}", year_diff);
    println!("The Month Diff is {0}, again {0}", month_diff);

    {
        let mut counter = 0;
        
        loop {
            counter += 1;
            println!("{}",counter);
            if counter == 10 {
                break
            }
        }
        
        for number in 1..4 {
            println!("{}", number)
        }
        
    }
}

fn test_function(a:i32, b:i32) -> (i32, i32) {
    let today_year = 2025;
    let today_month = 8;
    (today_year-a, today_month-b)
}
