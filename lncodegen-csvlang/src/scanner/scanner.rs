use super::token::CSVToken;
use super::token::CSVTokenType;
use log::trace;
use std::collections::HashMap;

pub struct Scanner {
    keywords: HashMap<String, CSVToken>,
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
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
                "subtype".to_string(),
                CSVToken {
                    ty: CSVTokenType::SubTy,
                    val: "subtype".to_string(),
                },
            ),
            (
                "subtypedata".to_string(),
                CSVToken {
                    ty: CSVTokenType::SubMsgData,
                    val: "subtypedata".to_string(),
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
        Scanner { keywords }
    }

    pub fn add_token(&mut self, tokenize: &mut Vec<CSVToken>, buffer: &str) {
        // sanity check if the buffer is empty we can not perform any operation.
        // FIXME: condition like the following on `init, ,16` are not valid?
        if !buffer.is_empty() {
            if buffer.trim().parse::<f64>().is_ok() {
                tokenize.push(CSVToken {
                    ty: CSVTokenType::Number,
                    val: buffer.to_owned(),
                });
            } else {
                tokenize.push(CSVToken {
                    ty: CSVTokenType::LiteralString,
                    val: buffer.to_owned(),
                });
            }
        }
    }

    pub fn scan(&mut self, content: &str) -> Vec<CSVToken> {
        // We can split the content by new line terminator
        let lines = content.split_terminator('\n');
        let mut tokenize: Vec<CSVToken> = Vec::new();
        for line in lines {
            log::debug!("looking at the line: {line}");
            if line.trim().starts_with('#') {
                // it is a comment
                continue;
            }
            // Splitting the line in tokens by `,`
            let tokens = line.split(',');
            for token in tokens {
                let token = token.trim();
                if let Some(keyword) = self.keywords.get(token) {
                    tokenize.push(keyword.to_owned());
                } else {
                    self.add_token(&mut tokenize, token);
                }
            }
        }
        tokenize.push(CSVToken {
            ty: CSVTokenType::EOF,
            val: "EOF".to_string(),
        });
        trace!("tokens list: {:?}", tokenize);
        tokenize
    }
}
