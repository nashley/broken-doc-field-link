pub struct Test {
	pub foo: u8,
}

impl Test {
	// This link will correctly point to `#structfield.foo`:
	/// Returns [Test::foo]
	pub fn get_foo(&self) -> u8 {
		self.foo
	}
}
