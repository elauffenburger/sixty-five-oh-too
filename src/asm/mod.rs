use cpu::addr::AddrMode;

extern crate regex;

lazy_static! {
    static ref INDIRECT_X_REGEX: regex::Regex = regex::Regex::new(r"\(\$(.*?),X\)").unwrap();
    static ref INDIRECT_Y_REGEX: regex::Regex = regex::Regex::new(r"\(\$(.*?)\),Y").unwrap();
    static ref ABSOLUTE_AND_ZERO_PAGE_REGEX: regex::Regex = regex::Regex::new(r"(?m)\$(.*?),?($|X|Y)").unwrap();
}

pub struct Line {
    instr: String,
    rest: Option<String>,
    addr_mode: AddrMode,
    value: Option<u16>
}

#[derive(Default)]
pub struct Parser {

}

impl Parser {
    pub fn assemble(&mut self, input: &str) -> Vec<u8> {
        let lines = self.into_lines(input);
        let mut vec = Vec::new();
        vec.push(0x0f);

        vec
    }

    pub fn into_lines(&mut self, input: &str) -> Box<Vec<Line>> {
        let result = input.split('\n')
             .into_iter()
             .map(|line| line.trim())
             .filter(|line| line.len() != 0)
             .map(|line| {
                let mut splitter = line.splitn(2, ' ');
                let instr = splitter.next().unwrap();

                let rest = splitter.next();
                let addr_mode_results = Parser::get_instr_addr_mode(rest);

                Line {
                    instr: String::from(instr),
                    rest: rest.map(|rest| String::from(rest)),
                    addr_mode: addr_mode_results.0,
                    value: addr_mode_results.1
                }            
             })
             .collect();
        
        Box::new(result)
    }

    fn to_addr_mode_with_value(addr_mode: AddrMode, value_str: &str) -> (AddrMode, Option<u16>) {
        (addr_mode, Some(u16::from_str_radix(value_str, 16).unwrap()))
    }

    fn get_first_capture(captures: Option<regex::Captures>) -> &str {
        captures.unwrap().get(1).unwrap().as_str()
    }

    pub fn get_instr_addr_mode(rest_or: Option<&str>) -> (AddrMode, Option<u16>) {
        // implicit
        if rest_or.is_none() {
            return (AddrMode::Implicit, None);
        }

        let rest = rest_or.unwrap();

        let mut chars = rest.chars();
        let first_char = chars.nth(0).unwrap_or('\0');
        let last_char = chars.last().unwrap_or('\0');

        // immediate
        if first_char == '#' {
            return Parser::to_addr_mode_with_value(AddrMode::Immediate, &rest[2..]);
        }

        // indirect
        if first_char == '(' {
            if last_char == 'Y' {
                let value_str = Parser::get_first_capture(INDIRECT_Y_REGEX.captures(rest));
                return Parser::to_addr_mode_with_value(AddrMode::IndirectY, &value_str);
            }

            let value_str = Parser::get_first_capture(INDIRECT_X_REGEX.captures(rest));
            return Parser::to_addr_mode_with_value(AddrMode::IndirectX, &value_str);
        }

        let (addr, addr_reg) = ABSOLUTE_AND_ZERO_PAGE_REGEX
            .captures(rest)
            .map(|captures| {
                let addr_reg = captures.get(2);
                let addr = captures
                    .get(1)
                    .map(|a| u16::from_str_radix(&a.as_str(), 16).unwrap())
                    .unwrap();

                (addr, addr_reg)
            })
            .unwrap();

        let is_zero_page = addr <= 0xff;

        // absolute / zero page
        return match addr_reg.unwrap().as_str() {
            "X" => {
                return match is_zero_page {
                    true => (AddrMode::ZeroPageX, Some(addr)),
                    _ => (AddrMode::AbsoluteX, Some(addr))
                };
            },
            "Y" => (AddrMode::AbsoluteY, Some(addr)),
            "" => {
                match is_zero_page {
                    true => (AddrMode::ZeroPage, Some(addr)),
                    _ => (AddrMode::Absolute, Some(addr))
                }
            }
            _ => {
                (AddrMode::Unknown, None)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use super::AddrMode;
    use super::Line;
    use std::time;

    fn assert_line(line: &Line, instr: &str, rest: Option<&str>, addr_mode: AddrMode, value: Option<u16>) {
        assert_eq!(&line.instr, instr);
        assert_eq!(line.rest.as_ref().map(|r| r.as_str()), rest);
        assert_eq!(line.addr_mode, addr_mode);
        assert_eq!(line.value, value);
    }

    #[test]
    fn into_lines() {
        let now = time::Instant::now();

        let mut parser : Parser = Parser::default();
        let lines = parser.into_lines("
            lda #$01
            sta $beef
            beq $0f
            bit
            lda ($1000),Y
        ");

        assert_line(&lines[0], "lda", Some("#$01"), AddrMode::Immediate, Some(0x01));
        assert_line(&lines[1], "sta", Some("$beef"), AddrMode::Absolute, Some(0xbeef));
        assert_line(&lines[2], "beq", Some("$0f"), AddrMode::ZeroPage, Some(0x0f));
        assert_line(&lines[3], "bit", None, AddrMode::Implicit, None);
        assert_line(&lines[4], "lda", Some("($1000),Y"), AddrMode::IndirectY, Some(0x1000));

        println!("into_lines | elapsed: {:?}", now.elapsed().as_secs());
    }

    #[test]
    fn get_instr_addr_mode() {
        let imm = Parser::get_instr_addr_mode(Some("#$0011"));
        assert_eq!(imm.0, AddrMode::Immediate);
        assert_eq!(imm.1, Some(0x0011));
    }
}