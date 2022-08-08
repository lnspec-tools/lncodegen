use super::token::CSVToken;
use super::token::CSVTokenType;
/// Core implementation of the scanner
use std::collections::HashMap;

pub struct Scanner {
    line: u64,
    keywords: HashMap<String, CSVToken>,
}

impl Scanner {
    pub fn new(line: u64) -> Scanner {
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
                    ty: CSVTokenType::MsgTy,
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
                "\n".to_string(),
                CSVToken {
                    ty: CSVTokenType::NewLine,
                    val: "\n".to_string(),
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
                ",".to_string(),
                CSVToken {
                    ty: CSVTokenType::Comma,
                    val: ",".to_string(),
                },
            ),
        ]);

        return Scanner { line, keywords };
    }

    pub fn scan(&mut self, _symbols: &Vec<char>) -> Vec<CSVToken> {
        let mut tokenize: Vec<CSVToken> = Vec::new();
        let mut pos = 0;
        let mut current_buffer: String = String::new();
        let size = _symbols.len();
        while pos < size {
            if self.keywords.contains_key(current_buffer.as_str()) {
                tokenize.push(
                    self.keywords
                        .get(current_buffer.as_str())
                        .unwrap()
                        .to_owned()
                        .clone(),
                );
                current_buffer = String::new();
                if _symbols[pos] == '\n' {
                    self.line += 1;
                }
            } else {
                if _symbols[pos] == ',' || _symbols[pos] == '\n' {
                    if current_buffer.trim().parse::<f64>().is_ok() {
                        tokenize.push(CSVToken {
                            ty: CSVTokenType::Number,
                            val: current_buffer.clone(),
                        });
                    } else {
                        tokenize.push(CSVToken {
                            ty: CSVTokenType::LiteralString,
                            val: current_buffer.clone(),
                        });
                    }
                    current_buffer = String::new();
                }
            }
            current_buffer.push(_symbols[pos]);
            pos += 1;
        }
        if current_buffer.len() > 0 {
            if current_buffer.trim().parse::<f64>().is_ok() {
                tokenize.push(CSVToken {
                    ty: CSVTokenType::Number,
                    val: current_buffer.clone(),
                });
            } else {
                tokenize.push(CSVToken {
                    ty: CSVTokenType::LiteralString,
                    val: current_buffer.clone(),
                });
            }
        }
        return tokenize;
    }
}
