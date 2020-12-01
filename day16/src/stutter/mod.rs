use std::iter::Peekable;

#[derive(Clone)]
pub struct Stutter<I>
where
	I: Iterator + Sized + Clone,
	<I as Iterator>::Item: Clone,
{
	inner: Peekable<I>,
	count: u8,
	target: u8,
}

impl<I> Stutter<I>
where
	I: Iterator + Sized + Clone,
	<I as Iterator>::Item: Clone,
{
	/// Create a new iterator that yields each item <repetition> additional times before advancing.
	/// ```
	///# use day16::stutter::Stutter;
	/// let nums = vec![0, 1, 2];
	/// let mut stuttering = Stutter::new(nums.iter(), 1);
	/// assert_eq!(Some(&0), stuttering.next());
	/// assert_eq!(Some(&0), stuttering.next());
	/// assert_eq!(Some(&1), stuttering.next());
	/// assert_eq!(Some(&1), stuttering.next());
	/// assert_eq!(Some(&2), stuttering.next());
	/// assert_eq!(Some(&2), stuttering.next());
	/// assert_eq!(None, stuttering.next());
	/// ```
	pub fn new(other: I, repetitions: u8) -> Self {
		Self {
			inner: other.peekable(),
			count: 0,
			target: repetitions,
		}
	}
}

impl<I> Iterator for Stutter<I>
where
	I: Iterator + Sized + Clone,
	<I as Iterator>::Item: Clone,
{
	type Item = <I as Iterator>::Item;

	fn next(&mut self) -> Option<Self::Item> {
		if self.count >= self.target {
			self.count = 0;
			self.inner.next()
		} else {
			self.count += 1;
			self.inner.peek().cloned()
		}
	}
}
