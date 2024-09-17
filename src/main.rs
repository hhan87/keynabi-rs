use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::thread;
use std::time::Duration;

fn char_to_big_char(c: char) -> Vec<String> {
    match c {
        'A' => vec![
            "   A   ",
            "  A A  ",
            " AAAAA ",
            "A     A",
            "A     A",
        ],
        'B' => vec![
            "BBBB   ",
            "B   B  ",
            "BBBB   ",
            "B   B  ",
            "BBBB   ",
        ],
        'C' => vec![
            " CCCC  ",
            "C      ",
            "C      ",
            "C      ",
            " CCCC  ",
        ],
        'D' => vec![
            "DDDD   ",
            "D   D  ",
            "D    D ",
            "D   D  ",
            "DDDD   ",
        ],
        'E' => vec![
            "EEEEE  ",
            "E      ",
            "EEE    ",
            "E      ",
            "EEEEE  ",
        ],
        'F' => vec![
            "FFFFF  ",
            "F      ",
            "FFF    ",
            "F      ",
            "F      ",
        ],
        'G' => vec![
            " GGGG  ",
            "G      ",
            "G  GGG ",
            "G    G ",
            " GGGG  ",
        ],
        'H' => vec![
            "H    H ",
            "H    H ",
            "HHHHHH ",
            "H    H ",
            "H    H ",
        ],
        'I' => vec![
            "IIIII  ",
            "  I    ",
            "  I    ",
            "  I    ",
            "IIIII  ",
        ],
        'J' => vec![
            "JJJJJ  ",
            "   J   ",
            "   J   ",
            "J  J   ",
            " JJ    ",
        ],
        'K' => vec![
            "K   K  ",
            "K  K   ",
            "KKK    ",
            "K  K   ",
            "K   K  ",
        ],
        'L' => vec![
            "L      ",
            "L      ",
            "L      ",
            "L      ",
            "LLLLL  ",
        ],
        'M' => vec![
            "M   M  ",
            "MM MM  ",
            "M M M  ",
            "M   M  ",
            "M   M  ",
        ],
        'N' => vec![
            "N    N ",
            "NN   N ",
            "N N  N ",
            "N  N N ",
            "N   NN ",
        ],
        'O' => vec![
            " OOO   ",
            "O   O  ",
            "O   O  ",
            "O   O  ",
            " OOO   ",
        ],
        'P' => vec![
            "PPPP   ",
            "P   P  ",
            "PPPP   ",
            "P      ",
            "P      ",
        ],
        'Q' => vec![
            " QQQ   ",
            "Q   Q  ",
            "Q Q Q  ",
            "Q  QQ  ",
            " QQQQ  ",
        ],
        'R' => vec![
            "RRRR   ",
            "R   R  ",
            "RRRR   ",
            "R  R   ",
            "R   R  ",
        ],
        'S' => vec![
            " SSSS  ",
            "S      ",
            " SSS   ",
            "    S  ",
            "SSSS   ",
        ],
        'T' => vec![
            "TTTTT  ",
            "  T    ",
            "  T    ",
            "  T    ",
            "  T    ",
        ],
        'U' => vec![
            "U   U  ",
            "U   U  ",
            "U   U  ",
            "U   U  ",
            " UUU   ",
        ],
        'V' => vec![
            "V   V  ",
            "V   V  ",
            "V   V  ",
            " V V   ",
            "  V    ",
        ],
        'W' => vec![
            "W   W  ",
            "W   W  ",
            "W W W  ",
            "WW WW  ",
            "W   W  ",
        ],
        'X' => vec![
            "X   X  ",
            " X X   ",
            "  X    ",
            " X X   ",
            "X   X  ",
        ],
        'Y' => vec![
            "Y   Y  ",
            " Y Y   ",
            "  Y    ",
            "  Y    ",
            "  Y    ",
        ],
        'Z' => vec![
            "ZZZZZ  ",
            "   Z   ",
            "  Z    ",
            " Z     ",
            "ZZZZZ  ",
        ],
        '0' => vec![
            " 000   ",
            "0   0  ",
            "0   0  ",
            "0   0  ",
            " 000   ",
        ],
        '1' => vec![
            "  1    ",
            " 11    ",
            "  1    ",
            "  1    ",
            "11111  ",
        ],
        '2' => vec![
            " 222   ",
            "2   2  ",
            "   2   ",
            "  2    ",
            "22222  ",
        ],
        '3' => vec![
            "3333   ",
            "    3  ",
            " 333   ",
            "    3  ",
            "3333   ",
        ],
        '4' => vec![
            "   4   ",
            "  44   ",
            " 4 4   ",
            "44444  ",
            "   4   ",
        ],
        '5' => vec![
            "55555  ",
            "5      ",
            "5555   ",
            "    5  ",
            "5555   ",
        ],
        '6' => vec![
            " 666   ",
            "6      ",
            "6666   ",
            "6   6  ",
            " 666   ",
        ],
        '7' => vec![
            "77777  ",
            "   7   ",
            "  7    ",
            " 7     ",
            "7      ",
        ],
        '8' => vec![
            " 888   ",
            "8   8  ",
            " 888   ",
            "8   8  ",
            " 888   ",
        ],
        '9' => vec![
            " 999   ",
            "9   9  ",
            " 9999  ",
            "    9  ",
            " 999   ",
        ],
        _ => vec![
            "ㅁ"
        ],
    }.into_iter().map(|s| s.to_string()).collect()
}

