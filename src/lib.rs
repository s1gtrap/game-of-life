pub fn gameoflife(counts: &mut [u8], states: &mut [bool], width: usize, height: usize) {
    countcounts(counts, &*states, width, height);
    for i in 0..height {
        for j in 0..width {
            let coord = (i + 1) * (width + 2) + (j + 1);
            let currentvalue = states[coord];
            let neighbors = counts[(i + 1) * (width + 2) + j + 1];
            if currentvalue {
                if neighbors < 2 || neighbors > 3 {
                    states[coord] = false;
                }
            } else {
                if neighbors == 3 {
                    states[coord] = true;
                }
            }
        }
    }
}

fn countcounts(counts: &mut [u8], states: &[bool], width: usize, height: usize) {
    counts.iter_mut().for_each(|x| *x = 0);
    let awidth = width + 2;
    for i in 0..height {
        for j in 0..width {
            let val = states[(i + 1) * awidth + (j + 1)] as u8;
            counts[(i + 1 + 1) * awidth + j + 1] += val;
            counts[(i + 1 - 1) * awidth + j + 1] += val;
            counts[(i + 1) * awidth + j + 1 + 1] += val;
            counts[(i + 1) * awidth + j - 1 + 1] += val;
            counts[(i + 1 + 1) * awidth + (j + 1) + 1] += val;
            counts[(i + 1 - 1) * awidth + (j + 1) - 1] += val;
            counts[(i + 1 - 1) * awidth + j + 1 + 1] += val;
            counts[(i + 1 + 1) * awidth + (j + 1) - 1] += val;
        }
    }
}

#[test]
fn test_countcounts() {
    let w = 3;
    let h = 3;
    let mut counts = vec![0; (w + 2) * (h + 2)];
    let mut states = vec![
        false, false, false, false, false, false, false, true, false, false, false, false, true,
        false, false, false, false, true, false, false, false, false, false, false, false,
    ];
    countcounts(&mut counts, &states, 3, 3);
    assert_eq!(
        counts,
        vec![0, 1, 1, 1, 0, 0, 2, 1, 2, 0, 0, 3, 2, 3, 0, 0, 2, 1, 2, 0, 0, 1, 1, 1, 0,]
    )
}
