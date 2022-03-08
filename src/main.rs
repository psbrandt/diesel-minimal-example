struct MyInsertStatement<T: diesel::Table, M: diesel::Insertable<T>>(diesel::query_builder::InsertStatement<T, M::Values>);

impl<T: diesel::Table, M: diesel::Insertable<T>> MyInsertStatement<T, M> {
  pub fn sql_string(&self) -> String {
    diesel::debug_query::<diesel::pg::Pg, _>(&self.0).to_string()
  }
}

fn main() {
    println!("Hello, world!");
}
