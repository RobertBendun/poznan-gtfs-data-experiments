#![allow(clippy::print_with_newline)]
use anyhow::Context;
use std::collections::HashSet;

use gtfs_structures::{Gtfs, Trip};

/// Parses dates from filenames in yyyymmdd_yyyymmdd.zip format
fn dates_from_filename(mut name: &str) -> (String, String) {
    if let Some(end_of_dir_part) = name.rfind('/') {
        name = &name[end_of_dir_part + 1..];
    }
    if let Some(extension_start) = name.rfind('/') {
        name = &name[..extension_start];
    }
    assert!(name.get(8..9) == Some("_"));

    (
        format!("{}-{}-{}", &name[0..4], &name[4..6], &name[6..8]),
        format!("{}-{}-{}", &name[9..13], &name[13..15], &name[15..17]),
    )
}

#[test]
fn dates_from_filename_test() {
    assert_eq!(
        dates_from_filename("some/path/to/file/20230102_20240203.zip"),
        ("2023-01-02".to_string(), "2024-02-03".to_string())
    );
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next().expect("missing program name");
    let gtfs_path = args.next().context("missing argument: gtfs_path")?;
    let data = Gtfs::from_path(&gtfs_path)?;

    let mut trips: Vec<_> = data
        .trips
        .values()
        .filter(|t| trip_time(t).is_some())
        .collect();

    let (file_start_date, file_end_date) = dates_from_filename(&gtfs_path);

    trips.sort_by_key(|trip| std::cmp::Reverse(trip_time(trip)));

    let mut seen: HashSet<String> = HashSet::new();

    for trip in &trips {
        if seen.insert(trip.route_id.clone()) {
            let route = data.get_route(&trip.route_id).unwrap();
            let time = trip_time(trip).unwrap() as f32 / 60.0;
            let route_short = route.short_name.as_ref().unwrap();
            let route_type = route.route_type;
            let from = trip.stop_times.first().unwrap().stop.name.as_ref().unwrap();
            let to = trip.stop_times.last().unwrap().stop.name.as_ref().unwrap();

            let (days, start_date, end_date) =
                if let Some(calendar) = data.calendar.get(&trip.service_id) {
                    let days: String = [
                        calendar.monday,
                        calendar.tuesday,
                        calendar.wednesday,
                        calendar.thursday,
                        calendar.friday,
                        calendar.saturday,
                        calendar.sunday,
                    ]
                    .iter()
                    .map(|x| if *x { '1' } else { '0' })
                    .collect();

                    let start_date = calendar.start_date.format("%Y-%m-%d").to_string();
                    let end_date = calendar.end_date.format("%Y-%m-%d").to_string();

                    (days, start_date, end_date)
                } else {
                    (
                        String::from("1111111"),
                        file_start_date.clone(),
                        file_end_date.clone(),
                    )
                };

            let start = trip.stop_times.first().unwrap().departure_time.unwrap();
            let mut start_h = start / 60 / 60;
            let start_m = start / 60 % 60;
            if start_h >= 24 {
                start_h -= 24;
            }
            let end = trip.stop_times.last().unwrap().arrival_time.unwrap();
            let mut end_h = end / 60 / 60;
            if end_h >= 24 {
                end_h -= 24;
            }
            let end_m = end / 60 % 60;

            print!("{time:.0}\t");
            print!("{route_short}\t");
            print!("{route_type:?}\t");
            print!("{start_h:02}:{start_m:02}\t");
            print!("{from}\t");
            print!("{end_h:02}:{end_m:02}\t");
            print!("{to}\t");
            print!("{start_date}\t");
            print!("{end_date}\t");
            print!("{days}\n");
        }
    }

    Ok(())
}

fn trip_time(trip: &Trip) -> Option<u32> {
    let departure = trip.stop_times.first().and_then(|x| x.departure_time)?;
    let arrival = trip.stop_times.last().and_then(|x| x.arrival_time)?;
    Some(if departure > arrival {
        (24 * 60 - departure) + arrival
    } else {
        arrival - departure
    })
}
