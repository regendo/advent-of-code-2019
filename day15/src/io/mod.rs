use std::{io, rc::Rc};

use crate::GameState;

struct AI {
	game_state: Rc<GameState>,
}

impl io::Read for AI {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		unimplemented!()
	}
}

impl io::BufRead for AI {
	fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
		unimplemented!()
	}

	fn consume(&mut self, amt: usize) {
		unimplemented!()
	}

	fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
		*buf = "1\n".to_string();
		return Ok(2);
		// todo!()
	}
}

struct Output {
	game_state: Rc<GameState>,
}

impl io::Write for Output {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		println!(
			"{}",
			String::from_utf8(buf.to_owned())
				.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?
		);
		todo!()
	}

	fn flush(&mut self) -> std::io::Result<()> {
		unimplemented!()
	}
}
