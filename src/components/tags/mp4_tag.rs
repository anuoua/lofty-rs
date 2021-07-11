use crate::{
	Album, AnyTag, AudioTag, AudioTagEdit, AudioTagWrite, LoftyError, MimeType, Picture,
	PictureType, Result, TagType, ToAny, ToAnyTag,
};

use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Seek};

use lofty_attr::impl_tag;
pub use mp4ameta::{Fourcc, Tag as Mp4InnerTag};

#[impl_tag(Mp4InnerTag, TagType::Mp4)]
pub struct Mp4Tag {}

impl Mp4Tag {
	#[allow(missing_docs)]
	#[allow(clippy::missing_errors_doc)]
	pub fn read_from<R>(reader: &mut R) -> Result<Self>
	where
		R: Read + Seek,
	{
		Ok(Self {
			inner: Mp4InnerTag::read_from(reader)?,
		})
	}
}

impl std::convert::TryFrom<mp4ameta::Data> for Picture {
	type Error = LoftyError;

	fn try_from(inp: mp4ameta::Data) -> Result<Self> {
		Ok(match inp {
			mp4ameta::Data::Png(data) => Self {
				pic_type: PictureType::Other,
				mime_type: MimeType::Png,
				description: None,
				data: Cow::from(data),
			},
			mp4ameta::Data::Jpeg(data) => Self {
				pic_type: PictureType::Other,
				mime_type: MimeType::Jpeg,
				description: None,
				data: Cow::from(data),
			},
			mp4ameta::Data::Bmp(data) => Self {
				pic_type: PictureType::Other,
				mime_type: MimeType::Bmp,
				description: None,
				data: Cow::from(data),
			},
			_ => return Err(LoftyError::NotAPicture),
		})
	}
}

impl AudioTagEdit for Mp4Tag {
	fn title(&self) -> Option<&str> {
		self.inner.title()
	}
	fn set_title(&mut self, title: &str) {
		self.inner.set_title(title)
	}
	fn remove_title(&mut self) {
		self.inner.remove_title();
	}

	fn artist(&self) -> Option<&str> {
		self.inner.artist()
	}
	fn set_artist(&mut self, artist: &str) {
		self.inner.set_artist(artist)
	}
	fn remove_artist(&mut self) {
		self.inner.remove_artists();
	}

	fn year(&self) -> Option<i32> {
		self.inner.year().and_then(|x| str::parse(x).ok())
	}
	fn set_year(&mut self, year: i32) {
		self.inner.set_year(year.to_string())
	}
	fn remove_year(&mut self) {
		self.inner.remove_year()
	}

	fn copyright(&self) -> Option<&str> {
		self.inner.copyright()
	}
	fn set_copyright(&mut self, copyright: &str) {
		self.inner.set_copyright(copyright)
	}
	fn remove_copyright(&mut self) {
		self.inner.remove_copyright()
	}

	fn genre(&self) -> Option<&str> {
		self.inner.genre()
	}
	fn set_genre(&mut self, genre: &str) {
		self.inner.set_genre(genre)
	}
	fn remove_genre(&mut self) {
		self.inner.remove_genres()
	}

	fn lyrics(&self) -> Option<&str> {
		self.inner.lyrics()
	}
	fn set_lyrics(&mut self, lyrics: &str) {
		self.inner.set_lyrics(lyrics)
	}
	fn remove_lyrics(&mut self) {
		self.inner.remove_lyrics()
	}

	fn bpm(&self) -> Option<u16> {
		self.inner.bpm()
	}
	fn set_bpm(&mut self, bpm: u16) {
		self.inner.set_bpm(bpm)
	}
	fn remove_bpm(&mut self) {
		self.inner.remove_bpm()
	}

	fn lyricist(&self) -> Option<&str> {
		self.inner.lyricist()
	}
	fn set_lyricist(&mut self, lyricist: &str) {
		self.inner.set_lyricist(lyricist);
	}
	fn remove_lyricist(&mut self) {
		self.inner.remove_lyricists()
	}

	fn encoder(&self) -> Option<&str> {
		self.inner.encoder()
	}
	fn set_encoder(&mut self, encoder: &str) {
		self.inner.set_encoder(encoder)
	}
	fn remove_encoder(&mut self) {
		self.inner.remove_encoder()
	}

	fn album_title(&self) -> Option<&str> {
		self.inner.album()
	}
	fn set_album_title(&mut self, title: &str) {
		self.inner.set_album(title)
	}
	fn remove_album_title(&mut self) {
		self.inner.remove_album();
	}

