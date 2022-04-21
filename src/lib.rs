pub struct Handle(usize, usize, Vec<u8>);

fn count(w: usize, h: usize, s: &[u8], c: &mut [u8]) {
    for y in 0..h {
        for x in 0..w {
            let mut v = s[((x as isize - 1).rem_euclid(w as _) as usize
                + (y as isize - 1).rem_euclid(h as _) as usize * w)
                * 4
                + 3] as usize
                + s[(x + (y as isize - 1).rem_euclid(h as _) as usize * w) * 4 + 3] as usize
                + s[((x + 1) % w + (y as isize - 1).rem_euclid(h as _) as usize * w) * 4 + 3]
                    as usize
                + s[((x as isize - 1).rem_euclid(w as _) as usize + y * w) * 4 + 3] as usize
                + s[((x + 1) % w as usize + y * w) * 4 + 3] as usize
                + s[((x as isize - 1).rem_euclid(w as _) as usize + ((y + 1) % h) * w) * 4 + 3]
                    as usize
                + s[(x + ((y + 1) % h) * w) * 4 + 3] as usize
                + s[((x + 1) % w + ((y + 1) % h) * w) * 4 + 3] as usize;
            c[x + y * w] = (v / 255) as u8;
        }
    }
}

#[test]
fn test_counts() {
    let _ = env_logger::builder().is_test(true).try_init();
    fn counts(w: usize, h: usize, s: &[u8]) -> Vec<u8> {
        let mut c = vec![0; w * h];
        count(w, h, s, &mut c);
        c
    }

    assert_eq!(counts(1, 1, &[0, 0, 0, 0]), vec![0]);
    assert_eq!(counts(1, 1, &[0, 0, 0, 255]), vec![8]);
    assert_eq!(
        counts(
            5,
            1,
            &[0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255],
        ),
        vec![8, 8, 8, 8, 8],
    );
    assert_eq!(
        counts(
            3,
            3,
            &[
                0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0,
            ],
        ),
        vec![3, 2, 3, 3, 2, 3, 3, 2, 3],
    );
    assert_eq!(
        counts(
            3,
            5,
            &[
                0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ),
        vec![2, 1, 2, 3, 2, 3, 2, 1, 2, 1, 1, 1, 1, 1, 1],
    );
    assert_eq!(
        counts(
            5,
            5,
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ),
        vec![0, 1, 1, 1, 0, 0, 2, 1, 2, 0, 0, 3, 2, 3, 0, 0, 2, 1, 2, 0, 0, 1, 1, 1, 0],
    );
}

pub fn init(w: usize, h: usize, s: &[u8]) -> Handle {
    let mut c = vec![0; (w + 2) * (h + 2)];
    count(w, h, s, &mut c);

    //log::debug!("init {}x{}, states={:?}, counts={:?}", w, h, s, c);
    Handle(w, h, c)
}

pub fn step(h: &mut Handle, buf: &mut [u8]) {
    //log::debug!("step {}x{}, counts={:?}", h.0, h.1, h.2);
    for i in 0..h.1 {
        for j in 0..h.0 {
            if buf[(j + i * h.0) * 4 + 3] == 255 {
                if h.2[j + i * h.0] < 2 {
                    //log::info!("{},{} dies of lonely", j, i);
                    buf[(j + i * h.0) * 4 + 3] = 0;
                } else if h.2[j + i * h.0] > 3 {
                    //log::info!("{},{} dies of crowding", j, i);
                    buf[(j + i * h.0) * 4 + 3] = 0;
                }
            } else {
                if h.2[j + i * h.0] == 3 {
                    //log::info!("{},{} born bc h.2[{j} + {i} * {}] == 3", j, i, h.0);
                    buf[(j + i * h.0) * 4 + 3] = 255;
                }
            }
        }
    }
    h.2.iter_mut().for_each(|b| *b = 0);
    count(h.0, h.1, buf, &mut h.2);
    /*for i in 0..buf.len() / 4 {
        log::debug!("was {:?}", buf[i * 4 + 3]);
        buf[i * 4 + 3] = !buf[i * 4 + 3];
        log::debug!("is {:?}", buf[i * 4 + 3]);
    }*/
}
