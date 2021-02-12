use std::env;
use std::io;
// Why must we import trait?
use std::io::Read;
use std::cmp;

const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;

type Screen = [[u8; SCREEN_WIDTH / 8]; SCREEN_HEIGHT];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please, give a file name");
        return;
    }

    match read_file(&args[1]) {
        Ok(program) => run(program),
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

fn run (program: Vec<u8>) {
    let mut memory = init_memory(program);
    // todo: make state structure?
    let mut screen: Screen = [[1; SCREEN_WIDTH / 8]; SCREEN_HEIGHT];
    let mut program_counter = 0x200;
    let mut index_reg = 0;
    let mut var_regs: [u8; 16] = [0; 16];

    loop {
        // todo: use slice
        let first = memory[program_counter];
        let second = memory[program_counter + 1];
        program_counter += 2;

        // todo: use pattern matching
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
        } else if (first >> 4) == 0xD {
            let vx = var_regs[usize::from(first & 0x0F)];
            let vy = var_regs[usize::from(second >> 4)];
            let rows_amount = second & 0x0F;
            draw(vx, vy, rows_amount, &mut var_regs[0xf], &mut memory, index_reg, &mut screen);
        }

        // Just for IBM Logo testing
        if program_counter >= 552 {
            break;
        }
    }

    draw_screen(screen);
}

fn init_memory (mut program: Vec<u8>) -> Vec<u8> {
    let mut memory: Vec<u8> = vec![0; 0x50];
    let mut font = vec![
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];
    let mut rest = vec![0; 0x160];

    memory.append(&mut font);
    memory.append(&mut rest);
    memory.append(&mut program);

    memory
}

fn clear_screen (screen: &mut Screen) {
    for line in screen.iter_mut() {
        for px in line.iter_mut() {
            *px = 0;
        }
    }
}

fn draw (
    vx: u8,
    vy: u8,
    n: u8,
    vf: &mut u8,
    memory: &Vec<u8>,
    index_reg: usize,
    screen: &mut Screen
) {
    let x = vx % (SCREEN_WIDTH as u8);
    let mut y = (vy as usize) % SCREEN_HEIGHT;
    *vf = 0;

    let n = n as usize;
    let n = cmp::min(n, SCREEN_HEIGHT - n);
    for row in &memory[index_reg..index_reg+n] {
        let column_byte_i = usize::from(x / 8);
        
        let first_sprite = row >> (x % 8);
        
        let first_screen = &mut screen[y][column_byte_i];
        if (first_sprite & *first_screen) > 0 {
            *vf = 1;
        }
        *first_screen ^= first_sprite;
        
        if x % 8 != 0 {
            let second_sprite = row << (8 - x % 8);

            let second_screen = &mut screen[y][column_byte_i+1];
            if (second_sprite & *second_screen) > 0 {
                *vf = 1;
            }
            *second_screen ^= second_sprite;
        }

        y += 1;
    }
}

fn draw_screen (screen: Screen) {
    for i in 0..screen.len() {
        let row = screen[i];
        let mut full_row: u64 = 0;
        for j in 0..row.len() {
            full_row = (full_row << 8) + u64::from(row[j]);
        }

        println!("{:064b}", !full_row);
    }
}
