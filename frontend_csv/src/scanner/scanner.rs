use super::token::CSVToken;
use super::token::CSVTokenType;
use std::collections::VecDeque;
/// Core implementation of the scanner
pub struct Scanner {
    pub pos: usize,
    // line: u64,
    // tokens: Vec<CSVToken>,
    // identifier_symbols: Vec<char>,
}

impl Scanner {
    pub fn scan(&mut self, _symbols: &Vec<char>) -> VecDeque<CSVToken> {
        let mut tokenize: VecDeque<CSVToken> = VecDeque::new();
        let mut current_buffer: String = String::new();
        for _symbol in _symbols.iter() {
            match current_buffer.as_str() {
                "," => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::Comma,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "\n" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::NewLine,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "msgtype" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::MsgTy,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "msgdata" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::MsgData,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "tlvtype" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::TlvType,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "tlvdata" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::TlvData,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "short_channel_id" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::ShortChannelId,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "sha256" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::Sha256,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "u16" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::U16,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "u32" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::U32,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "u64" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::U64,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "channel_id" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::ChannelId,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "signature" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::Signature,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "point" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::Point,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "chain_hash" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::ChainHash,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "byte" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::Byte,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                "bigsize" => {
                    tokenize.push_back(CSVToken {
                        _type: CSVTokenType::BigSize,
                        _value: current_buffer.clone(),
                    });
                    current_buffer = String::new();
                }
                // "" => tokenToAdd = CSVToken{_type: CSVTokenType::, _value: current_buffer.clone()},
                &_ => {
                    assert_eq!(1, 1);
                }
            }

            match _symbol.to_string().as_str() {
                "," | "\n" => {
                    if !current_buffer.is_empty() {
                        tokenize.push_back(CSVToken {
                            _type: CSVTokenType::LiteralString,
                            _value: current_buffer.clone(),
                        });
                        current_buffer = String::new()
                    }
                }
                &_ => assert_eq!(1, 1),
            }

            current_buffer.push(*_symbol);
        }
        return tokenize;
    }
}
