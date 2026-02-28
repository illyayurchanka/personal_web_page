use automata_rust::Automata;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let rule: u8 = args[1].parse().unwrap();
    let width: usize = args[2].parse().unwrap();
    let height: usize = args[3].parse().unwrap();

    let automata = Automata::new(rule, width, height);

    let result_flatted = automata.generate_automata();

    let result: Vec<_> = result_flatted.chunks(width).collect();

    result.iter().for_each(|row| {
        let line: String = row
            .iter()
            .map(|cell| if *cell == 1 { '#' } else { ' ' })
            .collect();
        println!("{}", line);
    });
}
