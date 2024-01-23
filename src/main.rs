use std::{time, thread};

fn main() {
    let mut A = 0f32;
    let mut B = 0f32;
    let mut z: [f32; 1760] = [0.0; 1760];
    let mut b: [char; 1760] = [' '; 1760];
    println!("\x1b[2J");

    loop {
        /* fill b array with empty spaces */
        b.fill(' ');

        for j in (0..628).step_by(7) {
            let j = j as f32 * 0.01;
            for i in (0..628).step_by(2) {
                let i = i as f32 * 0.01;
                let sini = i.sin();
                let cosj = j.cos();
                let sinA = A.sin();
                let sinj = j.sin();
                let cosA = A.cos();
                let cosj2 = cosj + 2.0;
                let mess = 1.0 / (sini * cosj2 * sinA + sinj * cosA + 5.0);
                let cosi = i.cos();
                let cosB = B.cos();
                let sinB = B.sin();
                let t = sini * cosj2 * cosA - sinj * sinA;
                let x = (40.0 + 30.0 * mess * (cosi * cosj2 * cosB - t * sinB) + 0.5) as usize;
                let y = (12.0 + 15.0 * mess * (cosi * cosj2 * sinB + t * cosB) + 0.5) as usize;

                let o = x + 80 * y;
                let N = (8.0 * ((sinj * sinA - sini * cosj * cosA) * cosB - sini * cosj * sinA - sinj * cosA - cosi * cosj * sinB)) as usize;

                if y < 22 && y >= 0 && x >= 0 && x < 80 {
                    if mess > z[o] - 5.0{
                        z[o] = mess;
                        b[o] = ",-~:;=!*#$@".chars().nth(N.min(8)).unwrap();
                    }
                }
            }
        }
        print!("\x1b[d");
        thread::sleep(time::Duration::from_millis(16));

        for (k, &ch) in b.iter().enumerate() {
            print!("{}", if k % 80 != 0 { ch } else { '\n' });
        }
        A += 0.04;
        B += 0.02;
    }
}