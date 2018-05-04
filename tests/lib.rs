extern crate lhe;

#[test]
fn test_parse_particle_line() {
    let line = "        1 2    3    4  5    6 +7e+00 +8e+00 +9e+00 1e+01 11e+00 12e+00 13e+00";
    let particle = lhe::reader::parse_particle_line(line);
    assert_eq!(
        particle,
        lhe::event::Particle {
            id: 1,
            status: 2,
            mother_id0: 3,
            mother_id1: 4,
            color0: 5,
            color1: 6,
            px: 7.0,
            py: 8.0,
            pz: 9.0,
            e: 10.0,
            m: 11.0,
            lifetime: 12.0,
            spin: 13.0,
        }
    );
}

#[test]
fn test_parse_event_lines() {
    let lines = r#"
 1      2 3.0 4.0 5.0 6.0
        1 2    3    4  5    6 +7e+00 +8e+00 +9e+00 1e+01 11e+00 12e+00 13e+00
"#;
    let event = lhe::reader::parse_event_lines(lines);
    assert_eq!(
        event,
        lhe::event::Event {
            n: 1,
            id: 2,
            event_weight: 3.0,
            scale: 4.0,
            alpha_qed: 5.0,
            alpha_qcd: 6.0,
            particles: vec![
                lhe::event::Particle {
                    id: 1,
                    status: 2,
                    mother_id0: 3,
                    mother_id1: 4,
                    color0: 5,
                    color1: 6,
                    px: 7.0,
                    py: 8.0,
                    pz: 9.0,
                    e: 10.0,
                    m: 11.0,
                    lifetime: 12.0,
                    spin: 13.0,
                },
            ],
        }
    );
}

#[test]
fn test_reader() {
    let xml = r#"<LesHouchesEvents version="3.0">
<event>
 1      2 3.0 4.0 5.0 6.0
        1 2    3    4  5    6 +7e+00 +8e+00 +9e+00 1e+01 11e+00 12e+00 13e+00
    <mgrwt>
<rscale>  0 0.22766914E+03</rscale>
<asrwt>0</asrwt>
<pdfrwt beam="1">  1        2 0.32595403E+00 0.22766917E+03</pdfrwt>
<pdfrwt beam="2">  1       -2 0.16747995E+00 0.22766917E+03</pdfrwt>
<totfact> 0.24068310E+01</totfact>
</mgrwt>
</event>
<event>
 1      20 30.0 40.0 50.0 60.0
        10 20    30    40  50    60 +70e+00 +80e+00 +90e+00 10e+01 110e+00 120e+00 130e+00
<mgrwt>
<rscale>  0 0.22766914E+03</rscale>
<asrwt>0</asrwt>
<pdfrwt beam="1">  1        2 0.32595403E+00 0.22766917E+03</pdfrwt>
<pdfrwt beam="2">  1       -2 0.16747995E+00 0.22766917E+03</pdfrwt>
<totfact> 0.24068310E+01</totfact>
</mgrwt>
</event>
</LesHouchesEvents>
"#;

    let lhe_reader = lhe::reader::LHEReader::new(xml.as_bytes());
    let events = lhe_reader.into_iter().collect::<Vec<_>>();
    assert_eq!(
        events,
        vec![
            lhe::event::Event {
                n: 1,
                id: 2,
                event_weight: 3.0,
                scale: 4.0,
                alpha_qed: 5.0,
                alpha_qcd: 6.0,
                particles: vec![
                    lhe::event::Particle {
                        id: 1,
                        status: 2,
                        mother_id0: 3,
                        mother_id1: 4,
                        color0: 5,
                        color1: 6,
                        px: 7.0,
                        py: 8.0,
                        pz: 9.0,
                        e: 10.0,
                        m: 11.0,
                        lifetime: 12.0,
                        spin: 13.0,
                    },
                ],
            },
            lhe::event::Event {
                n: 1,
                id: 20,
                event_weight: 30.0,
                scale: 40.0,
                alpha_qed: 50.0,
                alpha_qcd: 60.0,
                particles: vec![
                    lhe::event::Particle {
                        id: 10,
                        status: 20,
                        mother_id0: 30,
                        mother_id1: 40,
                        color0: 50,
                        color1: 60,
                        px: 70.0,
                        py: 80.0,
                        pz: 90.0,
                        e: 100.0,
                        m: 110.0,
                        lifetime: 120.0,
                        spin: 130.0,
                    },
                ],
            },
        ]
    )
}
