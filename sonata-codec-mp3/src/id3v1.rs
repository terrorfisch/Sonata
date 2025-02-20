// Sonata
// Copyright (c) 2019 The Sonata Project Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use sonata_core::errors::{Result, unsupported_error};
use sonata_core::tags::{StandardTagKey, Tag};
use sonata_core::io::Bytestream;

static GENRES: &'static [&'static str] = &[
    // Standard Genres as per ID3v1 specificaation
    "Blues",
    "Classic rock",
    "Country",
    "Dance",
    "Disco",
    "Funk",
    "Grunge",
    "Hip-Hop",
    "Jazz",
    "Metal",
    "New Age",
    "Oldies",
    "Other",
    "Pop",
    "Rhythm and Blues",
    "Rap",
    "Reggae",
    "Rock",
    "Techno",
    "Industrial",
    "Alternative",
    "Ska",
    "Death metal",
    "Pranks",
    "Soundtrack",
    "Euro-Techno",
    "Ambient",
    "Trip-Hop",
    "Vocal",
    "Jazz & Funk",
    "Fusion",
    "Trance",
    "Classical",
    "Instrumental",
    "Acid",
    "House",
    "Game",
    "Sound clip",
    "Gospel",
    "Noise",
    "Alternative Rock",
    "Bass",
    "Soul",
    "Punk",
    "Space",
    "Meditative",
    "Instrumental Pop",
    "Instrumental Rock",
    "Ethnic",
    "Gothic",
    "Darkwave",
    "Techno-Industrial",
    "Electronic",
    "Pop-Folk",
    "Eurodance",
    "Dream",
    "Southern Rock",
    "Comedy",
    "Cult",
    "Gangsta",
    "Top 40",
    "Christian Rap",
    "Pop/Funk",
    "Jungle",
    "Native US",
    "Cabaret",
    "New Wave",
    "Psychedelic",
    "Rave",
    "Show tunes",
    "Trailer",
    "Lo-Fi",
    "Tribal",
    "Acid Punk",
    "Acid Jazz",
    "Polka",
    "Retro",
    "Musical",
    "Rock ’n’ Roll",
    "Hard Rock",
    // Winamp 1.91+ Extended Genres
    "Folk",
    "Folk-Rock",
    "National Folk",
    "Swing",
    "Fast Fusion",
    "Bebop",
    "Latin",
    "Revival",
    "Celtic",
    "Bluegrass",
    "Avantgarde",
    "Gothic Rock",
    "Progressive Rock",
    "Psychedelic Rock",
    "Symphonic Rock",
    "Slow rock",
    "Big Band",
    "Chorus",
    "Easy Listening",
    "Acoustic",
    "Humour",
    "Speech",
    "Chanson",
    "Opera",
    "Chamber music",
    "Sonata",
    "Symphony",
    "Booty bass",
    "Primus",
    "Porn groove",
    "Satire",
    "Slow jam",
    "Club",
    "Tango",
    "Samba",
    "Folklore",
    "Ballad",
    "Power ballad",
    "Rhythmic Soul",
    "Freestyle",
    "Duet",
    "Punk Rock",
    "Drum solo",
    "A cappella",
    "Euro-House",
    "Dance Hall",
    "Goa",
    "Drum & Bass",
    "Club-House",
    "Hardcore Techno",
    "Terror",
    "Indie",
    "BritPop",
    "(133)",
    "Polsk Punk",
    "Beat",
    "Christian Gangsta Rap",
    "Heavy Metal",
    "Black Metal",
    "Crossover",
    "Contemporary Christian",
    "Christian rock",
    "Merengue",
    "Salsa",
    "Thrash Metal",
    "Anime",
    "Jpop",
    "Synthpop",
    // Winamp 5.0+ Extended Genres
    "Abstract",
    "Art Rock",
    "Baroque",
    "Bhangra",
    "Big beat",
    "Breakbeat",
    "Chillout",
    "Downtempo",
    "Dub",
    "EBM",
    "Eclectic",
    "Electro",
    "Electroclash",
    "Emo",
    "Experimental",
    "Garage",
    "Global",
    "IDM",
    "Illbient",
    "Industro-Goth",
    "Jam Band",
    "Krautrock",
    "Leftfield",
    "Lounge",
    "Math Rock",
    "New Romantic",
    "Nu-Breakz",
    "Post-Punk",
    "Post-Rock",
    "Psytrance",
    "Shoegaze",
    "Space Rock",
    "Trop Rock",
    "World Music",
    "Neoclassical",
    "Audiobook",
    "Audio theatre",
    "Neue Deutsche Welle",
    "Podcast",
    "Indie-Rock",
    "G-Funk",
    "Dubstep",
    "Garage Rock",
    "Psybient"
];

pub fn read_id3v1<B: Bytestream>(reader: &mut B) -> Result<Vec<Tag>> {
    // Read the "TAG" header.
    let marker = reader.read_triple_bytes()?;

    if marker != *b"TAG" {
        return unsupported_error("Not an ID3v1 tag.");
    }

    let mut tags = Vec::new();

    let buf = reader.read_boxed_slice_bytes(125)?;

    let title = decode_iso8859_text(&buf[0..30]);
    if !title.is_empty() {
        tags.push(Tag::new(Some(StandardTagKey::TrackTitle), "TITLE", &title));
    }

    let artist = decode_iso8859_text(&buf[30..60]);
    if !artist.is_empty() {
        tags.push(Tag::new(Some(StandardTagKey::Artist), "ARTIST", &artist));
    }

    let album = decode_iso8859_text(&buf[60..90]);
    if !album.is_empty() {
        tags.push(Tag::new(Some(StandardTagKey::Album), "ALBUM", &album));
    }

    let year = decode_iso8859_text(&buf[90..94]);
    if !year.is_empty() {
        tags.push(Tag::new(Some(StandardTagKey::Date), "DATE", &year));
    }

    let comment = if buf[122] == 0 {
        let track = buf[123];
        tags.push(Tag::new(Some(StandardTagKey::TrackNumber), "TRACK", &track.to_string()));

        decode_iso8859_text(&buf[94..122])
    }
    else {
        decode_iso8859_text(&buf[94..124])
    };

    if !comment.is_empty() {
        tags.push(Tag::new(Some(StandardTagKey::Comment), "COMMENT", &comment));
    }

    let genre_idx = buf[124] as usize;

    // Convert the genre index to an actual genre name using the GENRES lookup table. Genre #133 is 
    // an offensive term and is excluded from Sonata.
    if genre_idx < GENRES.len() && genre_idx != 133 {
        tags.push(Tag::new(Some(StandardTagKey::Genre), "GENRE", GENRES[genre_idx]));
    }

    Ok(tags)
}

fn decode_iso8859_text(data: &[u8]) -> String {
    data.iter().filter(|&b| *b > 0x1f).map(|&b| b as char).collect()
}
