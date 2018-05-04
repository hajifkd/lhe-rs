use std::io::Read;

use xml::reader::{EventReader, XmlEvent};
use event::{Event, Particle};

///
/// A reader for Les Houches Event Files.
///
pub struct LHEReader<R: Read> {
    reader: EventReader<R>,
}

pub struct Events<R: Read> {
    lhe_reader: LHEReader<R>,
}

macro_rules! construct {
    ($struct_name: path, $line_split: expr, $($field_names: ident),*; $($e: tt)*) => {{
        let mut split = $line_split;
        $struct_name {
            $(
                $field_names: {
                    let el = split.next().expect("Invalid element numbers");
                    el.parse().expect("Invalid character")
                }
            ),*,
            $($e)*
        }
    }};
}

pub fn parse_particle_line(line: &str) -> Particle {
    construct!(
        Particle,
        line.split(char::is_whitespace).filter(|s| !s.is_empty()),
        id,
        status,
        mother_id0,
        mother_id1,
        color0,
        color1,
        px,
        py,
        pz,
        e,
        m,
        lifetime,
        spin;
    )
}

pub fn parse_event_lines(lines: &str) -> Event {
    let mut lines = lines
        .split('\n')
        .filter(|s| !s.chars().all(char::is_whitespace));

    let header = lines.next().expect("Invalid LHEF format");

    construct!(
        Event,
        header.split(char::is_whitespace).filter(|s| !s.is_empty()),
        n,
        id,
        event_weight,
        scale,
        alpha_qed,
        alpha_qcd;
        particles: lines.map(parse_particle_line).collect()
    )
}

impl<R: Read> LHEReader<R> {
    /// Create a new reader base on an existing stream `read`.
    pub fn new(read: R) -> LHEReader<R> {
        LHEReader {
            reader: EventReader::new(read),
        }
    }
}

impl<R: Read> IntoIterator for LHEReader<R> {
    type Item = Event;
    type IntoIter = Events<R>;
    fn into_iter(self) -> Events<R> {
        Events { lhe_reader: self }
    }
}

impl<R: Read> Iterator for Events<R> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        loop {
            match self.lhe_reader.reader.next() {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    if name.local_name == "event" {
                        if let Ok(XmlEvent::Characters(lines)) = self.lhe_reader.reader.next() {
                            return Some(parse_event_lines(&lines));
                        } else {
                            panic!("Invalid LHE file.");
                        }
                    }
                }

                Err(e) => {
                    panic!("{:?}", e);
                }

                Ok(XmlEvent::EndDocument) => {
                    return None;
                }

                _ => {}
            }
        }
    }
}
