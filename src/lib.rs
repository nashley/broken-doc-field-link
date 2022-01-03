pub struct Test {
	pub foo: u8,
}

impl Test {
	// This link will point to `#method.foo` instead of `#structfield.foo`:
	/// Returns [Test::foo]
	pub fn foo(&self) -> u8 {
		self.foo
	}
}
