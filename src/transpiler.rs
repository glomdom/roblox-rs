use crate::indent_manager::IndentManager;
use std::collections::HashSet;
use syn::visit::Visit;

pub struct LuauTranspiler<'a> {
    indent_manager: &'a mut IndentManager,
    local_variables: HashSet<String>,
    output: String,
    in_function: bool,
}

impl<'a> LuauTranspiler<'a> {
    pub fn new(indent_manager: &'a mut IndentManager) -> Self {
        Self {
            indent_manager,
            local_variables: HashSet::new(),
            output: String::new(),
            in_function: false,
        }
    }

    pub fn render(self) -> String {
        self.output
    }

    fn add_line(&mut self, line: &str) {
        self.output
            .push_str(&format!("{}{}\n", self.indent_manager.get_indent(), line));
    }

    fn map_type(&self, rust_type: &syn::Type) -> &str {
        if let syn::Type::Path(type_path) = rust_type {
            if let Some(segment) = type_path.path.segments.last() {
                return match segment.ident.to_string().as_str() {
                    "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64"
                    | "usize" | "f32" | "f64" => "number",
                    "bool" => "boolean",
                    "String" | "str" => "string",
        
                    _ => "any",
                };
            }
        }

        "any"
    }

    fn transpile_expr(&mut self, expr: &syn::Expr) -> String {
        match expr {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Str(s) => format!("\"{}\"", s.value()),
                syn::Lit::Int(i) => i.base10_digits().to_string(),
                syn::Lit::Float(f) => f.base10_digits().to_string(),
                syn::Lit::Bool(b) => b.value.to_string(),
                _ => "nil".to_string(),
            },
            
            syn::Expr::Path(path) => path
                .path
                .get_ident()
                .map_or("nil".to_string(), |ident| ident.to_string()),
            
            syn::Expr::Binary(bin_expr) => {
                let left = self.transpile_expr(&bin_expr.left);
                let right = self.transpile_expr(&bin_expr.right);
                let op = match bin_expr.op {
                    syn::BinOp::Add(_) => "+",
                    syn::BinOp::Sub(_) => "-",
                    syn::BinOp::Mul(_) => "*",
                    syn::BinOp::Div(_) => "/",
                    syn::BinOp::Eq(_) => "==",
                    syn::BinOp::Ne(_) => "~=",
                    syn::BinOp::Lt(_) => "<",
                    syn::BinOp::Le(_) => "<=",
                    syn::BinOp::Gt(_) => ">",
                    syn::BinOp::Ge(_) => ">=",
                    syn::BinOp::And(_) => "and",
                    syn::BinOp::Or(_) => "or",
                
                    _ => panic!("unimplemented binary op!"),
                };

                format!("{} {} {}", left, op, right)
            }

            syn::Expr::Match(match_expr) => {
                self.visit_expr_match(match_expr);
                String::new()
            }

            _ => "nil".to_string(),
        }
    }

    fn add_local_variable(&mut self, name: &str) {
        self.local_variables.insert(name.to_string());
    }

    fn is_local_variable(&self, name: &str) -> bool {
        self.local_variables.contains(name)
    }

    fn clear_local_variables(&mut self) {
        self.local_variables.clear();
    }

    fn transpile_pat(&mut self, pat: &syn::Pat) -> String {
        match pat {
            syn::Pat::Lit(pat_lit) => self.transpile_expr(&syn::Expr::Lit(pat_lit.clone())),
            syn::Pat::Range(pat_range) => {
                let start = self.transpile_expr(pat_range.start.as_ref().unwrap());
                let end = self.transpile_expr(pat_range.end.as_ref().unwrap());
                match &pat_range.limits {
                    syn::RangeLimits::HalfOpen(_) => format!("{} <= x and x < {}", start, end),
                    syn::RangeLimits::Closed(_) => format!("{} <= x and x <= {}", start, end),
                }
            }

            syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
            syn::Pat::Wild(_) => "_".to_string(),
            syn::Pat::Or(pat_or) => pat_or
                .cases
                .iter()
                .map(|pat| self.transpile_pat(pat))
                .collect::<Vec<_>>()
                .join(" or "),

            _ => panic!("Unsupported pattern type"),
        }
    }
}

