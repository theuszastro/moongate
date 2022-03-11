use std::mem::ManuallyDrop;

use super::{expression, readBlock};
use crate::utils::{HoistingBlock, ParsedToken, Pointer, StatementToken, Token};

fn _else(pointer: &mut ManuallyDrop<Pointer>, body: &mut HoistingBlock) -> Vec<ParsedToken> {
    pointer.take("Keyword", true, true);

    match pointer.token.clone() {
        Some(Token::Brackets(brack, _)) if brack == "{" => {
            pointer.take("Brackets", true, true);

            let mut elseBlock = HoistingBlock {
                block: Box::new(Some(body.clone())),
                current: vec![],
            };

            readBlock(pointer, &mut elseBlock);

            return elseBlock.current.clone();
        }
        _ => {
            pointer.error("Expected '{'".to_string());

            vec![]
        }
    }
}

pub fn _if(
    pointer: &mut ManuallyDrop<Pointer>,
    body: &mut HoistingBlock,
) -> Option<StatementToken> {
    pointer.take("Keyword", true, true);

    if let Some(expr) = expression(pointer, body) {
        match pointer.token.clone() {
            Some(Token::Brackets(brac, _)) if brac == "{" => {
                pointer.take("Brackets", true, true);

                let mut ifBlock = HoistingBlock {
                    block: Box::new(Some(body.clone())),
                    current: vec![],
                };

                readBlock(pointer, &mut ifBlock);

                match pointer.token.clone() {
                    Some(Token::Keyword(key, _)) if key == "else" => {
                        return Some(StatementToken::IfDeclaration(
                            expr,
                            ifBlock.current.clone(),
                            _else(pointer, body),
                        ));
                    }
                    _ => {}
                }

                return Some(StatementToken::IfDeclaration(expr, ifBlock.current, vec![]));
            }
            _ => pointer.error("Expected '{'".to_string()),
        }
    }

    pointer.error("Expected a condition".to_string());

    None
}