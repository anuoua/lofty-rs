/// Errors that could occur within Lofty.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	/// Unknown file extension.
	#[error("Failed to guess the metadata format based on the file extension.")]
	UnknownFileExtension,

	/// Unable to guess the format
	#[error("No format could be determined from the provided file.")]
	UnknownFormat,
	/// Provided an empty file
	#[error("File contains no data")]
	EmptyFile,
	/// Provided a file with invalid/malformed data
	#[error("File has invalid data")]
	InvalidData,

	/// Unsupported file extension
	#[error("Unsupported format: {0}")]
	UnsupportedFormat(String),
	/// Picture has an unsupported mime type
	#[error("Unsupported mime type: {0}")]
	UnsupportedMimeType(String),

	/// Any error from [`ape`]
	#[error(transparent)]
	ApeTag(#[from] ape::Error),
	/// Any error from [`metaflac`]
	#[error(transparent)]
	FlacTag(#[from] metaflac::Error),
	/// Any error from [`id3`]
	#[error(transparent)]
	Id3Tag(#[from] id3::Error),
	/// Any error from [`mp3_duration`]
	#[cfg(feature = "duration")]
	#[error(transparent)]
	Mp3Duration(#[from] mp3_duration::MP3DurationError),
	/// Any error from [`mp4ameta`]
	#[error(transparent)]
	Mp4Tag(#[from] mp4ameta::Error),
	/// Any error from [`opus_headers`]
	#[error(transparent)]
	OpusTag(#[from] opus_headers::ParseError),
	/// Any error from [`lewton`]
	#[error(transparent)]
	Lewton(#[from] lewton::VorbisError),
	/// Any error from [`ogg`]
	#[error(transparent)]
	Ogg(#[from] ogg::OggReadError),
	/// Errors that arise while reading/writing to wav files
	#[error("{0}")]
	Wav(String),

	/// Failed to convert data to a picture
	#[error("")]
	NotAPicture,

	/// If a string isn't Utf8
	#[error(transparent)]
	Utf8(#[from] std::str::Utf8Error),
	/// Unable to convert bytes to a String
	#[error(transparent)]
	FromUtf8(#[from] std::string::FromUtf8Error),
	/// Represents all cases of `std::io::Error`.
	#[error(transparent)]
	#[allow(clippy::upper_case_acronyms)]
	IO(#[from] std::io::Error),
}

/// Type for the result of tag operations.
pub type Result<T> = std::result::Result<T, Error>;