	fn album_artist(&self) -> Option<&str> {
		self.inner.album_artist()
	}
	fn set_album_artist(&mut self, album_artist: &str) {
		self.inner.set_album_artist(album_artist)
	}
	fn remove_album_artist(&mut self) {
		self.inner.remove_album_artists();
	}

	fn front_cover(&self) -> Option<Picture> {
		if let Some(picture) = &self.inner.artwork() {
			return match picture {
				mp4ameta::Data::Jpeg(d) => Some(Picture {
					pic_type: PictureType::Other,
					mime_type: MimeType::Jpeg,
					description: None,
					data: Cow::from(d.clone()),
				}),
				mp4ameta::Data::Png(d) => Some(Picture {
					pic_type: PictureType::Other,
					mime_type: MimeType::Png,
					description: None,
					data: Cow::from(d.clone()),
				}),
				mp4ameta::Data::Bmp(d) => Some(Picture {
					pic_type: PictureType::Other,
					mime_type: MimeType::Bmp,
					description: None,
					data: Cow::from(d.clone()),
				}),
				_ => None,
			};
		}

		None
	}
	fn set_front_cover(&mut self, cover: Picture) {
		match cover.mime_type {
			MimeType::Png => self
				.inner
				.add_artwork(mp4ameta::Data::Png(Vec::from(cover.data))),
			MimeType::Jpeg => self
				.inner
				.add_artwork(mp4ameta::Data::Jpeg(Vec::from(cover.data))),
			MimeType::Bmp => self
				.inner
				.add_artwork(mp4ameta::Data::Bmp(Vec::from(cover.data))),
			_ => {},
		}
	}
	fn remove_front_cover(&mut self) {
		self.inner.remove_artwork();
	}

	fn back_cover(&self) -> Option<Picture> {
		self.front_cover()
	}
	fn set_back_cover(&mut self, cover: Picture) {
		self.set_front_cover(cover)
	}

	fn pictures(&self) -> Option<Cow<'static, [Picture]>> {
		let mut pictures = Vec::new();

		for art in self.inner.artworks() {
			let info = match art {
				mp4ameta::Data::Png(d) => Some((MimeType::Png, d.clone())),
				mp4ameta::Data::Jpeg(d) => Some((MimeType::Jpeg, d.clone())),
				mp4ameta::Data::Bmp(d) => Some((MimeType::Bmp, d.clone())),
				_ => None,
			};

			if let Some((mime_type, data)) = info {
				pictures.push(Picture {
					pic_type: PictureType::Other,
					mime_type,
					description: None,
					data: Cow::from(data),
				})
			}
		}

		(!(pictures.is_empty())).then(|| Cow::from(pictures))
	}
	fn set_pictures(&mut self, pictures: Vec<Picture>) {
		self.remove_pictures();

		for p in pictures {
			self.set_front_cover(p)
		}
	}
	fn remove_pictures(&mut self) {
		self.inner.remove_artwork()
	}

	fn track_number(&self) -> Option<u32> {
		self.inner.track_number().map(u32::from)
	}
	fn set_track_number(&mut self, track: u32) {
		self.inner.set_track_number(track as u16);
	}
	fn remove_track_number(&mut self) {
		self.inner.remove_track_number();
	}

	fn total_tracks(&self) -> Option<u32> {
		self.inner.total_tracks().map(u32::from)
	}
	fn set_total_tracks(&mut self, total_track: u32) {
		self.inner.set_total_tracks(total_track as u16);
	}
	fn remove_total_tracks(&mut self) {
		self.inner.remove_total_tracks();
	}

	fn disc_number(&self) -> Option<u32> {
		self.inner.disc_number().map(u32::from)
	}
	fn set_disc_number(&mut self, disc_number: u32) {
		self.inner.set_disc_number(disc_number as u16)
	}
	fn remove_disc_number(&mut self) {
		self.inner.remove_disc_number();
	}

	fn total_discs(&self) -> Option<u32> {
		self.inner.total_discs().map(u32::from)
	}
	fn set_total_discs(&mut self, total_discs: u32) {
		self.inner.set_total_discs(total_discs as u16)
	}
	fn remove_total_discs(&mut self) {
		self.inner.remove_total_discs();
	}

	fn tag_type(&self) -> TagType {
		TagType::Mp4
	}
}

impl AudioTagWrite for Mp4Tag {
	fn write_to(&self, file: &mut File) -> Result<()> {
		self.inner.write_to(file)?;
		Ok(())
	}
	fn write_to_path(&self, path: &str) -> Result<()> {
		self.inner.write_to_path(path)?;
		Ok(())
	}
}
