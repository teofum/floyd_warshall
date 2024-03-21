use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut vert_count = None;

    while let None = vert_count {
        println!("Enter vertex count:");
        std::io::stdin().read_line(&mut input)?;

        if let Ok(size) = input.trim().parse::<usize>() {
            vert_count = Some(size);
        } else {
            input.clear();
        }
    }
    let vert_count = vert_count.unwrap();

    // Matrix input
    let mut d0: Vec<f64> = Vec::with_capacity(vert_count * vert_count);
    println!(
        "Enter coefficients for D0, separated by spaces in rows of {} at a time (? for infinity):",
        vert_count
    );
    for _ in 0..vert_count {
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        let numbers = input.trim().split(' ').map(|s| {
            if s == "?" {
                f64::INFINITY
            } else {
                s.parse::<f64>().unwrap()
            }
        });

        let mut i = 0;
        for n in numbers.take(vert_count) {
            d0.push(n);
            i += 1;
        }
        assert_eq!(i, vert_count, "Invalid arg count");
    }

    let mut d = Vec::with_capacity(vert_count);
    d.push(d0);

    let idx = |i: usize, j: usize| i * vert_count + j;

    for k in 0..vert_count {
        let last = &d[k];
        let mut d_k: Vec<f64> = Vec::with_capacity(vert_count * vert_count);

        for i in 0..vert_count {
            for j in 0..vert_count {
                let n = last[idx(i, j)].min(last[idx(i, k)] + last[idx(k, j)]);
                d_k.push(n);
            }
        }

        d.push(d_k);
    }

    print_matrix(&d[vert_count], vert_count);

    Ok(())
}

fn print_matrix(m: &Vec<f64>, size: usize) {
    for row in 0..size {
        print!("| ");
        for n in &m[(row * size)..(row * size + size)] {
            let s = n.to_string().replace("inf", "∞");
            print!("{:>2} ", s);
        }
        print!("|\n");
    }
}