fn key_to_string(key: &Key) -> String {
    match key {
        Key::Char(c) => c.to_string(),
        Key::Alt(c) => format!("Alt+{}", c),
        Key::Ctrl(c) => format!("Ctrl+{}", c),
        Key::Left => "←".to_string(),
        Key::Right => "→".to_string(),
        Key::Up => "↑".to_string(),
        Key::Down => "↓".to_string(),
        Key::Backspace => "⌫".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PgUp".to_string(),
        Key::PageDown => "PgDn".to_string(),
        Key::BackTab => "⇤".to_string(),
        Key::Delete => "Del".to_string(),
        Key::Insert => "Ins".to_string(),
        Key::F(n) => format!("F{}", n),
        Key::Esc => "Esc".to_string(),
        _ => "?".to_string(),
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}키를 입력하세요. 종료하려면 Ctrl+C를 누르세요.", 
           termion::clear::All, 
           termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}", termion::clear::All).unwrap();

        match c.unwrap() {
            Key::Ctrl('c') => {
                // Ctrl+C가 눌렸을 때 프로그램 종료
                write!(stdout, "{}{}프로그램을 종료합니다.", 
                       termion::clear::All,
                       termion::cursor::Goto(1, 1)).unwrap();
                stdout.flush().unwrap();
                break;
            },
            Key::Char(key) => {
                let (width, height) = termion::terminal_size().unwrap();
                let center_y = height / 2;

                // 맨 아래부터 중앙까지 이동하며 출력
                for y in (center_y..=height).rev() {
                    write!(stdout, "{}{}{}",
                           termion::clear::All,
                           termion::cursor::Goto(width / 2, y),
                           key).unwrap();
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(50)); // 애니메이션 속도 조절
                }

                // 중앙에 도달하면 큰 문자로 변환
                let big_char = char_to_big_char(key);
                let art_height = big_char.len() as u16;
                let art_width = big_char[0].len() as u16;
                let start_y = center_y - art_height / 2;
                let start_x = width / 2 - art_width / 2;

                for (i, line) in big_char.iter().enumerate() {
                    write!(stdout, "{}{}",
                           termion::cursor::Goto(start_x, start_y + i as u16),
                           line).unwrap();
                }
                stdout.flush().unwrap();
                thread::sleep(Duration::from_millis(500)); // 큰 문자 표시 시간
            },
            _ => (), // 다른 키는 무시
        }
        stdout.flush().unwrap();
    }
}