fn parse_range(expr: &syn::Expr) -> Option<(Option<String>, Option<String>, bool)> {
    if let syn::Expr::Range(syn::ExprRange {
        start, end, limits, ..
    }) = expr
    {
        let start = start.as_ref().and_then(|expr| extract_literal_value(expr));
        let end = end.as_ref().and_then(|expr| extract_literal_value(expr));
        let inclusive = matches!(limits, syn::RangeLimits::Closed(_));

        Some((start, end, inclusive))
    } else {
        None
    }
}

fn extract_literal_value(expr: &syn::Expr) -> Option<String> {
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(lit_int),
        ..
    }) = expr
    {
        Some(lit_int.base10_digits().to_string())
    } else {
        None
    }
}

impl<'ast, 'a> Visit<'ast> for LuauTranspiler<'a> {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.in_function = true;
        self.clear_local_variables();

        let fn_name = i.sig.ident.to_string();
        let params: Vec<(String, &str)> = i
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        Some((pat_ident.ident.to_string(), self.map_type(&pat_type.ty)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let return_type = if let syn::ReturnType::Type(_, ty) = &i.sig.output {
            Some(self.map_type(ty))
        } else {
            None
        };

        let params_str = params
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .collect::<Vec<_>>()
            .join(", ");

        let ret_type_str = return_type.map_or(String::new(), |ty| format!(": {}", ty));

        self.add_line(&format!(
            "function {}({}){}",
            fn_name, params_str, ret_type_str
        ));

        self.indent_manager.increase();

        for stmt in &i.block.stmts {
            self.visit_stmt(stmt);
        }

        self.indent_manager.decrease();
        self.add_line("end");

        self.in_function = false;
    }

    fn visit_local(&mut self, i: &'ast syn::Local) {
        if let syn::Pat::Type(pat_type) = &i.pat {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                let var_name = pat_ident.ident.to_string();
                let var_value = i.init.as_ref().map(|init| self.transpile_expr(&init.expr));
                let var_type = self.map_type(&pat_type.ty);
                let var_type_str = var_type.to_string();

                self.add_local_variable(&var_name);
                self.add_line(&format!(
                    "local {}: {}{}",
                    var_name,
                    var_type_str,
                    var_value.map_or(String::new(), |v| format!(" = {}", v))
                ));
            }
        } else if let syn::Pat::Ident(pat_ident) = &i.pat {
            let var_name = pat_ident.ident.to_string();

            if let Some(init) = &i.init {
                if let syn::Expr::Match(expr_match) = &*init.expr {
                    let match_expr_str = self.transpile_expr(&expr_match.expr);

                    self.add_line(&format!("local {} = nil", var_name));

                    let mut is_first = true;
                    for arm in &expr_match.arms {
                        if arm.guard.is_some() {
                            panic!("Guard clauses are not yet supported");
                        }
                        
                        let condition = match &arm.pat {
                            syn::Pat::Lit(_) | syn::Pat::Ident(_) => {
                                format!("{} == {}", match_expr_str, self.transpile_pat(&arm.pat))
                            }
                        
                            syn::Pat::Range(_) => self.transpile_pat(&arm.pat),
                            syn::Pat::Or(pat_or) => pat_or
                                .cases
                                .iter()
                                .map(|pat| {
                                    format!("{} == {}", match_expr_str, self.transpile_pat(pat))
                                })
                                .collect::<Vec<_>>()
                                .join(" or "),
                        
                            syn::Pat::Wild(_) => "_".to_string(),
                        
                            _ => panic!("Unsupported pattern type in match arm"),
                        };

                        if condition == "_" {
                            self.add_line("else");
                        } else if is_first {
                            self.add_line(&format!("if {} then", condition));
                            is_first = false;
                        } else {
                            self.add_line(&format!("elseif {} then", condition));
                        }
                        
                        self.indent_manager.increase();
                        
                        let transpiled_body = self.transpile_expr(&arm.body);
                        
                        self.add_line(&format!("{} = {}", var_name, transpiled_body));
                        self.indent_manager.decrease();
                    }

                    self.add_line("end");
                }
            } else {
                let var_value = i.init.as_ref().map(|init| self.transpile_expr(&init.expr));

                self.add_local_variable(&var_name);
                self.add_line(&format!(
                    "local {}{}",
                    var_name,
                    var_value.map_or(String::new(), |v| format!(" = {}", v))
                ));
            }
        }
    }

