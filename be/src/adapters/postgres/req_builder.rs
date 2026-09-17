use sqlx::AssertSqlSafe;

#[derive(Debug, strum::EnumString, strum::Display, serde::Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug, strum::Display)]
pub enum CompOp {
    #[strum(to_string = "=")]
    Equal,
    #[strum(to_string = "<=")]
    LowerOrEq,
    #[strum(to_string = "<")]
    Lower,
    #[strum(to_string = ">=")]
    GreaterOrEq,
    #[strum(to_string = ">")]
    Greater,
    #[strum(to_string = "!=")]
    Different,
}

#[derive(Default, Debug)]
pub struct ReqBuilder {
    where_clauses: String,
    assignments: String,
    order_by: String,
    limit: String,
    pub args: sqlx::postgres::PgArguments,
    bind_idx: usize,
}

impl ReqBuilder {
    pub fn r#where<T>(
        &mut self,
        field_name: &str,
        op: CompOp,
        value: T,
    ) -> Result<(), Box<dyn std::error::Error + 'static + Send + Sync>>
    where
        for<'v> T: sqlx::Encode<'v, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
    {
        use sqlx::Arguments;
        self.args.add(value)?;
        self.bind_idx += 1;
        let head = if self.where_clauses.is_empty() {
            "WHERE"
        } else {
            " AND"
        };
        self.where_clauses
            .push_str(&format!("{head} {field_name} {} ${}", op, self.bind_idx));

        Ok(())
    }

    pub fn tuple_where<T1, T2>(
        &mut self,
        field1: &str,
        field2: &str,
        op: CompOp,
        value1: T1,
        value2: T2,
    ) -> Result<(), Box<dyn std::error::Error + 'static + Send + Sync>>
    where
        for<'v> T1: sqlx::Encode<'v, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
        for<'v> T2: sqlx::Encode<'v, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
    {
        use sqlx::Arguments;

        self.args.add(value1)?;
        self.bind_idx += 1;
        let idx1 = self.bind_idx;

        self.args.add(value2)?;
        self.bind_idx += 1;
        let idx2 = self.bind_idx;

        let head = if self.where_clauses.is_empty() {
            "WHERE"
        } else {
            " AND"
        };

        self.where_clauses.push_str(&format!(
            "{head} ({field1}, {field2}) {} (${}, ${})",
            op, idx1, idx2
        ));

        Ok(())
    }

    pub fn assign<T>(
        &mut self,
        field_name: &str,
        value: T,
    ) -> Result<(), Box<dyn std::error::Error + 'static + Send + Sync>>
    where
        for<'v> T: sqlx::Encode<'v, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
    {
        use sqlx::Arguments;
        self.args.add(value)?;
        self.bind_idx += 1;
        let head = if self.assignments.is_empty() {
            "SET "
        } else {
            ",\n"
        };
        self.assignments
            .push_str(&format!("{head}{field_name} = ${}", self.bind_idx));
        Ok(())
    }

    pub fn order_by(&mut self, field_name: &str, order: Order) {
        let head = if self.order_by.is_empty() {
            "ORDER BY "
        } else {
            ", "
        };
        self.order_by
            .push_str(&format!("{head}{field_name} {order}"));
    }

    pub fn limit(
        &mut self,
        limit: u8,
    ) -> Result<(), Box<dyn std::error::Error + 'static + Send + Sync>> {
        use sqlx::Arguments;
        self.args.add(limit as i64)?;
        self.bind_idx += 1;
        self.limit = format!("LIMIT ${}", self.bind_idx);
        Ok(())
    }

    pub fn build(&self, prefix: &str, sufix: &str) -> AssertSqlSafe<String> {
        AssertSqlSafe(format!(
            "{} {} {}",
            prefix,
            [
                self.assignments.as_str(),
                self.where_clauses.as_str(),
                self.order_by.as_str(),
                self.limit.as_str(),
            ]
            .into_iter()
            .filter(|component| !component.is_empty())
            .collect::<Vec<_>>()
            .join(" "),
            sufix
        ))
    }
}
