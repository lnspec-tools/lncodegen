use super::token::CSVToken;
use super::token::CSVTokenType;
/// Core implementation of the scanner
use std::collections::HashMap;

pub struct Scanner {
    line: u64,
    keywords: HashMap<String, CSVToken>,
}

impl Scanner {
    pub fn new() -> Scanner {
        // mapping table for keywords to CSVTokenType
        let keywords = HashMap::from([
            (
                "msgtype".to_string(),
                CSVToken {
                    ty: CSVTokenType::MsgTy,
                    val: "msgtype".to_string(),
                },
            ),
            (
                "msgdata".to_string(),
                CSVToken {
                    ty: CSVTokenType::MsgData,
                    val: "msgdata".to_string(),
                },
            ),
            (
                "tlvtype".to_string(),
                CSVToken {
                    ty: CSVTokenType::TlvType,
                    val: "tlvtype".to_string(),
                },
            ),
            (
                "tlvdata".to_string(),
                CSVToken {
                    ty: CSVTokenType::TlvData,
                    val: "tlvdata".to_string(),
                },
            ),
            (
                "short_channel_id".to_string(),
                CSVToken {
                    ty: CSVTokenType::ShortChannelId,
                    val: "short_channel_id".to_string(),
                },
            ),
            (
                "sha256".to_string(),
                CSVToken {
                    ty: CSVTokenType::Sha256,
                    val: "sha256".to_string(),
                },
            ),
            (
                "u16".to_string(),
                CSVToken {
                    ty: CSVTokenType::U16,
                    val: "u16".to_string(),
                },
            ),
            (
                "u32".to_string(),
                CSVToken {
                    ty: CSVTokenType::U32,
                    val: "u32".to_string(),
                },
            ),
            (
                "u64".to_string(),
                CSVToken {
                    ty: CSVTokenType::U64,
                    val: "u64".to_string(),
                },
            ),
            (
                "channel_id".to_string(),
                CSVToken {
                    ty: CSVTokenType::ChannelId,
                    val: "channel_id".to_string(),
                },
            ),
            (
                "signature".to_string(),
                CSVToken {
                    ty: CSVTokenType::Signature,
                    val: "signature".to_string(),
                },
            ),
            (
                "point".to_string(),
                CSVToken {
                    ty: CSVTokenType::Point,
                    val: "point".to_string(),
                },
            ),
            (
                "chain_hash".to_string(),
                CSVToken {
                    ty: CSVTokenType::ChainHash,
                    val: "chain_hash".to_string(),
                },
            ),
            (
                "byte".to_string(),
                CSVToken {
                    ty: CSVTokenType::Byte,
                    val: "byte".to_string(),
                },
            ),
            (
                "bigsize".to_string(),
                CSVToken {
                    ty: CSVTokenType::BigSize,
                    val: "bigsize".to_string(),
                },
            ),
            (
                "tu32".to_string(),
                CSVToken {
                    ty: CSVTokenType::Tu32,
                    val: "tu32".to_string(),
                },
            ),
            (
                "tu64".to_string(),
                CSVToken {
                    ty: CSVTokenType::Tu64,
                    val: "tu64".to_string(),
                },
            ),
            (
                "tlvs".to_string(),
                CSVToken {
                    ty: CSVTokenType::Tlvs,
                    val: "tlvs".to_string(),
                },
            ),
            (
                "...".to_string(),
                CSVToken {
                    ty: CSVTokenType::Dotdotdot,
                    val: "...".to_string(),
                },
            ),
            (
                "data".to_string(),
                CSVToken {
                    ty: CSVTokenType::Data,
                    val: "data".to_string(),
                },
            ),
        ]);
        let line = 1;
        return Scanner { line, keywords };
    }

    pub fn add_token(&mut self, tokenize: &mut Vec<CSVToken>, buffer: &mut String) {
        // sanity check if the buffer is empty we can not perform any operation.
        // FIXME: condition like the following on `init, ,16` are not valid?
        if !buffer.is_empty() {
            if buffer.trim().parse::<f64>().is_ok() {
                tokenize.push(CSVToken {
                    ty: CSVTokenType::Number,
                    val: buffer.clone(),
                });
            } else {
                tokenize.push(CSVToken {
                    ty: CSVTokenType::LiteralString,
                    val: buffer.clone(),
                });
            }
            buffer.clear();
        }
    }

    pub fn scan(&mut self, symbols: &Vec<char>) -> Vec<CSVToken> {
        let mut tokenize: Vec<CSVToken> = Vec::new();
        let mut current_buffer: String = String::new();
        let size = symbols.len();
        for pos in 0..size {
            // Before go on, check if the current buffer is a keyword
            // if yes add the keyword value in the token list.
            if self.keywords.contains_key(current_buffer.as_str()) {
                tokenize.push(
                    self.keywords
                        .get(current_buffer.as_str())
                        .unwrap()
                        .to_owned()
                        .clone(),
                );
                current_buffer = String::new();
                continue;
            }
            // if the current buffer is not a keyword, we check if we are found
            // a comma or an end-line token, and if nothing of the previous condition is satisfied
            // we keep putting the token in the current buffer.
            match symbols[pos] {
                ',' => {
                    // Here we panic if we found a comma but the current buffer is empty.
                    // to handle situation like double commas in seqence.
                    if current_buffer.is_empty() {
                        panic!("Empty token between two seperators")
                    };
                    self.add_token(&mut tokenize, &mut current_buffer)
                }
                '\n' => {
                    self.add_token(&mut tokenize, &mut current_buffer);
                    self.line += 1;
                }
                '\t' | ' ' => {
                    // FIXME: we need to skip this characters?
                }
                _ => {
                    if pos == size - 1 {
                        // Panic due to No end-line character found at last position.
                        panic!("Need EOF symbol")
                    };
                    current_buffer.push(symbols[pos]);
                }
            }
        }
        // put an EOF token at the end of the result for easy processing in parser.
        tokenize.push(CSVToken {
            ty: CSVTokenType::EOF,
            val: "".to_string(),
        });
        return tokenize;
    }
}
