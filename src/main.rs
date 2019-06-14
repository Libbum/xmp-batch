extern crate rexiv2;
extern crate failure;
extern crate quick_xml;

use failure::Error;
use quick_xml::events::Event;
use quick_xml::Reader;

fn main() -> Result<(), Error> {
    let mut reader = Reader::from_file("by-nc-sa.xmp")?;
    reader.trim_text(true);

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            // for triggering namespaced events, use this instead:
            // match reader.read_namespaced_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // for namespaced:
                // Ok((ref namespace_value, Event::Start(ref e)))
                match e.name() {
                    b"rdf:RDF" => {println!(
                        "attributes values: {:?}",
                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                    )},
                    b"rdf:Description" => count += 1,
                    _ => (),
                }
            }
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("{}", count);
    println!("{:?}", txt);
    Ok(())
}
