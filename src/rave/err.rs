// Represents a error at a specific source location
pub struct SourceError<T> {
	pub loc: ::loc::Loc,
	pub err: T
}

pub type SourceResult<R, T> = Result<R, SourceError<T>>;