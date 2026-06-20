pub mod ast;
use crate::lexer::token::{Span, Token, TokenKind};
use crate::parser::ast::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).cloned().unwrap_or(Token {
            kind: TokenKind::Eof,
            span: Span::new(0, 0),
        })
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek();
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        tok
    }

    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.peek().kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind, msg: &str) -> Result<Token, ParseError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(ParseError {
                message: msg.to_string(),
                span: self.peek().span,
            })
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        while self.peek().kind != TokenKind::Eof {
            statements.push(self.declaration()?);
        }
        Ok(Program { statements })
    }

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        match self.peek().kind {
            TokenKind::Function => self.function_decl(),
            TokenKind::Let => self.var_decl(),
            TokenKind::Dim => self.dim_stmt(),
            _ => self.statement(),
        }
    }

    fn function_decl(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Function, "expected FUNCTION")?;
        let name = if let TokenKind::Identifier(s) = self.advance().kind {
            s
        } else {
            return Err(ParseError {
                message: "expected function name".into(),
                span: self.peek().span,
            });
        };
        self.expect(TokenKind::LParen, "expected '(' after function name")?;
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                let param_name = if let TokenKind::Identifier(s) = self.advance().kind {
                    s
                } else {
                    return Err(ParseError {
                        message: "expected parameter name".into(),
                        span: self.peek().span,
                    });
                };
                self.expect(TokenKind::Colon, "expected ':' after parameter name")?;
                let typ = self.type_ref()?;
                params.push(Param {
                    name: param_name,
                    typ,
                });
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen, "expected ')' after parameters")?;
        let ret_type = if self.match_kind(TokenKind::Returns) {
            Some(self.type_ref()?)
        } else {
            None
        };
        let body = self.body_block()?;
        self.expect(TokenKind::End, "expected END after function body")?;
        self.expect(TokenKind::Function, "expected FUNCTION after END")?;
        Ok(Statement::FunctionDecl {
            name,
            params,
            ret_type,
            body,
        })
    }

    fn var_decl(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Let, "expected LET")?;
        let is_mut = if self.peek().kind == TokenKind::Mut {
            self.advance();
            true
        } else {
            false
        };
        let name = if let TokenKind::Identifier(s) = self.advance().kind {
            s
        } else {
            return Err(ParseError {
                message: "expected identifier after LET".into(),
                span: self.peek().span,
            });
        };
        let typ = if self.match_kind(TokenKind::Colon) {
            Some(self.type_ref()?)
        } else {
            None
        };
        self.expect(TokenKind::Assign, "expected '=' in variable declaration")?;
        let init = self.expression()?;
        Ok(Statement::VarDecl {
            name,
            is_mut,
            typ,
            init,
        })
    }

    fn dim_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Dim, "expected DIM")?;
        let mut declarations = Vec::new();
        loop {
            let name = if let TokenKind::Identifier(s) = self.advance().kind {
                s
            } else {
                return Err(ParseError {
                    message: "expected array name after DIM".into(),
                    span: self.peek().span,
                });
            };
            self.expect(TokenKind::LParen, "expected '(' after array name")?;
            let mut dimensions = Vec::new();
            loop {
                let dim_expr = self.expression()?;
                dimensions.push(dim_expr);
                if self.match_kind(TokenKind::Comma) {
                    continue;
                } else {
                    break;
                }
            }
            self.expect(TokenKind::RParen, "expected ')' after array dimensions")?;
            let base_type = if self.match_kind(TokenKind::As) {
                self.type_ref()?
            } else {
                TypeRef {
                    name: "INTEGER".to_string(),
                }
            };
            let array_type = ArrayType {
                base_type: Box::new(base_type),
                dimensions,
            };
            declarations.push(ArrayDecl {
                name,
                array_type,
                init: None,
            });
            if !self.match_kind(TokenKind::Comma) {
                break;
            }
        }
        Ok(Statement::Dim { declarations })
    }

    fn on_error_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::On, "expected ON")?;
        self.expect(TokenKind::Error, "expected ERROR")?;
        self.expect(TokenKind::Goto, "expected GOTO")?;
        let label = if let TokenKind::Identifier(s) = self.advance().kind {
            s
        } else {
            return Err(ParseError {
                message: "expected error label after ON ERROR GOTO".into(),
                span: self.peek().span,
            });
        };
        Ok(Statement::OnError { label })
    }

    fn input_or_input_hash(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Input, "expected INPUT")?;
        // Check if this is INPUT# (file I/O)
        if self.peek().kind == TokenKind::Hash {
            self.advance(); // consume #
            let handle = self.expression()?;
            self.expect(TokenKind::Comma, "expected ',' after file handle")?;
            let mut targets = Vec::new();
            loop {
                let target = self.expect_ident("expected variable name")?;
                targets.push(target);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
            return Ok(Statement::InputHash { handle, targets });
        }
        // Regular INPUT statement
        let (prompt, target) = if let TokenKind::StringLit(_) = self.peek().kind.clone() {
            let prompt_str = if let TokenKind::StringLit(p) = self.advance().kind {
                p
            } else {
                unreachable!();
            };
            self.expect(TokenKind::Comma, "expected ',' after INPUT prompt")?;
            let target_name = if let TokenKind::Identifier(name) = self.advance().kind {
                name
            } else {
                return Err(ParseError {
                    message: "expected identifier after INPUT prompt,".into(),
                    span: self.peek().span,
                });
            };
            (Some(prompt_str), target_name)
        } else {
            let target_name = if let TokenKind::Identifier(name) = self.advance().kind {
                name
            } else {
                return Err(ParseError {
                    message: "expected identifier after INPUT".into(),
                    span: self.peek().span,
                });
            };
            (None, target_name)
        };
        Ok(Statement::Input { prompt, target })
    }

    fn open_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Open, "expected OPEN")?;
        let filename = self.expression()?;
        self.expect(TokenKind::For, "expected FOR after filename")?;
        let mode = match self.peek().kind {
            TokenKind::Input => {
                self.advance();
                FileMode::Input
            }
            TokenKind::Identifier(ref s) if s.to_uppercase() == "OUTPUT" => {
                self.advance();
                FileMode::Output
            }
            TokenKind::Append => {
                self.advance();
                FileMode::Append
            }
            TokenKind::Random => {
                self.advance();
                FileMode::Random
            }
            TokenKind::Binary => {
                self.advance();
                FileMode::Binary
            }
            _ => {
                return Err(ParseError {
                    message: "expected INPUT, OUTPUT, APPEND, RANDOM, or BINARY".into(),
                    span: self.peek().span,
                })
            }
        };
        let handle = if self.match_kind(TokenKind::As) {
            self.expect(TokenKind::Hash, "expected '#' before file handle")?;
            self.expression()?
        } else {
            Expression::Literal(Literal::Int(1)) // Default handle
        };
        let record_len = if let TokenKind::Identifier(ref s) = self.peek().kind {
            if s.to_uppercase() == "LEN" {
                self.advance();
                self.expect(TokenKind::Assign, "expected '=' after LEN")?;
                Some(self.expression()?)
            } else {
                None
            }
        } else {
            None
        };
        Ok(Statement::Open {
            filename,
            mode,
            handle,
            record_len,
        })
    }

    fn close_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Close, "expected CLOSE")?;
        let mut handles = Vec::new();
        if self.peek().kind == TokenKind::Hash {
            loop {
                self.advance(); // consume #
                handles.push(self.expression()?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        Ok(Statement::Close { handles })
    }

    fn line_input_hash(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Line, "expected LINE")?;
        self.expect(TokenKind::Input, "expected INPUT after LINE")?;
        self.expect(TokenKind::Hash, "expected '#' before file handle")?;
        let handle = self.expression()?;
        self.expect(TokenKind::Comma, "expected ',' after file handle")?;
        let target = self.expect_ident("expected variable name")?;
        Ok(Statement::LineInputHash { handle, target })
    }

    fn select_case_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Select, "expected SELECT")?;
        self.expect(TokenKind::Case, "expected CASE after SELECT")?;
        let expr = self.expression()?;
        let mut cases = Vec::new();
        let mut else_case = None;

        loop {
            match self.peek().kind {
                TokenKind::Case => {
                    self.advance(); // consume CASE
                    if self.peek().kind == TokenKind::Else {
                        self.advance(); // consume ELSE
                        else_case = Some(self.statements_until_end_select()?);
                    } else {
                        let values = self.parse_case_values()?;
                        let body = self.statements_until_case_or_end_select()?;
                        cases.push(CaseClause { values, body });
                    }
                }
                TokenKind::End => {
                    self.advance(); // consume END
                    self.expect(TokenKind::Select, "expected SELECT after END")?;
                    break;
                }
                TokenKind::Eof => {
                    return Err(ParseError {
                        message: "expected END SELECT or CASE".into(),
                        span: self.peek().span,
                    });
                }
                _ => {
                    return Err(ParseError {
                        message: "expected CASE or END SELECT".into(),
                        span: self.peek().span,
                    });
                }
            }
        }

        Ok(Statement::SelectCase {
            expr,
            cases,
            else_case,
        })
    }

    fn parse_case_values(&mut self) -> Result<Vec<CaseValue>, ParseError> {
        let mut values = Vec::new();
        let first = self.expression()?;
        if self.peek().kind == TokenKind::To {
            self.advance(); // consume TO
            let second = self.expression()?;
            values.push(CaseValue::Range(first, second));
        } else {
            values.push(CaseValue::Single(first));
        }
        while self.match_kind(TokenKind::Comma) {
            let val = self.expression()?;
            if self.peek().kind == TokenKind::To {
                self.advance(); // consume TO
                let second = self.expression()?;
                values.push(CaseValue::Range(val, second));
            } else {
                values.push(CaseValue::Single(val));
            }
        }
        Ok(values)
    }

    fn statements_until_case_or_end_select(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut stmts = Vec::new();
        loop {
            match self.peek().kind {
                TokenKind::Case => break,
                TokenKind::End => {
                    // Check if this is END SELECT
                    if self
                        .tokens
                        .get(self.current + 1)
                        .map(|t| t.kind == TokenKind::Select)
                        .unwrap_or(false)
                    {
                        break;
                    }
                    stmts.push(self.declaration()?);
                }
                TokenKind::Eof => break,
                _ => stmts.push(self.declaration()?),
            }
        }
        Ok(stmts)
    }

    fn statements_until_end_select(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut stmts = Vec::new();
        loop {
            match self.peek().kind {
                TokenKind::End => {
                    // Check if this is END SELECT
                    if self
                        .tokens
                        .get(self.current + 1)
                        .map(|t| t.kind == TokenKind::Select)
                        .unwrap_or(false)
                    {
                        break;
                    }
                    stmts.push(self.declaration()?);
                }
                TokenKind::Eof => break,
                _ => stmts.push(self.declaration()?),
            }
        }
        Ok(stmts)
    }

    fn resume_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Resume, "expected RESUME")?;
        let label = if matches!(self.peek().kind, TokenKind::Identifier(_)) {
            let label_str = if let TokenKind::Identifier(s) = self.advance().kind {
                s
            } else {
                return Err(ParseError {
                    message: "expected label after RESUME".into(),
                    span: self.peek().span,
                });
            };
            Some(label_str)
        } else {
            None
        };
        Ok(Statement::Resume { label })
    }

    fn type_ref(&mut self) -> Result<TypeRef, ParseError> {
        if let TokenKind::Identifier(s) = self.advance().kind {
            Ok(TypeRef { name: s })
        } else {
            Err(ParseError {
                message: "expected type identifier".into(),
                span: self.peek().span,
            })
        }
    }

    fn body_block(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut stmts = Vec::new();
        while self.peek().kind != TokenKind::End && self.peek().kind != TokenKind::Eof {
            stmts.push(self.declaration()?);
        }
        Ok(stmts)
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek().kind {
            TokenKind::Print => {
                self.advance();
                // Check if this is PRINT# (file I/O)
                if self.peek().kind == TokenKind::Hash {
                    self.advance(); // consume #
                    let handle = self.expression()?;
                    self.expect(TokenKind::Comma, "expected ',' after file handle")?;
                    let mut items = Vec::new();
                    loop {
                        match self.peek().kind {
                            TokenKind::Comma => {
                                self.advance();
                                items.push(PrintItem::Comma);
                            }
                            TokenKind::Semi => {
                                self.advance();
                                items.push(PrintItem::Semi);
                            }
                            TokenKind::Eof => break,
                            // Break on tokens that start new statements
                            TokenKind::Print
                            | TokenKind::Return
                            | TokenKind::If
                            | TokenKind::While
                            | TokenKind::For
                            | TokenKind::Do
                            | TokenKind::On
                            | TokenKind::Resume
                            | TokenKind::Input
                            | TokenKind::Open
                            | TokenKind::Close
                            | TokenKind::Line
                            | TokenKind::Let
                            | TokenKind::Dim
                            | TokenKind::Function
                            | TokenKind::Select
                            | TokenKind::End => break,
                            _ => {
                                items.push(PrintItem::Expr(self.expression()?));
                            }
                        }
                    }
                    return Ok(Statement::PrintHash { handle, items });
                }
                let expr = self.expression()?;
                Ok(Statement::Print { expr })
            }
            TokenKind::Return => {
                self.advance();
                let expr =
                    if self.peek().kind != TokenKind::End && self.peek().kind != TokenKind::Eof {
                        Some(self.expression()?)
                    } else {
                        None
                    };
                Ok(Statement::Return { expr })
            }
            TokenKind::If => self.if_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::For => self.for_stmt(),
            TokenKind::Do => self.do_stmt(),
            TokenKind::On => self.on_error_stmt(),
            TokenKind::Resume => self.resume_stmt(),
            TokenKind::Input => self.input_or_input_hash(),
            TokenKind::Open => self.open_stmt(),
            TokenKind::Close => self.close_stmt(),
            TokenKind::Line => self.line_input_hash(),
            TokenKind::Select => self.select_case_stmt(),
            TokenKind::Identifier(_) => {
                // Check for array assignment: arr(0) = 42
                let next_is_lparen = self
                    .tokens
                    .get(self.current + 1)
                    .map(|t| t.kind == TokenKind::LParen)
                    .unwrap_or(false);
                if next_is_lparen {
                    // Could be array assignment arr(...) = expr
                    // Peek further: find matching ')' and check if '=' follows
                    if self.is_array_assign() {
                        let name = if let TokenKind::Identifier(s) = self.advance().kind {
                            s
                        } else {
                            unreachable!()
                        };
                        self.advance(); // consume (
                        let mut indices = Vec::new();
                        loop {
                            indices.push(self.expression()?);
                            if !self.match_kind(TokenKind::Comma) {
                                break;
                            }
                        }
                        self.expect(TokenKind::RParen, "expected ')' after array indices")?;
                        self.expect(TokenKind::Assign, "expected '=' in array assignment")?;
                        let value = self.expression()?;
                        return Ok(Statement::ArrayAssign {
                            name,
                            indices,
                            value,
                        });
                    }
                }
                let next_kind = self.tokens.get(self.current + 1).map(|t| &t.kind).cloned();
                let compound_op = match &next_kind {
                    Some(TokenKind::PlusEqual) => Some(CompoundAssignOp::AddEq),
                    Some(TokenKind::MinusEqual) => Some(CompoundAssignOp::SubEq),
                    Some(TokenKind::StarEqual) => Some(CompoundAssignOp::MulEq),
                    Some(TokenKind::SlashEqual) => Some(CompoundAssignOp::DivEq),
                    Some(TokenKind::BackslashEqual) => Some(CompoundAssignOp::IntDivEq),
                    Some(TokenKind::ModEqual) => Some(CompoundAssignOp::ModEq),
                    _ => None,
                };
                if next_kind == Some(TokenKind::Assign) {
                    let id = self.advance();
                    let name = if let TokenKind::Identifier(s) = id.kind {
                        s
                    } else {
                        unreachable!()
                    };
                    self.advance(); // consume =
                    let expr = self.expression()?;
                    Ok(Statement::Assign { name, expr })
                } else if let Some(op) = compound_op {
                    let id = self.advance();
                    let name = if let TokenKind::Identifier(s) = id.kind {
                        s
                    } else {
                        unreachable!()
                    };
                    self.advance(); // consume compound operator
                    let expr = self.expression()?;
                    Ok(Statement::AssignOp { name, op, expr })
                } else {
                    let expr = self.expression()?;
                    Ok(Statement::ExpressionStmt { expr })
                }
            }
            TokenKind::End => {
                // Standalone END terminates the program
                self.advance();
                Ok(Statement::ExpressionStmt {
                    expr: Expression::Call {
                        callee: "std::process::exit".to_string(),
                        args: vec![Expression::Literal(Literal::Int(0))],
                    },
                })
            }
            _ => {
                let expr = self.expression()?;
                Ok(Statement::ExpressionStmt { expr })
            }
        }
    }

    fn if_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::If, "expected IF")?;
        let condition = self.expression()?;
        self.expect(TokenKind::Then, "expected THEN")?;
        // parse then-branch until ELSE or END
        let then_branch = {
            let mut stmts = Vec::new();
            while self.peek().kind != TokenKind::Else
                && self.peek().kind != TokenKind::End
                && self.peek().kind != TokenKind::Eof
            {
                stmts.push(self.declaration()?);
            }
            stmts
        };
        let else_branch = if self.match_kind(TokenKind::Else) {
            // parse else-branch until END
            let mut stmts = Vec::new();
            while self.peek().kind != TokenKind::End && self.peek().kind != TokenKind::Eof {
                stmts.push(self.declaration()?);
            }
            Some(stmts)
        } else {
            None
        };
        self.expect(TokenKind::End, "expected END after IF")?;
        self.expect(TokenKind::If, "expected IF after END")?;
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn for_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::For, "expected FOR")?;
        let var = if let TokenKind::Identifier(s) = self.advance().kind {
            s
        } else {
            return Err(ParseError {
                message: "expected loop variable name after FOR".into(),
                span: self.peek().span,
            });
        };
        self.expect(TokenKind::Assign, "expected '=' after loop variable")?;
        let start = self.expression()?;
        self.expect(TokenKind::To, "expected TO after start value")?;
        let end = self.expression()?;
        let step = if self.match_kind(TokenKind::Step) {
            Some(self.expression()?)
        } else {
            None
        };
        let body = self.body_block()?;
        self.expect(TokenKind::End, "expected END after FOR body")?;
        self.match_kind(TokenKind::For);
        Ok(Statement::For {
            var,
            start,
            end,
            step,
            body,
        })
    }

    fn do_body(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut stmts = Vec::new();
        while self.peek().kind != TokenKind::Loop && self.peek().kind != TokenKind::Eof {
            stmts.push(self.declaration()?);
        }
        Ok(stmts)
    }

    fn do_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Do, "expected DO")?;
        // Pre-test: DO WHILE cond ... LOOP
        if self.peek().kind == TokenKind::While {
            self.advance();
            let condition = self.expression()?;
            let body = self.do_body()?;
            self.expect(TokenKind::Loop, "expected LOOP after DO WHILE body")?;
            return Ok(Statement::DoLoop {
                variant: DoLoopVariant::WhilePre,
                condition: Some(condition),
                body,
            });
        }
        // Pre-test: DO UNTIL cond ... LOOP
        if self.peek().kind == TokenKind::Until {
            self.advance();
            let condition = self.expression()?;
            let body = self.do_body()?;
            self.expect(TokenKind::Loop, "expected LOOP after DO UNTIL body")?;
            return Ok(Statement::DoLoop {
                variant: DoLoopVariant::UntilPre,
                condition: Some(condition),
                body,
            });
        }
        // Post-test: DO ... LOOP WHILE cond
        let body = self.do_body()?;
        self.expect(TokenKind::Loop, "expected LOOP after DO body")?;
        if self.peek().kind == TokenKind::While {
            self.advance();
            let condition = self.expression()?;
            return Ok(Statement::DoLoop {
                variant: DoLoopVariant::WhilePost,
                condition: Some(condition),
                body,
            });
        }
        // Post-test: DO ... LOOP UNTIL cond
        if self.peek().kind == TokenKind::Until {
            self.advance();
            let condition = self.expression()?;
            return Ok(Statement::DoLoop {
                variant: DoLoopVariant::UntilPost,
                condition: Some(condition),
                body,
            });
        }
        Err(ParseError {
            message: "expected WHILE or UNTIL after LOOP".into(),
            span: self.peek().span,
        })
    }

    fn while_stmt(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::While, "expected WHILE")?;
        let condition = self.expression()?;
        let body = self.body_block()?;
        self.expect(TokenKind::End, "expected END after WHILE body")?;
        // Verify but do not consume the trailing WHILE token.
        self.match_kind(TokenKind::While);
        Ok(Statement::While { condition, body })
    }

    /// Peek ahead to check if the current identifier is followed by
    /// `( ... ) =` (array assignment pattern).
    fn is_array_assign(&self) -> bool {
        let mut depth = 0;
        let mut i = self.current + 1; // skip the identifier token
        loop {
            let tok = self.tokens.get(i);
            match tok.map(|t| &t.kind) {
                Some(TokenKind::LParen) => {
                    depth += 1;
                    i += 1;
                }
                Some(TokenKind::RParen) => {
                    depth -= 1;
                    i += 1;
                    if depth == 0 {
                        // Check if next token is Assign
                        return self
                            .tokens
                            .get(i)
                            .map(|t| t.kind == TokenKind::Assign)
                            .unwrap_or(false);
                    }
                }
                Some(_) => {
                    i += 1;
                }
                None => return false,
            }
        }
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.logical_or()?;
        while self.peek().kind == TokenKind::As {
            self.advance();
            let target_type = self.expect_ident("type name after AS")?;
            expr = Expression::Cast {
                expr: Box::new(expr),
                target_type,
            };
        }
        Ok(expr)
    }

    fn expect_ident(&mut self, context: &str) -> Result<String, ParseError> {
        let tok = self.advance();
        if let TokenKind::Identifier(s) = tok.kind {
            Ok(s)
        } else {
            Err(ParseError {
                message: format!("expected identifier ({})", context),
                span: tok.span,
            })
        }
    }

    fn logical_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.logical_and()?;
        while matches!(self.peek().kind, TokenKind::Or | TokenKind::Xor) {
            let op = match self.advance().kind {
                TokenKind::Or => BinaryOp::Or,
                TokenKind::Xor => BinaryOp::Xor,
                _ => unreachable!(),
            };
            let right = self.logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;
        while self.peek().kind == TokenKind::And {
            let op = BinaryOp::And;
            self.advance();
            let right = self.equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;
        while matches!(
            self.peek().kind,
            TokenKind::EqualEqual | TokenKind::NotEqual
        ) {
            let op = match self.advance().kind {
                TokenKind::EqualEqual => BinaryOp::Eq,
                TokenKind::NotEqual => BinaryOp::NotEq,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;
        while matches!(
            self.peek().kind,
            TokenKind::Less | TokenKind::LessEqual | TokenKind::Greater | TokenKind::GreaterEqual
        ) {
            let op = match self.advance().kind {
                TokenKind::Less => BinaryOp::Lt,
                TokenKind::LessEqual => BinaryOp::Lte,
                TokenKind::Greater => BinaryOp::Gt,
                TokenKind::GreaterEqual => BinaryOp::Gte,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;
        while matches!(self.peek().kind, TokenKind::Plus | TokenKind::Minus) {
            let op = match self.advance().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.power()?;
        while matches!(
            self.peek().kind,
            TokenKind::Star | TokenKind::Slash | TokenKind::Backslash | TokenKind::Mod
        ) {
            let op = match self.advance().kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Backslash => BinaryOp::IntDiv,
                TokenKind::Mod => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.power()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        // Bit shift operators
        while matches!(self.peek().kind, TokenKind::Shl | TokenKind::Shr) {
            let op = match self.advance().kind {
                TokenKind::Shl => BinaryOp::Shl,
                TokenKind::Shr => BinaryOp::Shr,
                _ => unreachable!(),
            };
            let right = self.power()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn power(&mut self) -> Result<Expression, ParseError> {
        let expr = self.unary()?;
        if self.peek().kind == TokenKind::Caret {
            self.advance();
            // Right-associative: ^
            let right = self.power()?;
            Ok(Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::Pow,
                right: Box::new(right),
            })
        } else {
            Ok(expr)
        }
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if self.match_kind(TokenKind::Minus) {
            let expr = self.unary()?;
            Ok(Expression::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            })
        } else if self.match_kind(TokenKind::Not) {
            let expr = self.unary()?;
            Ok(Expression::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        match self.advance().kind {
            TokenKind::Int(v) => Ok(Expression::Literal(Literal::Int(v))),
            TokenKind::Float(v) => Ok(Expression::Literal(Literal::Float(v))),
            TokenKind::Bool(b) => Ok(Expression::Literal(Literal::Bool(b))),
            TokenKind::StringLit(s) => Ok(Expression::Literal(Literal::String(s))),
            TokenKind::Identifier(name) => {
                if self.match_kind(TokenKind::LParen) {
                    let mut args = Vec::new();
                    if self.peek().kind != TokenKind::RParen {
                        loop {
                            args.push(self.expression()?);
                            if !self.match_kind(TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RParen, "expected ')' after arguments")?;
                    Ok(Expression::Call { callee: name, args })
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            TokenKind::LParen => {
                let expr = self.expression()?;
                self.expect(TokenKind::RParen, "expected ')' after grouping")?;
                Ok(Expression::Grouping(Box::new(expr)))
            }
            other => Err(ParseError {
                message: format!("unexpected token: {:?}", other),
                span: self.peek().span,
            }),
        }
    }
}