    fn visit_expr_assign(&mut self, i: &'ast syn::ExprAssign) {
        if let syn::Expr::Path(path) = &*i.left {
            if let Some(ident) = path.path.get_ident() {
                let var_name = ident.to_string();
                let value = self.transpile_expr(&i.right);

                if self.in_function && !self.is_local_variable(&var_name) {
                    self.add_line(&format!("local {} = {}", var_name, value));
                    self.add_local_variable(&var_name);
                } else {
                    self.add_line(&format!("{} = {}", var_name, value));
                }
            }
        }
    }

    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        let condition = self.transpile_expr(&i.cond);

        self.add_line(&format!("if {} then", condition));
        self.indent_manager.increase();

        for stmt in &i.then_branch.stmts {
            self.visit_stmt(stmt);
        }

        self.indent_manager.decrease();

        if let Some((_, else_branch)) = &i.else_branch {
            match &**else_branch {
                syn::Expr::If(else_if) => {
                    let elseif_condition = self.transpile_expr(&else_if.cond);
                    
                    self.add_line(&format!("elseif {} then", elseif_condition));
                    self.indent_manager.increase();
                    
                    for stmt in &else_if.then_branch.stmts {
                        self.visit_stmt(stmt);
                    }
                    
                    self.indent_manager.decrease();
                    
                    if let Some((_, nested_else_branch)) = &else_if.else_branch {
                        match &**nested_else_branch {
                            syn::Expr::If(nested_else_if) => self.visit_expr_if(nested_else_if),
                    
                            _ => {
                                self.add_line("else");
                                self.indent_manager.increase();
                                self.visit_expr(nested_else_branch);
                                self.indent_manager.decrease();
                                self.add_line("end");
                            }
                        }
                    } else {
                        self.add_line("end");
                    }
                }
                
                _ => {
                    self.add_line("else");
                    self.indent_manager.increase();

                    if let syn::Expr::Block(block) = &**else_branch {
                        for stmt in &block.block.stmts {
                            self.visit_stmt(stmt);
                        }
                    } else {
                        self.visit_expr(else_branch);
                    }
                    
                    self.indent_manager.decrease();
                    self.add_line("end");
                }
            }
        } else {
            self.add_line("end");
        }
    }

    fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
        let loop_var = if let syn::Pat::Ident(pat_ident) = &*i.pat {
            pat_ident.ident.to_string()
        } else {
            panic!("Unsupported loop variable pattern");
        };
        
        if let Some((start, end, inclusive)) = parse_range(&i.expr) {
            let end_val = if inclusive {
                end
            } else {
                end.map(|v| format!("{} - 1", v))
            };
        
            self.add_line(&format!(
                "for {} = {}, {} do",
                loop_var,
                start.unwrap_or_else(|| "0".to_string()),
                end_val.unwrap_or_else(|| "math.huge".to_string())
            ));
        } else {
            panic!("Unsupported iterator expression");
        }
        
        self.indent_manager.increase();
        
        for stmt in &i.body.stmts {
            self.visit_stmt(stmt);
        }
        
        self.indent_manager.decrease();
        self.add_line("end");
    }

    fn visit_expr_while(&mut self, i: &'ast syn::ExprWhile) {
        let condition = self.transpile_expr(&i.cond);
        
        self.add_line(&format!("while {} do", condition));
        self.indent_manager.increase();
        
        for stmt in &i.body.stmts {
            self.visit_stmt(stmt);
        }
        
        self.indent_manager.decrease();
        self.add_line("end");
    }

    fn visit_expr_loop(&mut self, i: &'ast syn::ExprLoop) {
        self.add_line("while true do");
        self.indent_manager.increase();
        
        for stmt in &i.body.stmts {
            self.visit_stmt(stmt);
        }
        
        self.indent_manager.decrease();
        self.add_line("end");
    }
}
