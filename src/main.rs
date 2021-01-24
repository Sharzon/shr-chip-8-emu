use std::env;
use std::io;
// Why must we import trait?
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please, give a file name");
        return;
    }

    match read_file(&args[1]) {
        Ok(program) => interpret(program),
        Err(err) => eprintln!("Troubles with file reading: {}", err)
    }
}

fn read_file (filename: &String) -> io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(filename)?;
    let mut bytes = Vec::new();
    let chunk_size = 2;

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        // What is adaptor?
        let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk)?;
        if n == 0 { break; }
        bytes.append(&mut chunk);
        if n < chunk_size { break; }
    }

    Ok(bytes)
}

fn interpret (program: Vec<u8>) {
    let mut screen: [[u8; 64]; 32] = [[1; 64]; 32];
    let mut program_counter = 0;
    let mut index_reg = 0;
    let mut var_regs: [u8; 16] = [0; 16];

    loop {
        let first = program[program_counter];
        let second = program[program_counter + 1];
        program_counter += 2;

        if first == 0x00 && second == 0xe0 {
            clear_screen(&mut screen);
        } else if (first >> 4) == 0x1 {
            program_counter = ((usize::from(first) - 0x10) << 8) +
                usize::from(second);
        } else if (first >> 4) == 0xA {
            index_reg = ((usize::from(first) - 0xA0) << 8) +
                usize::from(second);
        } else if (first >> 4) == 0x6 {
            let reg_number = usize::from(first - 0x60);
            var_regs[reg_number] = second;
        } else if (first >> 4) == 0x7 {
            let reg_number = usize::from(first - 0x70);
            var_regs[reg_number] += second;
        }
    }
}

fn clear_screen (screen: &mut [[u8; 64]; 32]) {
    for line in screen.iter_mut() {
        for px in line.iter_mut() {
            *px = 0;
        }
    }
}
