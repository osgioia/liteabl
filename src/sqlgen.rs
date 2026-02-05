use crate::ast::{Statement, Expr, Op};

pub fn statement_to_sql(stmt: &Statement) -> Option<String> {
	match stmt {
		Statement::ForEach { table, where_clause, body } => {
			for s in body {
				if let Statement::Display { fields } = s {
					if fields.iter().all(|f| f.starts_with('"') && f.ends_with('"')) {
						let msg = fields.iter().map(|f| f.trim_matches('"')).collect::<Vec<_>>().join(" ");
						println!("{}", msg);
						return None;
					} else {
						let campos = if fields.is_empty() { "*".to_string() } else { fields.join(", ") };
						let mut sql = format!("SELECT {} FROM {}", campos, table);
						if let Some(expr) = where_clause {
							if let Some(w) = expr_to_sql(expr) {
								sql.push_str(&format!(" WHERE {}", w));
							}
						}
						return Some(sql);
					}
				}
			}
			let mut sql = format!("SELECT * FROM {}", table);
			if let Some(expr) = where_clause {
				if let Some(w) = expr_to_sql(expr) {
					sql.push_str(&format!(" WHERE {}", w));
				}
			}
			Some(sql)
		}
		Statement::FindFirst { table, where_clause } => {
			let mut sql = format!("SELECT * FROM {}", table);
			if let Some(expr) = where_clause {
				if let Some(w) = expr_to_sql(expr) {
					sql.push_str(&format!(" WHERE {}", w));
				}
			}
			sql.push_str(" LIMIT 1");
			Some(sql)
		}
		Statement::Create { table } => {
			Some(format!("CREATE TABLE IF NOT EXISTS {} ();", table))
		}
		Statement::Delete { table } => {
			Some(format!("DELETE FROM {}", table))
		}
		Statement::Display { fields } => {
			if fields.iter().all(|f| f.starts_with('"') && f.ends_with('"')) {
				let msg = fields.iter().map(|f| f.trim_matches('"')).collect::<Vec<_>>().join(" ");
				println!("{}", msg);
				None
			} else {
				None
			}
		}
	}
}

pub fn expr_to_sql(expr: &Expr) -> Option<String> {
	match expr {
		Expr::Identifier(s) => Some(s.clone()),
		Expr::String(s) => Some(format!("'{}'", s)),
		Expr::Number(n) => Some(n.to_string()),
		Expr::BinOp { left, op, right } => {
			let l = expr_to_sql(left)?;
			let r = expr_to_sql(right)?;
			match op {
				Op::Eq => Some(format!("{} = {}", l, r)),
			}
		}
	}
}
