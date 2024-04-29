pub fn parse_source(source: String) -> Vec<u16> {
    let mut program: Vec<u16> = Vec::new();

    for l in source.lines().map(|line| line.trim()) {
        let mut value: u16 = 0;
        let segments: Vec<&str> = l.split_whitespace().collect();

        for s in 0..segments.len() {
            match segments[s] {
                "ADD" => {
                    value = 100;
                    value += segments[s + 1].parse::<u8>().unwrap() as u16;
                }
                "SUB" => {
                    value = 200;
                    value += segments[s + 1].parse::<u8>().unwrap() as u16;
                }
                "STA" => {
                    value = 300;
                    value += segments[s + 1].parse::<u8>().unwrap() as u16;
                }
                "LDA" => {
                    value = 500;
                    value += segments[s + 1].parse::<u8>().unwrap() as u16;
                }
                "BRA" => {
                    value = 600;
                    value += segments[s + 1].parse::<u8>().unwrap() as u16;
                }
                "BRZ" => {
                    value = 700;
                    value += segments[s + 1].parse::<u8>().unwrap() as u16;
                }
                "BRP" => {
                    value = 800;
                    value += segments[s + 1].parse::<u8>().unwrap() as u16;
                }
                "INP" => value = 901,
                "OUT" => value = 902,
                "HLT" | "COB" => value = 0,
                "DAT" => {
                    value = if segments.len() > s + 1 {
                        segments[s + 1].parse::<u16>().unwrap()
                    } else {
                        0
                    }
                }
                _ => (),
            }
        }

        program.push(value);
    }

    program
}
