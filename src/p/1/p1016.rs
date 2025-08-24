/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/24/2025
 */
use std::io;

struct Station {
    dist: f64,
    price: f64,
}

impl Station {
    fn new(dist: f64, price: f64) -> Self {
        Self { dist, price }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let data: Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let d1 = data[0];
    let c = data[1];
    let d2 = data[2];
    let p = data[3];
    let n = data[4] as usize;

    let mut stations = Vec::new();
    stations.push(Station::new(0.0, p));

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let data: Vec<f64> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        stations.push(Station::new(data[0], data[1]));
    }
    stations.push(Station::new(d1, 0.0));
    stations.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    for i in 0..stations.len() - 1 {
        let gap = stations[i + 1].dist - stations[i].dist;
        if gap > c * d2 {
            println!("No Solution");
            return;
        }
    }

    let mut i = 0;
    let mut gas = 0.0;
    let mut cost = 0.0;

    while i < stations.len() - 1 {
        let current = &stations[i];
        let mut next_stations = Vec::new();
        for j in i + 1..stations.len() {
            if stations[j].dist <= current.dist + c * d2 {
                next_stations.push((j, &stations[j]));
            } else {
                break;
            }
        }

        if next_stations.is_empty() {
            println!("No Solution");
            return;
        }

        let mut found_cheaper = None;
        for (idx, s) in &next_stations {
            if s.price < current.price {
                found_cheaper = Some((*idx, *s));
                break;
            }
        }

        if let Some((j_index, j_station)) = found_cheaper {
            let need = (j_station.dist - current.dist) / d2;
            if gas < need {
                cost += (need - gas) * current.price;
                gas = need;
            }
            gas -= need;
            i = j_index;
        } else {
            let mut min_price = next_stations[0].1.price;
            let mut min_index = next_stations[0].0;
            let mut max_dist = next_stations[0].1.dist;
            for (idx, s) in &next_stations {
                if s.price < min_price {
                    min_price = s.price;
                    min_index = *idx;
                    max_dist = s.dist;
                } else if (s.price - min_price).abs() < 1e-5 {
                    if s.dist > max_dist {
                        max_dist = s.dist;
                        min_index = *idx;
                    }
                }
            }

            let j_station = &stations[min_index];
            let need = (j_station.dist - current.dist) / d2;
            cost += (c - gas) * current.price;
            gas = c - need;
            i = min_index;
        }
    }
    println!("{:.2}", cost);
}
