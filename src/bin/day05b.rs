fn translate(val: (usize, usize), table: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize)> {
    // Map format is: target source size
    // Value format is: base size
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let mut vals: Vec<(usize, usize)> = vec![val];


    'vals: while let Some(val) = vals.pop() {
        for (dst, src, sz) in table {
            let mut start = val.0;
            let mut end = val.0 + val.1;
            let sstart = *src;
            let send = *src + *sz;
            // Starts before end, and ends after start, i.e. overlaps
            if start < send && end > sstart {
                if start < sstart {
                    // Queue up preceding fragment for next iteration
                    vals.push((start, sstart - start));
                    start = sstart;
                }
                if end >= send {
                    // Queue up following fragment for next iteration
                    vals.push((send + 1, end - send));
                    end = send;
                }
                // By now it is fully contained in the range
                ret.push((dst + (start - sstart), end - start));
                continue 'vals;
            }
        }
        ret.push(val);
    }
    ret
}

fn mkmap(s: &str) -> Vec<(usize, usize, usize)> {
    let (_header, table) = s.split_once(":").unwrap();
    let table: Vec<usize> = table
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let mut ret: Vec<_> = table
        .chunks_exact(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect();
    ret.sort();
    ret
}

fn main() {
    let mut input = include_str!("input-day05.txt").split("\n\n");

    let seeds: Vec<_> = input
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut data: Vec<(usize, usize)> = seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    data.sort();

    let translations: Vec<_> = input.map(|t| mkmap(t)).collect();

    for t in translations {
        let newdata: Vec<_> = data.iter().flat_map(|&d| translate(d, &t)).collect();
        data = newdata;
    }

    data.sort();

    println!("Closest: {}", data.iter().map(|d| d.0).min().unwrap());

}
